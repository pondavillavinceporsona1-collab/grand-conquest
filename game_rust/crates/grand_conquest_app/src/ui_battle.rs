use egui::epaint::Shape;
use egui::{
    Align2, Color32, CornerRadius, Margin, Painter, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2,
};
use grand_conquest_core::ai_weights::{unit_weight_vector, AiBrainTelemetry};
use grand_conquest_core::model::{
    AiDifficulty, AiPosture, ArmyOrder, MissionArchetype, PassiveKey, SpellFxType, StructureKind,
    Team, AURA_RADIUS_STUDS, LANE_BOTTOM, LANE_TOP, STUD_TO_PX, WORLD_H, WORLD_W,
};
use grand_conquest_core::sim::{
    MatchOutcomeSummary, MatchState, VisualEffectKind,
};
use grand_conquest_core::units::get_unit;

use crate::assets::{image_rect_tinted, portrait_rect, AssetBank};
use crate::theme::{
    beveled_progress_bar, body_font, crest_socket, cta_button, display_font, draw_badge,
    equal_columns, filigree_divider, mono_font, nav_tab_button, paint_beveled_box,
    paint_parchment_backdrop, paint_star, paint_unit_shape, paint_vertical_gradient,
    portrait_pedestal, sculpted_card, sculpted_panel, tier_color, well_frame, AMETHYST, BG_BAR,
    BG_PANEL, BG_VOID, BLUE_LEGION, BORDER_STRONG, BORDER_SUBTLE, CYAN, EMERALD, GOLD,
    GOLD_BRIGHT, GOLD_DARK, GOLD_DIM, RED_ENEMY, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    SkeletalRig,
    GeometricShapes,
}

pub enum BattleAction {
    None,
    RestartBattle,
    ReturnToMenu,
    OpenSettings,
    NextMission,
}

const SPAWN_MULTIPLIERS: [usize; 6] = [1, 5, 10, 20, 30, 50];

pub struct BattleViewState {
    pub paused: bool,
    pub show_help: bool,
    pub show_brain: bool,
    pub speed_mult: f32,
    pub spawn_mult: usize,
    pub render_mode: RenderMode,
    pub show_labels: bool,
    pub outcome_committed: Option<MatchOutcomeSummary>,
    pub hp_bar_mode: grand_conquest_core::settings::HpBarMode,
    pub show_damage_numbers: bool,
    pub screen_shake_enabled: bool,
    pub show_minimap: bool,
    pub has_next_mission: bool,
    pub shake_trauma: f32,
    last_fx_sig: usize,
    unit_sort_buf: Vec<(f32, usize)>,
    unit_draw_meta: Vec<(Pos2, f32, bool)>,
}

impl Default for BattleViewState {
    fn default() -> Self {
        Self {
            paused: false,
            show_help: false,
            show_brain: false,
            speed_mult: 1.0,
            spawn_mult: 1,
            render_mode: RenderMode::SkeletalRig,
            show_labels: false,
            outcome_committed: None,
            hp_bar_mode: grand_conquest_core::settings::HpBarMode::Smart,
            show_damage_numbers: true,
            screen_shake_enabled: true,
            show_minimap: true,
            has_next_mission: false,
            shake_trauma: 0.0,
            last_fx_sig: 0,
            unit_sort_buf: Vec::with_capacity(512),
            unit_draw_meta: Vec::with_capacity(512),
        }
    }
}

impl BattleViewState {
    pub fn reset_for_new_match(&mut self) {
        self.paused = false;
        self.show_help = false;
        self.show_brain = false;
        self.spawn_mult = 1;
        self.outcome_committed = None;
        self.shake_trauma = 0.0;
        self.last_fx_sig = 0;
        self.unit_sort_buf.clear();
        self.unit_draw_meta.clear();
    }

    pub fn apply_settings(&mut self, s: &grand_conquest_core::settings::GameSettings) {
        self.hp_bar_mode = s.hp_bar_mode;
        self.show_damage_numbers = s.show_damage_numbers;
        self.screen_shake_enabled = s.screen_shake;
        self.show_minimap = s.show_minimap;
    }

    fn cycle_spawn_mult(&mut self) {
        let idx = SPAWN_MULTIPLIERS
            .iter()
            .position(|&m| m == self.spawn_mult)
            .unwrap_or(0);
        self.spawn_mult = SPAWN_MULTIPLIERS[(idx + 1) % SPAWN_MULTIPLIERS.len()];
    }

    pub fn draw(
        &mut self,
        ui: &mut Ui,
        state: &mut MatchState,
        assets: &AssetBank,
        rng: &mut impl rand::Rng,
    ) -> BattleAction {
        let mut action = BattleAction::None;

        // Keyboard hotkeys during battle
        if !state.over {
            ui.input(|i| {
                if i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Space) {
                    self.paused = !self.paused;
                }
                if i.key_pressed(egui::Key::V) {
                    self.render_mode = match self.render_mode {
                        RenderMode::SkeletalRig => RenderMode::GeometricShapes,
                        RenderMode::GeometricShapes => RenderMode::SkeletalRig,
                    };
                }
                if i.key_pressed(egui::Key::Tab) {
                    self.speed_mult = if self.speed_mult < 0.75 {
                        1.0
                    } else if self.speed_mult < 1.5 {
                        2.0
                    } else if self.speed_mult < 2.5 {
                        3.0
                    } else {
                        0.5
                    };
                }
                if i.key_pressed(egui::Key::F) {
                    self.cycle_spawn_mult();
                }
                if i.key_pressed(egui::Key::B) {
                    self.show_brain = !self.show_brain;
                }
            });

            if !self.paused {
                ui.input(|i| {
                    let keys = [
                        egui::Key::Num1,
                        egui::Key::Num2,
                        egui::Key::Num3,
                        egui::Key::Num4,
                        egui::Key::Num5,
                        egui::Key::Num6,
                    ];
                    for (idx, k) in keys.iter().enumerate() {
                        if i.key_pressed(*k) {
                            state.player_try_spawn_slot_count(idx, self.spawn_mult, rng);
                        }
                    }
                    if i.key_pressed(egui::Key::U) {
                        state.player_try_upgrade_income();
                    }
                    if i.key_pressed(egui::Key::T) {
                        state.player_try_unlock_tier();
                    }
                    if i.key_pressed(egui::Key::Q) {
                        state.cast_commander_spell_q(Team::Blue);
                    }
                    if i.key_pressed(egui::Key::E) {
                        state.cast_commander_spell_e(Team::Blue);
                    }
                    if i.key_pressed(egui::Key::Z) {
                        state.set_army_order(Team::Blue, ArmyOrder::Attack);
                    }
                    if i.key_pressed(egui::Key::X) {
                        state.set_army_order(Team::Blue, ArmyOrder::Hold);
                    }
                    if i.key_pressed(egui::Key::C) {
                        state.set_army_order(Team::Blue, ArmyOrder::Retreat);
                    }
                });
            }
        }

