use egui::{Color32, CornerRadius, Margin, Pos2, ScrollArea, Sense, Stroke, Ui, Vec2};
use grand_conquest_core::model::FactionId;
use grand_conquest_core::units::{get_faction, get_unit, UNITS};

use crate::assets::{
    compute_sheet_frame, image_rect_tinted, AnimTiming, AssetBank, PreviewPose,
};
use crate::theme::{
    beveled_progress_bar, body_font, crest_socket, display_font, equal_columns, faction_color,
    mono_font, nav_tab_button, paint_parchment_backdrop, paint_unit_shape,
    paint_vertical_gradient, portrait_pedestal, sculpted_card, BG_VOID, BLUE_LEGION, EMERALD, GOLD,
    GOLD_BRIGHT, RED_ENEMY, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

pub struct CodexViewState {
    pub selected_unit_id: &'static str,
    pub filter_faction: Option<FactionId>,
    pub preview_pose: PreviewPose,
    pub facing_right: bool,
    pub show_exploded_parts: bool,
    pub anim_t: f32,
}

impl Default for CodexViewState {
    fn default() -> Self {
        Self {
            selected_unit_id: "sol_1",
            filter_faction: None,
            preview_pose: PreviewPose::Walk,
            facing_right: true,
            show_exploded_parts: false,
            anim_t: 0.0,
        }
    }
}

impl CodexViewState {
    pub fn draw(&mut self, ui: &mut Ui, assets: &AssetBank, dt: f32) {
        self.anim_t += dt;

        // Compute dynamic unit stat maxima for honest relative gauge scaling
        let (max_hp, max_dmg, max_rng, max_dps, max_spd) = UNITS.iter().fold(
            (1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32),
            |(hp, dmg, rng, dps, spd), u| {
                let u_dps = u.dmg / u.cd.max(0.1);
                (
                    hp.max(u.hp),
                    dmg.max(u.dmg),
                    rng.max(u.rng_studs),
                    dps.max(u_dps),
                    spd.max(u.spd_studs),
                )
            },
        );

        let vp_w = ui.ctx().input(|i| i.content_rect().width());
        let side_w = (vp_w * 0.235).clamp(248.0, 298.0);

        // Left Roster Sidebar
        egui::Panel::left("codex_unit_list")
            .exact_size(side_w)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(16, 14, 15))
                    .inner_margin(Margin::same(12)),
            )
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Unit Codex")
                        .font(display_font(16.0))
                        .color(GOLD_BRIGHT),
                );
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
                    if nav_tab_button(
                        ui,
                        "All",
                        self.filter_faction.is_none(),
                        GOLD,
                        Vec2::new(40.0, 24.0),
                    )
                    .clicked()
                    {
                        self.filter_faction = None;
                    }
                    for fid in FactionId::ALL {
                        let active = self.filter_faction == Some(fid);
                        if nav_tab_button(
                            ui,
                            fid.code_name(),
                            active,
                            faction_color(fid),
                            Vec2::new(66.0, 24.0),
                        )
                        .clicked()
                        {
                            self.filter_faction = Some(fid);
                        }
                    }
                });
                ui.add_space(8.0);

                ScrollArea::vertical()
                    .id_salt("codex_roster_scroll")
                    .show(ui, |ui| {
                        for u in UNITS
                            .iter()
                            .filter(|u| self.filter_faction.map(|f| f == u.faction).unwrap_or(true))
                        {
                            let sel = self.selected_unit_id == u.id;
                            let fac_acc = faction_color(u.faction);
                            let inner = ui.scope(|ui| {
                                let card_acc = if sel { Some(GOLD_BRIGHT) } else { None };
                                let row_fill = if sel {
                                    Color32::from_rgba_unmultiplied(40, 32, 26, 220)
                                } else {
                                    Color32::TRANSPARENT
                                };
                                let frame = if sel {
                                    egui::Frame::new()
                                        .fill(row_fill)
                                        .stroke(Stroke::new(1.0, card_acc.unwrap_or(GOLD_BRIGHT).linear_multiply(0.7)))
                                        .corner_radius(CornerRadius::same(8))
                                        .inner_margin(Margin::symmetric(8, 6))
                                } else {
                                    egui::Frame::new()
                                        .fill(Color32::TRANSPARENT)
                                        .corner_radius(CornerRadius::same(8))
                                        .inner_margin(Margin::symmetric(8, 6))
                                };
                                frame.show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        let (pr, _) = ui.allocate_exact_size(
                                            Vec2::splat(32.0),
                                            Sense::hover(),
                                        );
                                        portrait_pedestal(
                                            ui.painter(),
                                            pr,
                                            assets.get_portrait(u.sprite_key),
                                            if sel { GOLD_BRIGHT } else { fac_acc },
                                            sel,
                                        );
                                        ui.add_space(6.0);
                                        ui.vertical(|ui| {
                                            ui.spacing_mut().item_spacing.y = 1.0;
                                            ui.add(
                                                egui::Label::new(
                                                    egui::RichText::new(u.name)
                                                        .font(body_font(12.5))
                                                        .color(if sel {
                                                            GOLD_BRIGHT
                                                        } else {
                                                            TEXT_PRIMARY
                                                        }),
                                                )
                                                .truncate(),
                                            );
                                            ui.label(
                                                egui::RichText::new(format!(
                                                    "{} · T{} · {}G",
                                                    u.faction.code_name(),
                                                    u.tier.code(),
                                                    u.cost
                                                ))
                                                .font(mono_font(9.5))
                                                .color(if sel { GOLD } else { TEXT_MUTED }),
                                            );
                                        });
                                    });
                                });
                            });
                            if inner
                                .response
                                .interact(Sense::click())
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                self.selected_unit_id = u.id;
                            }
                            ui.add_space(4.0);
                        }
                    });
            });

        // Right Stage: Live GPU Stage + Visual Stat Bars + Tactical Dossier
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG_VOID).inner_margin(Margin::same(16)))
            .show(ui, |ui| {
                let full_rect = ui.max_rect().expand(16.0);
                if let Some(parch) = assets.parchment_dark.as_ref() {
                    paint_parchment_backdrop(
                        ui.painter(),
                        full_rect,
                        parch,
                        Color32::from_rgba_unmultiplied(195, 180, 155, 62),
                    );
                }
                let Some(u) = get_unit(self.selected_unit_id) else {
                    return;
                };
                let fac = get_faction(u.faction);
                let fac_acc = faction_color(u.faction);

                ScrollArea::vertical()
                    .id_salt("codex_detail_scroll")
                    .show(ui, |ui| {
                        // Open Dossier Header (no outer box or stacked badge pills)
                        egui::Frame::new()
                            .inner_margin(Margin::symmetric(4, 4))
                            .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.horizontal_wrapped(|ui| {
                                if let Some(crest) = assets.get_crest(u.faction) {
                                    let (cr, _) =
                                        ui.allocate_exact_size(Vec2::splat(44.0), Sense::hover());
                                    crest_socket(ui.painter(), cr, crest, fac_acc);
                                    ui.add_space(6.0);
                                }
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new(u.name)
                                            .font(display_font(23.0))
                                            .color(GOLD_BRIGHT),
                                    );
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{}  ·  Tier {}  ·  {}  ·  {}G",
                                            fac.id.display_name(),
                                            u.tier.code(),
                                            u.shape.short_role(),
                                            u.cost
                                        ))
                                        .font(mono_font(10.5))
                                        .color(fac_acc),
                                    );
                                });
                            });
                        });

                        ui.add_space(10.0);

                        // Animation Timing State
                        let has_sheet = assets.has_sheet(u.sprite_key);
                        let anim = match self.preview_pose {
                            PreviewPose::Idle => AnimTiming {
                                atk_anim_timer: 0.0,
                                atk_anim_duration: 0.38,
                                is_moving: false,
                                walk_phase: 0.0,
                                idle_phase: self.anim_t * 3.4,
                            },
                            PreviewPose::Walk => AnimTiming {
                                atk_anim_timer: 0.0,
                                atk_anim_duration: 0.38,
                                is_moving: true,
                                walk_phase: self.anim_t * 8.0,
                                idle_phase: 0.0,
                            },
                            PreviewPose::Attack => {
                                let dur = 0.55;
                                let cyc = self.anim_t % 0.85;
                                let rem = if cyc < dur { dur - cyc } else { 0.0 };
                                AnimTiming {
                                    atk_anim_timer: rem,
                                    atk_anim_duration: dur,
                                    is_moving: false,
                                    walk_phase: 0.0,
                                    idle_phase: self.anim_t * 3.4,
                                }
                            }
                        };
                        let active_sheet_frame = if has_sheet {
                            Some(compute_sheet_frame(anim).frame_idx)
                        } else {
                            None
                        };

                        let avail_center = ui.available_width();
                        let top_cols = if avail_center >= 720.0 { 2 } else { 1 };
                        let row_min_h = 312.0;

                        let draw_stage_panel = |ui: &mut Ui, st: &mut Self| {
                            sculpted_card(ui, None, 12, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.set_min_height(row_min_h);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new("Preview")
                                            .font(mono_font(10.5))
                                            .color(GOLD),
                                    );
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if nav_tab_button(
                                                ui,
                                                if st.facing_right {
                                                    "Facing ->"
                                                } else {
                                                    "<- Facing"
                                                },
                                                false,
                                                GOLD,
                                                Vec2::new(78.0, 24.0),
                                            )
                                            .clicked()
                                            {
                                                st.facing_right = !st.facing_right;
                                            }
                                        },
                                    );
                                });

                                ui.add_space(6.0);
                                let stage_w = ui.available_width();
                                let (stage_rect, _) = ui.allocate_exact_size(
                                    Vec2::new(stage_w, 214.0),
                                    Sense::hover(),
                                );
                                paint_vertical_gradient(
                                    ui.painter(),
                                    stage_rect,
                                    Color32::from_rgb(24, 19, 16),
                                    Color32::from_rgb(12, 10, 10),
                                );
                                if let Some(arena) = assets.get_arena(u.faction) {
                                    image_rect_tinted(
                                        ui.painter(),
                                        arena,
                                        stage_rect.shrink(2.0),
                                        Color32::from_rgba_unmultiplied(150, 142, 132, 188),
                                    );
                                }
                                // Warm horizon/floor contrast vignette
                                let floor_band = Rect_from_min_max_y(
                                    stage_rect,
                                    stage_rect.bottom() - 72.0,
                                    stage_rect.bottom() - 2.0,
                                );
                                paint_vertical_gradient(
                                    ui.painter(),
                                    floor_band,
                                    Color32::from_rgba_unmultiplied(14, 11, 10, 0),
                                    Color32::from_rgba_unmultiplied(12, 10, 9, 175),
                                );

                                // Pedestal ground shadow / energy ring
                                let foot_center = stage_rect.center() + Vec2::new(0.0, 52.0);
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(stage_rect.min.x + 22.0, foot_center.y),
                                        Pos2::new(stage_rect.max.x - 22.0, foot_center.y),
                                    ],
                                    Stroke::new(1.5, fac_acc.gamma_multiply(0.4)),
                                );
                                ui.painter().circle_filled(
                                    foot_center + Vec2::new(0.0, -4.0),
                                    32.0,
                                    Color32::from_rgba_unmultiplied(
                                        fac_acc.r(),
                                        fac_acc.g(),
                                        fac_acc.b(),
                                        22,
                                    ),
                                );

                                let facing = if st.facing_right { 1.0 } else { -1.0 };
                                assets.paint_unit_sprite(
                                    ui.painter(),
                                    u.sprite_key,
                                    stage_rect.center() + Vec2::new(0.0, 22.0),
                                    154.0,
                                    facing,
                                    anim,
                                    Color32::WHITE,
                                );

                                ui.add_space(8.0);
                                ui.horizontal_wrapped(|ui| {
                                    ui.spacing_mut().item_spacing = Vec2::new(6.0, 6.0);
                                    for (p_mode, lbl, w) in [
                                        (PreviewPose::Idle, "Idle", 54.0),
                                        (PreviewPose::Walk, "March", 68.0),
                                        (PreviewPose::Attack, "Attack", 68.0),
                                    ] {
                                        if nav_tab_button(
                                            ui,
                                            lbl,
                                            st.preview_pose == p_mode,
                                            GOLD,
                                            Vec2::new(w, 26.0),
                                        )
                                        .clicked()
                                        {
                                            st.preview_pose = p_mode;
                                        }
                                    }
                                    if nav_tab_button(
                                        ui,
                                        if has_sheet { "Frames" } else { "Dossier" },
                                        st.show_exploded_parts,
                                        EMERALD,
                                        Vec2::new(72.0, 26.0),
                                    )
                                    .clicked()
                                    {
                                        st.show_exploded_parts = !st.show_exploded_parts;
                                    }
                                });
                            });
                        };

                        let draw_stats_panel = |ui: &mut Ui| {
                            sculpted_card(ui, None, 12, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.set_min_height(row_min_h);
                                ui.label(
                                    egui::RichText::new("Combat Stats")
                                        .font(mono_font(10.5))
                                        .color(GOLD),
                                );
                                ui.add_space(6.0);

                                let u_dps = u.dmg / u.cd.max(0.1);
                                let stats = [
                                    (
                                        "Health",
                                        format!("{:.0}", u.hp),
                                        u.hp / max_hp,
                                        EMERALD,
                                    ),
                                    (
                                        "Damage",
                                        format!("{:.0}", u.dmg),
                                        u.dmg / max_dmg,
                                        RED_ENEMY,
                                    ),
                                    (
                                        "Range",
                                        format!("{:.1} studs", u.rng_studs),
                                        u.rng_studs / max_rng,
                                        BLUE_LEGION,
                                    ),
                                    (
                                        "Cadence",
                                        format!("{:.2}s · {:.1} DPS", u.cd, u_dps),
                                        u_dps / max_dps,
                                        GOLD_BRIGHT,
                                    ),
                                    (
                                        "Speed",
                                        format!("{:.1} /s", u.spd_studs),
                                        u.spd_studs / max_spd,
                                        TEXT_PRIMARY,
                                    ),
                                ];
                                let bar_w = ui.available_width();
                                for (lbl, val, frac, col) in stats {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(lbl)
                                                .font(body_font(12.0))
                                                .color(TEXT_SECONDARY),
                                        );
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    egui::RichText::new(val)
                                                        .font(mono_font(11.5))
                                                        .color(col),
                                                );
                                            },
                                        );
                                    });
                                    beveled_progress_bar(
                                        ui,
                                        frac.clamp(0.04, 1.0),
                                        Vec2::new(bar_w, 7.0),
                                        col,
                                        None,
                                    );
                                    ui.add_space(4.0);
                                }

                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    let (sr, _) = ui.allocate_exact_size(
                                        Vec2::splat(24.0),
                                        Sense::hover(),
                                    );
                                    paint_unit_shape(
                                        ui.painter(),
                                        u.shape,
                                        sr.center(),
                                        9.0,
                                        0.0,
                                        fac_acc,
                                        Color32::WHITE,
                                        1.4,
                                    );
                                    ui.vertical(|ui| {
                                        ui.add(
                                            egui::Label::new(
                                                egui::RichText::new(format!(
                                                    "{} — {}",
                                                    u.shape.label(),
                                                    u.shape.role_hint()
                                                ))
                                                .font(mono_font(10.8))
                                                .color(TEXT_PRIMARY),
                                            )
                                            .wrap(),
                                        );
                                        ui.add(
                                            egui::Label::new(
                                                egui::RichText::new(u.passive_desc)
                                                    .font(body_font(11.8))
                                                    .color(EMERALD),
                                            )
                                            .wrap(),
                                        );
                                    });
                                });

                                ui.add_space(6.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(format!(
                                            "Unlock: {}",
                                            u.unlock_hint
                                        ))
                                        .font(mono_font(10.2))
                                        .color(TEXT_MUTED),
                                    )
                                    .wrap(),
                                );
                            });
                        };

                        if top_cols == 2 {
                            equal_columns(ui, 2, |ui, col| {
                                if col == 0 {
                                    draw_stage_panel(ui, self);
                                } else {
                                    draw_stats_panel(ui);
                                }
                            });
                        } else {
                            draw_stage_panel(ui, self);
                            ui.add_space(10.0);
                            draw_stats_panel(ui);
                        }

                        // 8-Frame Sprite Sheet or Field Dossier
                        if self.show_exploded_parts {
                            ui.add_space(12.0);
                            sculpted_card(ui, None, 14, |ui| {
                                ui.set_min_width(ui.available_width());
                                if let Some(sheet_tex) = assets.get_sheet(u.sprite_key) {
                                    ui.label(
                                        egui::RichText::new("8-Frame Animation Sheet")
                                            .font(mono_font(10.5))
                                            .color(GOLD),
                                    );
                                    ui.add_space(8.0);
                                    let frame_labels = [
                                        "Walk 1", "Walk 2", "Walk 3", "Walk 4",
                                        "Atk 1", "Atk 2", "Atk 3", "Atk 4",
                                    ];
                                    for row in 0..2 {
                                        equal_columns(ui, 4, |ui, col| {
                                            let idx = row * 4 + col;
                                            let is_active = active_sheet_frame == Some(idx);
                                            sculpted_card(
                                                ui,
                                                if is_active { Some(GOLD_BRIGHT) } else { None },
                                                8,
                                                |ui| {
                                                    ui.set_min_width(ui.available_width());
                                                    ui.vertical_centered(|ui| {
                                                        let (r, _) = ui.allocate_exact_size(
                                                            Vec2::splat(78.0),
                                                            Sense::hover(),
                                                        );
                                                        assets.paint_sheet_cell(
                                                            ui.painter(),
                                                            sheet_tex,
                                                            r.shrink(2.0),
                                                            idx,
                                                            1.0,
                                                            Color32::WHITE,
                                                        );
                                                        ui.add_space(3.0);
                                                        ui.label(
                                                            egui::RichText::new(frame_labels[idx])
                                                                .font(mono_font(9.5))
                                                                .color(if is_active {
                                                                    GOLD_BRIGHT
                                                                } else {
                                                                    TEXT_SECONDARY
                                                                }),
                                                        );
                                                    });
                                                },
                                            );
                                        });
                                        if row == 0 {
                                            ui.add_space(6.0);
                                        }
                                    }
                                } else {
                                    ui.label(
                                        egui::RichText::new("Tactical Notes")
                                            .font(mono_font(10.5))
                                            .color(fac_acc),
                                    );
                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        let (r, _) = ui.allocate_exact_size(
                                            Vec2::splat(78.0),
                                            Sense::hover(),
                                        );
                                        portrait_pedestal(
                                            ui.painter(),
                                            r,
                                            assets.get_portrait(u.sprite_key),
                                            fac_acc,
                                            true,
                                        );
                                        ui.add_space(12.0);
                                        ui.vertical(|ui| {
                                            let dps = u.dmg / u.cd.max(0.1);
                                            let hp_per_g = u.hp / (u.cost.max(1) as f32);
                                            let range_class = if u.rng_studs >= 20.0 {
                                                "Long Range"
                                            } else if u.rng_studs >= 10.0 {
                                                "Mid Range"
                                            } else {
                                                "Frontline"
                                            };

                                            ui.label(
                                                egui::RichText::new(format!(
                                                    "{range_class}  ·  {dps:.1} DPS  ·  {hp_per_g:.1} HP/G"
                                                ))
                                                .font(mono_font(10.5))
                                                .color(GOLD_BRIGHT),
                                            );

                                            ui.add_space(5.0);
                                            ui.add(
                                                egui::Label::new(
                                                    egui::RichText::new(format!(
                                                        "{} — {}",
                                                        u.shape.role_hint(),
                                                        u.passive_desc
                                                    ))
                                                    .font(body_font(12.2))
                                                    .color(TEXT_PRIMARY),
                                                )
                                                .wrap(),
                                            );
                                            ui.add_space(4.0);
                                            ui.add(
                                                egui::Label::new(
                                                    egui::RichText::new(format!(
                                                        "{} Synergy (6/6): {} · [Q] {} · [E] {}",
                                                        fac.id.display_name(),
                                                        fac.affinity_3pc_desc,
                                                        fac.q_spell_name,
                                                        fac.e_spell_name
                                                    ))
                                                    .font(body_font(11.5))
                                                    .color(TEXT_SECONDARY),
                                                )
                                                .wrap(),
                                            );
                                        });
                                    });
                                }
                            });
                        }
                    });
            });
    }
}

#[allow(non_snake_case)]
fn Rect_from_min_max_y(r: egui::Rect, min_y: f32, max_y: f32) -> egui::Rect {
    egui::Rect::from_min_max(
        Pos2::new(r.min.x, min_y.max(r.min.y)),
        Pos2::new(r.max.x, max_y.min(r.max.y)),
    )
}
