use egui::{Color32, CornerRadius, Margin, ScrollArea, Ui, Vec2};
use grand_conquest_core::model::MAX_RELIC_POWER;
use grand_conquest_core::relics::RELICS;
use grand_conquest_core::save::ConquestSave;

use crate::assets::{image_rect_tinted, AssetBank};
use crate::theme::{
    beveled_progress_bar, body_font, display_font, equal_columns, mono_font, nav_tab_button,
    paint_parchment_backdrop, portrait_pedestal, sculpted_card, subtle_button, BG_VOID, CYAN,
    EMERALD, GOLD, GOLD_BRIGHT, GOLD_DIM, RED_ENEMY, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

#[derive(Default)]
pub struct VaultViewState {
    pub toast_msg: String,
    pub show_unlocked_only: bool,
}

impl VaultViewState {
    pub fn draw(&mut self, ui: &mut Ui, save: &mut ConquestSave, assets: &AssetBank, on_save_dirty: &mut bool) {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG_VOID).inner_margin(Margin::same(18)))
            .show(ui, |ui| {
                let full_rect = ui.max_rect().expand(18.0);
                if let Some(parch) = assets.parchment_dark.as_ref() {
                    paint_parchment_backdrop(
                        ui.painter(),
                        full_rect,
                        parch,
                        Color32::from_rgba_unmultiplied(195, 180, 155, 68),
                    );
                }
                let stars = save.total_conquest_stars;
                let used_rp = save.used_relic_power();

                // Header & Arcane RP Bar
                egui::Frame::new()
                    .fill(Color32::from_rgba_unmultiplied(18, 16, 18, 165))
                    .corner_radius(CornerRadius::same(12))
                    .inner_margin(Margin::symmetric(16, 12))
                    .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.horizontal_wrapped(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Relic Vault")
                                    .font(display_font(20.0))
                                    .color(GOLD_BRIGHT),
                            );
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(
                                        "Unlock relics with Conquest Stars and attune up to 4 RP.",
                                    )
                                    .font(body_font(12.0))
                                    .color(TEXT_SECONDARY),
                                )
                                .wrap(),
                            );
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if nav_tab_button(
                                ui,
                                if self.show_unlocked_only { "Unlocked" } else { "All Relics" },
                                self.show_unlocked_only,
                                GOLD,
                                Vec2::new(104.0, 30.0),
                            )
                            .clicked()
                            {
                                self.show_unlocked_only = !self.show_unlocked_only;
                            }

                            ui.add_space(16.0);
                            // Clean inline 4-Pip Arcane RP Meter (zero nested box frame)
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{used_rp}/{MAX_RELIC_POWER} RP Attuned  ·  {stars}/72 Stars"
                                        ))
                                        .font(mono_font(11.0))
                                        .color(GOLD_BRIGHT),
                                    );
                                });
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    for pip in 1..=MAX_RELIC_POWER {
                                        let active = pip <= used_rp;
                                        let (r, _) = ui.allocate_exact_size(
                                            Vec2::new(42.0, 6.0),
                                            egui::Sense::hover(),
                                        );
                                        ui.painter().rect_filled(
                                            r,
                                            CornerRadius::same(3),
                                            if active { CYAN } else { Color32::from_rgba_unmultiplied(45, 40, 44, 180) },
                                        );
                                    }
                                });
                            });
                        });
                    });
                    if !self.toast_msg.is_empty() {
                        ui.add_space(6.0);
                        let is_err = self.toast_msg.contains("Not enough")
                            || self.toast_msg.contains("locked")
                            || self.toast_msg.contains("Exceeds");
                        ui.label(
                            egui::RichText::new(&self.toast_msg)
                                .font(body_font(11.5))
                                .color(if is_err { RED_ENEMY } else { EMERALD }),
                        );
                    }
                });

                ui.add_space(12.0);

                // Responsive Relic Altar Grid (3 cols at >=1160px, 2 cols at 1080px)
                ScrollArea::vertical().show(ui, |ui| {
                    let avail_w = ui.available_width();
                    let cols = ((avail_w / 375.0).floor() as usize).clamp(1, 3);
                    let list: Vec<_> = RELICS
                        .iter()
                        .filter(|r| {
                            !self.show_unlocked_only
                                || stars >= r.stars_req
                                || save.unlocked_relic_ids.iter().any(|x| x == r.id.as_str())
                        })
                        .collect();

                    for row in list.chunks(cols) {
                        equal_columns(ui, cols, |ui, ci| {
                            if let Some(relic) = row.get(ci) {
                                let rid_str = relic.id.as_str();
                                let unlocked = stars >= relic.stars_req
                                    || save.unlocked_relic_ids.iter().any(|x| x == rid_str);
                                let equipped =
                                    save.equipped_relic_ids.iter().any(|x| x == rid_str);
                                let card_acc = if equipped { Some(GOLD_BRIGHT) } else { None };

                                egui::Frame::NONE.show(ui, |ui| {
                                    sculpted_card(ui, card_acc, 12, |ui| {
                                        ui.set_min_width(ui.available_width());
                                        ui.set_min_height(126.0);
                                        ui.horizontal_top(|ui| {
                                            let (sock, _) = ui.allocate_exact_size(
                                                Vec2::splat(52.0),
                                                egui::Sense::hover(),
                                            );
                                            if unlocked {
                                                portrait_pedestal(
                                                    ui.painter(),
                                                    sock,
                                                    assets.get_relic_icon(relic.id),
                                                    if equipped { GOLD_BRIGHT } else { CYAN },
                                                    equipped,
                                                );
                                            } else if let Some(icon) = assets.get_relic_icon(relic.id) {
                                                ui.painter().circle_filled(
                                                    sock.center(),
                                                    sock.width() * 0.42,
                                                    Color32::from_rgba_unmultiplied(30, 26, 28, 120),
                                                );
                                                image_rect_tinted(
                                                    ui.painter(),
                                                    icon,
                                                    sock.shrink(5.0),
                                                    Color32::from_rgba_unmultiplied(120, 108, 94, 130),
                                                );
                                            }
                                            ui.add_space(10.0);
                                            ui.vertical(|ui| {
                                                let vw = ui.available_width();
                                                ui.set_max_width(vw);
                                                ui.horizontal(|ui| {
                                                    ui.label(
                                                        egui::RichText::new(relic.badge)
                                                            .font(mono_font(9.5))
                                                            .color(if equipped { GOLD_BRIGHT } else { TEXT_MUTED }),
                                                    );
                                                    ui.with_layout(
                                                        egui::Layout::right_to_left(egui::Align::Center),
                                                        |ui| {
                                                            ui.label(
                                                                egui::RichText::new(format!("{} RP", relic.rp_cost))
                                                                    .font(mono_font(10.0))
                                                                    .color(if equipped { GOLD_BRIGHT } else { CYAN }),
                                                            );
                                                        },
                                                    );
                                                });
                                                ui.add_space(2.0);
                                                ui.add(
                                                    egui::Label::new(
                                                        egui::RichText::new(relic.name)
                                                            .font(display_font(13.8))
                                                            .color(if equipped {
                                                                GOLD_BRIGHT
                                                            } else if unlocked {
                                                                TEXT_PRIMARY
                                                            } else {
                                                                TEXT_MUTED
                                                            }),
                                                    )
                                                    .truncate(),
                                                );
                                                ui.add(
                                                    egui::Label::new(
                                                        egui::RichText::new(relic.desc)
                                                            .font(body_font(11.8))
                                                            .color(if unlocked {
                                                                TEXT_SECONDARY
                                                            } else {
                                                                TEXT_MUTED
                                                            }),
                                                    )
                                                    .wrap(),
                                                );
                                            });
                                        });

                                        ui.add_space(8.0);
                                        ui.horizontal(|ui| {
                                            if equipped {
                                                ui.label(
                                                    egui::RichText::new("Attuned")
                                                        .font(mono_font(10.0))
                                                        .color(GOLD_BRIGHT),
                                                );
                                            } else if unlocked {
                                                ui.label(
                                                    egui::RichText::new("Unlocked")
                                                        .font(mono_font(10.0))
                                                        .color(EMERALD),
                                                );
                                            } else {
                                                let prog_frac = if relic.stars_req == 0 {
                                                    0.0
                                                } else {
                                                    (stars as f32 / relic.stars_req as f32).clamp(0.0, 1.0)
                                                };
                                                ui.vertical(|ui| {
                                                    ui.label(
                                                        egui::RichText::new(format!(
                                                            "{stars}/{} Stars",
                                                            relic.stars_req
                                                        ))
                                                        .font(mono_font(10.0))
                                                        .color(TEXT_MUTED),
                                                    );
                                                    beveled_progress_bar(
                                                        ui,
                                                        prog_frac,
                                                        Vec2::new(120.0, 4.0),
                                                        GOLD_DIM,
                                                        None,
                                                    );
                                                });
                                            }

                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    if unlocked {
                                                        let lbl = if equipped { "Remove" } else { "Attune" };
                                                        let can_equip = equipped
                                                            || used_rp + relic.rp_cost <= MAX_RELIC_POWER;
                                                        if subtle_button(
                                                            ui,
                                                            lbl,
                                                            equipped,
                                                            if can_equip { GOLD } else { RED_ENEMY },
                                                            Vec2::new(82.0, 26.0),
                                                        )
                                                        .clicked()
                                                        {
                                                            match save.try_toggle_relic(rid_str) {
                                                                Ok(eq) => {
                                                                    *on_save_dirty = true;
                                                                    self.toast_msg = if eq {
                                                                        format!(
                                                                            "Attuned {} ({} RP).",
                                                                            relic.name, relic.rp_cost
                                                                        )
                                                                    } else {
                                                                        format!(
                                                                            "Removed {}.",
                                                                            relic.name
                                                                        )
                                                                    };
                                                                }
                                                                Err(err) => {
                                                                    self.toast_msg = err.to_string();
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                            );
                                        });
                                    });
                                });
                            }
                        });
                        ui.add_space(10.0);
                    }
                });
            });
    }
}