        // 1. Top Telemetry & Fortress Bar (symmetric left/right fortress plates + dead-center telemetry, 1080px-safe)
        egui::Panel::top("battle_top_bar")
            .exact_size(50.0)
            .frame(
                egui::Frame::new()
                    .fill(BG_BAR)
                    .stroke(Stroke::NONE)
                    .inner_margin(Margin::symmetric(12, 4)),
            )
            .show(ui, |ui| {
                let bar_r = ui.max_rect().expand2(Vec2::new(12.0, 4.0));
                paint_vertical_gradient(
                    ui.painter(),
                    bar_r,
                    Color32::from_rgb(24, 20, 19),
                    Color32::from_rgb(15, 13, 14),
                );
                ui.painter().line_segment(
                    [
                        Pos2::new(bar_r.left(), bar_r.bottom() - 1.0),
                        Pos2::new(bar_r.right(), bar_r.bottom() - 1.0),
                    ],
                    Stroke::new(0.8, Color32::from_rgba_unmultiplied(195, 160, 100, 38)),
                );

                let p_base = state.structures.iter().find(|s| s.id == "p_base").unwrap();
                let p_t2 = state.structures.iter().find(|s| s.id == "p_t2").unwrap();
                let p_t1 = state.structures.iter().find(|s| s.id == "p_t1").unwrap();
                let e_t1 = state.structures.iter().find(|s| s.id == "e_t1").unwrap();
                let e_t2 = state.structures.iter().find(|s| s.id == "e_t2").unwrap();
                let e_base = state.structures.iter().find(|s| s.id == "e_base").unwrap();

                let blue_cnt = state.units.iter().filter(|u| u.team == Team::Blue && u.hp > 0.0).count();
                let red_cnt = state.units.iter().filter(|u| u.team == Team::Red && u.hp > 0.0).count();

                let total_w = ui.available_width();
                let compact = total_w < 1200.0;
                let side_w = if compact { 296.0 } else { 366.0 };
                let center_w = (total_w - side_w * 2.0 - 12.0).max(320.0);

                ui.horizontal_centered(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;
                    // Left: Blue Bastion
                    ui.allocate_ui(Vec2::new(side_w, 44.0), |ui| {
                        ui.set_min_size(Vec2::new(side_w, 44.0));
                        ui.horizontal_centered(|ui| {
                            if let Some(bc) = assets.get_crest(state.player.faction.id) {
                                let (cr, _) = ui.allocate_exact_size(
                                    Vec2::splat(if compact { 24.0 } else { 30.0 }),
                                    Sense::hover(),
                                );
                                crest_socket(ui.painter(), cr, bc, BLUE_LEGION);
                            }
                            draw_fortress_cluster(
                                ui,
                                true,
                                p_base.hp,
                                p_base.max_hp,
                                p_t2.hp,
                                p_t1.hp,
                                false,
                                compact,
                            );
                        });
                    });

                    // Center: Match Clock, Headcount & Speed/Inspector Toggles
                    ui.allocate_ui(Vec2::new(center_w, 44.0), |ui| {
                        ui.set_min_size(Vec2::new(center_w, 44.0));
                        ui.horizontal_centered(|ui| {
                            let item_sp = if compact { 4.0 } else { 6.0 };
                            ui.spacing_mut().item_spacing.x = item_sp;
                            let cluster_w = if compact { 312.0 } else { 378.0 };
                            let pad = ((center_w - cluster_w) * 0.5).max(0.0);
                            if pad > 0.0 {
                                ui.add_space(pad);
                            }
                            let mins = (state.time as u32) / 60;
                            let secs = (state.time as u32) % 60;
                            ui.label(
                                egui::RichText::new(format!("{mins:02}:{secs:02}"))
                                    .font(mono_font(if compact { 13.0 } else { 14.5 }))
                                    .color(GOLD_BRIGHT),
                            );
                            ui.label(
                                egui::RichText::new(format!("{blue_cnt}v{red_cnt}"))
                                    .font(mono_font(10.5))
                                    .color(TEXT_SECONDARY),
                            );

                            if nav_tab_button(
                                ui,
                                if self.paused { ">" } else { "||" },
                                self.paused,
                                GOLD,
                                Vec2::new(if compact { 26.0 } else { 28.0 }, 26.0),
                            )
                            .on_hover_text("Pause / Resume [Space]")
                            .clicked()
                            {
                                self.paused = !self.paused;
                            }

                            let spd_lbl = if self.speed_mult < 0.9 {
                                "0.5x".to_string()
                            } else {
                                format!("{}x", self.speed_mult as u32)
                            };
                            if nav_tab_button(
                                ui,
                                &spd_lbl,
                                (self.speed_mult - 1.0).abs() > 0.1,
                                GOLD,
                                Vec2::new(if compact { 32.0 } else { 38.0 }, 26.0),
                            )
                            .on_hover_text("Cycle Sim Speed (0.5x / 1x / 2x / 3x) [Tab]")
                            .clicked()
                            {
                                self.speed_mult = if self.speed_mult < 0.75 {
                                    1.0
                                } else if self.speed_mult < 1.5 {
                                    2.0
                                } else if self.speed_mult < 2.5 {
                                    3.0
                                } else {
                                    0.5
                                };
                            }

                            let rig_lbl = match self.render_mode {
                                RenderMode::SkeletalRig => "Sprites",
                                RenderMode::GeometricShapes => "Shapes",
                            };
                            if nav_tab_button(
                                ui,
                                rig_lbl,
                                false,
                                BLUE_LEGION,
                                Vec2::new(if compact { 50.0 } else { 58.0 }, 26.0),
                            )
                            .on_hover_text("Toggle Progressive 2D Sprites vs Geometric Shapes [V]")
                            .clicked()
                            {
                                self.render_mode = match self.render_mode {
                                    RenderMode::SkeletalRig => RenderMode::GeometricShapes,
                                    RenderMode::GeometricShapes => RenderMode::SkeletalRig,
                                };
                            }

                            if nav_tab_button(
                                ui,
                                "Names",
                                self.show_labels,
                                EMERALD,
                                Vec2::new(if compact { 44.0 } else { 52.0 }, 26.0),
                            )
                            .on_hover_text("Show/Hide unit names above heads")
                            .clicked()
                            {
                                self.show_labels = !self.show_labels;
                            }
                            if nav_tab_button(
                                ui,
                                "Brain",
                                self.show_brain,
                                AMETHYST,
                                Vec2::new(if compact { 42.0 } else { 50.0 }, 26.0),
                            )
                            .on_hover_text("Inspect AI Brain Weights, Node Tree & Live Counter Adaptation [B]")
                            .clicked()
                            {
                                self.show_brain = !self.show_brain;
                            }
                            if nav_tab_button(
                                ui,
                                "?",
                                self.show_help,
                                GOLD,
                                Vec2::new(if compact { 24.0 } else { 26.0 }, 26.0),
                            )
                            .on_hover_text("Battlefield Controls & Guide")
                            .clicked()
                            {
                                self.show_help = !self.show_help;
                            }
                        });
                    });

                    // Right: Red Citadel + Retreat
                    ui.allocate_ui(Vec2::new(side_w, 44.0), |ui| {
                        ui.set_min_size(Vec2::new(side_w, 44.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.spacing_mut().item_spacing.x = if compact { 4.0 } else { 6.0 };
                            if nav_tab_button(
                                ui,
                                "Retreat",
                                false,
                                RED_ENEMY,
                                Vec2::new(if compact { 54.0 } else { 66.0 }, 28.0),
                            )
                            .clicked()
                            {
                                action = BattleAction::ReturnToMenu;
                            }
                            if let Some(rc) = assets.get_crest(state.ai.faction.id) {
                                let (cr, _) = ui.allocate_exact_size(
                                    Vec2::splat(if compact { 24.0 } else { 30.0 }),
                                    Sense::hover(),
                                );
                                crest_socket(ui.painter(), cr, rc, RED_ENEMY);
                            }
                            draw_fortress_cluster(
                                ui,
                                false,
                                e_base.hp,
                                e_base.max_hp,
                                e_t2.hp,
                                e_t1.hp,
                                e_base.is_invulnerable_override,
                                compact,
                            );
                        });
                    });
                });
            });

        // 1b. Docked Tactical Ticker Strip (clean borderless strip)
        egui::Panel::top("battle_sub_bar")
            .exact_size(26.0)
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(16, 14, 15))
                    .stroke(Stroke::NONE)
                    .inner_margin(Margin::symmetric(14, 2)),
            )
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    let (obj_title, obj_badge, obj_desc) = objective_hud_strings(state);
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("{obj_title}  ·  {obj_badge}"))
                                .font(mono_font(10.0))
                                .color(GOLD),
                        );
                    })
                    .response
                    .on_hover_text(obj_desc);

                    let posture_col = match state.ai_posture {
                        AiPosture::EcoBoom => EMERALD,
                        AiPosture::TechUp => AMETHYST,
                        AiPosture::Defend => RED_ENEMY,
                        AiPosture::CounterPush => GOLD,
                        AiPosture::AllIn => GOLD_BRIGHT,
                    };
                    let banner_msg = if state.time < 14.0
                        && state.stats.units_spawned < 2
                        && state.mission.map(|m| m.id == "M1_1").unwrap_or(false)
                    {
                        "First Step: [U] Upgrade Income, then [1]/[2] to deploy troops"
                    } else {
                        &state.banner_text
                    };

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let ai_resp = ui
                            .horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{} AI · T{} · L{} · {}",
                                        state.ai_difficulty.name(),
                                        state.ai.tier,
                                        state.ai.income_lvl,
                                        state.ai_posture.short_tag()
                                    ))
                                    .font(mono_font(9.8))
                                    .color(posture_col),
                                );
                            })
                            .response
                            .interact(Sense::click())
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .on_hover_text(format!(
                                "Enemy AI Commander ({})\n{}\nPosture: {}\nArmy Order: {} ({})\nGold Rate: +{:.1}G/s\nActive Node: {}\nClick or press [B] to inspect AI Brain & Weights",
                                state.ai_difficulty.name(),
                                state.ai_difficulty.desc(),
                                state.ai_posture.label(),
                                state.ai.order.label(),
                                state.ai.order.desc(),
                                state.get_income_rate(Team::Red),
                                state.ai_brain.active_plan_summary,
                            ));
                        if ai_resp.clicked() {
                            self.show_brain = !self.show_brain;
                        }
                        if self.show_brain {
                            ui.label(
                                egui::RichText::new(format!(
                                    "WGT {:.2} vs {:.2}",
                                    state.ai_brain.blue_effective_weight,
                                    state.ai_brain.red_effective_weight
                                ))
                                .font(mono_font(9.5))
                                .color(AMETHYST),
                            );
                        }

                        ui.add_space(10.0);
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(banner_msg)
                                        .font(body_font(11.2))
                                        .color(TEXT_SECONDARY),
                                )
                                .truncate(),
                            );
                        });
                    });
                });
            });

        // 2. Bottom Commander Deck Console
        egui::Panel::bottom("battle_console_bar")
            .exact_size(132.0)
            .frame(
                egui::Frame::new()
                    .fill(BG_BAR)
                    .stroke(Stroke::NONE)
                    .inner_margin(Margin::symmetric(12, 8)),
            )
            .show(ui, |ui| {
                let console_rect = ui.max_rect().expand2(Vec2::new(12.0, 8.0));
                paint_vertical_gradient(
                    ui.painter(),
                    console_rect,
                    Color32::from_rgb(24, 19, 17),
                    Color32::from_rgb(13, 11, 12),
                );
                if let Some(parch) = assets.parchment_dark.as_ref() {
                    paint_parchment_backdrop(
                        ui.painter(),
                        console_rect,
                        parch,
                        Color32::from_rgba_unmultiplied(195, 178, 154, 42),
                    );
                }
                ui.painter().line_segment(
                    [
                        Pos2::new(console_rect.left(), console_rect.top() + 1.0),
                        Pos2::new(console_rect.right(), console_rect.top() + 1.0),
                    ],
                    Stroke::new(1.5, BORDER_STRONG),
                );
                ui.painter().line_segment(
                    [
                        Pos2::new(console_rect.left(), console_rect.top() + 3.0),
                        Pos2::new(console_rect.right(), console_rect.top() + 3.0),
                    ],
                    Stroke::new(0.8, Color32::from_rgba_unmultiplied(255, 224, 150, 26)),
                );

                ui.horizontal(|ui| {
                    let total_w = ui.available_width();
                    let compact_deck = total_w < 1180.0;
                    let econ_w = (total_w * 0.215).clamp(216.0, 310.0);
                    let spell_w = (total_w * 0.195).clamp(200.0, 250.0);
                    let deck_w = (total_w - econ_w - spell_w - 22.0).max(400.0);

                    // Left: Economy Pod (Gold + [U] Income + [T] Tech Tier)
                    ui.allocate_ui(Vec2::new(econ_w, 112.0), |ui| {
                        equal_columns(ui, 3, |ui, col_idx| {
                            match col_idx {
                                0 => {
                                    // Clean Gold Reservoir (no square box outline)
                                    let g_r = ui.available_rect_before_wrap();
                                    let box_r = Rect::from_min_size(
                                        g_r.min,
                                        Vec2::new(ui.available_width(), 108.0),
                                    );
                                    paint_beveled_box(
                                        ui.painter(),
                                        box_r,
                                        10,
                                        Color32::from_rgba_unmultiplied(36, 29, 21, 215),
                                        Color32::from_rgba_unmultiplied(20, 17, 15, 225),
                                        Color32::TRANSPARENT,
                                        0.0,
                                        None,
                                    );
                                    egui::Frame::new()
                                        .inner_margin(Margin::same(8))
                                        .show(ui, |ui| {
                                            ui.set_min_height(92.0);
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    egui::RichText::new("GOLD")
                                                        .font(mono_font(9.8))
                                                        .color(TEXT_MUTED),
                                                );
                                                ui.with_layout(
                                                    egui::Layout::right_to_left(egui::Align::Center),
                                                    |ui| {
                                                        ui.label(
                                                            egui::RichText::new(format!("T{}", state.player.tier))
                                                                .font(mono_font(10.0))
                                                                .color(AMETHYST),
                                                        );
                                                    },
                                                );
                                            });
                                            ui.label(
                                                egui::RichText::new(format!(
                                                    "{:.0}G",
                                                    state.player.gold.floor()
                                                ))
                                                .font(mono_font(if compact_deck { 16.5 } else { 19.0 }))
                                                .color(GOLD_BRIGHT),
                                            );
                                            let rate = state.get_income_rate(Team::Blue);
                                            ui.label(
                                                egui::RichText::new(format!("+{rate:.1}/s"))
                                                    .font(mono_font(10.0))
                                                    .color(EMERALD),
                                            );
                                            ui.with_layout(
                                                egui::Layout::bottom_up(egui::Align::LEFT),
                                                |ui| {
                                                    beveled_progress_bar(
                                                        ui,
                                                        state.income_timer.clamp(0.0, 1.0),
                                                        Vec2::new(ui.available_width(), 5.0),
                                                        EMERALD,
                                                        None,
                                                    );
                                                },
                                            );
                                        });
                                }
                                1 => {
                                    // [U] Income Upgrade
                                    let inc_cost = state.income_upgrade_cost(Team::Blue);
                                    let can_inc = !state.over
                                        && !self.paused
                                        && state.player.gold >= inc_cost as f32;
                                    if draw_cmd_button(
                                        ui,
                                        "[U]",
                                        &format!("Lv{}", state.player.income_lvl),
                                        "Income",
                                        &format!("{inc_cost}G"),
                                        can_inc,
                                        EMERALD,
                                    )
                                    .on_hover_text("Upgrade passive gold income (+2G/s). Upgrading once or twice early snowballs your army!")
                                    .clicked()
                                        && !self.paused
                                    {
                                        state.player_try_upgrade_income();
                                    }
                                }
                                _ => {
                                    // [T] Tech Tier
                                    let max_deck_tier = state.max_player_tech_tier();
                                    let (t_top, t_mid, t_bot, can_t, t_tip) = if max_deck_tier == 1 {
                                        (
                                            "T1".into(),
                                            "Base Only",
                                            "—".into(),
                                            false,
                                            "You don't have Tier B or A troops in your deck yet. Win M1.1 to unlock Knights!",
                                        )
                                    } else if state.player.tier >= max_deck_tier {
                                        (
                                            "MAX".into(),
                                            "Tiers",
                                            "Open".into(),
                                            false,
                                            "All tiers present in your current deck are unlocked for battle!",
                                        )
                                    } else {
                                        let tc = MatchState::next_tier_cost(state.player.tier).unwrap_or(0);
                                        let title = if state.player.tier == 1 {
                                            "Unlock B"
                                        } else {
                                            "Unlock A"
                                        };
                                        (
                                            format!("T{}", state.player.tier + 1),
                                            title,
                                            format!("{tc}G"),
                                            !state.over && !self.paused && state.player.gold >= tc as f32,
                                            "Spend gold mid-battle to unlock your Tier B ([3-4]) or Tier A ([5-6]) units!",
                                        )
                                    };
                                    if draw_cmd_button(ui, "[T]", &t_top, t_mid, &t_bot, can_t, AMETHYST)
                                        .on_hover_text(t_tip)
                                        .clicked()
                                        && !self.paused
                                    {
                                        state.player_try_unlock_tier();
                                    }
                                }
                            }
                        });
                    });

                    // Center: 6 Embossed Deployment Unit Cards
                    ui.allocate_ui(Vec2::new(deck_w, 112.0), |ui| {
                        equal_columns(ui, 6, |ui, idx| {
                            let slot_tier = match idx {
                                0 | 1 => "Tier C",
                                2 | 3 => "Tier B",
                                _ => "Tier A",
                            };
                            let Some(u) = state.player.deck[idx] else {
                                let req_msg = match idx {
                                    2 | 3 => "Win M1.1 Battle",
                                    _ => "Beat M1.4 Boss",
                                };
                                let box_r = Rect::from_min_size(
                                    ui.available_rect_before_wrap().min,
                                    Vec2::new(ui.available_width(), 108.0),
                                );
                                paint_beveled_box(
                                    ui.painter(),
                                    box_r,
                                    7,
                                    Color32::from_rgb(20, 17, 16),
                                    Color32::from_rgb(13, 12, 12),
                                    BORDER_SUBTLE,
                                    0.9,
                                    None,
                                );
                                egui::Frame::new()
                                    .inner_margin(Margin::same(7))
                                    .show(ui, |ui| {
                                        ui.set_min_height(92.0);
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                egui::RichText::new(format!("[{}]", idx + 1))
                                                    .font(mono_font(10.0))
                                                    .color(TEXT_MUTED),
                                            );
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    ui.label(
                                                        egui::RichText::new(slot_tier)
                                                            .font(mono_font(9.0))
                                                            .color(TEXT_MUTED),
                                                    );
                                                },
                                            );
                                        });
                                        ui.add_space(12.0);
                                        ui.label(
                                            egui::RichText::new("LOCKED")
                                                .font(mono_font(10.0))
                                                .color(TEXT_MUTED),
                                        );
                                        ui.label(
                                            egui::RichText::new(req_msg)
                                                .font(body_font(10.5))
                                                .color(TEXT_MUTED),
                                        );
                                    });
                                return;
                            };

                            let req_tier = u.tier.required_tech_tier();
                            let unlocked = state.player.tier >= req_tier;
                            let mult = self.spawn_mult.max(1);
                            let unit_cost = u.cost as f32;
                            let batch_cost = (u.cost as usize) * mult;
                            let max_afford = if unit_cost > 0.0 {
                                (state.player.gold / unit_cost).floor() as usize
                            } else {
                                mult
                            };
                            let will_spawn = max_afford.min(mult);
                            let affordable = unlocked && will_spawn > 0;
                            let full_batch = unlocked && will_spawn >= mult;

                            let (top_col, bot_col, stroke_col, stroke_w) = if !unlocked {
                                (
                                    Color32::from_rgba_unmultiplied(22, 19, 18, 180),
                                    Color32::from_rgba_unmultiplied(15, 13, 13, 195),
                                    Color32::TRANSPARENT,
                                    0.0,
                                )
                            } else if full_batch {
                                (
                                    Color32::from_rgb(40, 33, 24),
                                    Color32::from_rgb(22, 19, 16),
                                    GOLD.linear_multiply(0.58),
                                    1.1,
                                )
                            } else if affordable {
                                (
                                    Color32::from_rgb(34, 29, 23),
                                    Color32::from_rgb(20, 17, 15),
                                    GOLD.linear_multiply(0.35),
                                    0.9,
                                )
                            } else {
                                (
                                    Color32::from_rgba_unmultiplied(26, 22, 20, 200),
                                    Color32::from_rgba_unmultiplied(18, 15, 15, 210),
                                    Color32::TRANSPARENT,
                                    0.0,
                                )
                            };

                            let box_r = Rect::from_min_size(
                                ui.available_rect_before_wrap().min,
                                Vec2::new(ui.available_width(), 108.0),
                            );
                            paint_beveled_box(
                                ui.painter(),
                                box_r,
                                10,
                                top_col,
                                bot_col,
                                stroke_col,
                                stroke_w,
                                None,
                            );

                            let resp = egui::Frame::new()
                                .inner_margin(Margin::same(if compact_deck { 6 } else { 8 }))
                                .show(ui, |ui| {
                                    ui.set_min_height(92.0);
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(format!("[{}]", idx + 1))
                                                .font(mono_font(9.8))
                                                .color(if affordable { GOLD } else { TEXT_MUTED }),
                                        );
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    egui::RichText::new(u.tier.code())
                                                        .font(mono_font(9.2))
                                                        .color(tier_color(u.tier).linear_multiply(0.8)),
                                                );
                                            },
                                        );
                                    });
                                    ui.add_space(3.0);
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = if compact_deck { 4.0 } else { 7.0 };
                                        let ped_sz = if compact_deck { 26.0 } else { 36.0 };
                                        let (pr, _) = ui.allocate_exact_size(
                                            Vec2::splat(ped_sz),
                                            Sense::hover(),
                                        );
                                        portrait_pedestal(
                                            ui.painter(),
                                            pr,
                                            assets.get_portrait(u.sprite_key),
                                            if full_batch { GOLD_BRIGHT } else { BLUE_LEGION },
                                            affordable,
                                        );
                                        ui.vertical(|ui| {
                                            ui.add_space(6.0);
                                            ui.add(
                                                egui::Label::new(
                                                    egui::RichText::new(u.name)
                                                        .font(body_font(if compact_deck { 10.2 } else { 11.8 }))
                                                        .color(if unlocked {
                                                            TEXT_PRIMARY
                                                        } else {
                                                            TEXT_MUTED
                                                        }),
                                                )
                                                .truncate(),
                                            );
                                        });
                                    });
                                    ui.with_layout(
                                        egui::Layout::bottom_up(egui::Align::LEFT),
                                        |ui| {
                                            ui.horizontal(|ui| {
                                                let cost_txt = if mult == 1 {
                                                    format!("{}G", u.cost)
                                                } else {
                                                    format!("{batch_cost}G")
                                                };
                                                let cost_col = if !unlocked {
                                                    TEXT_MUTED
                                                } else if full_batch {
                                                    GOLD_BRIGHT
                                                } else if affordable {
                                                    GOLD
                                                } else {
                                                    TEXT_MUTED
                                                };
                                                ui.label(
                                                    egui::RichText::new(cost_txt)
                                                        .font(mono_font(11.0))
                                                        .color(cost_col),
                                                );
                                                ui.with_layout(
                                                    egui::Layout::right_to_left(egui::Align::Center),
                                                    |ui| {
                                                        let st = if !unlocked {
                                                            Some((format!("T{req_tier}"), RED_ENEMY))
                                                        } else if mult > 1 && full_batch {
                                                            Some((format!("x{mult}"), EMERALD))
                                                        } else if mult > 1 && affordable {
                                                            Some((format!("x{will_spawn}"), GOLD_BRIGHT))
                                                        } else {
                                                            None
                                                        };
                                                        if let Some((lbl, col)) = st {
                                                            ui.label(
                                                                egui::RichText::new(lbl)
                                                                    .font(mono_font(9.5))
                                                                    .color(col),
                                                            );
                                                        }
                                                    },
                                                );
                                            });
                                        },
                                    );
                                });

                            let u_w = unit_weight_vector(u);
                            let (dom_role, _) = u_w.dominant_label();
                            let w_total = u_w.raw_sum();
                            let w_batch = w_total * (will_spawn.max(1) as f32);
                            let click_resp = resp
                                .response
                                .interact(Sense::click())
                                .on_hover_cursor(if unlocked {
                                    egui::CursorIcon::PointingHand
                                } else {
                                    egui::CursorIcon::NotAllowed
                                })
                                .on_hover_text(format!(
                                    "{} ({} • {}G each | {}G for x{})\nClick or [{}]: Deploy {} unit(s)\nField Weight: {:.2} {} each ({:.2} for x{})\nHP {:.0} • DMG {:.0} • CD {:.2}s\n{}",
                                    u.name,
                                    u.shape.role_hint(),
                                    u.cost,
                                    batch_cost,
                                    mult,
                                    idx + 1,
                                    will_spawn,
                                    w_total,
                                    dom_role,
                                    w_batch,
                                    will_spawn.max(1),
                                    u.hp,
                                    u.dmg,
                                    u.cd,
                                    u.passive_desc
                                ));
                            if click_resp.clicked() && !self.paused {
                                state.player_try_spawn_slot_count(idx, self.spawn_mult, rng);
                            }
                        });
                    });

                    // Right: Commander Spells [Q] & [E]
                    ui.allocate_ui(Vec2::new(spell_w, 112.0), |ui| {
                        equal_columns(ui, 2, |ui, col_idx| {
                            if col_idx == 0 {
                                let q_rdy = state.player.q_cd <= 0.0 && !self.paused && !state.over;
                                let q_prog = if state.player.q_cd <= 0.0 {
                                    1.0
                                } else {
                                    (1.0 - state.player.q_cd / state.player.q_cd_max).clamp(0.0, 1.0)
                                };
                                let q_status = if state.player.q_cd > 0.0 {
                                    format!("{:.0}s", state.player.q_cd.ceil())
                                } else {
                                    "READY".into()
                                };
                                if draw_spell_button(
                                    ui,
                                    "[Q]",
                                    &q_status,
                                    state.player.faction.q_spell_name,
                                    "",
                                    q_prog,
                                    q_rdy,
                                    RED_ENEMY,
                                )
                                .on_hover_text(state.player.faction.q_spell_desc)
                                .clicked()
                                    && !self.paused
                                {
                                    state.cast_commander_spell_q(Team::Blue);
                                }
                            } else {
                                let e_rdy = state.player.e_cd <= 0.0 && !self.paused && !state.over;
                                let e_prog = if state.player.e_cd <= 0.0 {
                                    1.0
                                } else {
                                    (1.0 - state.player.e_cd / state.player.e_cd_max).clamp(0.0, 1.0)
                                };
                                let e_status = if state.player.e_cd > 0.0 {
                                    format!("{:.0}s", state.player.e_cd.ceil())
                                } else {
                                    "Ready".into()
                                };
                                if draw_spell_button(
                                    ui,
                                    "[E]",
                                    &e_status,
                                    state.player.faction.e_spell_name,
                                    "",
                                    e_prog,
                                    e_rdy,
                                    BLUE_LEGION,
                                )
                                .on_hover_text(state.player.faction.e_spell_desc)
                                .clicked()
                                    && !self.paused
                                {
                                    state.cast_commander_spell_e(Team::Blue);
                                }
                            }
                        });
                    });
                });
            });

        // 2b. Docked Tactical Army Orders & Batch Deploy Bar
        egui::Panel::bottom("battle_orders_bar")
            .exact_size(32.0)
            .frame(
                egui::Frame::new()
                    .fill(BG_BAR)
                    .inner_margin(Margin::symmetric(14, 4)),
            )
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    for (ord, hotkey, col) in [
                        (ArmyOrder::Attack, "Z", GOLD),
                        (ArmyOrder::Hold, "X", EMERALD),
                        (ArmyOrder::Retreat, "C", RED_ENEMY),
                    ] {
                        let active = state.player.order == ord;
                        let lbl = format!("[{hotkey}] {}", ord.label());
                        let btn = nav_tab_button(ui, &lbl, active, col, Vec2::new(90.0, 24.0))
                            .on_hover_text(ord.desc());
                        if btn.clicked() && !self.paused {
                            state.set_army_order(Team::Blue, ord);
                        }
                    }

                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(6.0);
                    for &mult in &SPAWN_MULTIPLIERS {
                        let active = self.spawn_mult == mult;
                        let lbl = format!("x{mult}");
                        let btn = nav_tab_button(ui, &lbl, active, GOLD_BRIGHT, Vec2::new(38.0, 24.0))
                            .on_hover_text(format!(
                                "Deploy up to {mult} troop{} per click or [1]–[6] press ([F] cycles)",
                                if mult == 1 { "" } else { "s" }
                            ));
                        if btn.clicked() {
                            self.spawn_mult = mult;
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let ai_ord_col = match state.ai.order {
                            ArmyOrder::Attack => RED_ENEMY,
                            ArmyOrder::Hold => GOLD,
                            ArmyOrder::Retreat => AMETHYST,
                        };
                        ui.label(
                            egui::RichText::new(format!("Enemy · {}", state.ai.order.label()))
                                .font(mono_font(10.0))
                                .color(ai_ord_col),
                        );
                    });
                });
            });

        // 3. Main Battlefield Canvas
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG_VOID))
            .show(ui, |ui| {
                let outer = ui.available_rect_before_wrap();
                let resp = ui.allocate_rect(outer, Sense::hover());

                // Tactical camera trauma update for SpellQ / BossImpact FX
                let high_fx = state
                    .effects
                    .iter()
                    .filter(|fx| {
                        matches!(
                            fx.kind,
                            VisualEffectKind::SpellQ(_) | VisualEffectKind::BossImpact { .. }
                        )
                    })
                    .count();
                if high_fx > self.last_fx_sig && !self.paused {
                    self.shake_trauma = (self.shake_trauma + 0.55).min(1.0);
                }
                self.last_fx_sig = high_fx;
                let dt_ui = ui.ctx().input(|i| i.stable_dt.min(0.05));
                if !self.paused {
                    self.shake_trauma = (self.shake_trauma - dt_ui * 2.2).max(0.0);
                }

                let mut cam_offset = Vec2::ZERO;
                if self.screen_shake_enabled && self.shake_trauma > 0.01 && !self.paused {
                    let amp = self.shake_trauma * self.shake_trauma * 8.0;
                    cam_offset = Vec2::new(
                        (state.time * 57.0).sin() * amp,
                        (state.time * 73.0).cos() * amp * 0.7,
                    );
                }

                // Uniform letterboxed viewport preserving exact 1400x500 tactical proportions
                let scale = (outer.width() / WORLD_W).min(outer.height() / WORLD_H).max(0.2);
                let arena_size = Vec2::new(WORLD_W * scale, WORLD_H * scale);
                let arena_rect = Rect::from_center_size(outer.center() + cam_offset, arena_size);

                let painter = ui.painter_at(outer);
                painter.rect_filled(outer, CornerRadius::ZERO, BG_VOID);

                self.paint_battlefield(&painter, arena_rect, scale, state, assets);

                // Hover Unit Inspector Tooltip
                if let Some(mpos) = resp.hover_pos() {
                    if arena_rect.contains(mpos) {
                        let wx = (mpos.x - arena_rect.min.x) / scale;
                        let wy = (mpos.y - arena_rect.min.y) / scale;
                        if let Some(hov) = state.units.iter().find(|u| {
                            u.hp > 0.0 && (u.x - wx).hypot(u.y - wy) <= u.radius + 8.0
                        }) {
                            let tip_pos = mpos + Vec2::new(14.0, -34.0);
                            egui::Area::new(egui::Id::unique("battle_unit_hover_tip"))
                                .fixed_pos(tip_pos)
                                .fade_in(false)
                                .interactable(false)
                                .show(ui.ctx(), |ui| {
                                    egui::Frame::new()
                                        .fill(BG_PANEL.linear_multiply(0.96))
                                        .stroke(Stroke::new(1.0, GOLD))
                                        .corner_radius(CornerRadius::same(8))
                                        .inner_margin(Margin::symmetric(10, 6))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    egui::RichText::new(&hov.name)
                                                        .font(body_font(12.0))
                                                        .color(if hov.team == Team::Blue { BLUE_LEGION } else { RED_ENEMY }),
                                                );
                                                ui.label(
                                                    egui::RichText::new(format!(
                                                        "{:.0}/{:.0} HP",
                                                        hov.hp.ceil(),
                                                        hov.max_hp
                                                    ))
                                                    .font(mono_font(11.0))
                                                    .color(TEXT_PRIMARY),
                                                );
                                                if hov.shield_hp > 0.0 {
                                                    ui.label(
                                                        egui::RichText::new(format!("+{:.0} SHD", hov.shield_hp))
                                                            .font(mono_font(10.5))
                                                            .color(CYAN),
                                                    );
                                                }
                                            });
                                            ui.label(
                                                egui::RichText::new(hov.def.passive_desc)
                                                    .font(body_font(10.5))
                                                    .color(GOLD),
                                            );
                                        });
                                });
                        }
                    }
                }

                // Top-Center Floating Boss Citadel HUD (only if a Titan Boss is alive)
                if let Some(boss) = state.active_boss() {
                    egui::Area::new(egui::Id::unique("battle_boss_hud"))
                        .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 92.0))
                        .fade_in(false)
                        .interactable(false)
                        .show(ui.ctx(), |ui| {
                            egui::Frame::new()
                                .fill(Color32::from_rgba_unmultiplied(15, 11, 14, 195))
                                .corner_radius(CornerRadius::same(12))
                                .inner_margin(Margin::symmetric(16, 9))
                                .show(ui, |ui| {
                                ui.set_width(356.0);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(&boss.name)
                                            .font(display_font(13.5))
                                            .color(GOLD_BRIGHT),
                                    );
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            let p_str = match boss.boss_phase {
                                                3 => "Phase 3 · Enraged",
                                                2 => "Phase 2 · Shielded",
                                                _ => "Phase 1",
                                            };
                                            ui.label(
                                                egui::RichText::new(p_str)
                                                    .font(mono_font(10.0))
                                                    .color(RED_ENEMY),
                                            );
                                        },
                                    );
                                });
                                ui.add_space(5.0);
                                let hp_pct = (boss.hp / boss.max_hp).clamp(0.0, 1.0);
                                beveled_progress_bar(
                                    ui,
                                    hp_pct,
                                    Vec2::new(356.0, 13.0),
                                    RED_ENEMY,
                                    Some(&format!(
                                        "{:.0} / {:.0} ({:.0}%)",
                                        boss.hp.ceil(),
                                        boss.max_hp,
                                        hp_pct * 100.0
                                    )),
                                );
                                if boss.shield_hp > 0.0 {
                                    ui.add_space(3.0);
                                    let shd_pct = (boss.shield_hp / 1200.0).clamp(0.0, 1.0);
                                    beveled_progress_bar(
                                        ui,
                                        shd_pct,
                                        Vec2::new(356.0, 8.0),
                                        CYAN,
                                        Some(&format!("Barrier {:.0}", boss.shield_hp.ceil())),
                                    );
                                }
                            });
                        });
                }

                // Help Controls Overlay Modal
                if self.show_help && !state.over {
                    draw_battle_help_modal(ui, &mut self.show_help);
                }

                // AI Brain & Weight Inspector Modal
                if self.show_brain && !state.over {
                    draw_ai_brain_modal(ui, &state.ai_brain, state.ai_difficulty, &mut self.show_brain);
                }

                // Pause Modal
                if self.paused && !state.over {
                    action = draw_pause_modal(ui, &mut self.paused);
                }

                // Victory / Defeat Debrief Modal
                if state.over {
                    if let Some(summary) = &self.outcome_committed {
                        action = draw_debrief_modal(ui, summary, state, assets, self.has_next_mission);
                    }
                }
            });

        action
    }

    fn paint_battlefield(
        &mut self,
        painter: &Painter,
        arena: Rect,
        scale: f32,
        state: &MatchState,
        assets: &AssetBank,
    ) {
        let to_scr = |wx: f32, wy: f32| -> Pos2 {
            Pos2::new(arena.min.x + wx * scale, arena.min.y + wy * scale)
        };
        let vis = painter.clip_rect().expand(120.0 * scale.max(0.5));

        // 1. Arena Biome Floor
        let enemy_col = state.ai.faction.id.accent_rgb();
        painter.rect_filled(arena, CornerRadius::same(8), Color32::from_rgb(16, 13, 11));

        if let Some(arena_tex) = assets.get_arena(state.ai.faction.id) {
            image_rect_tinted(
                painter,
                arena_tex,
                arena.shrink(2.0),
                Color32::from_rgb(238, 232, 224),
            );
            // Soft team endzone washes confined to the walkable battle floor (keeping sky/horizon clear)
            let blue_wing = Rect::from_min_max(
                to_scr(0.0, LANE_TOP),
                to_scr(WORLD_W * 0.22, LANE_BOTTOM),
            );
            painter.rect_filled(
                blue_wing,
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(46, 112, 224, 18),
            );
            let red_wing = Rect::from_min_max(
                to_scr(WORLD_W * 0.78, LANE_TOP),
                to_scr(WORLD_W, LANE_BOTTOM),
            );
            painter.rect_filled(
                red_wing,
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(enemy_col[0], enemy_col[1], enemy_col[2], 18),
            );
        } else {
            let blue_wing = Rect::from_min_max(arena.min, Pos2::new(arena.min.x + arena.width() * 0.25, arena.max.y));
            painter.rect_filled(blue_wing, CornerRadius::same(8), Color32::from_rgba_unmultiplied(37, 99, 235, 16));
            let red_wing = Rect::from_min_max(Pos2::new(arena.max.x - arena.width() * 0.25, arena.min.y), arena.max);
            painter.rect_filled(red_wing, CornerRadius::same(8), Color32::from_rgba_unmultiplied(enemy_col[0], enemy_col[1], enemy_col[2], 22));

            let lane_rect = Rect::from_min_max(to_scr(45.0, LANE_TOP), to_scr(WORLD_W - 45.0, LANE_BOTTOM));
            painter.rect_filled(lane_rect, CornerRadius::same(8), Color32::from_rgba_unmultiplied(28, 23, 19, 200));

            let grid_col = Stroke::new(1.0, Color32::from_rgba_unmultiplied(186, 148, 86, 18));
            for gx in (100..1400).step_by(70) {
                painter.line_segment([to_scr(gx as f32, LANE_TOP), to_scr(gx as f32, LANE_BOTTOM)], grid_col);
            }
            let mut gy = LANE_TOP;
            while gy <= LANE_BOTTOM {
                painter.line_segment([to_scr(45.0, gy), to_scr(WORLD_W - 45.0, gy)], grid_col);
                gy += 68.0;
            }
            painter.line_segment(
                [to_scr(45.0, LANE_TOP), to_scr(WORLD_W - 45.0, LANE_TOP)],
                Stroke::new(2.0, BORDER_STRONG),
            );
            painter.line_segment(
                [to_scr(45.0, LANE_BOTTOM), to_scr(WORLD_W - 45.0, LANE_BOTTOM)],
                Stroke::new(2.0, BORDER_STRONG),
            );
        }

        // Soft Arena Frame
        painter.rect_stroke(arena, CornerRadius::same(10), Stroke::new(1.0, BORDER_SUBTLE), StrokeKind::Inside);

        // Center Clash Line
        for step in (0..18).step_by(2) {
            let y0 = LANE_TOP + (LANE_BOTTOM - LANE_TOP) * (step as f32 / 18.0);
            let y1 = LANE_TOP + (LANE_BOTTOM - LANE_TOP) * ((step + 1) as f32 / 18.0);
            painter.line_segment(
                [to_scr(WORLD_W * 0.5, y0), to_scr(WORLD_W * 0.5, y1)],
                Stroke::new(1.5, GOLD.linear_multiply(0.38)),
            );
        }

        // Tactical Hold Indicator when Blue Commander issues HOLD (stops troops in place)
        if state.player.order == ArmyOrder::Hold {
            let mut sum_x = 0.0_f32;
            let mut count = 0usize;
            let mut hold_mesh = egui::Mesh::default();
            let hold_col = EMERALD.linear_multiply(0.55);
            for u in &state.units {
                if u.hp > 0.0 && u.team == Team::Blue {
                    sum_x += u.x;
                    count += 1;
                    // Subtle anchor ring under held troops (batched)
                    let upos = to_scr(u.x, u.y + u.radius * 0.4);
                    if vis.contains(upos) {
                        crate::assets::push_ring_stroke_mesh(
                            &mut hold_mesh,
                            upos,
                            u.radius * 0.85 * scale,
                            1.2 * scale,
                            hold_col,
                        );
                        crate::assets::flush_mesh_if_large(painter, &mut hold_mesh);
                    }
                }
            }
            if !hold_mesh.indices.is_empty() {
                painter.add(hold_mesh);
            }
            if count > 0 {
                let cx = sum_x / count as f32;
                let tag_p = to_scr(cx, LANE_TOP + 14.0);
                painter.text(
                    tag_p,
                    Align2::CENTER_TOP,
                    "HOLDING POSITION",
                    mono_font((9.5 * scale).clamp(8.0, 12.0)),
                    EMERALD,
                );
            }
        }

        // Frontline Tactical Minimap Strip along bottom parapet of arena (beveled housing + grid markers)
        if self.show_minimap && arena.width() > 420.0 {
            let mm_w = (arena.width() * 0.30).clamp(210.0, 340.0);
            let mm_h = 18.0;
            let mm_rect = Rect::from_center_size(
                Pos2::new(arena.center().x, arena.max.y - 14.0),
                Vec2::new(mm_w, mm_h),
            );
            paint_beveled_box(
                painter,
                mm_rect,
                5,
                Color32::from_rgba_unmultiplied(18, 15, 14, 215),
                Color32::from_rgba_unmultiplied(10, 9, 9, 225),
                GOLD.gamma_multiply(0.55),
                1.1,
                None,
            );
            let inner = mm_rect.shrink2(Vec2::new(7.0, 4.0));
            // Subtle 25%, 50%, 75% tactical midline hairlines
            for frac in [0.25_f32, 0.5, 0.75] {
                let gx = inner.left() + frac * inner.width();
                painter.line_segment(
                    [Pos2::new(gx, inner.top()), Pos2::new(gx, inner.bottom())],
                    Stroke::new(
                        if (frac - 0.5).abs() < 0.01 { 1.0 } else { 0.6 },
                        Color32::from_rgba_unmultiplied(212, 168, 72, if (frac - 0.5).abs() < 0.01 { 55 } else { 25 }),
                    ),
                );
            }
            for st in &state.structures {
                if st.hp <= 0.0 {
                    continue;
                }
                let nx = (st.x / WORLD_W).clamp(0.0, 1.0);
                let mx = inner.left() + nx * inner.width();
                let col = if st.team == Team::Blue { BLUE_LEGION } else { RED_ENEMY };
                let sz = if st.kind == grand_conquest_core::model::StructureKind::Base { 5.5 } else { 3.5 };
                painter.rect_filled(
                    Rect::from_center_size(Pos2::new(mx, inner.center().y), Vec2::splat(sz)),
                    CornerRadius::same(1),
                    col,
                );
            }
            let mut mm_mesh = egui::Mesh::default();
            for u in &state.units {
                if u.hp <= 0.0 || u.is_boss {
                    continue;
                }
                let nx = (u.x / WORLD_W).clamp(0.0, 1.0);
                let ny = ((u.y - LANE_TOP) / (LANE_BOTTOM - LANE_TOP)).clamp(0.0, 1.0);
                let mx = inner.left() + nx * inner.width();
                let my = inner.top() + ny * inner.height();
                let col = if u.team == Team::Blue { BLUE_LEGION } else { RED_ENEMY };
                crate::assets::push_solid_rect_mesh(
                    &mut mm_mesh,
                    Rect::from_center_size(Pos2::new(mx, my), Vec2::splat(3.0)),
                    col,
                );
                crate::assets::flush_mesh_if_large(painter, &mut mm_mesh);
            }
            if !mm_mesh.indices.is_empty() {
                painter.add(mm_mesh);
            }
            for u in &state.units {
                if u.hp > 0.0 && u.is_boss {
                    let nx = (u.x / WORLD_W).clamp(0.0, 1.0);
                    let ny = ((u.y - LANE_TOP) / (LANE_BOTTOM - LANE_TOP)).clamp(0.0, 1.0);
                    let mx = inner.left() + nx * inner.width();
                    let my = inner.top() + ny * inner.height();
                    painter.circle_stroke(Pos2::new(mx, my), 4.2, Stroke::new(1.0, RED_ENEMY));
                    painter.circle_filled(Pos2::new(mx, my), 2.8, GOLD_BRIGHT);
                }
            }
        }

        // Blitzkrieg <20s critical red border warning
        if let Some(bt) = state.blitz_timer {
            if bt <= 20.0 && (state.time * 4.0).sin() > 0.0 {
                painter.rect_stroke(
                    arena,
                    CornerRadius::same(8),
                    Stroke::new(4.0, Color32::from_rgba_unmultiplied(239, 68, 68, 140)),
                    StrokeKind::Inside,
                );
            }
        }

        // 2. Turret Rule Protection Tethers & Structures
        for st in &state.structures {
            if st.hp <= 0.0 || st.order == 1 {
                continue;
            }
            if let Some(prot) = state
                .structures
                .iter()
                .find(|o| o.team == st.team && o.order == st.order - 1 && o.hp > 0.0)
            {
                painter.line_segment(
                    [to_scr(prot.x, prot.y), to_scr(st.x, st.y)],
                    Stroke::new(1.8 * scale, Color32::from_rgba_unmultiplied(82, 196, 206, 52)),
                );
            }
        }

        for st in &state.structures {
            let center = to_scr(st.x, st.y);
            let rad = st.radius * scale;
            let is_blue = st.team == Team::Blue;
            let vuln = state.is_structure_vulnerable(st);

            if st.hp <= 0.0 {
                painter.circle(
                    center,
                    rad * 0.78,
                    Color32::from_rgba_unmultiplied(22, 17, 14, 215),
                    Stroke::new(1.6, BORDER_STRONG),
                );
                painter.text(center, Align2::CENTER_CENTER, "RUINED", mono_font(9.0 * scale), TEXT_MUTED);
                continue;
            }

            if !vuln {
                painter.circle(
                    center,
                    rad + 10.0 * scale,
                    Color32::from_rgba_unmultiplied(56, 208, 232, 10),
                    Stroke::new(1.3 * scale, Color32::from_rgba_unmultiplied(82, 196, 206, 130)),
                );
            }

            let drew_fort = self.render_mode == RenderMode::SkeletalRig
                && assets.get_fortress(st.team, st.kind).map(|fort_tex| {
                    let team_glow = if is_blue {
                        Color32::from_rgba_unmultiplied(64, 144, 232, 42)
                    } else {
                        Color32::from_rgba_unmultiplied(224, 68, 72, 42)
                    };
                    let ring_col = if is_blue { BLUE_LEGION } else { RED_ENEMY };
                    painter.circle(
                        center + Vec2::new(0.0, rad * 0.14),
                        rad * 1.06,
                        team_glow,
                        Stroke::new(1.5 * scale, ring_col.linear_multiply(0.55)),
                    );
                    let sprite_sz = if st.kind == StructureKind::Base {
                        rad * 3.05
                    } else {
                        rad * 2.85
                    };
                    let fort_rect = Rect::from_center_size(
                        center - Vec2::new(0.0, sprite_sz * 0.10),
                        Vec2::splat(sprite_sz),
                    );
                    portrait_rect(painter, fort_tex, fort_rect);
                }).is_some();

            if !drew_fort {
                let sides = if st.kind == StructureKind::Base { 8 } else { 6 };
                let pts: Vec<Pos2> = (0..sides)
                    .map(|i| {
                        let a = (std::f32::consts::TAU / sides as f32) * (i as f32 + 0.5);
                        Pos2::new(center.x + a.cos() * rad, center.y + a.sin() * rad)
                    })
                    .collect();
                let fill = if is_blue { Color32::from_rgb(29, 78, 216) } else { Color32::from_rgb(190, 18, 60) };
                let rim = if is_blue { Color32::from_rgb(147, 197, 253) } else { Color32::from_rgb(253, 164, 175) };
                painter.add(Shape::convex_polygon(pts, fill, Stroke::new(2.4 * scale, rim)));
                painter.circle_filled(center, rad * 0.42, rim);
            }

            let tag_offset = if drew_fort { rad * 1.44 } else { rad + 22.0 * scale };
            let bar_offset = if drew_fort { rad * 1.44 - 8.0 * scale } else { rad + 14.0 * scale };
            let tag = match st.kind {
                StructureKind::Base if st.is_invulnerable_override => "NEXUS [LOCKED]",
                StructureKind::Base => "BASE",
                StructureKind::Turret2 => "TURRET 2",
                StructureKind::Turret1 => "TURRET 1",
            };
            painter.text(
                center - Vec2::new(0.0, tag_offset + 5.0 * scale),
                Align2::CENTER_BOTTOM,
                tag,
                mono_font((10.0 * scale).max(8.0)),
                TEXT_PRIMARY,
            );
            if !vuln {
                painter.text(
                    center + Vec2::new(0.0, rad + 18.0 * scale),
                    Align2::CENTER_TOP,
                    "SHIELDED",
                    mono_font((8.5 * scale).max(7.0)),
                    CYAN,
                );
            }

            let bw = rad * 2.1;
            let bh = (5.5 * scale).max(3.5);
            let bar_r = Rect::from_min_size(Pos2::new(center.x - bw * 0.5, center.y - bar_offset), Vec2::new(bw, bh));
            painter.rect_filled(bar_r, CornerRadius::same(2), BG_VOID);
            painter.rect_filled(
                Rect::from_min_size(bar_r.min, Vec2::new(bw * (st.hp / st.max_hp).clamp(0.0, 1.0), bh)),
                CornerRadius::same(2),
                if !vuln { CYAN } else if is_blue { BLUE_LEGION } else { RED_ENEMY },
            );
        }

        // 3. Aura Circles (Commanders & Bosses)
        let aura_r_px = AURA_RADIUS_STUDS * STUD_TO_PX * 0.45 * scale;
        for u in &state.units {
            if u.hp <= 0.0 {
                continue;
            }
            let has_aura = u.is_boss
                || matches!(
                    u.passive,
                    PassiveKey::EliteKnight
                        | PassiveKey::EliteRiot
                        | PassiveKey::VoidWarden
                        | PassiveKey::CyberCmdr
                );
            if has_aura {
                let c = to_scr(u.x, u.y);
                let fill = if u.team == Team::Blue {
                    Color32::from_rgba_unmultiplied(59, 130, 246, 12)
                } else {
                    Color32::from_rgba_unmultiplied(239, 68, 68, 12)
                };
                let stroke = if u.team == Team::Blue {
                    Color32::from_rgba_unmultiplied(96, 165, 250, 65)
                } else {
                    Color32::from_rgba_unmultiplied(248, 113, 113, 65)
                };
                painter.circle(c, aura_r_px, fill, Stroke::new(1.2, stroke));
            }
        }

        // Ground Plasma Craters & Boss Telegraph Reticles (drawn under units)
        for fx in &state.effects {
            let p = 1.0 - (fx.life / fx.max_life).clamp(0.0, 1.0);
            let [r, g, b] = fx.color_rgb;
            let c = to_scr(fx.x, fx.y);
            match &fx.kind {
                VisualEffectKind::PlasmaCrater { radius, .. } => {
                    painter.circle(
                        c,
                        *radius * scale,
                        Color32::from_rgba_unmultiplied(r, g, b, 38),
                        Stroke::new(2.0, Color32::from_rgba_unmultiplied(r, g, b, 150)),
                    );
                }
                VisualEffectKind::BossTelegraph { radius, line_length, aim_angle, .. } => {
                    if *line_length > 0.0 {
                        let end = to_scr(
                            fx.x + aim_angle.cos() * *line_length,
                            fx.y + aim_angle.sin() * *line_length,
                        );
                        painter.line_segment([c, end], Stroke::new(42.0 * scale, Color32::from_rgba_unmultiplied(r, g, b, 55)));
                        painter.line_segment([c, end], Stroke::new(3.0, Color32::from_rgb(r, g, b)));
                    } else {
                        let rr = *radius * scale;
                        painter.circle(
                            c,
                            rr,
                            Color32::from_rgba_unmultiplied(r, g, b, 28),
                            Stroke::new(2.0, Color32::from_rgb(r, g, b)),
                        );
                        painter.circle_filled(
                            c,
                            rr * p,
                            Color32::from_rgba_unmultiplied(r, g, b, 55),
                        );
                    }
                }
                _ => {}
            }
        }

        // 4. Units Sorted by Y Depth (Zero-alloc scratch + 3-pass batched ground/atlas/bars)
        self.unit_sort_buf.clear();
        for (idx, u) in state.units.iter().enumerate() {
            if u.hp <= 0.0 {
                continue;
            }
            let rough = to_scr(u.x, u.y);
            if vis.contains(rough) {
                self.unit_sort_buf.push((u.y, idx));
            }
        }
        self.unit_sort_buf
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        self.unit_draw_meta.clear();
        self.unit_draw_meta.reserve(self.unit_sort_buf.len());

        // Pass 1: Status rings + Ground team ellipse pedestals in 1 untextured mesh batch
        let mut ground_mesh = egui::Mesh::default();
        for &(_, unit_idx) in &self.unit_sort_buf {
            let u = &state.units[unit_idx];
            let is_blue = u.team == Team::Blue;
            let surge_on = if is_blue {
                state.player.surge_timer > 0.0
            } else {
                state.ai.surge_timer > 0.0
            };
            let lx = u.x + u.aim_angle.cos() * u.lunge_offset;
            let ly = u.y + u.aim_angle.sin() * u.lunge_offset;
            let scr = to_scr(lx, ly);
            let rad = u.radius * scale;

            if surge_on {
                crate::assets::push_ring_stroke_mesh(
                    &mut ground_mesh,
                    scr,
                    rad + 5.0 * scale,
                    1.8,
                    EMERALD,
                );
            }
            if u.death_immune_timer > 0.0 || u.ranged_immune_timer > 0.0 {
                crate::assets::push_ellipse_fill_and_ring_mesh(
                    &mut ground_mesh,
                    scr,
                    rad + 7.0 * scale,
                    rad + 7.0 * scale,
                    Color32::from_rgba_unmultiplied(192, 132, 252, 68),
                    2.2,
                    AMETHYST,
                );
            }
            if u.poison_timer > 0.0 {
                crate::assets::push_ring_stroke_mesh(
                    &mut ground_mesh,
                    scr,
                    rad + 3.5 * scale,
                    1.8,
                    EMERALD,
                );
            }
            if u.boss_phase == 3 {
                crate::assets::push_ring_stroke_mesh(
                    &mut ground_mesh,
                    scr,
                    rad + 9.0 * scale,
                    2.4,
                    RED_ENEMY,
                );
            }
            if self.render_mode == RenderMode::SkeletalRig {
                crate::assets::push_soldier_ground_mesh(&mut ground_mesh, u, scr, scale, is_blue);
            }
            crate::assets::flush_mesh_if_large(painter, &mut ground_mesh);
            self.unit_draw_meta.push((scr, rad, false));
        }
        if !ground_mesh.indices.is_empty() {
            painter.add(ground_mesh);
        }

        // Pass 2: Y-sorted atlas unit sprites + melee slash crescents (automatic fallback preserves Y-sort)
        let mut sprites_mesh = assets
            .battle_atlas
            .as_ref()
            .map(|tex| egui::Mesh::with_texture(tex.id()));
        let mut slash_mesh = egui::Mesh::default();
        for (slot, &(_, unit_idx)) in self.unit_sort_buf.iter().enumerate() {
            let u = &state.units[unit_idx];
            let (scr, rad, _) = self.unit_draw_meta[slot];
            let is_blue = u.team == Team::Blue;
            let mut drew_rig = false;

            if self.render_mode == RenderMode::SkeletalRig {
                let meta = crate::assets::compute_soldier_draw_params(u, scale);
                if let Some(ref mut sm) = sprites_mesh {
                    if assets.push_soldier_atlas_sprite(
                        sm,
                        u,
                        scr,
                        meta.draw_size,
                        meta.facing,
                        meta.tint,
                    ) {
                        drew_rig = true;
                        crate::assets::flush_mesh_if_large(painter, sm);
                        assets.push_soldier_slash_mesh(
                            &mut slash_mesh,
                            u,
                            scr,
                            meta.draw_size,
                            meta.facing,
                            is_blue,
                        );
                        crate::assets::flush_mesh_if_large(painter, &mut slash_mesh);
                    }
                }
                if !drew_rig {
                    if let Some(ref mut sm) = sprites_mesh {
                        if !sm.indices.is_empty() {
                            let tid = sm.texture_id;
                            let full = std::mem::replace(sm, egui::Mesh::with_texture(tid));
                            painter.add(full);
                        }
                    }
                    let anim = crate::assets::AnimTiming {
                        atk_anim_timer: u.atk_anim_timer,
                        atk_anim_duration: u.atk_anim_duration,
                        is_moving: u.is_moving,
                        walk_phase: u.walk_phase,
                        idle_phase: u.idle_phase,
                    };
                    drew_rig = assets.paint_unit_sprite(
                        painter,
                        u.def.sprite_key,
                        scr,
                        meta.draw_size,
                        meta.facing,
                        anim,
                        meta.tint,
                    );
                    if drew_rig {
                        assets.push_soldier_slash_mesh(
                            &mut slash_mesh,
                            u,
                            scr,
                            meta.draw_size,
                            meta.facing,
                            is_blue,
                        );
                        crate::assets::flush_mesh_if_large(painter, &mut slash_mesh);
                    }
                }
            }

            if !drew_rig {
                if let Some(ref mut sm) = sprites_mesh {
                    if !sm.indices.is_empty() {
                        let tid = sm.texture_id;
                        let full = std::mem::replace(sm, egui::Mesh::with_texture(tid));
                        painter.add(full);
                    }
                }
                let fill = if u.hit_flash > 0.0 {
                    Color32::WHITE
                } else if is_blue {
                    BLUE_LEGION
                } else {
                    RED_ENEMY
                };
                let rim = if u.def.tier == grand_conquest_core::model::UnitTier::A || u.is_boss {
                    Color32::from_rgb(253, 224, 71)
                } else if is_blue {
                    Color32::from_rgb(147, 197, 253)
                } else {
                    Color32::from_rgb(252, 165, 165)
                };
                paint_unit_shape(painter, u.shape, scr, rad, u.aim_angle, fill, rim, 2.0 * scale);
            }

            self.unit_draw_meta[slot].2 = drew_rig;
        }
        if let Some(sm) = sprites_mesh {
            if !sm.indices.is_empty() {
                painter.add(sm);
            }
        }
        if !slash_mesh.indices.is_empty() {
            painter.add(slash_mesh);
        }

        // Pass 3: Batched HP & Shield Bars (all rendered in 1 draw call above units)
        let mut bars_mesh = egui::Mesh::default();
        let bar_bg = Color32::from_rgba_unmultiplied(10, 14, 24, 235);
        let bar_border = Color32::from_rgba_unmultiplied(2, 4, 8, 200);
        let shield_border = Color32::from_rgba_unmultiplied(2, 4, 8, 180);
        for (slot, &(_, unit_idx)) in self.unit_sort_buf.iter().enumerate() {
            let u = &state.units[unit_idx];
            let (scr, rad, drew_rig) = self.unit_draw_meta[slot];
            let is_blue = u.team == Team::Blue;
            let bar_y = if drew_rig {
                scr.y - rad * 2.15 - 7.0 * scale
            } else {
                scr.y - rad - 9.0 * scale
            };

            let show_hp = match self.hp_bar_mode {
                grand_conquest_core::settings::HpBarMode::Always => true,
                grand_conquest_core::settings::HpBarMode::Minimal => {
                    u.is_boss || u.hp < u.max_hp * 0.55 || u.shield_hp > 0.0 || self.show_labels
                }
                grand_conquest_core::settings::HpBarMode::Smart => {
                    u.is_boss
                        || u.tier == grand_conquest_core::model::UnitTier::A
                        || u.hp < u.max_hp * 0.88
                        || u.hit_flash > 0.0
                        || u.shield_hp > 0.0
                        || self.show_labels
                }
            };

            if show_hp {
                let bar_w = match (u.is_boss, u.tier) {
                    (true, _) => (rad * 2.4).clamp(32.0 * scale, 56.0 * scale),
                    (_, grand_conquest_core::model::UnitTier::A) => {
                        (rad * 2.0).clamp(24.0 * scale, 38.0 * scale)
                    }
                    _ => (rad * 1.6).clamp(14.0 * scale, 22.0 * scale),
                };
                let bar_h = if u.is_boss {
                    (4.5 * scale).clamp(3.5, 6.0)
                } else {
                    (2.8 * scale).clamp(2.0, 3.8)
                };
                let hp_r =
                    Rect::from_min_size(Pos2::new(scr.x - bar_w * 0.5, bar_y), Vec2::new(bar_w, bar_h));
                crate::assets::push_solid_rect_mesh(&mut bars_mesh, hp_r, bar_bg);
                crate::assets::push_rect_outline_mesh(&mut bars_mesh, hp_r, 0.8, bar_border);

                let fill_pct = (u.hp / u.max_hp).clamp(0.0, 1.0);
                if fill_pct > 0.0 {
                    crate::assets::push_solid_rect_mesh(
                        &mut bars_mesh,
                        Rect::from_min_size(hp_r.min, Vec2::new(bar_w * fill_pct, bar_h)),
                        if is_blue { BLUE_LEGION } else { RED_ENEMY },
                    );
                }

                if u.shield_hp > 0.0 {
                    let sh_w = bar_w * (u.shield_hp / u.max_hp.max(200.0)).clamp(0.05, 1.0);
                    let sh_r = Rect::from_min_size(
                        Pos2::new(scr.x - bar_w * 0.5, bar_y - (bar_h + 1.5)),
                        Vec2::new(sh_w, (bar_h * 0.8).clamp(1.8, 3.0)),
                    );
                    crate::assets::push_solid_rect_mesh(&mut bars_mesh, sh_r, CYAN);
                    crate::assets::push_rect_outline_mesh(&mut bars_mesh, sh_r, 0.6, shield_border);
                }
                crate::assets::flush_mesh_if_large(painter, &mut bars_mesh);
            }
        }
        if !bars_mesh.indices.is_empty() {
            painter.add(bars_mesh);
        }

        if self.show_labels {
            let label_font = body_font((9.5 * scale).clamp(7.5, 13.0));
            for (slot, &(_, unit_idx)) in self.unit_sort_buf.iter().enumerate() {
                let u = &state.units[unit_idx];
                let (scr, rad, drew_rig) = self.unit_draw_meta[slot];
                let bar_y = if drew_rig {
                    scr.y - rad * 2.15 - 7.0 * scale
                } else {
                    scr.y - rad - 9.0 * scale
                };
                painter.text(
                    Pos2::new(scr.x, bar_y - 5.0 * scale),
                    Align2::CENTER_BOTTOM,
                    &u.name,
                    label_font.clone(),
                    if u.is_boss || u.def.tier == grand_conquest_core::model::UnitTier::A {
                        GOLD_BRIGHT
                    } else {
                        TEXT_SECONDARY
                    },
                );
            }
        }

        // 5. Projectiles (Batched luminous directional bolts + tapered motion trail)
        let mut proj_mesh = egui::Mesh::default();
        for p in &state.projectiles {
            let pos = to_scr(p.x, p.y);
            if !vis.contains(pos) {
                continue;
            }
            let [r, g, b] = p.color_rgb;
            let dx = p.tx - p.x;
            let dy = p.ty - p.y;
            let d_sq = dx * dx + dy * dy;
            let inv_d = if d_sq > 1.0 { d_sq.sqrt().recip() } else { 1.0 };
            let tail_len = (p.radius * 3.8).clamp(8.0, 20.0);
            let tail_scr = to_scr(p.x - dx * inv_d * tail_len, p.y - dy * inv_d * tail_len);
            crate::assets::push_line_ribbon_mesh(
                &mut proj_mesh,
                tail_scr,
                pos,
                (p.radius * 1.35 * scale).max(1.5),
                Color32::from_rgba_unmultiplied(r, g, b, 115),
            );
            crate::assets::push_filled_circle_lut_mesh(
                &mut proj_mesh,
                pos,
                (p.radius + 3.0) * scale,
                Color32::from_rgba_unmultiplied(r, g, b, 65),
                false,
            );
            crate::assets::push_filled_circle_lut_mesh(
                &mut proj_mesh,
                pos,
                p.radius * scale,
                Color32::from_rgb(r, g, b),
                false,
            );
            crate::assets::push_filled_circle_lut_mesh(
                &mut proj_mesh,
                pos,
                (p.radius * 0.5).max(1.2) * scale,
                Color32::from_rgba_unmultiplied(255, 255, 255, 210),
                false,
            );
            crate::assets::flush_mesh_if_large(painter, &mut proj_mesh);
        }
        if !proj_mesh.indices.is_empty() {
            painter.add(proj_mesh);
        }

        // 6. Combat & Spell FX (Batched in 1 mesh)
        let mut fx_mesh = egui::Mesh::default();
        for fx in &state.effects {
            let progress = 1.0 - (fx.life / fx.max_life).clamp(0.0, 1.0);
            let alpha = ((1.0 - progress) * 235.0).clamp(0.0, 255.0) as u8;
            let [r, g, b] = fx.color_rgb;
            let pos = to_scr(fx.x, fx.y);

            match &fx.kind {
                VisualEffectKind::SpawnRing | VisualEffectKind::DeathBurst => {
                    if !vis.contains(pos) {
                        continue;
                    }
                    crate::assets::push_ring_stroke_mesh(
                        &mut fx_mesh,
                        pos,
                        (fx.radius + progress * 18.0) * scale,
                        2.0 * scale,
                        Color32::from_rgba_unmultiplied(r, g, b, alpha),
                    );
                }
                VisualEffectKind::HealWave => {
                    if !vis.contains(pos) {
                        continue;
                    }
                    let rad_px = fx.radius * progress * scale;
                    crate::assets::push_ellipse_fill_and_ring_mesh(
                        &mut fx_mesh,
                        pos,
                        rad_px,
                        rad_px,
                        Color32::from_rgba_unmultiplied(74, 222, 128, (alpha as f32 * 0.2) as u8),
                        2.4 * scale,
                        Color32::from_rgba_unmultiplied(74, 222, 128, alpha),
                    );
                }
                VisualEffectKind::SpellQ(fx_t) => {
                    let scol = match fx_t {
                        SpellFxType::Excalibur => Color32::from_rgb(251, 191, 36),
                        SpellFxType::Airstrike => Color32::from_rgb(56, 189, 248),
                        SpellFxType::Plasma => Color32::from_rgb(16, 185, 129),
                        SpellFxType::Hellfire => Color32::from_rgb(244, 63, 94),
                        SpellFxType::Void => Color32::from_rgb(168, 85, 247),
                        SpellFxType::Ion => Color32::from_rgb(249, 115, 22),
                    };
                    let beam_w = fx.radius * (1.0 - progress * 0.5) * scale;
                    let beam_r = Rect::from_min_max(
                        Pos2::new(pos.x - beam_w * 0.5, arena.min.y),
                        Pos2::new(pos.x + beam_w * 0.5, pos.y),
                    );
                    crate::assets::push_solid_rect_mesh(
                        &mut fx_mesh,
                        beam_r,
                        scol.linear_multiply((1.0 - progress * 0.8) * 0.55),
                    );
                    crate::assets::push_ring_stroke_mesh(
                        &mut fx_mesh,
                        pos,
                        fx.radius * (0.4 + progress * 0.6) * scale,
                        4.0 * scale,
                        scol.linear_multiply(1.0 - progress * 0.7),
                    );
                }
                VisualEffectKind::SpellE => {
                    let wx = if fx.team == Team::Blue {
                        progress * WORLD_W
                    } else {
                        WORLD_W - progress * WORLD_W
                    };
                    crate::assets::push_line_ribbon_mesh(
                        &mut fx_mesh,
                        to_scr(wx, LANE_TOP),
                        to_scr(wx, LANE_BOTTOM),
                        4.0 * scale,
                        Color32::from_rgba_unmultiplied(r, g, b, alpha),
                    );
                }
                VisualEffectKind::BossImpact { radius, line_length, aim_angle } => {
                    if *line_length > 0.0 {
                        let end = to_scr(
                            fx.x + aim_angle.cos() * *line_length,
                            fx.y + aim_angle.sin() * *line_length,
                        );
                        crate::assets::push_line_ribbon_mesh(
                            &mut fx_mesh,
                            pos,
                            end,
                            50.0 * scale * (1.0 - progress),
                            Color32::from_rgba_unmultiplied(r, g, b, alpha),
                        );
                    } else {
                        let rad_px = *radius * (0.6 + progress * 0.4) * scale;
                        crate::assets::push_ellipse_fill_and_ring_mesh(
                            &mut fx_mesh,
                            pos,
                            rad_px,
                            rad_px,
                            Color32::from_rgba_unmultiplied(r, g, b, (alpha as f32 * 0.35) as u8),
                            4.0 * scale,
                            Color32::from_rgba_unmultiplied(r, g, b, alpha),
                        );
                    }
                }
                VisualEffectKind::BossTelegraph { .. } | VisualEffectKind::PlasmaCrater { .. } => {}
            }
            crate::assets::flush_mesh_if_large(painter, &mut fx_mesh);
        }
        if !fx_mesh.indices.is_empty() {
            painter.add(fx_mesh);
        }

        // 7. Floating Combat Numbers (Culled to visible viewport)
        if self.show_damage_numbers {
            for ft in &state.floating_texts {
                let pos = to_scr(ft.x, ft.y);
                if !vis.contains(pos) {
                    continue;
                }
                let [r, g, b] = ft.color_rgb;
                let alpha = ((ft.life / 0.4).min(1.0) * 255.0).clamp(0.0, 255.0) as u8;
                painter.text(
                    pos,
                    Align2::CENTER_CENTER,
                    &ft.text,
                    mono_font((ft.size * scale).clamp(8.5, 20.0)),
                    Color32::from_rgba_unmultiplied(r, g, b, alpha),
                );
            }
        }
    }
}

