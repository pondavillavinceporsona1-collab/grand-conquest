use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Instant;

use egui::{Color32, Margin, Stroke, Ui, Vec2, ViewportCommand};
use grand_conquest_core::honors::{
    settings_path, slot_career_path, slot_save_path, storage_dir, CareerProfile,
};
use grand_conquest_core::missions::MISSIONS;
use grand_conquest_core::model::{AiDifficulty, FactionId, MutatorId, MAX_RELIC_POWER};
use grand_conquest_core::save::ConquestSave;
use grand_conquest_core::settings::GameSettings;
use grand_conquest_core::sim::MatchState;
use grand_conquest_core::units::default_faction_deck;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::assets::{save_color_image_png, AssetBank, ASSETS_DIR};
use crate::audio::{MusicMode, SoundEngine};
use crate::theme::{
    body_font, crest_socket, cta_button, display_font, faction_color, filigree_divider, mono_font,
    nav_tab_button, paint_vertical_gradient, sculpted_card, sculpted_panel, subtle_button, BG_BAR,
    BLUE_LEGION, BORDER_STRONG, EMERALD, GOLD, GOLD_BRIGHT, RED_ENEMY, TEXT_MUTED, TEXT_PRIMARY,
    TEXT_SECONDARY,
};
use crate::ui_armory::{ArmoryAction, ArmoryViewState};
use crate::ui_battle::{BattleAction, BattleViewState};
use crate::ui_codex::CodexViewState;
use crate::ui_honors::{draw_honors_screen, HonorsTab, HonorsViewState};
use crate::ui_map::{MapAction, MapViewState};
use crate::ui_menu::{
    draw_overlays, draw_title_screen, MenuAction, MenuDestination, MenuOverlayState,
};
use crate::ui_skirmish::SkirmishConfig;
use crate::ui_vault::VaultViewState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Title,
    Atlas,
    Armory,
    Vault,
    Honors,
    Codex,
    Skirmish,
    Battle,
}

#[derive(Debug, Clone)]
enum LastBattleSpec {
    Campaign(&'static str),
    Skirmish,
}

#[derive(Debug, Clone)]
struct ToastBanner {
    title: String,
    subtitle: String,
    remaining_secs: f32,
}

pub struct QaCaptureDriver {
    pub out_dir: PathBuf,
    pub step: usize,
    pub frames_waited: usize,
    pub done: bool,
    pub pending_save: Option<(PathBuf, Receiver<std::sync::Arc<egui::ColorImage>>)>,
}

pub struct GrandConquestApp {
    screen: AppScreen,
    storage_dir: PathBuf,
    settings: GameSettings,
    save: ConquestSave,
    career: CareerProfile,
    save_path: PathBuf,
    career_path: PathBuf,
    save_dirty: bool,
    career_dirty: bool,
    tamper_warning: bool,
    show_help: bool,

    assets: AssetBank,
    audio: SoundEngine,
    rng: StdRng,
    last_frame: Instant,
    elapsed: f32,

    menu_overlay: MenuOverlayState,
    map_view: MapViewState,
    armory_view: ArmoryViewState,
    vault_view: VaultViewState,
    honors_view: HonorsViewState,
    codex_view: CodexViewState,
    skirmish_cfg: SkirmishConfig,
    battle_view: BattleViewState,
    active_match: Option<MatchState>,
    last_battle_spec: Option<LastBattleSpec>,
    toasts: Vec<ToastBanner>,

