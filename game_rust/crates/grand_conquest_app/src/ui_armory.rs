use egui::{Color32, Margin, ScrollArea, Sense, Ui, Vec2};
use grand_conquest_core::model::{FactionId, UnitDef, UnitTier};
use grand_conquest_core::save::ConquestSave;
use grand_conquest_core::units::{evaluate_deck_affinities, get_faction, get_unit, UNITS};

use crate::assets::{image_rect_tinted, AssetBank};
use crate::theme::{
    beveled_progress_bar, body_font, crest_socket, cta_button, display_font, equal_columns,
    faction_color, mono_font, nav_tab_button, paint_parchment_backdrop, portrait_pedestal,
    sculpted_card, subtle_button, tier_color, CYAN, EMERALD, GOLD, GOLD_BRIGHT, TEXT_MUTED,
    TEXT_PRIMARY, TEXT_SECONDARY,
};

pub struct ArmoryViewState {
    pub selected_slot: usize,
    pub filter_faction: Option<FactionId>,
    pub filter_tier: Option<UnitTier>,
    pub unlocked_only: bool,
    pub status_msg: String,
    pub pending_mission_id: Option<&'static str>,
}

impl Default for ArmoryViewState {
    fn default() -> Self {
        Self {
            selected_slot: 0,
            filter_faction: None,
            filter_tier: None,
            unlocked_only: false,
            status_msg: "Step 1: Pick a deck slot [1]–[6] below. Step 2: Click a matching-tier unit to equip it.".to_string(),
            pending_mission_id: None,
        }
    }
}