fn objective_hud_strings(state: &MatchState) -> (String, String, String) {
    match state.archetype {
        MissionArchetype::HoldTheLine => {
            let rem = state.defense_timer.unwrap_or(0.0).ceil() as u32;
            let waves_rem = state.wave_queue.iter().filter(|w| !w.fired).count();
            (
                "HOLD THE LINE".into(),
                format!("{:02}:{:02}", rem / 60, rem % 60),
                format!("Survive waves • {waves_rem} wave(s) left"),
            )
        }
        MissionArchetype::Blitzkrieg => {
            let rem = state.blitz_timer.unwrap_or(0.0).ceil() as u32;
            (
                "BLITZKRIEG".into(),
                format!("{:02}:{:02}", rem / 60, rem % 60),
                "Raze Red Base before orbital strike!".into(),
            )
        }
        MissionArchetype::TitanSlayer => (
            "TITAN SLAYER".into(),
            "BOSS ACTIVE".into(),
            "Defeat the Titan & topple Enemy Base".into(),
        ),
        MissionArchetype::Mutator => {
            let prefix = if state.is_campaign { "CAMPAIGN" } else { "SKIRMISH" };
            if let Some(m) = state.active_mutator {
                (format!("{prefix}: {}", m.name().to_uppercase()), "MUTATOR".into(), m.desc().into())
            } else {
                (format!("{prefix} MUTATOR"), "ACTIVE".into(), "Breach enemy fortress".into())
            }
        }
        MissionArchetype::Siege => (
            if state.is_campaign { "CAMPAIGN SIEGE" } else { "SKIRMISH SIEGE" }.into(),
            "BREAK LINE".into(),
            "Destroy Turret 1 -> Turret 2 -> Base".into(),
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_fortress_cluster(
    ui: &mut Ui,
    is_blue: bool,
    base_hp: f32,
    base_max: f32,
    t2_hp: f32,
    t1_hp: f32,
    invuln: bool,
    compact: bool,
) {
    let col = if is_blue { BLUE_LEGION } else { RED_ENEMY };
    let title = if is_blue {
        if compact { "BASTION" } else { "BLUE BASTION" }
    } else if invuln {
        if compact { "LOCKED" } else { "NEXUS [LOCK]" }
    } else if compact {
        "CITADEL"
    } else {
        "RED CITADEL"
    };

    ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        if is_blue {
            draw_base_pill(ui, title, base_hp, base_max, col, compact);
            draw_turret_chip(ui, "T2", t2_hp, col, compact);
            draw_turret_chip(ui, "T1", t1_hp, col, compact);
        } else {
            draw_turret_chip(ui, "T1", t1_hp, col, compact);
            draw_turret_chip(ui, "T2", t2_hp, col, compact);
            draw_base_pill(ui, title, base_hp, base_max, col, compact);
        }
    });
}

fn draw_base_pill(ui: &mut Ui, title: &str, hp: f32, max_hp: f32, col: Color32, compact: bool) {
    let pill_w = if compact { 104.0 } else { 142.0 };
    let pad_x = if compact { 4 } else { 6 };
    let bar_w = pill_w - (pad_x as f32) * 2.0;
    ui.allocate_ui_with_layout(
        Vec2::new(pill_w, 32.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_min_size(Vec2::new(pill_w, 32.0));
            egui::Frame::new()
                .inner_margin(Margin::symmetric(pad_x, 3))
                .show(ui, |ui| {
                    ui.set_width(bar_w);
                    ui.spacing_mut().item_spacing.y = 3.0;
                    ui.horizontal(|ui| {
                        let fsz = if compact { 8.5 } else { 9.5 };
                        ui.label(egui::RichText::new(title).font(mono_font(fsz)).color(col));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("{:.0}/{:.0}", hp.ceil(), max_hp))
                                    .font(mono_font(fsz))
                                    .color(TEXT_PRIMARY),
                            );
                        });
                    });
                    beveled_progress_bar(
                        ui,
                        (hp / max_hp).clamp(0.0, 1.0),
                        Vec2::new(bar_w, 5.0),
                        col,
                        None,
                    );
                });
        },
    );
}