    qa: Option<QaCaptureDriver>,
}

impl GrandConquestApp {
    pub fn new(cc: &eframe::CreationContext<'_>, qa_out_dir: Option<PathBuf>) -> Self {
        install_theme_and_fonts_safe(&cc.egui_ctx);
        let assets = AssetBank::load(&cc.egui_ctx);
        let base_dir = storage_dir();
        let settings = GameSettings::load_from_path(&settings_path(&base_dir));
        let save_path = slot_save_path(&base_dir, settings.active_slot);
        let career_path = slot_career_path(&base_dir, settings.active_slot);
        let had_existing_save = save_path.exists();
        let (mut save, tamper_warning) = ConquestSave::load_from_path(&save_path);
        let mut career = CareerProfile::load_from_path(&career_path);

        if settings.fullscreen && qa_out_dir.is_none() {
            cc.egui_ctx.send_viewport_cmd(ViewportCommand::Fullscreen(true));
        }

        let rng = StdRng::from_entropy();
        let mut audio = SoundEngine::new();
        audio.sync_settings(&settings);

        let is_qa = qa_out_dir.is_some();
        let qa = qa_out_dir.map(|out_dir| {
            save = ConquestSave::unlock_all_sandbox();
            let _ = save.try_toggle_relic("treasury_seal");
            let _ = save.try_toggle_relic("titanium_fort");
            career = CareerProfile::default();
            let _ = career.evaluate_unlocks(&mut save);
            audio.set_music_mode(MusicMode::Silent);
            QaCaptureDriver {
                out_dir,
                step: 0,
                frames_waited: 0,
                done: false,
                pending_save: None,
            }
        });

        let mut app = Self {
            screen: if is_qa { AppScreen::Atlas } else { AppScreen::Title },
            storage_dir: base_dir,
            settings,
            save,
            career,
            save_path,
            career_path,
            save_dirty: had_existing_save && !is_qa,
            career_dirty: false,
            tamper_warning: tamper_warning && !is_qa,
            show_help: false,
            assets,
            audio,
            rng,
            last_frame: Instant::now(),
            elapsed: 0.0,
            menu_overlay: MenuOverlayState::default(),
            map_view: MapViewState::default(),
            armory_view: ArmoryViewState::default(),
            vault_view: VaultViewState::default(),
            honors_view: HonorsViewState::default(),
            codex_view: CodexViewState::default(),
            skirmish_cfg: SkirmishConfig::default(),
            battle_view: BattleViewState::default(),
            active_match: None,
            last_battle_spec: None,
            toasts: Vec::new(),
            qa,
        };
        if !is_qa {
            let unlocked = app.career.evaluate_unlocks(&mut app.save);
            if !unlocked.is_empty() {
                app.save_dirty = true;
                app.career_dirty = true;
            }
        }
        app.focus_atlas_on_frontier(Vec2::new(1280.0, 680.0));
        app
    }

    fn focus_atlas_on_frontier(&mut self, view_size: Vec2) {
        if let Some(m) = MISSIONS
            .iter()
            .find(|m| self.save.is_mission_unlocked(m.id) && self.save.mission_stars(m.id) == 0)
            .or_else(|| MISSIONS.iter().rev().find(|m| self.save.is_mission_unlocked(m.id)))
        {
            self.map_view.selected_mission_id = Some(m.id);
            self.map_view.focus_chapter(m.chapter, view_size);
        }
    }

