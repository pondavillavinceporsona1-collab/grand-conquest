use egui::{Color32, CornerRadius, Margin, Pos2, Rect, ScrollArea, Sense, Ui, Vec2};
use grand_conquest_core::model::{AiDifficulty, FactionId, MutatorId, RelicId, UnitDef, UnitTier};
use grand_conquest_core::relics::RELICS;
use grand_conquest_core::save::ConquestSave;
use grand_conquest_core::units::{
    default_faction_deck, evaluate_deck_affinities, get_faction, get_unit, UNITS,
};

use crate::assets::AssetBank;
use crate::theme::{
    body_font, crest_socket, cta_button, display_font, equal_columns, faction_color, mono_font,
    nav_tab_button, paint_parchment_backdrop, paint_vertical_gradient, portrait_pedestal,
    sculpted_card, tier_color, BG_VOID, BLUE_LEGION, CYAN, EMERALD, GOLD, GOLD_BRIGHT, RED_ENEMY,
    TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkirmishRelicMode {
    CampaignEquipped,
    None,
    AllUnlockedSandbox,
}

pub struct SkirmishConfig {
    pub player_faction: FactionId,
    pub player_deck: [String; 6],
    pub selected_slot: usize,
    pub picker_faction: Option<FactionId>,
    pub enemy_faction: FactionId,
    pub ai_difficulty: AiDifficulty,
    pub mutator: Option<MutatorId>,
    pub relic_mode: SkirmishRelicMode,
}

impl Default for SkirmishConfig {
    fn default() -> Self {
        Self {
            player_faction: FactionId::Solaris,
            player_deck: default_faction_deck(FactionId::Solaris),
            selected_slot: 0,
            picker_faction: Some(FactionId::Solaris),
            enemy_faction: FactionId::Hellfire,
            ai_difficulty: AiDifficulty::Normal,
            mutator: None,
            relic_mode: SkirmishRelicMode::CampaignEquipped,
        }
    }
}

impl SkirmishConfig {
    pub fn resolve_relics(&self, save: &ConquestSave) -> Vec<RelicId> {
        match self.relic_mode {
            SkirmishRelicMode::CampaignEquipped => save
                .equipped_relic_ids
                .iter()
                .filter_map(|s| RelicId::from_str_id(s))
                .collect(),
            SkirmishRelicMode::None => Vec::new(),
            SkirmishRelicMode::AllUnlockedSandbox => RELICS.iter().map(|r| r.id).collect(),
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, save: &ConquestSave, assets: &AssetBank) -> bool {
        let mut launch = false;

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG_VOID).inner_margin(Margin::symmetric(16, 10)))
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

                // Open Header Banner
                egui::Frame::new()
                    .fill(Color32::from_rgba_unmultiplied(18, 16, 18, 165))
                    .corner_radius(CornerRadius::same(12))
                    .inner_margin(Margin::symmetric(16, 10))
                    .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.horizontal_wrapped(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Skirmish")
                                    .font(display_font(20.0))
                                    .color(GOLD_BRIGHT),
                            );
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(
                                        "Custom matches with all 54 units and mutators unlocked.",
                                    )
                                    .font(body_font(12.0))
                                    .color(TEXT_SECONDARY),
                                )
                                .wrap(),
                            );
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if cta_button(ui, "Start Match ->", Vec2::new(168.0, 34.0))
                                .clicked()
                            {
                                launch = true;
                            }
                            if nav_tab_button(
                                ui,
                                "Randomize",
                                false,
                                CYAN,
                                Vec2::new(92.0, 30.0),
                            )
                            .on_hover_text("Randomize factions, AI difficulty, and battle mutator")
                            .clicked()
                            {
                                use rand::Rng;
                                let mut rng = rand::thread_rng();
                                self.player_faction =
                                    FactionId::ALL[rng.gen_range(0..FactionId::ALL.len())];
                                self.enemy_faction =
                                    FactionId::ALL[rng.gen_range(0..FactionId::ALL.len())];
                                self.player_deck = default_faction_deck(self.player_faction);
                                self.ai_difficulty =
                                    AiDifficulty::ALL[rng.gen_range(0..AiDifficulty::ALL.len())];
                                self.mutator = if rng.gen_bool(0.6) {
                                    Some(MutatorId::ALL[rng.gen_range(0..MutatorId::ALL.len())])
                                } else {
                                    None
                                };
                            }
                            if nav_tab_button(
                                ui,
                                "Mirror",
                                false,
                                GOLD,
                                Vec2::new(72.0, 30.0),
                            )
                            .on_hover_text(
                                "Match Enemy Legion to Player Legion for pure skill duels",
                            )
                            .clicked()
                            {
                                self.enemy_faction = self.player_faction;
                            }
                        });
                    });
                });

                ui.add_space(10.0);

                let avail_w = ui.available_width();
                let avail_h = ui.available_height();
                let col_min_h = avail_h.clamp(580.0, 684.0);
                let stacked = avail_w < 920.0;

                let draw_left = |ui: &mut Ui, cfg: &mut Self| {
                    let pf_acc = faction_color(cfg.player_faction);
                    sculpted_card(ui, None, 12, |ui| {
                        ui.set_min_width(ui.available_width());
                        if !stacked {
                            ui.set_min_height(col_min_h);
                        }
                        ui.horizontal(|ui| {
                            if let Some(crest) = assets.get_crest(cfg.player_faction) {
                                let (cr, _) =
                                    ui.allocate_exact_size(Vec2::splat(30.0), Sense::hover());
                                crest_socket(ui.painter(), cr, crest, pf_acc);
                            }
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.label(
                                    egui::RichText::new("Player Commander")
                                        .font(display_font(14.5))
                                        .color(BLUE_LEGION),
                                );
                                ui.label(
                                    egui::RichText::new(cfg.player_faction.subtitle())
                                        .font(body_font(11.0))
                                        .color(TEXT_MUTED),
                                );
                            });
                        });
                        ui.add_space(6.0);

                        // Symmetric 3x2 Faction Selector Grid
                        for row_facs in FactionId::ALL.chunks(3) {
                            equal_columns(ui, 3, |ui, c| {
                                if let Some(&fid) = row_facs.get(c) {
                                    let active = cfg.player_faction == fid;
                                    let bw = ui.available_width();
                                    if nav_tab_button(
                                        ui,
                                        fid.display_name(),
                                        active,
                                        faction_color(fid),
                                        Vec2::new(bw, 26.0),
                                    )
                                    .clicked()
                                    {
                                        cfg.player_faction = fid;
                                        cfg.player_deck = default_faction_deck(fid);
                                        if cfg.picker_faction.is_some() {
                                            cfg.picker_faction = Some(fid);
                                        }
                                    }
                                }
                            });
                            ui.add_space(4.0);
                        }

                        // Open Commander Spells Summary (no nested box)
                        let pf = get_faction(cfg.player_faction);
                        ui.add_space(4.0);
                        ui.vertical(|ui| {
                            ui.set_min_width(ui.available_width());
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(format!(
                                        "[Q] {} — {}",
                                        pf.q_spell_name, pf.q_spell_desc
                                    ))
                                    .font(body_font(11.0))
                                    .color(GOLD_BRIGHT),
                                )
                                .wrap(),
                            );
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(format!(
                                        "[E] {} — {}",
                                        pf.e_spell_name, pf.e_spell_desc
                                    ))
                                    .font(body_font(11.0))
                                    .color(EMERALD),
                                )
                                .wrap(),
                            );
                        });

                        ui.add_space(8.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(6.0, 4.0);
                            ui.label(
                                egui::RichText::new("Active Formation")
                                    .font(mono_font(10.5))
                                    .color(GOLD),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if nav_tab_button(
                                    ui,
                                    "Campaign Deck",
                                    false,
                                    BLUE_LEGION,
                                    Vec2::new(108.0, 22.0),
                                )
                                .on_hover_text(
                                    "Copy your current Campaign Armory deck into Skirmish slots",
                                )
                                .clicked()
                                {
                                    let fallback = default_faction_deck(cfg.player_faction);
                                    for i in 0..6 {
                                        let cid = &save.equipped_deck_unit_ids[i];
                                        cfg.player_deck[i] = if get_unit(cid).is_some() {
                                            cid.clone()
                                        } else {
                                            fallback[i].clone()
                                        };
                                    }
                                }
                                if nav_tab_button(
                                    ui,
                                    "Default",
                                    false,
                                    GOLD,
                                    Vec2::new(68.0, 22.0),
                                )
                                .on_hover_text(
                                    "Reset all 6 slots to the selected Commander's default formation",
                                )
                                .clicked()
                                {
                                    cfg.player_deck = default_faction_deck(cfg.player_faction);
                                }
                            });
                        });
                        ui.add_space(4.0);

                        for row_idx in 0..2 {
                            equal_columns(ui, 3, |ui, ci| {
                                let slot_idx = row_idx * 3 + ci;
                                let req_tier = slot_tier(slot_idx);
                                let is_selected = cfg.selected_slot == slot_idx;
                                let uid = &cfg.player_deck[slot_idx];
                                if let Some(u) = get_unit(uid) {
                                    let card_acc = if is_selected { Some(GOLD_BRIGHT) } else { None };
                                    let resp = ui
                                        .scope(|ui| {
                                            sculpted_card(ui, card_acc, 7, |ui| {
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
                                                        if is_selected {
                                                            GOLD_BRIGHT
                                                        } else {
                                                            faction_color(u.faction)
                                                        },
                                                        is_selected,
                                                    );
                                                    ui.vertical(|ui| {
                                                        ui.spacing_mut().item_spacing.y = 0.0;
                                                        ui.horizontal(|ui| {
                                                            ui.label(
                                                                egui::RichText::new(format!(
                                                                    "[{}] {}",
                                                                    slot_idx + 1,
                                                                    req_tier.code()
                                                                ))
                                                                .font(mono_font(9.5))
                                                                .color(tier_color(req_tier)),
                                                            );
                                                            ui.label(
                                                                egui::RichText::new(
                                                                    u.faction.code_name(),
                                                                )
                                                                .font(mono_font(9.0))
                                                                .color(faction_color(u.faction)),
                                                            );
                                                        });
                                                        ui.add(
                                                            egui::Label::new(
                                                                egui::RichText::new(u.name)
                                                                    .font(body_font(12.0))
                                                                    .color(if is_selected {
                                                                        GOLD_BRIGHT
                                                                    } else {
                                                                        TEXT_PRIMARY
                                                                    }),
                                                            )
                                                            .truncate(),
                                                        );
                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "{}G • {}",
                                                                u.cost,
                                                                u.shape.short_role()
                                                            ))
                                                            .font(mono_font(9.2))
                                                            .color(GOLD_BRIGHT),
                                                        );
                                                    });
                                                });
                                            });
                                        })
                                        .response
                                        .interact(Sense::click())
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .on_hover_text(format!(
                                            "{} ({} • Tier {})\nHP {:.0} | DMG {:.0} | Range {:.0}\n{}\nClick to pick a replacement for Slot [{}].",
                                            u.name,
                                            u.shape.role_hint(),
                                            u.tier.code(),
                                            u.hp,
                                            u.dmg,
                                            u.rng_studs,
                                            u.passive_desc,
                                            slot_idx + 1
                                        ));
                                    if resp.clicked() {
                                        cfg.selected_slot = slot_idx;
                                    }
                                }
                            });
                            ui.add_space(4.0);
                        }

                        // Active Faction Synergy Badges
                        let aff = evaluate_deck_affinities(&cfg.player_deck);
                        ui.horizontal_wrapped(|ui| {
                            if aff.active_summary.is_empty() {
                                ui.label(
                                    egui::RichText::new(
                                        "Synergy: Mixed Legion (equip 3+ or 6/6 same faction for bonus)",
                                    )
                                    .font(body_font(11.0))
                                    .color(TEXT_MUTED),
                                );
                            } else {
                                for (fac, count, is_6) in &aff.active_summary {
                                    let bonus_txt = if *is_6 {
                                        format!(
                                            "{} {count}/6 (-10s Spell CD + 2x Affinity)",
                                            fac.code_name()
                                        )
                                    } else {
                                        format!(
                                            "{} {count}/6 (3-pc Affinity Active)",
                                            fac.code_name()
                                        )
                                    };
                                    ui.label(
                                        egui::RichText::new(bonus_txt)
                                            .font(mono_font(10.5))
                                            .color(faction_color(*fac)),
                                    );
                                }
                            }
                        });

                        ui.add_space(6.0);
                        cfg.draw_skirmish_troop_picker(ui, assets);
                    });
                };

                let draw_right = |ui: &mut Ui, cfg: &mut Self| {
                    let ef_acc = faction_color(cfg.enemy_faction);
                    let pf_acc = faction_color(cfg.player_faction);
                    sculpted_card(ui, None, 12, |ui| {
                        ui.set_min_width(ui.available_width());
                        if !stacked {
                            ui.set_min_height(col_min_h);
                        }
                        ui.horizontal(|ui| {
                            if let Some(crest) = assets.get_crest(cfg.enemy_faction) {
                                let (cr, _) =
                                    ui.allocate_exact_size(Vec2::splat(30.0), Sense::hover());
                                crest_socket(ui.painter(), cr, crest, ef_acc);
                            }
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.label(
                                    egui::RichText::new("Enemy & Match Rules")
                                        .font(display_font(14.5))
                                        .color(RED_ENEMY),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{} vs {} · {}",
                                        cfg.player_faction.code_name(),
                                        cfg.enemy_faction.display_name(),
                                        cfg.ai_difficulty.name()
                                    ))
                                    .font(body_font(11.0))
                                    .color(TEXT_SECONDARY),
                                );
                            });
                        });
                        ui.add_space(6.0);

                        // Open Confrontation Arena Preview Banner (no bordered frame)
                        if let Some(arena) = assets.get_arena(cfg.enemy_faction) {
                            let aw = ui.available_width();
                            let (ar, _) = ui.allocate_exact_size(Vec2::new(aw, 114.0), Sense::hover());
                            crate::assets::image_rect_tinted(
                                ui.painter(),
                                arena,
                                ar,
                                Color32::from_rgb(232, 220, 206),
                            );
                            let scrim = Rect::from_min_max(
                                Pos2::new(ar.left(), ar.bottom() - 40.0),
                                ar.max,
                            );
                            paint_vertical_gradient(
                                ui.painter(),
                                scrim,
                                Color32::from_rgba_unmultiplied(12, 10, 10, 0),
                                Color32::from_rgba_unmultiplied(12, 10, 10, 215),
                            );

                            ui.painter().text(
                                Pos2::new(ar.left() + 12.0, ar.bottom() - 15.0),
                                egui::Align2::LEFT_CENTER,
                                format!("{} Stronghold", cfg.enemy_faction.display_name()),
                                mono_font(10.5),
                                GOLD_BRIGHT,
                            );
                            ui.painter().text(
                                Pos2::new(ar.right() - 12.0, ar.bottom() - 15.0),
                                egui::Align2::RIGHT_CENTER,
                                format!(
                                    "{} vs {}",
                                    cfg.player_faction.code_name(),
                                    cfg.enemy_faction.code_name()
                                ),
                                mono_font(10.0),
                                Color32::from_rgb(
                                    ((pf_acc.r() as u16 + ef_acc.r() as u16) / 2) as u8,
                                    ((pf_acc.g() as u16 + ef_acc.g() as u16) / 2) as u8,
                                    ((pf_acc.b() as u16 + ef_acc.b() as u16) / 2) as u8,
                                ),
                            );
                        }
                        ui.add_space(8.0);

                        // Symmetric 3x2 Enemy Faction Selector Grid
                        for row_facs in FactionId::ALL.chunks(3) {
                            equal_columns(ui, 3, |ui, c| {
                                if let Some(&fid) = row_facs.get(c) {
                                    let active = cfg.enemy_faction == fid;
                                    let bw = ui.available_width();
                                    if nav_tab_button(
                                        ui,
                                        fid.display_name(),
                                        active,
                                        faction_color(fid),
                                        Vec2::new(bw, 26.0),
                                    )
                                    .clicked()
                                    {
                                        cfg.enemy_faction = fid;
                                    }
                                }
                            });
                            ui.add_space(4.0);
                        }

                        ui.add_space(8.0);

                        ui.label(
                            egui::RichText::new("Difficulty")
                                .font(mono_font(10.0))
                                .color(TEXT_MUTED),
                        );
                        ui.add_space(3.0);
                        equal_columns(ui, 4, |ui, di| {
                            if let Some(&diff) = AiDifficulty::ALL.get(di) {
                                let active = cfg.ai_difficulty == diff;
                                let short = match diff {
                                    AiDifficulty::Easy => "Easy (-15%)",
                                    AiDifficulty::Normal => "Medium (1.0x)",
                                    AiDifficulty::Hard => "Hard (+20%)",
                                    AiDifficulty::Extreme => "Extreme (+45%)",
                                };
                                let bw = ui.available_width();
                                if nav_tab_button(ui, short, active, RED_ENEMY, Vec2::new(bw, 28.0))
                                    .on_hover_text(diff.desc())
                                    .clicked()
                                {
                                    cfg.ai_difficulty = diff;
                                }
                            }
                        });
                        ui.add_space(4.0);
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(cfg.ai_difficulty.desc())
                                    .font(body_font(11.0))
                                    .color(TEXT_SECONDARY),
                            )
                            .wrap(),
                        );

                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("ENVIRONMENTAL MUTATOR")
                                .font(mono_font(10.0))
                                .color(TEXT_MUTED),
                        );
                        ui.add_space(3.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(6.0, 5.0);
                            if nav_tab_button(
                                ui,
                                "Standard Arena",
                                cfg.mutator.is_none(),
                                EMERALD,
                                Vec2::new(124.0, 26.0),
                            )
                            .clicked()
                            {
                                cfg.mutator = None;
                            }
                            for m in MutatorId::ALL {
                                let active = cfg.mutator == Some(m);
                                if nav_tab_button(
                                    ui,
                                    m.name(),
                                    active,
                                    crate::theme::AMETHYST,
                                    Vec2::new(218.0, 26.0),
                                )
                                .clicked()
                                {
                                    cfg.mutator = Some(m);
                                }
                            }
                        });
                        if let Some(m) = cfg.mutator {
                            ui.add_space(4.0);
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(m.desc())
                                        .font(body_font(11.5))
                                        .color(crate::theme::AMETHYST),
                                )
                                .wrap(),
                            );
                        }

                        ui.add_space(10.0);
                        let r_count = cfg.resolve_relics(save).len();
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Relics")
                                    .font(mono_font(10.0))
                                    .color(TEXT_MUTED),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    egui::RichText::new(format!("{r_count}/{} Active", RELICS.len()))
                                        .font(mono_font(10.0))
                                        .color(GOLD),
                                );
                            });
                        });
                        ui.add_space(3.0);
                        let relic_modes = [
                            (SkirmishRelicMode::CampaignEquipped, "Campaign"),
                            (SkirmishRelicMode::None, "None"),
                            (SkirmishRelicMode::AllUnlockedSandbox, "All Overdrive"),
                        ];
                        equal_columns(ui, 3, |ui, ri| {
                            let (mode, lbl) = relic_modes[ri];
                            let bw = ui.available_width();
                            if nav_tab_button(
                                ui,
                                lbl,
                                cfg.relic_mode == mode,
                                GOLD,
                                Vec2::new(bw, 26.0),
                            )
                            .clicked()
                            {
                                cfg.relic_mode = mode;
                            }
                        });
                    });
                };

                ScrollArea::vertical()
                    .id_salt("skirmish_page_scroll")
                    .show(ui, |ui| {
                        if stacked {
                            draw_left(ui, self);
                            ui.add_space(10.0);
                            draw_right(ui, self);
                        } else {
                            equal_columns(ui, 2, |ui, ci| {
                                if ci == 0 {
                                    draw_left(ui, self);
                                } else {
                                    draw_right(ui, self);
                                }
                            });
                        }
                    });
            });

        launch
    }

    fn draw_skirmish_troop_picker(&mut self, ui: &mut Ui, assets: &AssetBank) {
        let req_tier = slot_tier(self.selected_slot);
        egui::Frame::new()
            .inner_margin(Margin::symmetric(2, 4))
            .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "Swap Slot [{}] · Tier {}",
                        self.selected_slot + 1,
                        req_tier.code()
                    ))
                    .font(mono_font(10.0))
                    .color(GOLD_BRIGHT),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("Sandbox Unlocked")
                            .font(mono_font(9.5))
                            .color(EMERALD),
                    );
                });
            });
            ui.add_space(4.0);

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
                if nav_tab_button(
                    ui,
                    "All",
                    self.picker_faction.is_none(),
                    GOLD,
                    Vec2::new(44.0, 23.0),
                )
                .clicked()
                {
                    self.picker_faction = None;
                }
                for fid in FactionId::ALL {
                    let active = self.picker_faction == Some(fid);
                    if nav_tab_button(
                        ui,
                        fid.code_name(),
                        active,
                        faction_color(fid),
                        Vec2::new(56.0, 23.0),
                    )
                    .clicked()
                    {
                        self.picker_faction = Some(fid);
                    }
                }
            });
            ui.add_space(6.0);

            let mut candidates: Vec<&'static UnitDef> = UNITS
                .iter()
                .filter(|u| u.tier == req_tier)
                .filter(|u| self.picker_faction.map(|f| f == u.faction).unwrap_or(true))
                .collect();

            // Show current commander faction units first when browsing All Factions
            candidates.sort_by_key(|u| {
                let fac_prio = if u.faction == self.player_faction { 0 } else { 1 };
                (fac_prio, u.cost)
            });

            let equipped_in_slot = self.player_deck[self.selected_slot].clone();
            let list_cols = if ui.available_width() >= 420.0 { 3 } else { 2 };
            let remaining_h = ui.available_height().clamp(84.0, 210.0);
            ScrollArea::vertical()
                .id_salt("skirmish_troop_picker_scroll")
                .max_height(remaining_h)
                .show(ui, |ui| {
                    for row in candidates.chunks(list_cols) {
                        equal_columns(ui, list_cols, |ui, ci| {
                            if let Some(u) = row.get(ci) {
                                let is_cur = equipped_in_slot == u.id;
                                let card_acc = if is_cur { Some(GOLD_BRIGHT) } else { None };
                                let card = ui
                                    .scope(|ui| {
                                        sculpted_card(ui, card_acc, 6, |ui| {
                                            ui.set_min_width(ui.available_width());
                                            ui.horizontal(|ui| {
                                                let (pr, _) = ui.allocate_exact_size(
                                                    Vec2::splat(28.0),
                                                    Sense::hover(),
                                                );
                                                portrait_pedestal(
                                                    ui.painter(),
                                                    pr,
                                                    assets.get_portrait(u.sprite_key),
                                                    if is_cur {
                                                        GOLD_BRIGHT
                                                    } else {
                                                        faction_color(u.faction)
                                                    },
                                                    is_cur,
                                                );
                                                ui.vertical(|ui| {
                                                    ui.spacing_mut().item_spacing.y = 0.0;
                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            egui::RichText::new(
                                                                u.faction.code_name(),
                                                            )
                                                            .font(mono_font(8.5))
                                                            .color(faction_color(u.faction)),
                                                        );
                                                        if is_cur {
                                                            ui.label(
                                                                egui::RichText::new("Active")
                                                                    .font(mono_font(8.5))
                                                                    .color(EMERALD),
                                                            );
                                                        }
                                                    });
                                                    ui.add(
                                                        egui::Label::new(
                                                            egui::RichText::new(u.name)
                                                                .font(body_font(11.5))
                                                                .color(TEXT_PRIMARY),
                                                        )
                                                        .truncate(),
                                                    );
                                                    ui.label(
                                                        egui::RichText::new(format!(
                                                            "{}G • {}",
                                                            u.cost,
                                                            u.shape.short_role()
                                                        ))
                                                        .font(mono_font(9.2))
                                                        .color(GOLD_BRIGHT),
                                                    );
                                                });
                                            });
                                        });
                                    })
                                    .response
                                    .interact(Sense::click())
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .on_hover_text(format!(
                                        "{} — {} ({})\nHP {:.0} | DMG {:.0} | CD {:.1}s | SPD {:.1}\n{}",
                                        u.name,
                                        u.shape.role_hint(),
                                        u.faction.display_name(),
                                        u.hp,
                                        u.dmg,
                                        u.cd,
                                        u.spd_studs,
                                        u.passive_desc
                                    ));
                                if card.clicked() {
                                    self.player_deck[self.selected_slot] = u.id.to_string();
                                }
                            }
                        });
                        ui.add_space(4.0);
                    }
                });
        });
    }
}

fn slot_tier(slot_idx: usize) -> UnitTier {
    match slot_idx {
        0 | 1 => UnitTier::C,
        2 | 3 => UnitTier::B,
        _ => UnitTier::A,
    }
}