fn draw_turret_chip(ui: &mut Ui, tag: &str, hp: f32, col: Color32, compact: bool) {
    let alive = hp > 0.0;
    let w = if compact { 40.0 } else { 48.0 };
    let (r, _) = ui.allocate_exact_size(Vec2::new(w, 22.0), Sense::hover());
    ui.painter().text(
        r.center(),
        Align2::CENTER_CENTER,
        format!("{tag} {:.0}", hp.ceil()),
        mono_font(if compact { 9.0 } else { 9.5 }),
        if alive { col } else { TEXT_MUTED },
    );
}

fn draw_cmd_button(
    ui: &mut Ui,
    key: &str,
    top: &str,
    mid: &str,
    bot: &str,
    enabled: bool,
    accent: Color32,
) -> egui::Response {
    let w = ui.available_width();
    let box_r = Rect::from_min_size(ui.available_rect_before_wrap().min, Vec2::new(w, 108.0));
    let hovered = ui.rect_contains_pointer(box_r);
    let fill = if enabled {
        if hovered {
            Color32::from_rgba_unmultiplied(36, 31, 26, 232)
        } else {
            Color32::from_rgba_unmultiplied(28, 24, 21, 220)
        }
    } else {
        Color32::from_rgba_unmultiplied(20, 18, 19, 175)
    };
    ui.painter().rect_filled(box_r, CornerRadius::same(10), fill);
    if enabled || hovered {
        let stroke_col = if enabled {
            accent.linear_multiply(if hovered { 0.85 } else { 0.45 })
        } else {
            BORDER_SUBTLE
        };
        ui.painter().rect_stroke(box_r, CornerRadius::same(10), Stroke::new(1.0, stroke_col), StrokeKind::Inside);
    }
    let resp = egui::Frame::new()
        .inner_margin(Margin::same(7))
        .show(ui, |ui| {
            ui.set_min_height(92.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(key).font(mono_font(10.5)).color(accent));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(top).font(mono_font(9.5)).color(TEXT_SECONDARY));
                });
            });
            ui.add_space(5.0);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(mid)
                        .font(body_font(if w < 85.0 { 10.6 } else { 12.0 }))
                        .color(TEXT_PRIMARY),
                )
                .wrap(),
            );
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label(
                    egui::RichText::new(bot)
                        .font(mono_font(11.0))
                        .color(if enabled { GOLD_BRIGHT } else { TEXT_MUTED }),
                );
            });
        });
    resp.response
        .interact(Sense::click())
        .on_hover_cursor(if enabled { egui::CursorIcon::PointingHand } else { egui::CursorIcon::Default })
}