    fn next_mission_after(&self, current_id: &'static str) -> Option<&'static str> {
        let idx = MISSIONS.iter().position(|m| m.id == current_id)?;
        MISSIONS.get(idx + 1).map(|m| m.id)
    }

    fn push_toast(&mut self, title: impl Into<String>, subtitle: impl Into<String>) {
        self.toasts.push(ToastBanner {
            title: title.into(),
            subtitle: subtitle.into(),
            remaining_secs: 5.0,
        });
    }

    fn switch_to_slot(&mut self, slot_idx: usize) {
        self.flush_save_if_dirty();
        self.settings.active_slot = slot_idx.min(2);
        let _ = self.settings.save_to_path(&settings_path(&self.storage_dir));
        self.save_path = slot_save_path(&self.storage_dir, self.settings.active_slot);
        self.career_path = slot_career_path(&self.storage_dir, self.settings.active_slot);
        let (save, tamper) = ConquestSave::load_from_path(&self.save_path);
        self.save = save;
        self.career = CareerProfile::load_from_path(&self.career_path);
        self.tamper_warning = tamper;
        self.save_dirty = false;
        self.career_dirty = false;
        self.focus_atlas_on_frontier(Vec2::new(1280.0, 680.0));
    }

    fn start_new_campaign(&mut self, slot_idx: usize, faction: FactionId, commander_name: String) {
        self.settings.active_slot = slot_idx.min(2);
        let _ = self.settings.save_to_path(&settings_path(&self.storage_dir));
        self.save_path = slot_save_path(&self.storage_dir, self.settings.active_slot);
        self.career_path = slot_career_path(&self.storage_dir, self.settings.active_slot);
        self.save = ConquestSave::new_default(faction);
        self.career = CareerProfile {
            commander_name,
            ..CareerProfile::default()
        };
        self.tamper_warning = false;
        self.save_dirty = true;
        self.career_dirty = true;
        self.flush_save_if_dirty();
        self.focus_atlas_on_frontier(Vec2::new(1280.0, 680.0));
        self.screen = AppScreen::Atlas;
    }

    fn launch_campaign_mission(&mut self, mission_id: &'static str) {
        if let Ok(state) = MatchState::from_campaign_mission(mission_id, &self.save, &mut self.rng) {
            self.active_match = Some(state);
            self.last_battle_spec = Some(LastBattleSpec::Campaign(mission_id));
            self.battle_view.reset_for_new_match();
            self.battle_view.speed_mult = self.settings.default_speed;
            self.battle_view.has_next_mission = self
                .next_mission_after(mission_id)
                .map(|nid| self.save.is_mission_unlocked(nid))
                .unwrap_or(false);
            self.armory_view.pending_mission_id = None;
            self.screen = AppScreen::Battle;
        }
    }

    fn launch_skirmish(&mut self) {
        let relics = self.skirmish_cfg.resolve_relics(&self.save);
        let state = MatchState::new_skirmish(
            self.skirmish_cfg.player_faction,
            &self.skirmish_cfg.player_deck,
            self.skirmish_cfg.enemy_faction,
            self.skirmish_cfg.ai_difficulty,
            self.skirmish_cfg.mutator,
            relics,
            &mut self.rng,
        );
        self.active_match = Some(state);
        self.last_battle_spec = Some(LastBattleSpec::Skirmish);
        self.battle_view.reset_for_new_match();
        self.battle_view.speed_mult = self.settings.default_speed;
        self.battle_view.has_next_mission = false;
        self.screen = AppScreen::Battle;
    }

    fn restart_last_battle(&mut self) {
        match self.last_battle_spec.clone() {
            Some(LastBattleSpec::Campaign(mid)) => self.launch_campaign_mission(mid),
            Some(LastBattleSpec::Skirmish) => self.launch_skirmish(),
            None => self.screen = AppScreen::Atlas,
        }
    }

    fn flush_save_if_dirty(&mut self) {
        if self.qa.is_some() {
            return;
        }
        if self.save_dirty {
            let _ = self.save.save_to_path(&self.save_path);
            self.save_dirty = false;
        }
        if self.career_dirty {
            let _ = self.career.save_to_path(&self.career_path);
            self.career_dirty = false;
        }
    }

    fn step_qa_capture(&mut self, ctx: &egui::Context) {
        let Some(ref mut qa) = self.qa else {
            return;
        };

        if let Some((path, rx)) = qa.pending_save.take() {
            if let Ok(img) = rx.try_recv() {
                let _ = save_color_image_png(&img, &path);
                qa.step += 1;
                qa.frames_waited = 0;
            } else {
                qa.pending_save = Some((path, rx));
                ctx.request_repaint();
                return;
            }
        }

        if qa.done {
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }

        if qa.frames_waited == 0 {
            match qa.step {
                0 => {
                    self.menu_overlay.show_settings = false;
                    self.screen = AppScreen::Title;
                }
                1 => {
                    self.screen = AppScreen::Atlas;
                    self.map_view.selected_mission_id = Some("M1_1");
                }
                2 => {
                    self.screen = AppScreen::Armory;
                }
                3 => {
                    self.screen = AppScreen::Vault;
                }
                4 => {
                    self.screen = AppScreen::Codex;
                    self.codex_view.selected_unit_id = "sol_1";
                    self.codex_view.preview_pose = crate::assets::PreviewPose::Attack;
                    self.codex_view.show_exploded_parts = true;
                }
                5 => {
                    self.screen = AppScreen::Skirmish;
                }
                6 => {
                    let boss_save = ConquestSave::unlock_all_sandbox();
                    if let Ok(mut st) =
                        MatchState::from_campaign_mission("M1_4_BOSS", &boss_save, &mut self.rng)
                    {
                        st.player.gold = 350.0;
                        st.player.tier = 3;
                        for idx in [0, 1, 2, 4] {
                            st.player_try_spawn_slot(idx, &mut self.rng);
                        }
                        for _ in 0..90 {
                            st.update(1.0 / 30.0, &mut self.rng);
                        }
                        st.cast_commander_spell_q(grand_conquest_core::model::Team::Blue);
                        self.active_match = Some(st);
                        self.battle_view.reset_for_new_match();
                        self.screen = AppScreen::Battle;
                    }
                }
                7 => {
                    let mut mid_save = ConquestSave::new_default(FactionId::Solaris);
                    mid_save.unlock_blueprint("sol_4");
                    mid_save.unlock_blueprint("sol_5");
                    mid_save.unlock_blueprint("sol_6");
                    mid_save.account_gold_vault = 420;
                    for mid in ["M1_1", "M1_2"] {
                        mid_save.missions.insert(
                            mid.to_string(),
                            grand_conquest_core::save::MissionRecord {
                                stars: 3,
                                best_time_seconds: 62.0,
                                times_attempted: 1,
                            },
                        );
                    }
                    mid_save.recompute_integrity();
                    self.save = mid_save;
                    self.career = CareerProfile::default();
                    let _ = self.career.evaluate_unlocks(&mut self.save);
                    self.armory_view.selected_slot = 3;
                    self.armory_view.filter_tier = Some(grand_conquest_core::model::UnitTier::B);
                    self.armory_view.pending_mission_id = Some("M1_4_BOSS");
                    self.screen = AppScreen::Armory;
                }
                8 => {
                    self.honors_view.tab = HonorsTab::Treasury;
                    self.screen = AppScreen::Honors;
                }
                9 => {
                    self.honors_view.tab = HonorsTab::TrophiesAndAchievements;
                    if let Some(first) = grand_conquest_core::honors::TROPHIES.first() {
                        if !self.save.unlocked_trophies.iter().any(|x| x == first.id) {
                            self.save.unlocked_trophies.push(first.id.to_string());
                        }
                    }
                    self.screen = AppScreen::Honors;
                }
                10 => {
                    self.screen = AppScreen::Title;
                    self.menu_overlay.show_settings = true;
                }
                11 => {
                    self.menu_overlay.show_settings = false;
                    let mut vic_save = ConquestSave::unlock_all_sandbox();
                    if let Ok(mut st) =
                        MatchState::from_campaign_mission("M1_1", &vic_save, &mut self.rng)
                    {
                        st.player.gold = 350.0;
                        st.player.tier = 2;
                        for idx in [0, 1, 2] {
                            st.player_try_spawn_slot(idx, &mut self.rng);
                        }
                        for _ in 0..90 {
                            st.update(1.0 / 30.0, &mut self.rng);
                        }
                        st.winner = Some(grand_conquest_core::model::Team::Blue);
                        st.over = true;
                        let summary = st.evaluate_and_commit_outcome(&mut vic_save);
                        self.battle_view.reset_for_new_match();
                        self.battle_view.outcome_committed = Some(summary);
                        self.battle_view.has_next_mission = true;
                        self.active_match = Some(st);
                        self.screen = AppScreen::Battle;
                    }
                }
                12 => {
                    self.battle_view.outcome_committed = None;
                    ctx.send_viewport_cmd(ViewportCommand::InnerSize(Vec2::new(1080.0, 680.0)));
                    self.screen = AppScreen::Atlas;
                    self.map_view.selected_mission_id = Some("M2_1");
                    self.map_view.focus_chapter(2, Vec2::new(710.0, 550.0));
                    self.map_view.cam_x = self.map_view.target_cam_x;
                    self.map_view.cam_y = self.map_view.target_cam_y;
                }
                13 => {
                    let boss_save = ConquestSave::unlock_all_sandbox();
                    if let Ok(mut st) =
                        MatchState::from_campaign_mission("M1_4_BOSS", &boss_save, &mut self.rng)
                    {
                        st.player.gold = 350.0;
                        st.player.tier = 3;
                        for idx in [0, 1, 2, 4] {
                            st.player_try_spawn_slot(idx, &mut self.rng);
                        }
                        for _ in 0..90 {
                            st.update(1.0 / 30.0, &mut self.rng);
                        }
                        st.cast_commander_spell_q(grand_conquest_core::model::Team::Blue);
                        self.active_match = Some(st);
                        self.battle_view.reset_for_new_match();
                        self.screen = AppScreen::Battle;
                    }
                }
                _ => {
                    qa.done = true;
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                    return;
                }
            }
        }

        qa.frames_waited += 1;
        if qa.frames_waited == 4 {
            let fname = match qa.step {
                0 => "00_title.png",
                1 => "01_atlas.png",
                2 => "02_armory.png",
                3 => "03_vault.png",
                4 => "04_codex.png",
                5 => "05_skirmish.png",
                6 => "06_battle_boss.png",
                7 => "07_armory_beginner.png",
                8 => "08_honors_store.png",
                9 => "09_honors_trophies.png",
                10 => "10_settings.png",
                11 => "11_battle_debrief.png",
                12 => "12_atlas_m2_1_1080.png",
                _ => "13_battle_1080.png",
            };
            let path = qa.out_dir.join(fname);
            let (tx, rx): (Sender<_>, Receiver<_>) = mpsc::channel();
            ctx.request_screenshot(move |img| {
                let _ = tx.send(img);
            });
            qa.pending_save = Some((path, rx));
        }
        ctx.request_repaint();
    }
}

fn install_theme_and_fonts_safe(ctx: &egui::Context) {
    crate::theme::install_theme_and_fonts(ctx, &ASSETS_DIR);
}

impl eframe::App for GrandConquestApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let raw_dt = now.duration_since(self.last_frame).as_secs_f32().clamp(0.001, 0.05);
        self.last_frame = now;
        self.elapsed += raw_dt;
        let ctx = ui.ctx().clone();

