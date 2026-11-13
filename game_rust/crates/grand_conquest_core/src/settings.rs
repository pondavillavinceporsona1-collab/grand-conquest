use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HpBarMode {
    Smart,
    Always,
    Minimal,
}

impl HpBarMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Smart => "Smart (Recommended)",
            Self::Always => "Always Visible",
            Self::Minimal => "Minimal (Criticals & Bosses)",
        }
    }

    pub fn desc(self) -> &'static str {
        match self {
            Self::Smart => "Declutters swarms by hiding full-HP common troops while keeping wounded, elites, and bosses crisp.",
            Self::Always => "Renders health bars above every unit on the field regardless of current HP.",
            Self::Minimal => "Shows health bars only for bosses and critically wounded units below 55% HP.",
        }
    }
}

impl Default for HpBarMode {
    fn default() -> Self {
        Self::Smart
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GameSettings {
    pub active_slot: usize,
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub muted: bool,
    pub fullscreen: bool,
    pub screen_shake: bool,
    pub show_damage_numbers: bool,
    pub hp_bar_mode: HpBarMode,
    pub default_speed: f32,
    pub show_minimap: bool,
    pub edge_pan: bool,
    pub auto_pause_unfocused: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            active_slot: 0,
            master_volume: 0.85,
            music_volume: 0.45,
            sfx_volume: 0.85,
            muted: false,
            fullscreen: false,
            screen_shake: true,
            show_damage_numbers: true,
            hp_bar_mode: HpBarMode::Smart,
            default_speed: 1.0,
            show_minimap: true,
            edge_pan: false,
            auto_pause_unfocused: true,
        }
    }
}

impl GameSettings {
    pub fn clamp(&mut self) {
        self.active_slot = self.active_slot.min(2);
        self.master_volume = self.master_volume.clamp(0.0, 1.0);
        self.music_volume = self.music_volume.clamp(0.0, 1.0);
        self.sfx_volume = self.sfx_volume.clamp(0.0, 1.0);
        if ![0.5_f32, 1.0, 2.0].contains(&self.default_speed) {
            self.default_speed = 1.0;
        }
    }

    pub fn effective_music_gain(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            (self.master_volume * self.music_volume).clamp(0.0, 1.0)
        }
    }

    pub fn effective_sfx_gain(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            (self.master_volume * self.sfx_volume).clamp(0.0, 1.0)
        }
    }

    pub fn load_from_path(path: &Path) -> Self {
        let Ok(raw) = std::fs::read_to_string(path) else {
            return Self::default();
        };
        let mut parsed: Self = serde_json::from_str(&raw).unwrap_or_default();
        parsed.clamp();
        parsed
    }

    pub fn save_to_path(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut cloned = self.clone();
        cloned.clamp();
        let json = serde_json::to_string_pretty(&cloned)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }
}