#[allow(clippy::too_many_arguments)]
fn draw_spell_button(
    ui: &mut Ui,
    key: &str,
    status: &str,
    name: &str,
    desc: &str,
    progress: f32,
    ready: bool,
    accent: Color32,
) -> egui::Response {
    let w = ui.available_width();
    let box_r = Rect::from_min_size(ui.available_rect_before_wrap().min, Vec2::new(w, 108.0));
    let hovered = ui.rect_contains_pointer(box_r);
    let fill = if ready {
        Color32::from_rgb(
            (26 + (accent.r() as u16 / 10)).min(255) as u8,
            (22 + (accent.g() as u16 / 10)).min(255) as u8,
            (22 + (accent.b() as u16 / 10)).min(255) as u8,
        )
    } else {
        Color32::from_rgba_unmultiplied(20, 18, 19, 175)
    };
    ui.painter().rect_filled(box_r, CornerRadius::same(10), fill);
    if ready || hovered {
        let stroke_col = if ready {
            accent.linear_multiply(if hovered { 0.9 } else { 0.55 })
        } else {
            BORDER_SUBTLE
        };
        ui.painter().rect_stroke(box_r, CornerRadius::same(10), Stroke::new(1.0, stroke_col), StrokeKind::Inside);
    }
    let resp = egui::Frame::new()
        .inner_margin(Margin::same(8))
        .show(ui, |ui| {
            ui.set_min_height(92.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(key).font(mono_font(11.0)).color(accent));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(status)
                            .font(mono_font(10.0))
                            .color(if ready { EMERALD } else { TEXT_MUTED }),
                    );
                });
            });
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.add_space(4.0);
            let title_sz = if w < 110.0 { 10.5 } else { 12.0 };
            ui.add(
                egui::Label::new(
                    egui::RichText::new(name)
                        .font(body_font(title_sz))
                        .color(if ready { GOLD_BRIGHT } else { TEXT_PRIMARY }),
                )
                .wrap(),
            );
            if !desc.is_empty() {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(desc).font(body_font(if w < 110.0 { 9.5 } else { 10.2 })).color(TEXT_SECONDARY),
                    )
                    .truncate(),
                );
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                beveled_progress_bar(
                    ui,
                    progress.clamp(0.0, 1.0),
                    Vec2::new(ui.available_width(), 6.0),
                    accent,
                    None,
                );
            });
        });
    resp.response
        .interact(Sense::click())
        .on_hover_cursor(if ready { egui::CursorIcon::PointingHand } else { egui::CursorIcon::Default })
}