        // Update active toast timers
        for t in &mut self.toasts {
            t.remaining_secs -= raw_dt;
        }
        self.toasts.retain(|t| t.remaining_secs > 0.0);

        // Adaptive procedural soundtrack mode
        if self.qa.is_none() {
            let mode = match self.screen {
                AppScreen::Battle => {
                    let boss_up = self
                        .active_match
                        .as_ref()
                        .map(|m| m.units.iter().any(|u| u.is_boss && u.hp > 0.0))
                        .unwrap_or(false);
                    if boss_up {
                        MusicMode::Boss
                    } else {
                        MusicMode::Battle
                    }
                }
                _ => MusicMode::Menu,
            };
            self.audio.set_music_mode(mode);
        }

        // Auto-pause if window loses focus during an active battle
        if self.qa.is_none()
            && self.screen == AppScreen::Battle
            && !self.battle_view.paused
            && self.settings.auto_pause_unfocused
            && !ctx.input(|i| i.focused)
        {
            if let Some(ref st) = self.active_match {
                if !st.over {
                    self.battle_view.paused = true;
                }
            }
        }

        // Advance active battle simulation
        if self.screen == AppScreen::Battle && !self.battle_view.paused {
            let mut newly_finished: Option<(bool, bool, AiDifficulty, grand_conquest_core::sim::MatchStats)> =
                None;
            if let Some(ref mut state) = self.active_match {
                let sim_dt = raw_dt * self.battle_view.speed_mult;
                let steps = ((sim_dt / 0.025).ceil() as usize).clamp(1, 4);
                let step_dt = sim_dt / steps as f32;
                for _ in 0..steps {
                    state.update(step_dt, &mut self.rng);
                }
                for ev in state.sound_queue.drain(..) {
                    self.audio.play(ev);
                }
                if state.over && self.battle_view.outcome_committed.is_none() {
                    let summary = state.evaluate_and_commit_outcome(&mut self.save);
                    let won = summary.is_victory;
                    let is_skirmish = !state.is_campaign;
                    let diff = state.ai_difficulty;
                    let stats = state.stats.clone();
                    self.battle_view.outcome_committed = Some(summary);
                    if let Some(LastBattleSpec::Campaign(mid)) = self.last_battle_spec {
                        self.battle_view.has_next_mission = self
                            .next_mission_after(mid)
                            .map(|nid| self.save.is_mission_unlocked(nid))
                            .unwrap_or(false);
                    }
                    self.save_dirty = true;
                    newly_finished = Some((won, is_skirmish, diff, stats));
                }
            }
            if let Some((won, is_skirmish, diff, stats)) = newly_finished {
                self.career.record_match_end(&stats, won, is_skirmish, diff);
                let unlocked = self.career.evaluate_unlocks(&mut self.save);
                for a in unlocked {
                    self.audio.play_unlock();
                    self.push_toast(
                        format!("ACHIEVEMENT: {}", a.title),
                        format!("+{}G Vault Gold awarded", a.reward_gold),
                    );
                }
                self.save_dirty = true;
                self.career_dirty = true;
            }
        }

