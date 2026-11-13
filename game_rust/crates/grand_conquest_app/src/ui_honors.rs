use egui::{Color32, Margin, Vec2};
use grand_conquest_core::honors::{
    blueprint_treasury_price, AchievementCategory, CareerProfile, ACHIEVEMENTS, TITLES, TROPHIES,
    WAR_CHEST_COST,
};
use grand_conquest_core::model::{FactionId, UnitDef};
use grand_conquest_core::save::ConquestSave;
use grand_conquest_core::units::UNITS;
use rand::rngs::StdRng;

use crate::assets::AssetBank;
use crate::audio::SoundEngine;
use crate::theme::{
    beveled_progress_bar, body_font, crest_socket, cta_button, display_font, draw_badge,
    equal_columns, faction_color, filigree_divider, mono_font, nav_tab_button,
    paint_parchment_backdrop, portrait_pedestal, sculpted_card, sculpted_panel, styled_toggle,
    subtle_button, EMERALD, GOLD, GOLD_BRIGHT, RED_ENEMY, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HonorsTab {
    Treasury,
    TrophiesAndAchievements,
}

#[derive(Debug, Clone)]
pub struct HonorsViewState {
    pub tab: HonorsTab,
    pub faction_filter: Option<FactionId>,
    pub hide_owned_blueprints: bool,
    pub ach_filter: Option<AchievementCategory>,
    pub status_banner: Option<(String, bool)>,
    pub last_chest_unit_id: Option<&'static str>,
}

impl Default for HonorsViewState {
    fn default() -> Self {
        Self {
            tab: HonorsTab::Treasury,
            faction_filter: None,
            hide_owned_blueprints: false,
            ach_filter: None,
            status_banner: None,
            last_chest_unit_id: None,
        }
    }
}

pub fn draw_honors_screen(
    ui: &mut egui::Ui,
    save: &mut ConquestSave,
    career: &mut CareerProfile,
    view: &mut HonorsViewState,
    assets: &AssetBank,
    audio: &SoundEngine,
    rng: &mut StdRng,
) -> bool {
    let mut dirty = false;
    let rect = ui.available_rect_before_wrap();
    if let Some(ref tex) = assets.parchment_dark {
        paint_parchment_backdrop(
            ui.painter(),
            rect,
            tex,
            Color32::from_rgba_unmultiplied(20, 17, 15, 192),
        );
    }

    egui::Frame::new()
        .inner_margin(Margin::symmetric(18, 12))
        .show(ui, |ui| {
            // Sculpted Header Banner
            sculpted_panel(ui, GOLD, 12, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("IMPERIAL TREASURY & HALL OF HONORS")
                                .font(display_font(21.0))
                                .color(GOLD_BRIGHT),
                        );
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(format!(
                                    "Vault Gold: {}G  •  Rank: {}  •  Trophies: {}/{}  •  Achievements: {}/{}",
                                    save.account_gold_vault,
                                    career.active_title().label,
                                    save.unlocked_trophies.len(),
                                    TROPHIES.len(),
                                    career.unlocked_achievements.len(),
                                    ACHIEVEMENTS.len()
                                ))
                                .font(body_font(12.0))
                                .color(TEXT_SECONDARY),
                            )
                            .wrap(),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if nav_tab_button(
                            ui,
                            "Trophy Hall & Honors",
                            view.tab == HonorsTab::TrophiesAndAchievements,
                            GOLD,
                            Vec2::new(168.0, 34.0),
                        )
                        .clicked()
                        {
                            audio.play_click();
                            view.tab = HonorsTab::TrophiesAndAchievements;
                        }
                        if nav_tab_button(
                            ui,
                            "Treasury Store",
                            view.tab == HonorsTab::Treasury,
                            GOLD_BRIGHT,
                            Vec2::new(142.0, 34.0),
                        )
                        .clicked()
                        {
                            audio.play_click();
                            view.tab = HonorsTab::Treasury;
                        }
                    });
                });

                if let Some((ref msg, ok)) = view.status_banner {
                    ui.add_space(6.0);
                    filigree_divider(ui, if ok { EMERALD } else { RED_ENEMY });
                    ui.add_space(4.0);
                    draw_badge(
                        ui,
                        msg,
                        if ok { Color32::from_rgb(24, 64, 42) } else { Color32::from_rgb(76, 26, 26) },
                        if ok { EMERALD } else { RED_ENEMY },
                    );
                }
            });

            ui.add_space(10.0);

            match view.tab {
                HonorsTab::Treasury => {
                    dirty |= draw_treasury_tab(ui, save, career, view, assets, audio, rng);
                }
                HonorsTab::TrophiesAndAchievements => {
                    draw_trophies_tab(ui, save, career, view, assets, audio);
                }
            }
        });

    dirty
}