fn paint_modal_scrim(ctx: &egui::Context, id: &str) {
    let screen = ctx.input(|i| i.content_rect());
    egui::Area::new(egui::Id::unique(id))
        .fixed_pos(screen.min)
        .order(egui::Order::Middle)
        .interactable(true)
        .show(ctx, |ui| {
            let (r, _) = ui.allocate_exact_size(screen.size(), Sense::click());
            ui.painter()
                .rect_filled(r, CornerRadius::ZERO, Color32::from_rgba_unmultiplied(6, 5, 5, 196));
        });
}

fn draw_battle_help_modal(ui: &mut Ui, show_help: &mut bool) {
    paint_modal_scrim(ui.ctx(), "battle_help_scrim");
    let vp = ui.ctx().input(|i| i.content_rect());
    let modal_w = (vp.width() - 48.0).clamp(320.0, 460.0);
    let max_scroll_h = (vp.height() - 185.0).max(200.0);

    egui::Area::new(egui::Id::unique("battle_help_modal"))
        .order(egui::Order::Foreground)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .fade_in(false)
        .show(ui.ctx(), |ui| {
            sculpted_panel(ui, GOLD, 18, |ui| {
                ui.set_width(modal_w);
                ui.label(
                    egui::RichText::new("BATTLEFIELD QUICK GUIDE")
                        .font(display_font(19.0))
                        .color(GOLD_BRIGHT),
                );
                ui.add_space(6.0);
                filigree_divider(ui, GOLD_DIM);
                ui.add_space(8.0);
                egui::ScrollArea::vertical()
                    .id_salt("battle_help_scroll")
                    .max_height(max_scroll_h)
                    .show(ui, |ui| {
                        for (key, desc) in [
                            ("[U] Boost Economy", "Invest gold early to grow passive +2G/s income faster."),
                            ("[1]–[6] Deploy Troops", "Click cards or press 1–6 to march units from your Bastion."),
                            ("[F] Batch Deploy (x1–x50)", "Pick x1/x5/x10/x20/x30/x50 above deck (or press [F]) to spawn entire waves per click."),
                            ("[B] AI Brain Inspector", "Toggle the live node decision tree, troop weight vectors, and adaptive counter meters."),
                            ("[Z] [X] [C] Army Orders", "[Z] Attack forward • [X] Hold current ground • [C] Retreat to Bastion (+30% SPD)."),
                            ("[T] Unlock Tier B / A", "Once Tier B/A units are in your deck, press [T] mid-battle to summon them."),
                            ("[Q] & [E] Commander Spells", "[Q] blasts front enemies automatically; [E] heals & speeds up your whole army."),
                            ("Turret Protection Rule", "Back turrets & Base stay shielded until the front turret falls!"),
                            ("[Space] Pause • [Tab] Speed", "Cycle 0.5x / 1x / 2x / 3x battle speed anytime."),
                        ] {
                            well_frame().show(ui, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.label(egui::RichText::new(key).font(body_font(13.0)).color(GOLD_BRIGHT));
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(desc).font(body_font(12.0)).color(TEXT_SECONDARY),
                                    )
                                    .wrap(),
                                );
                            });
                            ui.add_space(5.0);
                        }
                    });
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    if cta_button(ui, "GOT IT, COMMANDER", Vec2::new((modal_w - 40.0).min(240.0), 36.0)).clicked() {
                        *show_help = false;
                    }
                });
            });
        });
}