        self.battle_view.apply_settings(&self.settings);

        // Top App Command Navigation Strip (hidden on Title and Battle screens)
        if !matches!(self.screen, AppScreen::Title | AppScreen::Battle) {
            egui::Panel::top("global_nav_bar")
                .exact_size(52.0)
                .frame(
                    egui::Frame::new()
                        .fill(BG_BAR)
                        .stroke(Stroke::NONE)
                        .inner_margin(Margin::symmetric(18, 8)),
                )
                .show(ui, |ui| {
                    let bar_r = ui.max_rect().expand2(Vec2::new(18.0, 8.0));
                    paint_vertical_gradient(
                        ui.painter(),
                        bar_r,
                        Color32::from_rgb(24, 20, 19),
                        Color32::from_rgb(15, 13, 14),
                    );
                    ui.painter().line_segment(
                        [
                            egui::Pos2::new(bar_r.left(), bar_r.bottom() - 1.0),
                            egui::Pos2::new(bar_r.right(), bar_r.bottom() - 1.0),
                        ],
                        Stroke::new(1.0, Color32::from_rgba_unmultiplied(195, 160, 100, 48)),
                    );

                    ui.horizontal_centered(|ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        if nav_tab_button(ui, "Menu", false, TEXT_SECONDARY, Vec2::new(56.0, 32.0))
                            .on_hover_text("Return to Main Title Screen")
                            .clicked()
                        {
                            self.audio.play_click();
                            self.screen = AppScreen::Title;
                        }
                        if let Some(crest) = self.assets.get_crest(self.save.player_commander_faction) {
                            let (cr, _) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
                            crest_socket(
                                ui.painter(),
                                cr,
                                crest,
                                faction_color(self.save.player_commander_faction),
                            );
                        }

                        ui.add_space(4.0);
                        let navs = [
                            (AppScreen::Atlas, "Campaign", Vec2::new(82.0, 32.0)),
                            (AppScreen::Armory, "Armory", Vec2::new(74.0, 32.0)),
                            (AppScreen::Vault, "Relics", Vec2::new(66.0, 32.0)),
                            (AppScreen::Honors, "Honors", Vec2::new(72.0, 32.0)),
                            (AppScreen::Codex, "Codex", Vec2::new(66.0, 32.0)),
                            (AppScreen::Skirmish, "Skirmish", Vec2::new(78.0, 32.0)),
                        ];
                        for (scr, label, size) in navs {
                            let active = self.screen == scr;
                            if nav_tab_button(ui, label, active, GOLD, size).clicked() {
                                self.audio.play_click();
                                if scr != AppScreen::Armory {
                                    self.armory_view.pending_mission_id = None;
                                }
                                self.screen = scr;
                            }
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.spacing_mut().item_spacing.x = 4.0;
                            if nav_tab_button(ui, "Settings", false, GOLD, Vec2::new(68.0, 30.0))
                                .clicked()
                            {
                                self.audio.play_click();
                                self.menu_overlay.show_settings = true;
                            }

                            let mute_lbl = if self.settings.muted { "Muted" } else { "Audio" };
                            if nav_tab_button(
                                ui,
                                mute_lbl,
                                !self.settings.muted,
                                BLUE_LEGION,
                                Vec2::new(56.0, 30.0),
                            )
                            .on_hover_text("Toggle Audio")
                            .clicked()
                            {
                                self.settings.muted = !self.settings.muted;
                                self.audio.sync_settings(&self.settings);
                                let _ = self.settings.save_to_path(&settings_path(&self.storage_dir));
                            }

                            if nav_tab_button(ui, "Guide", self.show_help, EMERALD, Vec2::new(54.0, 30.0))
                                .clicked()
                            {
                                self.audio.play_click();
                                self.show_help = !self.show_help;
                            }

                            ui.add_space(10.0);
                            // Clean borderless inline resource telemetry
                            let stars = self.save.total_conquest_stars;
                            let gold = self.save.account_gold_vault;
                            let used_rp = self.save.used_relic_power();
                            ui.label(
                                egui::RichText::new(format!("{used_rp}/{MAX_RELIC_POWER} RP"))
                                    .font(mono_font(11.0))
                                    .color(BLUE_LEGION),
                            );
                            ui.label(egui::RichText::new("·").font(body_font(12.0)).color(TEXT_MUTED));
                            ui.label(
                                egui::RichText::new(format!("{gold}G"))
                                    .font(mono_font(11.5))
                                    .color(GOLD_BRIGHT),
                            );
                            ui.label(egui::RichText::new("·").font(body_font(12.0)).color(TEXT_MUTED));
                            ui.label(
                                egui::RichText::new(format!("{stars}/72"))
                                    .font(mono_font(11.0))
                                    .color(GOLD),
                            );
                            let (star_r, _) = ui.allocate_exact_size(Vec2::splat(13.0), egui::Sense::hover());
                            crate::theme::paint_star(ui.painter(), star_r.center(), 5.2, true, GOLD_BRIGHT);
                        });
                    });
                });
        }

        // Main Screen Body
        match self.screen {
            AppScreen::Title => {
                let act = draw_title_screen(
                    ui,
                    &self.save,
                    &self.career,
                    self.settings.active_slot,
                    &self.assets,
                    &self.audio,
                    self.elapsed,
                    &mut self.menu_overlay,
                );
                self.handle_menu_action(&ctx, act);
            }
            AppScreen::Atlas => {
                match self.map_view.draw(ui, &self.save, &self.assets, raw_dt, self.elapsed) {
                    MapAction::None => {}
                    MapAction::LaunchMission(mid) => self.launch_campaign_mission(mid),
                    MapAction::EditDeckForMission(mid) => {
                        self.armory_view.pending_mission_id = Some(mid);
                        self.screen = AppScreen::Armory;
                    }
                }
            }
            AppScreen::Armory => {
                match self.armory_view.draw(ui, &mut self.save, &self.assets, &mut self.save_dirty) {
                    ArmoryAction::None => {}
                    ArmoryAction::LaunchPendingMission(mid) => self.launch_campaign_mission(mid),
                }
            }
            AppScreen::Vault => {
                self.vault_view.draw(ui, &mut self.save, &self.assets, &mut self.save_dirty);
            }
            AppScreen::Honors => {
                let changed = draw_honors_screen(
                    ui,
                    &mut self.save,
                    &mut self.career,
                    &mut self.honors_view,
                    &self.assets,
                    &self.audio,
                    &mut self.rng,
                );
                if changed {
                    let newly = self.career.evaluate_unlocks(&mut self.save);
                    for a in newly {
                        self.push_toast(
                            format!("ACHIEVEMENT: {}", a.title),
                            format!("+{}G Vault Gold awarded", a.reward_gold),
                        );
                    }
                    self.save_dirty = true;
                    self.career_dirty = true;
                }
            }
            AppScreen::Codex => {
                self.codex_view.draw(ui, &self.assets, raw_dt);
            }
            AppScreen::Skirmish => {
                if self.skirmish_cfg.draw(ui, &self.save, &self.assets) {
                    self.launch_skirmish();
                }
            }
            AppScreen::Battle => {
                if let Some(ref mut state) = self.active_match {
                    match self.battle_view.draw(ui, state, &self.assets, &mut self.rng) {
                        BattleAction::None => {}
                        BattleAction::OpenSettings => {
                            self.menu_overlay.show_settings = true;
                        }
                        BattleAction::RestartBattle => self.restart_last_battle(),
                        BattleAction::NextMission => {
                            if let Some(LastBattleSpec::Campaign(mid)) = self.last_battle_spec {
                                if let Some(next_id) = self.next_mission_after(mid) {
                                    self.launch_campaign_mission(next_id);
                                }
                            }
                        }
                        BattleAction::ReturnToMenu => {
                            let go_skirmish = matches!(self.last_battle_spec, Some(LastBattleSpec::Skirmish));
                            self.active_match = None;
                            if go_skirmish {
                                self.screen = AppScreen::Skirmish;
                            } else {
                                self.focus_atlas_on_frontier(Vec2::new(1280.0, 680.0));
                                self.screen = AppScreen::Atlas;
                            }
                        }
                    }
                }
            }
        }

        // Global Overlays (New Campaign / Profile Slots / Settings / Manual)
        let overlay_act = draw_overlays(
            &ctx,
            &self.storage_dir,
            self.settings.active_slot,
            &mut self.settings,
            &mut self.audio,
            &self.assets,
            &mut self.menu_overlay,
        );
        self.handle_menu_action(&ctx, overlay_act);

        // Floating Steam-style Achievement / Unlock Toast Popups
        if !self.toasts.is_empty() {
            egui::Area::new(egui::Id::unique("achievement_toast_stack"))
                .anchor(egui::Align2::RIGHT_TOP, Vec2::new(-18.0, 64.0))
                .order(egui::Order::Tooltip)
                .fade_in(false)
                .interactable(false)
                .show(&ctx, |ui| {
                    for toast in self.toasts.iter().take(3) {
                        sculpted_card(ui, Some(GOLD_BRIGHT), 12, |ui| {
                            ui.set_min_width(260.0);
                            ui.label(egui::RichText::new(&toast.title).font(display_font(13.0)).color(GOLD_BRIGHT));
                            ui.label(egui::RichText::new(&toast.subtitle).font(body_font(11.5)).color(EMERALD));
                        });
                        ui.add_space(6.0);
                    }
                });
        }

        if self.tamper_warning {
            egui::Area::new(egui::Id::unique("tamper_notice"))
                .anchor(egui::Align2::LEFT_BOTTOM, Vec2::new(14.0, -18.0))
                .fade_in(false)
                .show(&ctx, |ui| {
                    sculpted_card(ui, Some(RED_ENEMY), 10, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Save integrity checksum re-validated on load.")
                                    .font(body_font(11.5))
                                    .color(RED_ENEMY),
                            );
                            if subtle_button(ui, "OK", false, TEXT_SECONDARY, Vec2::new(36.0, 22.0)).clicked() {
                                self.tamper_warning = false;
                            }
                        });
                    });
                });
        }

        if self.show_help && !matches!(self.screen, AppScreen::Battle | AppScreen::Title) {
            egui::Area::new(egui::Id::unique("global_help_guide_backdrop"))
                .fixed_pos(egui::Pos2::ZERO)
                .order(egui::Order::Middle)
                .fade_in(false)
                .show(&ctx, |ui| {
                    let r = ctx.viewport_rect();
                    ui.painter().rect_filled(r, 0.0, Color32::from_rgba_unmultiplied(6, 5, 5, 195));
                    ui.allocate_rect(r, egui::Sense::click());
                });

            let max_h = (ctx.viewport_rect().height() - 48.0).max(380.0);
            egui::Area::new(egui::Id::unique("global_help_guide_modal"))
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .order(egui::Order::Foreground)
                .fade_in(false)
                .show(&ctx, |ui| {
                    sculpted_panel(ui, GOLD, 22, |ui| {
                        ui.set_width(530.0);
                        ui.set_max_height(max_h);
                        ui.label(
                            egui::RichText::new("COMMANDER'S HANDBOOK — HOW TO PLAY")
                                .font(display_font(19.0))
                                .color(GOLD_BRIGHT),
                        );
                        filigree_divider(ui, GOLD);
                        ui.add_space(4.0);
                        egui::ScrollArea::vertical()
                            .max_height((max_h - 118.0).max(200.0))
                            .show(ui, |ui| {
                                for (step_title, step_desc) in [
                                    (
                                        "1. Pick a Mission (Campaign Atlas)",
                                        "Click an unlocked stronghold territory on the world map and press DEPLOY BATTLE.",
                                    ),
                                    (
                                        "2. Unlock & Equip Stronger Tiers (Armory)",
                                        "Clear Campaign missions to open Tier B ([3–4]) and Tier A ([5–6]) slots, or requisition cross-faction units early in Treasury & Honors!",
                                    ),
                                    (
                                        "3. Fight & Manage Gold (In Battle)",
                                        "Press [U] to upgrade gold income, deploy squads with [1]–[6] (multiplier [F]), command [Z] Attack / [X] Hold / [C] Retreat, and cast [Q]/[E] Commander Spells.",
                                    ),
                                    (
                                        "4. Attune Relics & Claim Honors",
                                        "Equip relics up to 4 Relic Power in the Relic Vault and track all 6 Boss Trophies & 16 Career Achievements in Treasury & Honors.",
                                    ),
                                ] {
                                    sculpted_card(ui, Some(BORDER_STRONG), 12, |ui| {
                                        ui.set_min_width(ui.available_width());
                                        ui.label(egui::RichText::new(step_title).font(body_font(13.5)).color(GOLD));
                                        ui.add(
                                            egui::Label::new(
                                                egui::RichText::new(step_desc)
                                                    .font(body_font(12.0))
                                                    .color(TEXT_SECONDARY),
                                            )
                                            .wrap(),
                                        );
                                    });
                                    ui.add_space(6.0);
                                }
                            });
                        ui.add_space(10.0);
                        ui.vertical_centered(|ui| {
                            if cta_button(ui, "GOT IT, COMMANDER", Vec2::new(206.0, 38.0)).clicked() {
                                self.show_help = false;
                            }
                        });
                    });
                });
        }

        self.flush_save_if_dirty();
        self.step_qa_capture(&ctx);

        if matches!(
            self.screen,
            AppScreen::Title | AppScreen::Atlas | AppScreen::Codex | AppScreen::Battle
        ) {
            ctx.request_repaint();
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if self.qa.is_none() {
            let _ = self.save.save_to_path(&self.save_path);
            let _ = self.career.save_to_path(&self.career_path);
            let _ = self.settings.save_to_path(&settings_path(&self.storage_dir));
        }
    }
}