pub enum ArmoryAction {
    None,
    LaunchPendingMission(&'static str),
}

impl ArmoryViewState {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        save: &mut ConquestSave,
        assets: &AssetBank,
        on_save_dirty: &mut bool,
    ) -> ArmoryAction {
        let mut action = ArmoryAction::None;

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(crate::theme::BG_VOID).inner_margin(Margin::same(18)))
            .show(ui, |ui| {
                let full_rect = ui.max_rect().expand(18.0);
                if let Some(parch) = assets.parchment_dark.as_ref() {
                    paint_parchment_backdrop(
                        ui.painter(),
                        full_rect,
                        parch,
                        Color32::from_rgba_unmultiplied(195, 180, 155, 52),
                    );
                }

                // Header Bar
                ui.horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Armory & Deck")
                                .font(display_font(20.5))
                                .color(GOLD_BRIGHT),
                        );
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&self.status_msg)
                                    .font(body_font(12.2))
                                    .color(TEXT_SECONDARY),
                            )
                            .wrap(),
                        );
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some(mid) = self.pending_mission_id {
                            let mission_def = grand_conquest_core::missions::get_mission(mid);
                            let mission_label = mission_def
                                .map(|m| {
                                    let short = crate::theme::clamp_chars(&m.name, 26);
                                    format!("Deploy: {short} ->")
                                })
                                .unwrap_or_else(|| format!("Deploy M{mid} ->"));
                            let btn_w = (mission_label.chars().count() as f32 * 8.0 + 38.0)
                                .clamp(190.0, 340.0);
                            if cta_button(ui, &mission_label, Vec2::new(btn_w, 34.0))
                                .on_hover_text(
                                    mission_def
                                        .map(|m| format!("Ch. {} — {}", m.chapter, m.name))
                                        .unwrap_or_default(),
                                )
                                .clicked()
                            {
                                action = ArmoryAction::LaunchPendingMission(mid);
                            }
                            ui.add_space(12.0);
                        }
                        ui.label(
                            egui::RichText::new(format!("{}/54 Unlocked", save.unlocked_blueprint_ids.len()))
                                .font(mono_font(11.0))
                                .color(EMERALD),
                        );
                    });
                });

                ui.add_space(10.0);

                // Top Grid: 6 Active Deck Slots (Left) + Commander & Spells (Right)
                let avail = ui.available_width();
                let stacked = avail < 960.0;
                if stacked {
                    self.draw_deck_bar(ui, avail, 0.0, save, assets);
                    ui.add_space(8.0);
                    draw_commander_panel(ui, avail, save, assets, on_save_dirty);
                } else {
                    let slots_w = (avail * 0.59).clamp(540.0, 800.0);
                    let cmd_w = (avail - slots_w - 12.0).max(360.0);
                    let row_min_h = if avail < 1180.0 { 268.0 } else { 250.0 };
                    ui.horizontal_top(|ui| {
                        ui.allocate_ui_with_layout(
                            Vec2::new(slots_w, row_min_h),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                self.draw_deck_bar(ui, slots_w, row_min_h, save, assets);
                            },
                        );
                        ui.add_space(12.0);
                        ui.allocate_ui_with_layout(
                            Vec2::new(cmd_w, row_min_h),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                draw_commander_panel(ui, cmd_w, save, assets, on_save_dirty);
                            },
                        );
                    });
                }

                ui.add_space(10.0);

                // Open Filter Strip (no boxed container or badge clutter)
                let selected_req_tier = match self.selected_slot {
                    0 | 1 => UnitTier::C,
                    2 | 3 => UnitTier::B,
                    _ => UnitTier::A,
                };
                let tier_hint = match selected_req_tier {
                    UnitTier::C => "Tier C (Basic)",
                    UnitTier::B => "Tier B (Veterans)",
                    UnitTier::A => "Tier A (Elites)",
                };

                egui::Frame::new()
                    .inner_margin(Margin::symmetric(2, 4))
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!(
                                    "Slot [{}] · {tier_hint}",
                                    self.selected_slot + 1
                                ))
                                .font(mono_font(11.0))
                                .color(GOLD_BRIGHT),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if nav_tab_button(
                                    ui,
                                    if self.unlocked_only { "Unlocked" } else { "All Units" },
                                    self.unlocked_only,
                                    EMERALD,
                                    Vec2::new(92.0, 26.0),
                                )
                                .clicked()
                                {
                                    self.unlocked_only = !self.unlocked_only;
                                }
                            });
                        });

                        ui.add_space(4.0);

                        // Wrapping Filter Strip
                        ui.horizontal_wrapped(|ui| {
                            if nav_tab_button(ui, "All", self.filter_faction.is_none(), GOLD, Vec2::new(46.0, 26.0)).clicked() {
                                self.filter_faction = None;
                            }
                            for fid in FactionId::ALL {
                                let active = self.filter_faction == Some(fid);
                                if nav_tab_button(ui, fid.code_name(), active, faction_color(fid), Vec2::new(76.0, 26.0)).clicked() {
                                    self.filter_faction = Some(fid);
                                }
                            }
                            ui.separator();
                            for (t_opt, lbl) in [
                                (None, "All"),
                                (Some(UnitTier::C), "C"),
                                (Some(UnitTier::B), "B"),
                                (Some(UnitTier::A), "A"),
                            ] {
                                let active = self.filter_tier == t_opt;
                                let w = if t_opt.is_none() { 42.0 } else { 36.0 };
                                if nav_tab_button(ui, lbl, active, GOLD, Vec2::new(w, 26.0)).clicked() {
                                    self.filter_tier = t_opt;
                                }
                            }
                        });
                    });

                ui.add_space(10.0);

                // Sort Catalog Intelligently:
                // 1) Unlocked & Matching Selected Slot Tier FIRST
                // 2) Other Unlocked Troops NEXT
                // 3) Locked Blueprints LAST (sorted matching tier first)
                let mut filtered: Vec<&'static UnitDef> = UNITS
                    .iter()
                    .filter(|u| self.filter_faction.map(|f| f == u.faction).unwrap_or(true))
                    .filter(|u| self.filter_tier.map(|t| t == u.tier).unwrap_or(true))
                    .filter(|u| !self.unlocked_only || save.unlocked_blueprint_ids.iter().any(|id| id == u.id))
                    .collect();

                filtered.sort_by_key(|u| {
                    let is_unlocked = save.unlocked_blueprint_ids.iter().any(|id| id == u.id);
                    let matches_slot = u.tier == selected_req_tier;
                    let bucket = match (is_unlocked, matches_slot) {
                        (true, true) => 0u8,
                        (true, false) => 1u8,
                        (false, true) => 2u8,
                        (false, false) => 3u8,
                    };
                    (bucket, u.tier.required_tech_tier(), u.cost)
                });

                ScrollArea::vertical().show(ui, |ui| {
                    let cols = ((ui.available_width() / 385.0).floor() as usize).clamp(1, 3);
                    for row in filtered.chunks(cols) {
                        equal_columns(ui, cols, |ui, ci| {
                            if let Some(u) = row.get(ci) {
                                self.draw_unit_card(
                                    ui,
                                    u,
                                    selected_req_tier,
                                    save,
                                    assets,
                                    on_save_dirty,
                                );
                            }
                        });
                        ui.add_space(10.0);
                    }
                });
            });

        action
    }

    fn draw_deck_bar(
        &mut self,
        ui: &mut Ui,
        slots_w: f32,
        min_panel_h: f32,
        save: &ConquestSave,
        assets: &AssetBank,
    ) {
        sculpted_card(ui, None, 12, |ui| {
            let inner_w = (slots_w - 24.0).max(460.0);
            ui.set_min_width(inner_w);
            ui.set_max_width(inner_w);
            if min_panel_h > 0.0 {
                ui.set_min_height((min_panel_h - 24.0).max(160.0));
            }
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Battle Deck")
                        .font(display_font(14.5))
                        .color(GOLD_BRIGHT),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("C [1–2]  ·  B [3–4]  ·  A [5–6]")
                            .font(mono_font(10.0))
                            .color(TEXT_MUTED),
                    );
                });
            });
            ui.add_space(8.0);
            equal_columns(ui, 6, |ui, idx| {
                let req_tier = match idx {
                    0 | 1 => UnitTier::C,
                    2 | 3 => UnitTier::B,
                    _ => UnitTier::A,
                };
                let tc = tier_color(req_tier);
                let is_sel = self.selected_slot == idx;
                let udef = get_unit(&save.equipped_deck_unit_ids[idx]);
                let has_tier_bp = save.has_unlocked_tier(req_tier.required_tech_tier());
                let card_accent = if is_sel { Some(GOLD_BRIGHT) } else { None };

                let resp = egui::Frame::NONE.show(ui, |ui| {
                    sculpted_card(ui, card_accent, 7, |ui| {
                        let card_h = if min_panel_h > 258.0 { 188.0 } else { 172.0 };
                        ui.set_min_height(card_h);
                        ui.set_min_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("[{}]", idx + 1))
                                    .font(mono_font(10.5))
                                    .color(if is_sel { GOLD_BRIGHT } else { GOLD }),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    egui::RichText::new(format!("T{}", req_tier.code()))
                                        .font(mono_font(10.0))
                                        .color(tc),
                                );
                            });
                        });
                        ui.add_space(6.0);

                        if let Some(u) = udef {
                            let fac_col = faction_color(u.faction);
                            ui.vertical_centered(|ui| {
                                let (pr, _) =
                                    ui.allocate_exact_size(Vec2::splat(52.0), Sense::hover());
                                portrait_pedestal(
                                    ui.painter(),
                                    pr,
                                    assets.get_portrait(u.sprite_key),
                                    fac_col,
                                    is_sel,
                                );
                                ui.add_space(6.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(u.name)
                                            .font(body_font(12.2))
                                            .color(if is_sel { GOLD_BRIGHT } else { TEXT_PRIMARY }),
                                    )
                                    .truncate(),
                                );
                            });
                            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{}G", u.cost))
                                            .font(mono_font(11.0))
                                            .color(GOLD_BRIGHT),
                                    );
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            ui.add(
                                                egui::Label::new(
                                                    egui::RichText::new(u.shape.short_role())
                                                        .font(body_font(10.5))
                                                        .color(TEXT_SECONDARY),
                                                )
                                                .truncate(),
                                            );
                                        },
                                    );
                                });
                            });
                        } else {
                            ui.vertical_centered(|ui| {
                                ui.add_space(16.0);
                                let title = if has_tier_bp { "EMPTY" } else { "LOCKED" };
                                let hint = if has_tier_bp {
                                    format!("Pick Tier {} below", req_tier.code())
                                } else if req_tier == UnitTier::B {
                                    "Win M1.1 Battle".to_string()
                                } else {
                                    "Beat M1.4 Boss".to_string()
                                };
                                ui.label(
                                    egui::RichText::new(title)
                                        .font(mono_font(10.8))
                                        .color(if has_tier_bp { tc } else { TEXT_MUTED }),
                                );
                                ui.add_space(4.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(hint)
                                            .font(body_font(11.0))
                                            .color(TEXT_MUTED),
                                    )
                                    .wrap(),
                                );
                            });
                        }
                    });
                });

                let tip = if let Some(u) = udef {
                    format!(
                        "{} ({} • {}G)\nHP {:.0} • DMG {:.0}\n{}\nClick to change Slot [{}].",
                        u.name, u.shape.role_hint(), u.cost, u.hp, u.dmg, u.passive_desc, idx + 1
                    )
                } else if has_tier_bp {
                    format!(
                        "Empty Tier {} slot — click here, then equip any Tier {} troop below.",
                        req_tier.code(),
                        req_tier.code()
                    )
                } else {
                    format!(
                        "Tier {} is locked! Clear earlier Chapter 1 battles to unlock Tier {} troops.",
                        req_tier.code(),
                        req_tier.code()
                    )
                };

                if resp
                    .response
                    .interact(Sense::click())
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .on_hover_text(tip)
                    .clicked()
                {
                    self.selected_slot = idx;
                    self.filter_tier = Some(req_tier);
                    self.status_msg = format!(
                        "Selected Slot [{}] (Tier {}) — click any unlocked Tier {} troop below to equip it.",
                        idx + 1,
                        req_tier.code(),
                        req_tier.code()
                    );
                }
            });
        });
    }

    fn draw_unit_card(
        &mut self,
        ui: &mut Ui,
        u: &'static UnitDef,
        selected_req_tier: UnitTier,
        save: &mut ConquestSave,
        assets: &AssetBank,
        on_save_dirty: &mut bool,
    ) {
        let is_unlocked = save.unlocked_blueprint_ids.iter().any(|id| id == u.id);
        let equipped_slot = save.equipped_deck_unit_ids.iter().position(|id| id == u.id);
        let matches_selected_slot = u.tier == selected_req_tier;

                let fac_col = faction_color(u.faction);
        let accent = if equipped_slot == Some(self.selected_slot) {
            Some(GOLD_BRIGHT)
        } else if equipped_slot.is_some() {
            Some(GOLD)
        } else {
            None
        };

        sculpted_card(ui, accent, 11, |ui| {
            let cw = ui.available_width();
            ui.set_min_width(cw);
            ui.set_max_width(cw);
            ui.set_min_height(96.0);
            ui.horizontal_top(|ui| {
                let (pr, _) = ui.allocate_exact_size(Vec2::splat(52.0), Sense::hover());
                if is_unlocked {
                    portrait_pedestal(
                        ui.painter(),
                        pr,
                        assets.get_portrait(u.sprite_key),
                        if equipped_slot.is_some() { GOLD } else { fac_col },
                        equipped_slot == Some(self.selected_slot),
                    );
                } else if let Some(tex) = assets.get_portrait(u.sprite_key) {
                    ui.painter().circle_filled(
                        pr.center(),
                        pr.width() * 0.42,
                        Color32::from_rgba_unmultiplied(28, 25, 27, 115),
                    );
                    image_rect_tinted(
                        ui.painter(),
                        tex,
                        pr.shrink(4.0),
                        Color32::from_rgba_unmultiplied(120, 112, 100, 125),
                    );
                }
                ui.add_space(8.0);
                ui.vertical(|ui| {
                    let vw = ui.available_width();
                    ui.set_max_width(vw);
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(u.name)
                                    .font(display_font(13.8))
                                    .color(if is_unlocked { TEXT_PRIMARY } else { TEXT_MUTED }),
                            )
                            .truncate(),
                        );
                        ui.label(
                            egui::RichText::new(format!("T{} · {}G", u.tier.code(), u.cost))
                                .font(mono_font(10.5))
                                .color(if is_unlocked { tier_color(u.tier) } else { TEXT_MUTED }),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if is_unlocked {
                                if equipped_slot == Some(self.selected_slot) {
                                    ui.label(
                                        egui::RichText::new("Equipped")
                                            .font(mono_font(10.5))
                                            .color(EMERALD),
                                    );
                                } else if matches_selected_slot {
                                    if subtle_button(
                                        ui,
                                        &format!("Equip [{}]", self.selected_slot + 1),
                                        true,
                                        GOLD,
                                        Vec2::new(82.0, 26.0),
                                    )
                                    .clicked()
                                    {
                                        if let Ok(()) = save.equip_unit_in_slot(self.selected_slot, u.id) {
                                            self.status_msg = format!(
                                                "Equipped {} into Slot [{}].",
                                                u.name,
                                                self.selected_slot + 1
                                            );
                                            *on_save_dirty = true;
                                        }
                                    }
                                } else {
                                    let slot_range = match u.tier {
                                        UnitTier::C => "[1–2]",
                                        UnitTier::B => "[3–4]",
                                        UnitTier::A => "[5–6]",
                                    };
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(format!(
                                                "Fits T{} {slot_range}",
                                                u.tier.code()
                                            ))
                                            .font(body_font(11.0))
                                            .color(TEXT_MUTED),
                                        )
                                        .truncate(),
                                    );
                                }
                            } else {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(format!("Locked: {}", u.unlock_hint))
                                            .font(body_font(11.0))
                                            .color(TEXT_MUTED),
                                    )
                                    .truncate(),
                                );
                            }
                        });
                    });

                    let range_str = if u.rng_studs >= 20.0 {
                        "Long Range"
                    } else if u.rng_studs >= 10.0 {
                        "Mid Range"
                    } else {
                        "Melee"
                    };
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!(
                                "{}  •  {}  •  {:.0} HP  •  {:.0} DMG",
                                u.shape.role_hint(),
                                range_str,
                                u.hp,
                                u.dmg
                            ))
                            .font(body_font(12.0))
                            .color(if is_unlocked { TEXT_SECONDARY } else { TEXT_MUTED }),
                        )
                        .wrap(),
                    );
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(u.passive_desc)
                                .font(body_font(11.6))
                                .color(if is_unlocked { EMERALD } else { TEXT_MUTED }),
                        )
                        .wrap(),
                    );
                });
            });
        });
    }
}