fn draw_pause_modal(ui: &mut Ui, paused: &mut bool) -> BattleAction {
    let mut action = BattleAction::None;
    paint_modal_scrim(ui.ctx(), "battle_pause_scrim");
    egui::Area::new(egui::Id::unique("battle_pause_modal"))
        .order(egui::Order::Foreground)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .fade_in(false)
        .show(ui.ctx(), |ui| {
            sculpted_panel(ui, GOLD, 22, |ui| {
                ui.set_width(308.0);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("BATTLE PAUSED").font(display_font(22.0)).color(GOLD_BRIGHT));
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("Command Simulation Suspended").font(mono_font(10.0)).color(TEXT_MUTED));
                    ui.add_space(8.0);
                    filigree_divider(ui, GOLD_DIM);
                    ui.add_space(12.0);
                    if cta_button(ui, "RESUME BATTLE", Vec2::new(248.0, 38.0)).clicked() {
                        *paused = false;
                    }
                    ui.add_space(8.0);
                    if nav_tab_button(ui, "Settings & Audio", false, CYAN, Vec2::new(248.0, 34.0)).clicked() {
                        action = BattleAction::OpenSettings;
                    }
                    ui.add_space(8.0);
                    if nav_tab_button(ui, "Restart Match", false, GOLD, Vec2::new(248.0, 34.0)).clicked() {
                        action = BattleAction::RestartBattle;
                    }
                    ui.add_space(8.0);
                    if nav_tab_button(ui, "Retreat to Command", false, RED_ENEMY, Vec2::new(248.0, 34.0)).clicked() {
                        action = BattleAction::ReturnToMenu;
                    }
                });
            });
        });
    action
}