impl GrandConquestApp {
    fn handle_menu_action(&mut self, ctx: &egui::Context, act: MenuAction) {
        match act {
            MenuAction::None => {}
            MenuAction::ContinueCampaign => {
                self.focus_atlas_on_frontier(Vec2::new(1280.0, 680.0));
                self.screen = AppScreen::Atlas;
            }
            MenuAction::StartNewCampaign {
                slot,
                faction,
                commander_name,
            } => {
                self.start_new_campaign(slot, faction, commander_name);
            }
            MenuAction::LoadSlot(slot) => {
                self.switch_to_slot(slot);
                self.screen = AppScreen::Atlas;
            }
            MenuAction::ResetActiveSlot => {
                let fac = self.save.player_commander_faction;
                let name = self.career.commander_name.clone();
                self.start_new_campaign(self.settings.active_slot, fac, name);
            }
            MenuAction::Navigate(dest) => {
                self.screen = match dest {
                    MenuDestination::Atlas => AppScreen::Atlas,
                    MenuDestination::Armory => AppScreen::Armory,
                    MenuDestination::Vault => AppScreen::Vault,
                    MenuDestination::Honors => AppScreen::Honors,
                    MenuDestination::Codex => AppScreen::Codex,
                    MenuDestination::Skirmish => AppScreen::Skirmish,
                };
            }
            MenuAction::Quit => {
                self.flush_save_if_dirty();
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        }
    }
}

#[allow(dead_code)]
fn _unused_imports_guard() {
    let _ = (
        AiDifficulty::Normal,
        MutatorId::ConsecratedSolar,
        default_faction_deck(FactionId::Solaris),
        TEXT_PRIMARY,
        TEXT_MUTED,
    );
}
