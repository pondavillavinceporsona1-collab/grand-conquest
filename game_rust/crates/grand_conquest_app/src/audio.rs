use std::f32::consts::TAU;
use std::num::NonZero;
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Duration;

use grand_conquest_core::settings::GameSettings;
use grand_conquest_core::sim::SoundEvent;
use rodio::buffer::SamplesBuffer;
use rodio::{ChannelCount, DeviceSinkBuilder, MixerDeviceSink, SampleRate, Source};

const SAMPLE_RATE: u32 = 22050;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MusicMode {
    Silent = 0,
    Menu = 1,
    Battle = 2,
    Boss = 3,
}

struct AudioShared {
    music_gain_bits: AtomicU32,
    sfx_gain_bits: AtomicU32,
    music_mode: AtomicU8,
}

impl AudioShared {
    fn new() -> Self {
        Self {
            music_gain_bits: AtomicU32::new(0.38_f32.to_bits()),
            sfx_gain_bits: AtomicU32::new(0.72_f32.to_bits()),
            music_mode: AtomicU8::new(MusicMode::Menu as u8),
        }
    }

    fn music_gain(&self) -> f32 {
        f32::from_bits(self.music_gain_bits.load(Ordering::Relaxed))
    }

    fn sfx_gain(&self) -> f32 {
        f32::from_bits(self.sfx_gain_bits.load(Ordering::Relaxed))
    }

    fn set_music_gain(&self, g: f32) {
        self.music_gain_bits.store(g.clamp(0.0, 1.0).to_bits(), Ordering::Relaxed);
    }

    fn set_sfx_gain(&self, g: f32) {
        self.sfx_gain_bits.store(g.clamp(0.0, 1.0).to_bits(), Ordering::Relaxed);
    }
}

struct ProceduralBgmSource {
    shared: Arc<AudioShared>,
    sample_idx: u64,
    smooth_gain: f32,
    pad_phase_a: f32,
    pad_phase_b: f32,
    pad_phase_c: f32,
    arp_phase: f32,
    drum_phase: f32,
    rng_state: u32,
}

impl ProceduralBgmSource {
    fn new(shared: Arc<AudioShared>) -> Self {
        Self {
            shared,
            sample_idx: 0,
            smooth_gain: 0.0,
            pad_phase_a: 0.0,
            pad_phase_b: 0.0,
            pad_phase_c: 0.0,
            arp_phase: 0.0,
            drum_phase: 0.0,
            rng_state: 0x9E37_79B9,
        }
    }

    #[inline]
    fn next_noise(&mut self) -> f32 {
        let mut x = self.rng_state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng_state = x;
        ((x as f32) / (u32::MAX as f32)) * 2.0 - 1.0
    }
}