fn draw_treasury_tab(
    ui: &mut egui::Ui,
    save: &mut ConquestSave,
    career: &mut CareerProfile,
    view: &mut HonorsViewState,
    assets: &AssetBank,
    audio: &SoundEngine,
    rng: &mut StdRng,
) -> bool {
    let mut dirty = false;

    // Top Showcase: War Chest + Command Titles
    let avail_w = ui.available_width();
    let showcase_cols = if avail_w >= 860.0 { 2 } else { 1 };
    let owned_bp_count = save.unlocked_blueprint_ids.len().min(UNITS.len());
    let locked_count = UNITS.len().saturating_sub(owned_bp_count);

    let mut draw_chest_card = |ui: &mut egui::Ui,
                           save: &mut ConquestSave,
                           career: &mut CareerProfile,
                           view: &mut HonorsViewState,
                           dirty: &mut bool| {
        sculpted_card(ui, None, 12, |ui| {
            ui.set_min_width(ui.available_width());
            ui.set_min_height(150.0);
            ui.horizontal(|ui| {
                // Left details + progress + CTA
                let right_reserve = 112.0;
                let left_w = (ui.available_width() - right_reserve - 12.0).max(180.0);
                ui.allocate_ui(Vec2::new(left_w, 132.0), |ui| {
                    ui.set_min_width(left_w);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Supply War Chest")
                                .font(display_font(16.5))
                                .color(GOLD_BRIGHT),
                        );
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(format!(
                                    "Unlocks 1 random unowned unit ({locked_count} left)."
                                ))
                                .font(body_font(11.8))
                                .color(TEXT_SECONDARY),
                            )
                            .wrap(),
                        );
                        ui.add_space(6.0);
                        let frac = owned_bp_count as f32 / UNITS.len().max(1) as f32;
                        beveled_progress_bar(
                            ui,
                            frac,
                            Vec2::new(left_w.min(290.0), 14.0),
                            GOLD,
                            Some(&format!("{owned_bp_count}/{} Unlocked", UNITS.len())),
                        );
                        ui.add_space(8.0);
                        let can_open = locked_count > 0 && save.account_gold_vault >= WAR_CHEST_COST;
                        if can_open {
                            if cta_button(
                                ui,
                                &format!("Open Crate · {WAR_CHEST_COST}G"),
                                Vec2::new(left_w.clamp(170.0, 210.0), 32.0),
                            )
                            .clicked()
                            {
                                match career.try_open_war_chest(save, rng) {
                                    Ok(u) => {
                                        audio.play_unlock();
                                        view.last_chest_unit_id = Some(u.id);
                                        view.status_banner = Some((
                                            format!(
                                                "Requisitioned {} ({} Tier {})!",
                                                u.name,
                                                u.faction.code_name(),
                                                u.tier.code()
                                            ),
                                            true,
                                        ));
                                        *dirty = true;
                                    }
                                    Err(e) => view.status_banner = Some((e.to_string(), false)),
                                }
                            }
                        } else {
                            let label = if locked_count == 0 {
                                "All 54 blueprints owned".to_string()
                            } else {
                                format!("Need {WAR_CHEST_COST}G (have {}G)", save.account_gold_vault)
                            };
                            ui.label(
                                egui::RichText::new(label)
                                    .font(mono_font(10.5))
                                    .color(TEXT_MUTED),
                            );
                        }
                    });
                });

                // Right Ornamental Reveal Pedestal (open glow, no inner box)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.allocate_ui(Vec2::new(104.0, 120.0), |ui| {
                        ui.vertical_centered(|ui| {
                            let revealed = view
                                .last_chest_unit_id
                                .and_then(|uid| UNITS.iter().find(|x| x.id == uid));
                            let ped_r = ui.allocate_space(Vec2::splat(64.0)).1;
                            if let Some(u) = revealed {
                                let acc = faction_color(u.faction);
                                portrait_pedestal(
                                    ui.painter(),
                                    ped_r,
                                    assets.get_portrait(u.sprite_key),
                                    acc,
                                    true,
                                );
                                ui.add_space(4.0);
                                ui.label(
                                    egui::RichText::new(crate::theme::clamp_chars(u.name, 12))
                                        .font(mono_font(9.5))
                                        .color(GOLD_BRIGHT),
                                );
                            } else {
                                if let Some(c) = assets.get_crest(save.player_commander_faction) {
                                    crest_socket(ui.painter(), ped_r, c, GOLD);
                                }
                                ui.add_space(4.0);
                                ui.label(
                                    egui::RichText::new(format!("{WAR_CHEST_COST}G"))
                                        .font(mono_font(10.0))
                                        .color(GOLD),
                                );
                            }
                        });
                    });
                });
            });
        });
    };

    let draw_titles_card = |ui: &mut egui::Ui,
                            save: &mut ConquestSave,
                            career: &mut CareerProfile,
                            view: &mut HonorsViewState,
                            dirty: &mut bool| {
        sculpted_card(ui, None, 12, |ui| {
            ui.set_min_width(ui.available_width());
            ui.set_min_height(150.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Commander Titles")
                        .font(display_font(15.0))
                        .color(GOLD_BRIGHT),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "Active · {}",
                            career.active_title().label
                        ))
                        .font(mono_font(10.0))
                        .color(GOLD),
                    );
                });
            });
            ui.add_space(6.0);
            egui::ScrollArea::vertical()
                .id_salt("honors_titles_scroll")
                .max_height(118.0)
                .show(ui, |ui| {
                    for t in TITLES {
                        let owned = career.owned_title_ids.iter().any(|id| id == t.id);
                        let equipped = career.active_title_id == t.id;
                        egui::Frame::new()
                            .fill(if equipped {
                                Color32::from_rgba_unmultiplied(46, 36, 22, 195)
                            } else {
                                Color32::TRANSPARENT
                            })
                            .corner_radius(6)
                            .inner_margin(Margin::symmetric(8, 3))
                            .show(ui, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.horizontal(|ui| {
                                    let btn_reserve = 86.0;
                                    let text_w = (ui.available_width() - btn_reserve - 6.0).max(140.0);
                                    ui.allocate_ui(Vec2::new(text_w, 22.0), |ui| {
                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing.x = 6.0;
                                            ui.label(
                                                egui::RichText::new(t.label)
                                                    .font(body_font(12.0))
                                                    .color(if equipped {
                                                        GOLD_BRIGHT
                                                    } else {
                                                        TEXT_PRIMARY
                                                    }),
                                            );
                                            ui.add(
                                                egui::Label::new(
                                                    egui::RichText::new(format!("— {}", t.requirement_desc))
                                                        .font(body_font(10.5))
                                                        .color(TEXT_MUTED),
                                                )
                                                .truncate(),
                                            );
                                        });
                                    });

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if equipped {
                                                ui.label(
                                                    egui::RichText::new("Equipped")
                                                        .font(mono_font(10.0))
                                                        .color(GOLD_BRIGHT),
                                                );
                                            } else if owned {
                                                if subtle_button(
                                                    ui,
                                                    "Equip",
                                                    false,
                                                    GOLD,
                                                    Vec2::new(62.0, 22.0),
                                                )
                                                .clicked()
                                                {
                                                    audio.play_click();
                                                    career.active_title_id = t.id.to_string();
                                                    *dirty = true;
                                                }
                                            } else if let Some(cost) = t.treasury_cost {
                                                let can = save.account_gold_vault >= cost;
                                                if subtle_button(
                                                    ui,
                                                    &format!("Buy {cost}G"),
                                                    can,
                                                    if can { GOLD } else { TEXT_MUTED },
                                                    Vec2::new(76.0, 22.0),
                                                )
                                                .clicked()
                                                    && can
                                                {
                                                    match career.try_buy_or_equip_title(save, t.id)
                                                    {
                                                        Ok(_) => {
                                                            audio.play_coin();
                                                            view.status_banner = Some((
                                                                format!(
                                                                    "Commissioned: {}",
                                                                    t.label
                                                                ),
                                                                true,
                                                            ));
                                                            *dirty = true;
                                                        }
                                                        Err(e) => {
                                                            view.status_banner =
                                                                Some((e.to_string(), false));
                                                        }
                                                    }
                                                }
                                            } else {
                                                ui.label(
                                                    egui::RichText::new("Earnable")
                                                        .font(mono_font(10.0))
                                                        .color(TEXT_MUTED),
                                                );
                                            }
                                        },
                                    );
                                });
                            });
                        ui.add_space(2.0);
                    }
                });
        });
    };

    if showcase_cols == 2 {
        equal_columns(ui, 2, |ui, col| {
            if col == 0 {
                draw_chest_card(ui, save, career, view, &mut dirty);
            } else {
                draw_titles_card(ui, save, career, view, &mut dirty);
            }
        });
    } else {
        draw_chest_card(ui, save, career, view, &mut dirty);
        ui.add_space(8.0);
        draw_titles_card(ui, save, career, view, &mut dirty);
    }

    ui.add_space(10.0);

    // Open Filter Strip for Blueprint Market
    egui::Frame::new()
        .inner_margin(Margin::symmetric(2, 4))
        .show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(4.0, 6.0);
            if nav_tab_button(
                ui,
                "All",
                view.faction_filter.is_none(),
                GOLD,
                Vec2::new(48.0, 26.0),
            )
            .clicked()
            {
                audio.play_click();
                view.faction_filter = None;
            }
            for fac in FactionId::ALL {
                if nav_tab_button(
                    ui,
                    fac.code_name(),
                    view.faction_filter == Some(fac),
                    faction_color(fac),
                    Vec2::new(76.0, 26.0),
                )
                .clicked()
                {
                    audio.play_click();
                    view.faction_filter = Some(fac);
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                styled_toggle(ui, &mut view.hide_owned_blueprints, "Hide Owned", GOLD);
            });
        });
    });

    ui.add_space(6.0);

    let list: Vec<&'static UnitDef> = UNITS
        .iter()
        .filter(|u| view.faction_filter.map(|f| f == u.faction).unwrap_or(true))
        .filter(|u| {
            !view.hide_owned_blueprints
                || !save.unlocked_blueprint_ids.iter().any(|id| id == u.id)
        })
        .collect();

    let market_cols = if ui.available_width() >= 960.0 {
        3
    } else if ui.available_width() >= 600.0 {
        2
    } else {
        1
    };

    egui::ScrollArea::vertical()
        .id_salt("honors_blueprint_market")
        .show(ui, |ui| {
            if list.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(24.0);
                    ui.label(
                        egui::RichText::new("All blueprints in this Legion filter are already acquired.")
                            .font(body_font(13.0))
                            .color(GOLD_BRIGHT),
                    );
                });
            }
            for chunk in list.chunks(market_cols) {
                equal_columns(ui, market_cols, |ui, col| {
                    if let Some(&unit) = chunk.get(col) {
                        let owned = save.unlocked_blueprint_ids.iter().any(|id| id == unit.id);
                        let price = blueprint_treasury_price(unit, save);
                        let acc = faction_color(unit.faction);

                        sculpted_card(ui, None, 10, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.set_min_height(118.0);
                            ui.horizontal(|ui| {
                                let port_r = ui.allocate_space(Vec2::splat(52.0)).1;
                                portrait_pedestal(
                                    ui.painter(),
                                    port_r,
                                    assets.get_portrait(unit.sprite_key),
                                    if owned { EMERALD } else { acc },
                                    !owned,
                                );
                                ui.add_space(8.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new(unit.name)
                                            .font(display_font(13.5))
                                            .color(if owned { TEXT_PRIMARY } else { GOLD_BRIGHT }),
                                    );
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{} · Tier {} · {} · {}G",
                                            unit.faction.code_name(),
                                            unit.tier.code(),
                                            unit.shape.short_role(),
                                            unit.cost
                                        ))
                                        .font(mono_font(10.0))
                                        .color(acc),
                                    );
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(unit.passive_desc)
                                                .font(body_font(11.2))
                                                .color(TEXT_SECONDARY),
                                        )
                                        .wrap(),
                                    );
                                });
                            });
                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "HP {:.0} · DMG {:.0} · RNG {:.0}",
                                        unit.hp, unit.dmg, unit.rng_studs
                                    ))
                                    .font(mono_font(10.0))
                                    .color(TEXT_MUTED),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if owned {
                                        ui.label(
                                            egui::RichText::new("Owned")
                                                .font(mono_font(10.5))
                                                .color(EMERALD),
                                        );
                                    } else {
                                        let can = save.account_gold_vault >= price;
                                        if subtle_button(
                                            ui,
                                            &format!("Unlock {price}G"),
                                            can,
                                            if can { GOLD } else { TEXT_MUTED },
                                            Vec2::new(100.0, 26.0),
                                        )
                                        .clicked()
                                        {
                                            match career.try_buy_blueprint(save, unit.id) {
                                                Ok(u) => {
                                                    audio.play_coin();
                                                    view.status_banner = Some((
                                                        format!("Unlocked {} blueprint!", u.name),
                                                        true,
                                                    ));
                                                    dirty = true;
                                                }
                                                Err(e) => {
                                                    view.status_banner = Some((e.to_string(), false));
                                                }
                                            }
                                        }
                                    }
                                });
                            });
                        });
                    }
                });
                ui.add_space(6.0);
            }
        });

    dirty
}