fn draw_commander_panel(
    ui: &mut Ui,
    panel_w: f32,
    save: &mut ConquestSave,
    assets: &AssetBank,
    on_save_dirty: &mut bool,
) {
    sculpted_card(ui, None, 12, |ui| {
        let inner_w = (panel_w - 24.0).max(320.0);
        ui.set_min_width(inner_w);
        ui.set_max_width(inner_w);

        let cmd_fac = get_faction(save.player_commander_faction);
        let cmd_col = faction_color(save.player_commander_faction);
        ui.horizontal(|ui| {
            if let Some(crest) = assets.get_crest(save.player_commander_faction) {
                let (cr, _) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::hover());
                crest_socket(ui.painter(), cr, crest, cmd_col);
            }
            ui.label(
                egui::RichText::new("Commander Spells")
                    .font(display_font(14.5))
                    .color(GOLD_BRIGHT),
            );
        });

        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            for fid in FactionId::ALL {
                let active = save.player_commander_faction == fid;
                if nav_tab_button(
                    ui,
                    fid.code_name(),
                    active,
                    faction_color(fid),
                    Vec2::new(72.0, 24.0),
                )
                .clicked()
                {
                    save.player_commander_faction = fid;
                    save.recompute_integrity();
                    *on_save_dirty = true;
                }
            }
        });

        ui.add_space(6.0);
        ui.vertical(|ui| {
            let sw = ui.available_width();
            ui.set_min_width(sw);
            ui.set_max_width(sw);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!(
                        "[Q] {} — {}",
                        cmd_fac.q_spell_name, cmd_fac.q_spell_desc
                    ))
                    .font(body_font(11.4))
                    .color(GOLD),
                )
                .wrap(),
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!(
                        "[E] {} — {}",
                        cmd_fac.e_spell_name, cmd_fac.e_spell_desc
                    ))
                    .font(body_font(11.4))
                    .color(CYAN),
                )
                .wrap(),
            );
        });

        ui.add_space(5.0);
        let affinity = evaluate_deck_affinities(&save.equipped_deck_unit_ids);
        let top_count = affinity
            .active_summary
            .iter()
            .map(|&(_, c, _)| c)
            .max()
            .unwrap_or(0);
        let (syn_lbl, syn_col) = if let Some((fid, count, is_6)) = affinity.active_summary.first().copied() {
            let fdef = get_faction(fid);
            (
                format!(
                    "{} ({}/6){}: {}",
                    fid.code_name(),
                    count,
                    if is_6 { " PURE x2!" } else { "" },
                    fdef.affinity_3pc_desc
                ),
                faction_color(fid),
            )
        } else {
            (
                "Equip 3+ troops from one Legion to trigger passive affinity.".to_string(),
                TEXT_MUTED,
            )
        };

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("ARMY SYNERGY")
                    .font(mono_font(10.0))
                    .color(EMERALD),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(format!("{top_count}/6"))
                        .font(mono_font(10.0))
                        .color(syn_col),
                );
            });
        });
        beveled_progress_bar(
            ui,
            (top_count as f32 / 6.0).clamp(0.0, 1.0),
            Vec2::new(ui.available_width(), 6.0),
            syn_col,
            None,
        );
        ui.add(
            egui::Label::new(
                egui::RichText::new(syn_lbl)
                    .font(body_font(11.4))
                    .color(syn_col),
            )
            .wrap(),
        );

        ui.add_space(4.0);
        // Warband tactical role balance summary + Auto-Build helper
        let mut frontline = 0usize;
        let mut ranged = 0usize;
        let mut support = 0usize;
        let mut total_cost = 0u32;
        for uid in &save.equipped_deck_unit_ids {
            if let Some(u) = get_unit(uid) {
                total_cost += u.cost.max(0) as u32;
                match u.shape {
                    grand_conquest_core::model::UnitShape::Circle
                    | grand_conquest_core::model::UnitShape::Square
                    | grand_conquest_core::model::UnitShape::Hexagon => frontline += 1,
                    grand_conquest_core::model::UnitShape::Triangle
                    | grand_conquest_core::model::UnitShape::Diamond => ranged += 1,
                    grand_conquest_core::model::UnitShape::Cross => support += 1,
                }
            }
        }
        let avg_cost = total_cost as f32 / 6.0;
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new(format!("Mix · {frontline}F · {ranged}R · {support}S · Avg {avg_cost:.0}G"))
                    .font(mono_font(10.5))
                    .color(TEXT_SECONDARY),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if subtle_button(ui, "Auto-Fill", false, GOLD, Vec2::new(92.0, 26.0))
                    .on_hover_text("Automatically builds the strongest balanced 6-unit warband from unlocked blueprints")
                    .clicked()
                {
                    let cmd = save.player_commander_faction;
                    let unlocked_ids = save.unlocked_blueprint_ids.clone();
                    let pick_for_tier = |tier: UnitTier, used: &mut Vec<&'static str>| -> Option<String> {
                        let mut cands: Vec<&UnitDef> = UNITS
                            .iter()
                            .filter(|u| {
                                u.tier == tier
                                    && unlocked_ids.iter().any(|id| id == u.id)
                                    && !used.contains(&u.id)
                            })
                            .collect();
                        cands.sort_by(|a, b| {
                            let score_a = (if a.faction == cmd { 1000.0 } else { 0.0 })
                                + a.hp * 0.4
                                + a.dmg * 2.0;
                            let score_b = (if b.faction == cmd { 1000.0 } else { 0.0 })
                                + b.hp * 0.4
                                + b.dmg * 2.0;
                            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
                        });
                        cands.first().map(|u| {
                            used.push(u.id);
                            u.id.to_string()
                        })
                    };
                    let mut used = Vec::new();
                    for slot in 0..6 {
                        let req_tier = match ConquestSave::slot_req_tier(slot) {
                            1 => UnitTier::C,
                            2 => UnitTier::B,
                            _ => UnitTier::A,
                        };
                        let picked = pick_for_tier(req_tier, &mut used)
                            .or_else(|| pick_for_tier(UnitTier::B, &mut used))
                            .or_else(|| pick_for_tier(UnitTier::C, &mut used));
                        if let Some(id) = picked {
                            let _ = save.equip_unit_in_slot(slot, &id);
                        }
                    }
                    *on_save_dirty = true;
                }
            });
        });
    });
}