impl Iterator for ProceduralBgmSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mode = self.shared.music_mode.load(Ordering::Relaxed);
        let target_gain = if mode == MusicMode::Silent as u8 {
            0.0
        } else {
            self.shared.music_gain() * 0.24
        };
        // Smooth volume ramp avoids clicks on mode/slider changes
        self.smooth_gain += (target_gain - self.smooth_gain) * 0.0025;

        let sr = SAMPLE_RATE as f32;
        let t = (self.sample_idx as f32) / sr;
        self.sample_idx = self.sample_idx.wrapping_add(1);

        if self.smooth_gain < 0.0005 {
            return Some(0.0);
        }

        // Tempo: Menu = slow noble 84 BPM, Battle = 114 BPM, Boss = 132 BPM
        let bpm = match mode {
            1 => 84.0_f32,
            2 => 114.0_f32,
            3 => 132.0_f32,
            _ => 84.0_f32,
        };
        let beat_dur = 60.0 / bpm;
        let step_dur = beat_dur * 0.5; // eighth-note steps
        let total_steps = (t / step_dur) as usize;
        let step_pos = (t % step_dur) / step_dur;

        // 4-bar chord progression (16 beats = 32 eighth steps per cycle)
        let bar = (total_steps / 8) % 4;
        // D Dorian / D Minor progressions (Hz)
        let (bass_hz, triad): (f32, [f32; 4]) = if mode == 1 {
            // Noble & reflective: Dm -> Bbmaj7 -> F -> C
            match bar {
                0 => (146.83, [293.66, 349.23, 440.00, 587.33]),
                1 => (116.54, [233.08, 293.66, 349.23, 523.25]),
                2 => (174.61, [261.63, 349.23, 440.00, 523.25]),
                _ => (130.81, [261.63, 329.63, 392.00, 523.25]),
            }
        } else {
            // Martial / Epic: Dm -> C -> Bb -> A
            match bar {
                0 => (73.42, [146.83, 220.00, 293.66, 349.23]),
                1 => (65.41, [130.81, 196.00, 261.63, 329.63]),
                2 => (58.27, [116.54, 174.61, 233.08, 293.66]),
                _ => (55.00, [110.00, 164.81, 220.00, 277.18]),
            }
        };

        // 1. Warm harmonic pad (chord roots + fifth + subtle octave)
        let inc_a = (bass_hz * TAU) / sr;
        let inc_b = (triad[0] * TAU) / sr;
        let inc_c = (triad[2] * TAU) / sr;
        self.pad_phase_a = (self.pad_phase_a + inc_a) % TAU;
        self.pad_phase_b = (self.pad_phase_b + inc_b) % TAU;
        self.pad_phase_c = (self.pad_phase_c + inc_c) % TAU;
        let swell = 0.65 + 0.35 * (t * 0.45).sin();
        let pad = (self.pad_phase_a.sin() * 0.48
            + (self.pad_phase_a * 2.0).sin() * 0.14
            + self.pad_phase_b.sin() * 0.24
            + self.pad_phase_c.sin() * 0.16)
            * swell;

        // 2. Plucked medieval harp / martial string arpeggio
        let arp_pattern = if mode == 1 {
            [0usize, 1, 2, 3, 2, 1, 0, 2]
        } else {
            [0usize, 2, 1, 3, 2, 3, 1, 2]
        };
        let note_idx = arp_pattern[total_steps % arp_pattern.len()];
        let arp_hz = triad[note_idx] * if mode == 1 { 1.0 } else { 2.0 };
        self.arp_phase = (self.arp_phase + (arp_hz * TAU) / sr) % TAU;
        let pluck_env = (-step_pos * if mode == 1 { 5.2 } else { 7.5 }).exp();
        let pluck = (self.arp_phase.sin() + 0.32 * (self.arp_phase * 2.0).sin())
            * pluck_env
            * if mode == 1 { 0.34 } else { 0.28 };

        // 3. War drums (Battle and Boss modes)
        let mut perc = 0.0_f32;
        if mode >= 2 {
            let beat_in_bar = total_steps % 8;
            let hit_kick = beat_in_bar == 0 || beat_in_bar == 3 || (mode == 3 && beat_in_bar == 6);
            let hit_rim = beat_in_bar == 2 || beat_in_bar == 4 || (mode == 3 && beat_in_bar == 7);
            if hit_kick {
                let pitch = 95.0 - 52.0 * step_pos;
                self.drum_phase = (self.drum_phase + (pitch.max(38.0) * TAU) / sr) % TAU;
                let k_env = (-step_pos * 12.0).exp();
                perc += self.drum_phase.sin() * k_env * 0.55;
            }
            if hit_rim {
                let s_env = (-step_pos * 26.0).exp();
                perc += self.next_noise() * s_env * if mode == 3 { 0.24 } else { 0.15 };
            }
        }

        let mixed = (pad * 0.42 + pluck + perc) * self.smooth_gain;
        Some(mixed.tanh())
    }
}

impl Source for ProceduralBgmSource {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> ChannelCount {
        NonZero::new(1).unwrap()
    }