fn draw_trophies_tab(
    ui: &mut egui::Ui,
    save: &ConquestSave,
    career: &CareerProfile,
    view: &mut HonorsViewState,
    assets: &AssetBank,
    audio: &SoundEngine,
) {
    egui::ScrollArea::vertical()
        .id_salt("honors_trophies_scroll")
        .show(ui, |ui| {
            let avail_w = ui.available_width();

            // Section 1: 6 Sovereign Boss Trophies
            let claimed_count = TROPHIES
                .iter()
                .filter(|t| save.unlocked_trophies.iter().any(|x| x == t.id))
                .count();
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Sovereign Trophies")
                        .font(display_font(16.0))
                        .color(GOLD_BRIGHT),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{claimed_count}/{} Claimed", TROPHIES.len()))
                            .font(mono_font(11.0))
                            .color(GOLD),
                    );
                });
            });
            ui.add_space(8.0);

            let trophy_cols = if avail_w >= 940.0 {
                3
            } else if avail_w >= 580.0 {
                2
            } else {
                1
            };

            for chunk in TROPHIES.chunks(trophy_cols) {
                equal_columns(ui, trophy_cols, |ui, col| {
                    if let Some(t) = chunk.get(col) {
                        let claimed = save.unlocked_trophies.iter().any(|x| x == t.id);
                        let acc = faction_color(t.faction);
                        let card_accent = if claimed { Some(acc) } else { None };
                        sculpted_card(ui, card_accent, 12, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.set_min_height(118.0);
                            ui.horizontal(|ui| {
                                let crest_r = ui.allocate_space(Vec2::splat(48.0)).1;
                                if let Some(c) = assets.get_crest(t.faction) {
                                    crest_socket(
                                        ui.painter(),
                                        crest_r,
                                        c,
                                        if claimed { acc } else { TEXT_MUTED },
                                    );
                                }
                                ui.add_space(8.0);
                                ui.vertical(|ui| {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(t.name)
                                                .font(display_font(14.0))
                                                .color(if claimed { GOLD_BRIGHT } else { TEXT_MUTED }),
                                        )
                                        .wrap(),
                                    );
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{} · {}",
                                            t.source_mission,
                                            if claimed { "Claimed" } else { "Locked" }
                                        ))
                                        .font(mono_font(10.0))
                                        .color(if claimed { EMERALD } else { TEXT_MUTED }),
                                    );
                                });
                            });
                            ui.add_space(6.0);
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(t.lore)
                                        .font(body_font(11.5))
                                        .color(if claimed {
                                            TEXT_SECONDARY
                                        } else {
                                            TEXT_MUTED
                                        }),
                                )
                                .wrap(),
                            );
                        });
                    }
                });
                ui.add_space(6.0);
            }

            ui.add_space(12.0);

            // Section 2: 16 Career Achievements
            let earned_count = ACHIEVEMENTS
                .iter()
                .filter(|a| career.has_achievement(a.id))
                .count();
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("Career Honors")
                        .font(display_font(16.0))
                        .color(GOLD_BRIGHT),
                );
                ui.add_space(12.0);
                if nav_tab_button(
                    ui,
                    "All",
                    view.ach_filter.is_none(),
                    GOLD,
                    Vec2::new(48.0, 26.0),
                )
                .clicked()
                {
                    audio.play_click();
                    view.ach_filter = None;
                }
                for cat in [
                    AchievementCategory::Campaign,
                    AchievementCategory::Combat,
                    AchievementCategory::Arsenal,
                    AchievementCategory::Mastery,
                ] {
                    if nav_tab_button(
                        ui,
                        cat.label(),
                        view.ach_filter == Some(cat),
                        GOLD,
                        Vec2::new(82.0, 26.0),
                    )
                    .clicked()
                    {
                        audio.play_click();
                        view.ach_filter = Some(cat);
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{earned_count}/{} Earned", ACHIEVEMENTS.len()))
                            .font(mono_font(11.0))
                            .color(EMERALD),
                    );
                });
            });
            ui.add_space(8.0);

            let achs: Vec<_> = ACHIEVEMENTS
                .iter()
                .filter(|a| view.ach_filter.map(|c| c == a.category).unwrap_or(true))
                .collect();

            let ach_cols = if avail_w >= 820.0 { 2 } else { 1 };

            for chunk in achs.chunks(ach_cols) {
                equal_columns(ui, ach_cols, |ui, col| {
                    if let Some(a) = chunk.get(col) {
                        let unlocked = career.has_achievement(a.id);
                        sculpted_card(
                            ui,
                            if unlocked { Some(GOLD) } else { None },
                            10,
                            |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.set_min_height(72.0);
                                ui.horizontal(|ui| {
                                    // Soft circular medal (no boxed square container)
                                    let badge_r = ui.allocate_space(Vec2::splat(38.0)).1;
                                    let p = ui.painter();
                                    p.circle_filled(
                                        badge_r.center(),
                                        17.0,
                                        if unlocked {
                                            Color32::from_rgba_unmultiplied(70, 52, 24, 180)
                                        } else {
                                            Color32::from_rgba_unmultiplied(32, 28, 30, 140)
                                        },
                                    );
                                    crate::theme::paint_star(
                                        p,
                                        badge_r.center(),
                                        8.5,
                                        unlocked,
                                        if unlocked { GOLD_BRIGHT } else { TEXT_MUTED },
                                    );

                                    ui.add_space(8.0);
                                    let reward_reserve = 72.0;
                                    let text_w =
                                        (ui.available_width() - reward_reserve - 6.0).max(150.0);
                                    ui.allocate_ui(Vec2::new(text_w, 54.0), |ui| {
                                        ui.vertical(|ui| {
                                            ui.label(
                                                egui::RichText::new(a.title)
                                                    .font(display_font(13.5))
                                                    .color(if unlocked {
                                                        GOLD_BRIGHT
                                                    } else {
                                                        TEXT_PRIMARY
                                                    }),
                                            );
                                            ui.add(
                                                egui::Label::new(
                                                    egui::RichText::new(a.desc)
                                                        .font(body_font(11.4))
                                                        .color(TEXT_SECONDARY),
                                                )
                                                .wrap(),
                                            );
                                        });
                                    });

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            ui.label(
                                                egui::RichText::new(if unlocked {
                                                    "Earned".to_string()
                                                } else {
                                                    format!("+{}G", a.reward_gold)
                                                })
                                                .font(mono_font(10.5))
                                                .color(if unlocked { EMERALD } else { GOLD }),
                                            );
                                        },
                                    );
                                });
                            },
                        );
                    }
                });
                ui.add_space(6.0);
            }
        });
}