fn draw_debrief_modal(
    ui: &mut Ui,
    summary: &MatchOutcomeSummary,
    state: &MatchState,
    assets: &AssetBank,
    has_next_mission: bool,
) -> BattleAction {
    let mut action = BattleAction::None;
    paint_modal_scrim(ui.ctx(), "battle_debrief_scrim");
    let vp = ui.ctx().input(|i| i.content_rect());
    let modal_w = (vp.width() - 48.0).clamp(360.0, 520.0);
    let max_scroll_h = (vp.height() - 175.0).max(240.0);

    egui::Area::new(egui::Id::unique("battle_debrief_modal"))
        .order(egui::Order::Foreground)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .fade_in(false)
        .show(ui.ctx(), |ui| {
            let rim = if summary.is_victory { GOLD_BRIGHT } else { RED_ENEMY };
            sculpted_panel(ui, rim, 20, |ui| {
                ui.set_width(modal_w);
                egui::ScrollArea::vertical()
                    .id_salt("battle_debrief_scroll")
                    .max_height(max_scroll_h)
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            let mins = (summary.elapsed_secs as u32) / 60;
                            let secs = (summary.elapsed_secs as u32) % 60;
                            let kd = state.stats.enemy_kills as f32 / (state.stats.blue_casualties.max(1) as f32);
                            let rank = if !summary.is_victory {
                                "C"
                            } else if summary.stars_earned >= 3 || (!summary.is_campaign && kd >= 2.2) {
                                "S"
                            } else if summary.stars_earned >= 2 || (!summary.is_campaign && kd >= 1.3) {
                                "A"
                            } else {
                                "B"
                            };
                            ui.horizontal_wrapped(|ui| {
                                draw_badge(
                                    ui,
                                    if summary.is_victory { "VICTORY ACCLAIMED" } else { "VANGUARD SHATTERED" },
                                    rim.linear_multiply(0.18),
                                    rim,
                                );
                                draw_badge(
                                    ui,
                                    &format!("TACTICAL GRADE: {rank}"),
                                    GOLD_DARK.linear_multiply(0.8),
                                    if rank == "S" { GOLD_BRIGHT } else { TEXT_PRIMARY },
                                );
                            });
                            ui.add_space(6.0);
                            ui.label(egui::RichText::new(&summary.mission_name).font(display_font(22.0)).color(TEXT_PRIMARY));
                            ui.label(
                                egui::RichText::new(format!("Match Duration: {mins:02}:{secs:02}"))
                                    .font(mono_font(12.0))
                                    .color(TEXT_SECONDARY),
                            );
                        });

                        ui.add_space(8.0);
                        filigree_divider(ui, rim.gamma_multiply(0.55));
                        ui.add_space(8.0);

                        // 4-Column After-Action Telemetry
                        equal_columns(ui, 4, |ui, col| {
                            let (lbl, val, tint) = match col {
                                0 => ("DEPLOYED", format!("{}", state.stats.units_spawned), BLUE_LEGION),
                                1 => ("SLAIN", format!("{}", state.stats.enemy_kills), GOLD_BRIGHT),
                                2 => ("LOSSES / K:D", format!("{} ({kd_str})", state.stats.blue_casualties, kd_str = format!("{:.1}x", state.stats.enemy_kills as f32 / (state.stats.blue_casualties.max(1) as f32))), EMERALD),
                                _ => ("GOLD EARNED", format!("{:.0}G", state.stats.gold_earned), GOLD),
                            };
                            sculpted_card(ui, Some(tint.gamma_multiply(0.55)), 8, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.label(egui::RichText::new(lbl).font(mono_font(9.2)).color(TEXT_MUTED));
                                ui.label(egui::RichText::new(val).font(display_font(15.0)).color(tint));
                            });
                        });

                        ui.add_space(10.0);
                        if summary.is_campaign {
                            ui.vertical_centered(|ui| {
                                ui.allocate_ui(Vec2::new(114.0, 36.0), |ui| {
                                    ui.horizontal(|ui| {
                                        for i in 1..=3u8 {
                                            let (r, _) = ui.allocate_exact_size(Vec2::splat(34.0), Sense::hover());
                                            let earned = summary.stars_earned >= i;
                                            paint_star(
                                                ui.painter(),
                                                r.center(),
                                                14.0,
                                                earned,
                                                if earned { GOLD_BRIGHT } else { TEXT_MUTED },
                                            );
                                        }
                                    });
                                });
                            });
                            ui.add_space(6.0);
                            well_frame().show(ui, |ui| {
                                ui.set_min_width(ui.available_width());
                                for sd in &summary.star_details {
                                    ui.horizontal(|ui| {
                                        draw_badge(
                                            ui,
                                            if sd.earned { "PASS" } else { "MISS" },
                                            (if sd.earned { EMERALD } else { TEXT_MUTED }).linear_multiply(0.16),
                                            if sd.earned { EMERALD } else { TEXT_MUTED },
                                        );
                                        ui.add(
                                            egui::Label::new(
                                                egui::RichText::new(&sd.desc)
                                                    .font(body_font(12.0))
                                                    .color(if sd.earned { TEXT_PRIMARY } else { TEXT_MUTED }),
                                            )
                                            .wrap(),
                                        );
                                    });
                                }
                            });
                        }

                        if summary.gold_bounty > 0
                            || !summary.unlocked_blueprints.is_empty()
                            || !summary.newly_unlocked_relics.is_empty()
                        {
                            ui.add_space(8.0);
                            sculpted_card(ui, Some(GOLD), 10, |ui| {
                                ui.set_min_width(ui.available_width());
                                if summary.gold_bounty > 0 {
                                    ui.label(
                                        egui::RichText::new(format!("+{}G Treasury Bounty", summary.gold_bounty))
                                            .font(mono_font(13.0))
                                            .color(GOLD_BRIGHT),
                                    );
                                }
                                for bp in &summary.unlocked_blueprints {
                                    if let Some(ud) = get_unit(bp) {
                                        ui.horizontal(|ui| {
                                            let (pr, _) = ui.allocate_exact_size(Vec2::splat(30.0), Sense::hover());
                                            portrait_pedestal(
                                                ui.painter(),
                                                pr,
                                                assets.get_portrait(ud.sprite_key),
                                                EMERALD,
                                                true,
                                            );
                                            ui.label(
                                                egui::RichText::new(format!("Unlocked Blueprint: {} ({}-Tier)", ud.name, ud.tier.code()))
                                                    .font(body_font(12.0))
                                                    .color(EMERALD),
                                            );
                                        });
                                    }
                                }
                                for r in &summary.newly_unlocked_relics {
                                    ui.label(
                                        egui::RichText::new(format!("Unlocked War Relic: {r}!"))
                                            .font(body_font(12.0))
                                            .color(BLUE_LEGION),
                                    );
                                }
                            });
                        }
                    });

                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if nav_tab_button(ui, "Replay", false, GOLD, Vec2::new(112.0, 38.0)).clicked() {
                        action = BattleAction::RestartBattle;
                    }
                    if summary.is_victory && summary.is_campaign && has_next_mission {
                        if nav_tab_button(ui, "Atlas", false, TEXT_SECONDARY, Vec2::new(112.0, 38.0)).clicked() {
                            action = BattleAction::ReturnToMenu;
                        }
                        if cta_button(ui, "NEXT MISSION ->", Vec2::new((modal_w - 240.0).max(160.0), 38.0)).clicked() {
                            action = BattleAction::NextMission;
                        }
                    } else if cta_button(ui, "CONTINUE ->", Vec2::new((modal_w - 122.0).max(200.0), 38.0)).clicked() {
                        action = BattleAction::ReturnToMenu;
                    }
                });
            });
        });
    action
}

fn draw_ai_brain_modal(
    ui: &mut Ui,
    brain: &AiBrainTelemetry,
    diff: AiDifficulty,
    show_brain: &mut bool,
) {
    paint_modal_scrim(ui.ctx(), "battle_brain_scrim");
    let vp = ui.ctx().input(|i| i.content_rect());
    let modal_w = (vp.width() - 48.0).clamp(380.0, 540.0);
    let max_scroll_h = (vp.height() - 170.0).max(220.0);

    egui::Area::new(egui::Id::unique("battle_ai_brain_modal"))
        .order(egui::Order::Foreground)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .fade_in(false)
        .show(ui.ctx(), |ui| {
            sculpted_panel(ui, AMETHYST, 18, |ui| {
                ui.set_width(modal_w);
                // Header
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        egui::RichText::new("AI BRAIN & WEIGHT INSPECTOR")
                            .font(display_font(18.0))
                            .color(GOLD_BRIGHT),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let badge_text = match diff {
                            AiDifficulty::Easy => "EASY PATROL",
                            AiDifficulty::Normal => "NORMAL • BALANCED",
                            AiDifficulty::Hard => "HARD • TRAINED TREE",
                            AiDifficulty::Extreme => "EXTREME • WARMIND GRANDMASTER",
                        };
                        draw_badge(ui, badge_text, AMETHYST.linear_multiply(0.20), AMETHYST);
                    });
                });
                ui.add_space(6.0);
                filigree_divider(ui, AMETHYST.gamma_multiply(0.5));
                ui.add_space(6.0);

                egui::ScrollArea::vertical()
                    .id_salt("battle_brain_scroll")
                    .max_height(max_scroll_h)
                    .show(ui, |ui| {
                        // Section 1: Live Army Weight Comparison (Blue vs Red)
                        sculpted_card(ui, Some(BORDER_STRONG), 10, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.label(
                                egui::RichText::new("BATTLEFIELD ARMY WEIGHT VECTORS")
                                    .font(mono_font(10.5))
                                    .color(GOLD_BRIGHT),
                            );
                            ui.add_space(4.0);
                            let max_w = [
                                brain.blue_weight.swarm,
                                brain.blue_weight.tank,
                                brain.blue_weight.ranged_dps,
                                brain.blue_weight.magic_aoe,
                                brain.blue_weight.support,
                                brain.blue_weight.assassin,
                                brain.red_weight.swarm,
                                brain.red_weight.tank,
                                brain.red_weight.ranged_dps,
                                brain.red_weight.magic_aoe,
                                brain.red_weight.support,
                                brain.red_weight.assassin,
                                1.0,
                            ]
                            .into_iter()
                            .fold(1.0_f32, f32::max);

                            equal_columns(ui, 2, |ui, col| {
                                if col == 0 {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Blue Total: {:.2} Wgt",
                                            brain.blue_effective_weight
                                        ))
                                        .font(mono_font(11.5))
                                        .color(BLUE_LEGION),
                                    );
                                    let bw = brain.blue_weight;
                                    draw_weight_row(ui, "Swarm (Fodder)", bw.swarm, max_w, BLUE_LEGION);
                                    draw_weight_row(ui, "Armor (Tanks)", bw.tank, max_w, BLUE_LEGION);
                                    draw_weight_row(ui, "Ranged (DPS)", bw.ranged_dps, max_w, BLUE_LEGION);
                                    draw_weight_row(ui, "Magic / AoE", bw.magic_aoe, max_w, BLUE_LEGION);
                                    draw_weight_row(ui, "Support / Heal", bw.support, max_w, BLUE_LEGION);
                                    draw_weight_row(ui, "Striker / Dive", bw.assassin, max_w, BLUE_LEGION);
                                } else {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Red Total: {:.2} Wgt",
                                            brain.red_effective_weight
                                        ))
                                        .font(mono_font(11.5))
                                        .color(RED_ENEMY),
                                    );
                                    let rw = brain.red_weight;
                                    draw_weight_row(ui, "Swarm (Fodder)", rw.swarm, max_w, RED_ENEMY);
                                    draw_weight_row(ui, "Armor (Tanks)", rw.tank, max_w, RED_ENEMY);
                                    draw_weight_row(ui, "Ranged (DPS)", rw.ranged_dps, max_w, RED_ENEMY);
                                    draw_weight_row(ui, "Magic / AoE", rw.magic_aoe, max_w, RED_ENEMY);
                                    draw_weight_row(ui, "Support / Heal", rw.support, max_w, RED_ENEMY);
                                    draw_weight_row(ui, "Striker / Dive", rw.assassin, max_w, RED_ENEMY);
                                }
                            });
                        });

                        ui.add_space(8.0);
                        // Section 2: Player Habit Tracking & Adaptive Counter Bias
                        sculpted_card(ui, Some(BORDER_SUBTLE), 10, |ui| {
                            ui.set_min_width(ui.available_width());
                            let tracker = &brain.player_tracker;
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    egui::RichText::new("ONLINE PLAYER STYLE TRACKING")
                                        .font(mono_font(10.5))
                                        .color(GOLD_BRIGHT),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Max Batch: x{} • Units Seen: {}",
                                            tracker.max_batch_seen.max(1),
                                            tracker.total_units_seen
                                        ))
                                        .font(mono_font(10.0))
                                        .color(TEXT_SECONDARY),
                                    );
                                });
                            });
                            ui.add_space(4.0);
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(tracker.adaptation_summary())
                                        .font(body_font(12.0))
                                        .color(if tracker.swarm_threat_bias >= 0.40 {
                                            RED_ENEMY
                                        } else {
                                            EMERALD
                                        }),
                                )
                                .wrap(),
                            );
                            if brain.bait_mode_active {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(
                                            "ACTIVE TACTIC: Baiting player troops into turret range to trade with +140% efficiency",
                                        )
                                        .font(mono_font(10.0))
                                        .color(GOLD_BRIGHT),
                                    )
                                    .wrap(),
                                );
                            }
                        });

                        ui.add_space(6.0);
                        let win_prob_pct = (brain.neural_win_prob * 100.0).clamp(-99.0, 99.0);
                        let prob_col = if win_prob_pct >= 0.0 { GOLD_BRIGHT } else { RED_ENEMY };
                        well_frame().show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    egui::RichText::new("NEURAL INTUITION:")
                                        .font(mono_font(10.0))
                                        .color(AMETHYST),
                                );
                                ui.label(
                                    egui::RichText::new(format!("Win Advantage {win_prob_pct:+.0}%"))
                                        .font(mono_font(10.0))
                                        .color(prob_col),
                                );
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(format!(
                                            "Policy [Push: {:.0}% | Bait: {:.0}% | Retreat: {:.0}% | SpellQ: {:.0}% | Macro: {:.0}%]",
                                            brain.neural_policy[0] * 100.0,
                                            brain.neural_policy[1] * 100.0,
                                            brain.neural_policy[2] * 100.0,
                                            brain.neural_policy[3] * 100.0,
                                            brain.neural_policy[5] * 100.0,
                                        ))
                                        .font(mono_font(9.5))
                                        .color(TEXT_MUTED),
                                    )
                                    .wrap(),
                                );
                            });
                        });

                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "CANDIDATE DECISION NODES ({} Evaluated • 2.5s Combat Rollout)",
                                brain.total_nodes_evaluated
                            ))
                            .font(mono_font(10.5))
                            .color(GOLD_BRIGHT),
                        );
                        ui.add_space(4.0);
                        for (i, node) in brain.top_nodes.iter().take(4).enumerate() {
                            let chosen = node.chosen;
                            let card_acc = if chosen { Some(GOLD_BRIGHT) } else { Some(BORDER_SUBTLE) };
                            sculpted_card(ui, card_acc, 8, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.horizontal(|ui| {
                                    let tag = if chosen {
                                        format!("[#{}] CHOSEN", i + 1)
                                    } else {
                                        format!("[#{}]", i + 1)
                                    };
                                    ui.label(
                                        egui::RichText::new(tag)
                                            .font(mono_font(10.0))
                                            .color(if chosen { GOLD_BRIGHT } else { TEXT_MUTED }),
                                    );
                                    ui.label(
                                        egui::RichText::new(&node.label)
                                            .font(body_font(12.0))
                                            .color(TEXT_PRIMARY),
                                    );
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        let eff_col = if node.efficiency_per_gold >= 2.5 {
                                            EMERALD
                                        } else {
                                            GOLD_BRIGHT
                                        };
                                        ui.label(
                                            egui::RichText::new(format!("Eff: {:.1}x", node.efficiency_per_gold))
                                                .font(mono_font(10.0))
                                                .color(eff_col),
                                        );
                                        let ord_col = match node.order {
                                            ArmyOrder::Attack => GOLD,
                                            ArmyOrder::Hold => EMERALD,
                                            ArmyOrder::Retreat => RED_ENEMY,
                                        };
                                        draw_badge(ui, node.order.label(), ord_col.linear_multiply(0.18), ord_col);
                                    });
                                });
                            });
                            ui.add_space(3.0);
                        }
                    });

                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    if cta_button(ui, "CLOSE INSPECTOR [B]", Vec2::new((modal_w - 40.0).min(280.0), 36.0)).clicked() {
                        *show_brain = false;
                    }
                });
            });
        });
}

fn draw_weight_row(ui: &mut Ui, label: &str, val: f32, max_val: f32, col: Color32) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).font(body_font(10.5)).color(TEXT_SECONDARY));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!("{:.2}", val))
                    .font(mono_font(10.0))
                    .color(if val > 0.05 { col } else { TEXT_MUTED }),
            );
        });
    });
    beveled_progress_bar(
        ui,
        (val / max_val).clamp(0.0, 1.0),
        Vec2::new(ui.available_width(), 5.0),
        col,
        None,
    );
    ui.add_space(2.0);
}