    fn sample_rate(&self) -> SampleRate {
        NonZero::new(SAMPLE_RATE).unwrap()
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct SoundEngine {
    sink: Option<MixerDeviceSink>,
    shared: Arc<AudioShared>,
    pub muted: bool,
    hit_counter: u32,
}

impl SoundEngine {
    pub fn new() -> Self {
        let shared = Arc::new(AudioShared::new());
        let sink = DeviceSinkBuilder::open_default_sink().ok();
        if let Some(mut s) = sink {
            s.log_on_drop(false);
            let bgm = ProceduralBgmSource::new(Arc::clone(&shared));
            s.mixer().add(bgm);
            return Self {
                sink: Some(s),
                shared,
                muted: false,
                hit_counter: 0,
            };
        }
        Self {
            sink: None,
            shared,
            muted: false,
            hit_counter: 0,
        }
    }

    pub fn sync_settings(&mut self, settings: &GameSettings) {
        self.muted = settings.muted;
        self.shared.set_music_gain(settings.effective_music_gain());
        self.shared.set_sfx_gain(settings.effective_sfx_gain());
    }

    pub fn set_music_mode(&self, mode: MusicMode) {
        self.shared.music_mode.store(mode as u8, Ordering::Relaxed);
    }

    pub fn play(&mut self, event: SoundEvent) {
        self.play_event(event);
    }

    pub fn play_event(&mut self, event: SoundEvent) {
        let gain = self.shared.sfx_gain();
        if gain <= 0.001 || self.sink.is_none() {
            return;
        }
        match event {
            SoundEvent::Deploy => {
                self.play_synth(310.0, 440.0, 0.085, 0.16 * gain, true, 0.25);
            }
            SoundEvent::SwordHit => {
                // Subtle pitch cycling prevents ear fatigue in big clashes
                self.hit_counter = self.hit_counter.wrapping_add(1);
                let base = if self.hit_counter % 2 == 0 { 165.0 } else { 195.0 };
                self.play_synth(base, 72.0, 0.065, 0.11 * gain, true, 0.55);
            }
            SoundEvent::SpellQ => {
                self.play_chord(&[220.0, 330.0, 440.0, 110.0], 0.075, 0.24 * gain, -0.35);
            }
            SoundEvent::SpellE => {
                self.play_arpeggio(&[440.0, 554.37, 659.25, 880.0], 0.08, 0.22 * gain);
            }
            SoundEvent::HealWave => {
                self.play_arpeggio(&[587.33, 739.99, 880.0], 0.065, 0.13 * gain);
            }
            SoundEvent::BossWarning => {
                // Deep twin brass horn
                self.play_chord(&[73.42, 110.0, 146.83], 0.22, 0.28 * gain, -0.08);
            }
            SoundEvent::Victory => {
                self.play_arpeggio(&[392.0, 493.88, 587.33, 783.99], 0.12, 0.24 * gain);
            }
            SoundEvent::Defeat => {
                self.play_arpeggio(&[293.66, 261.63, 220.0, 174.61], 0.14, 0.22 * gain);
            }
        }
    }

    pub fn play_click(&self) {
        let gain = self.shared.sfx_gain();
        if gain <= 0.001 || self.sink.is_none() {
            return;
        }
        self.play_synth(780.0, 420.0, 0.032, 0.08 * gain, false, 0.1);
    }

    pub fn play_coin(&self) {
        let gain = self.shared.sfx_gain();
        if gain <= 0.001 || self.sink.is_none() {
            return;
        }
        self.play_arpeggio(&[1174.66, 1567.98], 0.055, 0.17 * gain);
    }

    pub fn play_unlock(&self) {
        let gain = self.shared.sfx_gain();
        if gain <= 0.001 || self.sink.is_none() {
            return;
        }
        self.play_arpeggio(&[523.25, 659.25, 783.99, 1046.5], 0.08, 0.22 * gain);
    }

    pub fn play_order_horn(&self) {
        let gain = self.shared.sfx_gain();
        if gain <= 0.001 || self.sink.is_none() {
            return;
        }
        self.play_chord(&[196.0, 293.66], 0.11, 0.16 * gain, 0.03);
    }

    fn play_synth(
        &self,
        f0: f32,
        f1: f32,
        dur: f32,
        vol: f32,
        add_harmonic: bool,
        noise_mix: f32,
    ) {
        let Some(ref handle) = self.sink else { return };
        let sr = SAMPLE_RATE;
        let total = (sr as f32 * dur) as usize;
        let mut samples = Vec::with_capacity(total);
        let mut phase = 0.0_f32;
        let mut nstate = 0x517cc1b7_u32;
        for i in 0..total {
            let t = i as f32 / total as f32;
            let freq = f0 + (f1 - f0) * t;
            phase = (phase + (TAU * freq) / sr as f32) % TAU;
            nstate ^= nstate << 13;
            nstate ^= nstate >> 17;
            nstate ^= nstate << 5;
            let noise = ((nstate as f32) / (u32::MAX as f32)) * 2.0 - 1.0;
            let env = (1.0 - t).powf(1.8) * (t * 18.0).min(1.0);
            let mut body = phase.sin();
            if add_harmonic {
                body = body * 0.72 + (phase * 2.0).sin() * 0.28;
            }
            let mixed = body * (1.0 - noise_mix) + noise * noise_mix;
            samples.push((mixed * env * vol).clamp(-0.95, 0.95));
        }
        let buf = SamplesBuffer::new(NonZero::new(1).unwrap(), NonZero::new(sr).unwrap(), samples);
        handle.mixer().add(buf);
    }

    fn play_chord(&self, freqs: &[f32], note_dur: f32, vol: f32, pitch_glide: f32) {
        let Some(ref handle) = self.sink else { return };
        let sr = SAMPLE_RATE;
        let dur = note_dur * 2.6;
        let total = (sr as f32 * dur) as usize;
        let mut samples = Vec::with_capacity(total);
        let scale = 1.0 / (freqs.len().max(1) as f32);
        for i in 0..total {
            let t = i as f32 / total as f32;
            let time_s = i as f32 / sr as f32;
            let env = (1.0 - t).powf(1.6) * (t * 14.0).min(1.0);
            let glide = 1.0 + pitch_glide * t;
            let mut sum = 0.0_f32;
            for &f in freqs {
                let p = TAU * f * glide * time_s;
                sum += p.sin() + 0.25 * (p * 2.0).sin();
            }
            samples.push((sum * scale * env * vol).clamp(-0.95, 0.95));
        }
        let buf = SamplesBuffer::new(NonZero::new(1).unwrap(), NonZero::new(sr).unwrap(), samples);
        handle.mixer().add(buf);
    }

    fn play_arpeggio(&self, freqs: &[f32], step_dur: f32, vol: f32) {
        let Some(ref handle) = self.sink else { return };
        let sr = SAMPLE_RATE;
        let step_samples = (sr as f32 * step_dur) as usize;
        let tail_samples = (sr as f32 * (step_dur * 2.2)) as usize;
        let total = step_samples * freqs.len() + tail_samples;
        let mut samples = vec![0.0_f32; total];
        for (idx, &freq) in freqs.iter().enumerate() {
            let start = idx * step_samples;
            let note_len = step_samples * 3;
            for k in 0..note_len {
                if start + k >= total {
                    break;
                }
                let local_t = k as f32 / note_len as f32;
                let time_s = k as f32 / sr as f32;
                let env = (1.0 - local_t).powf(2.0) * (local_t * 25.0).min(1.0);
                let p = TAU * freq * time_s;
                samples[start + k] += (p.sin() * 0.78 + (p * 2.0).sin() * 0.22) * env * vol;
            }
        }
        for s in &mut samples {
            *s = s.clamp(-0.95, 0.95);
        }
        let buf = SamplesBuffer::new(NonZero::new(1).unwrap(), NonZero::new(sr).unwrap(), samples);
        handle.mixer().add(buf);
    }
}
