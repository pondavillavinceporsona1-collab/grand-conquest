use std::path::Path;

use egui::{Align2, Color32, Margin, Pos2, Rect, Stroke, Vec2};
use grand_conquest_core::honors::{inspect_slot, CareerProfile, TROPHIES};
use grand_conquest_core::missions::MISSIONS;
use grand_conquest_core::model::FactionId;
use grand_conquest_core::save::ConquestSave;
use grand_conquest_core::settings::{GameSettings, HpBarMode};
use grand_conquest_core::units::{get_faction, get_unit, starter_unit_ids};

use crate::assets::AssetBank;
use crate::audio::SoundEngine;
use crate::theme::{
    body_font, clamp_chars, crest_socket, cta_button, display_font, draw_badge, equal_columns,
    faction_color, filigree_divider, mono_font, nav_tab_button, paint_vertical_gradient,
    portrait_pedestal, sculpted_card, sculpted_panel, section_ribbon, styled_slider, styled_toggle,
    subtle_button, tier_color, well_frame, BG_CARD, BG_CARD_DIM, BG_VOID, BG_WELL, BORDER_SUBTLE,
    CYAN, EMERALD, GOLD, GOLD_BRIGHT, GOLD_DIM, RED_ENEMY, TEXT_MUTED, TEXT_PRIMARY,
    TEXT_SECONDARY,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MenuDestination {
    Atlas,
    Armory,
    Vault,
    Honors,
    Codex,
    Skirmish,
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    None,
    ContinueCampaign,
    StartNewCampaign {
        slot: usize,
        faction: FactionId,
        commander_name: String,
    },
    LoadSlot(usize),
    ResetActiveSlot,
    Navigate(MenuDestination),
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    Audio,
    Display,
    Gameplay,
}

#[derive(Debug, Clone)]
pub struct MenuOverlayState {
    pub show_new_campaign: bool,
    pub show_settings: bool,
    pub show_manual: bool,
    pub settings_tab: SettingsTab,
    pub chosen_slot: usize,
    pub chosen_faction: FactionId,
    pub commander_name_buf: String,
    pub overwrite_confirm_slot: Option<usize>,
    pub reset_slot_confirm: bool,
}

impl Default for MenuOverlayState {
    fn default() -> Self {
        Self {
            show_new_campaign: false,
            show_settings: false,
            show_manual: false,
            settings_tab: SettingsTab::Audio,
            chosen_slot: 0,
            chosen_faction: FactionId::Solaris,
            commander_name_buf: "Commander".to_string(),
            overwrite_confirm_slot: None,
            reset_slot_confirm: false,
        }
    }
}

pub fn draw_title_screen(
    ui: &mut egui::Ui,
    save: &ConquestSave,
    career: &CareerProfile,
    active_slot: usize,
    assets: &AssetBank,
    audio: &SoundEngine,
    elapsed: f32,
    overlay: &mut MenuOverlayState,
) -> MenuAction {
    let mut action = MenuAction::None;
    let avail = ui.available_rect_before_wrap();
    let painter = ui.painter_at(avail);

    // Full-screen atmospheric hero backdrop using active Legion arena artwork
    painter.rect_filled(avail, 0.0, BG_VOID);
    if let Some(arena) = assets.get_arena(save.player_commander_faction) {
        let zoom = 1.04 + (elapsed * 0.25).sin() * 0.015;
        let w = avail.width() * zoom;
        let h = avail.height() * zoom;
        let pan_x = (elapsed * 0.18).cos() * 8.0;
        let pan_y = (elapsed * 0.14).sin() * 4.0;
        let bg_rect = Rect::from_center_size(
            avail.center() + egui::vec2(pan_x, pan_y),
            egui::vec2(w, h),
        );
        crate::assets::image_rect_tinted(
            &painter,
            arena,
            bg_rect,
            Color32::from_rgba_unmultiplied(150, 140, 130, 135),
        );
    } else if let Some(ref map_tex) = assets.world_map {
        crate::assets::image_rect_tinted(
            &painter,
            map_tex,
            avail,
            Color32::from_rgba_unmultiplied(120, 110, 96, 95),
        );
    }

    // Smooth atmospheric vignette & horizontal gradient veil (zero hard seam!)
    {
        let mut mesh = egui::epaint::Mesh::default();
        let l_col = Color32::from_rgba_unmultiplied(11, 10, 11, 232);
        let m_col = Color32::from_rgba_unmultiplied(12, 11, 11, 185);
        let r_col = Color32::from_rgba_unmultiplied(14, 12, 11, 132);
        let mid_x = avail.left() + avail.width() * 0.50;
        let i0 = mesh.vertices.len() as u32;
        mesh.colored_vertex(avail.left_top(), l_col);
        mesh.colored_vertex(Pos2::new(mid_x, avail.top()), m_col);
        mesh.colored_vertex(Pos2::new(mid_x, avail.bottom()), m_col);
        mesh.colored_vertex(avail.left_bottom(), l_col);
        mesh.add_triangle(i0, i0 + 1, i0 + 2);
        mesh.add_triangle(i0, i0 + 2, i0 + 3);
        let i1 = mesh.vertices.len() as u32;
        mesh.colored_vertex(Pos2::new(mid_x, avail.top()), m_col);
        mesh.colored_vertex(avail.right_top(), r_col);
        mesh.colored_vertex(avail.right_bottom(), r_col);
        mesh.colored_vertex(Pos2::new(mid_x, avail.bottom()), m_col);
        mesh.add_triangle(i1, i1 + 1, i1 + 2);
        mesh.add_triangle(i1, i1 + 2, i1 + 3);
        painter.add(egui::Shape::mesh(mesh));
    }
    // Top & bottom subtle shadow frame vignette
    paint_vertical_gradient(
        &painter,
        Rect::from_min_max(avail.min, Pos2::new(avail.right(), avail.top() + 90.0)),
        Color32::from_rgba_unmultiplied(8, 7, 8, 130),
        Color32::TRANSPARENT,
    );
    paint_vertical_gradient(
        &painter,
        Rect::from_min_max(Pos2::new(avail.left(), avail.bottom() - 90.0), avail.max),
        Color32::TRANSPARENT,
        Color32::from_rgba_unmultiplied(8, 7, 8, 165),
    );

    // Ambient floating embers
    let accent = faction_color(save.player_commander_faction);
    for i in 0..26 {
        let seed = i as f32 * 1.73;
        let x_frac = ((seed * 0.37 + elapsed * 0.014) % 1.0 + 1.0) % 1.0;
        let y_frac = 1.0 - (((seed * 0.61 + elapsed * (0.028 + (i % 4) as f32 * 0.008)) % 1.0 + 1.0) % 1.0);
        let px = avail.left() + x_frac * avail.width();
        let py = avail.top() + y_frac * avail.height() + (elapsed * 1.5 + seed).sin() * 6.0;
        let alpha = (60.0 + 95.0 * (elapsed * 2.2 + seed * 3.0).sin().abs()) as u8;
        let r = if i % 3 == 0 { 2.4 } else { 1.4 };
        let col = if i % 4 == 0 { GOLD_BRIGHT } else { accent };
        painter.circle_filled(
            Pos2::new(px, py),
            r,
            Color32::from_rgba_unmultiplied(col.r(), col.g(), col.b(), alpha),
        );
    }

    let inner = avail.shrink2(egui::vec2(38.0, 32.0));
    let mut inner_ui = ui.new_child(egui::UiBuilder::new().max_rect(inner));
    inner_ui.horizontal(|ui| {
        // LEFT COLUMN: Title Hero + Clean Navigation
        let left_w = (inner.width() * 0.39).clamp(355.0, 445.0);
        ui.allocate_ui_with_layout(
            Vec2::new(left_w, inner.height()),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.add_space(10.0);
                // Heraldry + Title lockup
                ui.horizontal(|ui| {
                    let crest_rect = ui.allocate_space(Vec2::new(66.0, 66.0)).1;
                    if let Some(crest) = assets.get_crest(save.player_commander_faction) {
                        crest_socket(ui.painter(), crest_rect, crest, accent);
                    }
                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 3.0;
                        ui.label(
                            egui::RichText::new("GRAND CONQUEST")
                                .font(display_font(30.0))
                                .color(GOLD_BRIGHT),
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "{}  ·  {}",
                                career.commander_name,
                                save.player_commander_faction.display_name()
                            ))
                            .font(body_font(12.8))
                            .color(TEXT_SECONDARY),
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "Slot {}  ·  {}/72 Stars  ·  {}G",
                                active_slot + 1,
                                save.total_conquest_stars,
                                save.account_gold_vault
                            ))
                            .font(mono_font(10.8))
                            .color(GOLD),
                        );
                    });
                });

                ui.add_space(16.0);
                let btn_w = ui.available_width().min(420.0);

                let has_progress = save.total_conquest_stars > 0
                    || !save.missions.is_empty()
                    || save.skirmish_wins > 0;
                let continue_label = if has_progress {
                    "CONTINUE CAMPAIGN"
                } else {
                    "BEGIN CAMPAIGN"
                };
                if cta_button(ui, continue_label, Vec2::new(btn_w, 46.0)).clicked() {
                    audio.play_click();
                    action = MenuAction::ContinueCampaign;
                }
                ui.add_space(8.0);

                if subtle_button(
                    ui,
                    "Save Slots & New Campaign",
                    false,
                    GOLD,
                    Vec2::new(btn_w, 36.0),
                )
                .clicked()
                {
                    audio.play_click();
                    overlay.chosen_slot = active_slot;
                    overlay.chosen_faction = save.player_commander_faction;
                    overlay.commander_name_buf = career.commander_name.clone();
                    overlay.overwrite_confirm_slot = None;
                    overlay.show_new_campaign = true;
                }
                ui.add_space(7.0);

                if subtle_button(
                    ui,
                    "Skirmish vs AI",
                    false,
                    accent,
                    Vec2::new(btn_w, 36.0),
                )
                .clicked()
                {
                    audio.play_click();
                    action = MenuAction::Navigate(MenuDestination::Skirmish);
                }

                ui.add_space(18.0);
                ui.label(
                    egui::RichText::new("COMMAND QUARTERS")
                        .font(mono_font(10.2))
                        .color(TEXT_MUTED),
                );
                ui.add_space(6.0);

                let half_w = (btn_w - 8.0) * 0.5;
                ui.horizontal(|ui| {
                    if subtle_button(ui, "Armory & Deck", false, GOLD, Vec2::new(half_w, 34.0)).clicked() {
                        audio.play_click();
                        action = MenuAction::Navigate(MenuDestination::Armory);
                    }
                    if subtle_button(ui, "Relic Vault", false, GOLD, Vec2::new(half_w, 34.0)).clicked() {
                        audio.play_click();
                        action = MenuAction::Navigate(MenuDestination::Vault);
                    }
                });
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if subtle_button(ui, "Treasury & Honors", false, GOLD_BRIGHT, Vec2::new(half_w, 34.0))
                        .clicked()
                    {
                        audio.play_click();
                        action = MenuAction::Navigate(MenuDestination::Honors);
                    }
                    if subtle_button(ui, "Unit Codex", false, GOLD, Vec2::new(half_w, 34.0)).clicked() {
                        audio.play_click();
                        action = MenuAction::Navigate(MenuDestination::Codex);
                    }
                });

                ui.add_space(18.0);
                ui.horizontal(|ui| {
                    let third_w = (btn_w - 16.0) / 3.0;
                    if nav_tab_button(ui, "Settings", false, TEXT_SECONDARY, Vec2::new(third_w, 32.0)).clicked() {
                        audio.play_click();
                        overlay.show_settings = true;
                    }
                    if nav_tab_button(ui, "Field Manual", false, TEXT_SECONDARY, Vec2::new(third_w, 32.0)).clicked() {
                        audio.play_click();
                        overlay.show_manual = true;
                    }
                    if nav_tab_button(ui, "Quit", false, RED_ENEMY, Vec2::new(third_w, 32.0)).clicked() {
                        audio.play_click();
                        action = MenuAction::Quit;
                    }
                });
            },
        );

        ui.add_space(26.0);

        // RIGHT COLUMN: Clean Open War-Table Showcase
        let right_w = ui.available_width();
        ui.allocate_ui_with_layout(
            Vec2::new(right_w, inner.height()),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        sculpted_panel(ui, GOLD, 20, |ui| {
                            ui.set_min_width(ui.available_width());
                            let fac_def = get_faction(save.player_commander_faction);

                            // Open Faction Header Strip (soft fade, no square cage)
                            let banner_w = ui.available_width();
                            let (banner_r, _) =
                                ui.allocate_exact_size(Vec2::new(banner_w, 68.0), egui::Sense::hover());
                            {
                                let bp = ui.painter().with_clip_rect(banner_r);
                                if let Some(arena) = assets.get_arena(save.player_commander_faction) {
                                    crate::assets::image_rect_tinted(
                                        &bp,
                                        arena,
                                        banner_r,
                                        Color32::from_rgba_unmultiplied(210, 196, 178, 105),
                                    );
                                }
                                paint_vertical_gradient(
                                    &bp,
                                    banner_r,
                                    Color32::from_rgba_unmultiplied(16, 14, 14, 150),
                                    Color32::from_rgba_unmultiplied(16, 14, 14, 232),
                                );
                            }

                            let mut banner_ui = ui.new_child(egui::UiBuilder::new().max_rect(banner_r.shrink2(Vec2::new(8.0, 8.0))));
                            banner_ui.horizontal(|ui| {
                                if let Some(crest) = assets.get_crest(save.player_commander_faction) {
                                    let (cr, _) = ui.allocate_exact_size(Vec2::splat(46.0), egui::Sense::hover());
                                    crest_socket(ui.painter(), cr, crest, accent);
                                }
                                ui.add_space(6.0);
                                ui.vertical(|ui| {
                                    ui.spacing_mut().item_spacing.y = 1.0;
                                    ui.label(
                                        egui::RichText::new(fac_def.id.display_name().to_uppercase())
                                            .font(display_font(20.0))
                                            .color(accent),
                                    );
                                    ui.label(
                                        egui::RichText::new(fac_def.id.subtitle())
                                            .font(body_font(12.0))
                                            .color(TEXT_SECONDARY),
                                    );
                                });
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let next_txt = if let Some(m) = next_campaign_mission(save) {
                                        format!("Next · {}", clamp_chars(m.name, 22))
                                    } else {
                                        "24 / 24 Cleared".to_string()
                                    };
                                    ui.label(
                                        egui::RichText::new(next_txt)
                                            .font(body_font(11.8))
                                            .color(GOLD_BRIGHT),
                                    );
                                });
                            });

                            ui.add_space(16.0);
                            // Open Commander Spells Strip (no nested boxes)
                            equal_columns(ui, 2, |ui, col| {
                                let (key, name, desc, s_col) = if col == 0 {
                                    ("[Q]", fac_def.q_spell_name, fac_def.q_spell_desc, GOLD_BRIGHT)
                                } else {
                                    ("[E]", fac_def.e_spell_name, fac_def.e_spell_desc, CYAN)
                                };
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(key).font(mono_font(11.0)).color(s_col));
                                    ui.label(egui::RichText::new(name).font(display_font(13.5)).color(TEXT_PRIMARY));
                                });
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(desc)
                                            .font(body_font(11.5))
                                            .color(TEXT_SECONDARY),
                                    )
                                    .wrap(),
                                );
                            });

                            ui.add_space(18.0);
                            ui.label(
                                egui::RichText::new("ACTIVE WARBAND")
                                    .font(mono_font(10.2))
                                    .color(TEXT_MUTED),
                            );
                            ui.add_space(8.0);
                            equal_columns(ui, 6, |ui, slot| {
                                let uid = &save.equipped_deck_unit_ids[slot];
                                let unit = get_unit(uid);
                                let tc = unit.map(|u| tier_color(u.tier)).unwrap_or(BORDER_SUBTLE);
                                ui.vertical_centered(|ui| {
                                    let port_size = ui.available_width().min(60.0);
                                    let rect = ui.allocate_space(Vec2::splat(port_size)).1;
                                    if let Some(u) = unit {
                                        portrait_pedestal(
                                            ui.painter(),
                                            rect,
                                            assets.get_portrait(u.sprite_key),
                                            tc,
                                            true,
                                        );
                                        ui.add_space(4.0);
                                        ui.label(
                                            egui::RichText::new(clamp_chars(u.name, 11))
                                                .font(body_font(11.2))
                                                .color(TEXT_PRIMARY),
                                        );
                                        ui.label(
                                            egui::RichText::new(format!("{}G", u.cost))
                                                .font(mono_font(10.0))
                                                .color(tc),
                                        );
                                    } else {
                                        portrait_pedestal(ui.painter(), rect, None, BORDER_SUBTLE, false);
                                        ui.add_space(4.0);
                                        ui.label(
                                            egui::RichText::new("Locked")
                                                .font(body_font(11.0))
                                                .color(TEXT_MUTED),
                                        );
                                    }
                                });
                            });

                            ui.add_space(18.0);
                            filigree_divider(ui, BORDER_SUBTLE);
                            ui.add_space(10.0);

                            // Clean Open Mastery Telemetry Row (zero boxed sub-cards)
                            equal_columns(ui, 4, |ui, col| {
                                let (title, val, col_tint) = match col {
                                    0 => ("STARS", format!("{} / 72", save.total_conquest_stars), GOLD_BRIGHT),
                                    1 => ("BLUEPRINTS", format!("{} / 54", save.unlocked_blueprint_ids.len()), accent),
                                    2 => ("TROPHIES", format!("{} / {}", save.unlocked_trophies.len(), TROPHIES.len()), EMERALD),
                                    _ => ("VICTORIES", format!("{}", career.total_wins.max(save.skirmish_wins as u64)), GOLD),
                                };
                                ui.label(egui::RichText::new(title).font(mono_font(9.8)).color(TEXT_MUTED));
                                ui.label(egui::RichText::new(val).font(display_font(17.0)).color(col_tint));
                            });
                        });
                    });
            },
        );
    });

    action
}

fn draw_modal_shell(
    ctx: &egui::Context,
    id_tag: &str,
    title: &str,
    close_label: &str,
    max_size: Vec2,
    accent: Color32,
    close_clicked: &mut bool,
    body: impl FnOnce(&mut egui::Ui),
) {
    egui::Area::new(egui::Id::unique((id_tag, "backdrop")))
        .fixed_pos(Pos2::ZERO)
        .order(egui::Order::Middle)
        .fade_in(false)
        .show(ctx, |ui| {
            let r = ctx.viewport_rect();
            ui.painter()
                .rect_filled(r, 0.0, Color32::from_rgba_unmultiplied(6, 5, 5, 212));
            ui.allocate_rect(r, egui::Sense::click());
        });

    let vp = ctx.viewport_rect();
    let size = Vec2::new(
        (vp.width() - 40.0).max(320.0).min(max_size.x),
        (vp.height() - 40.0).max(280.0).min(max_size.y),
    );

    egui::Window::new(id_tag)
        .order(egui::Order::Foreground)
        .title_bar(false)
        .fade_in(false)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .fixed_size(size)
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            sculpted_panel(ui, accent, 18, |ui| {
                ui.set_min_size(Vec2::new(size.x - 36.0, size.y - 36.0));
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(title)
                            .font(display_font(19.5))
                            .color(GOLD_BRIGHT),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if subtle_button(ui, close_label, false, accent, Vec2::new(74.0, 28.0))
                            .clicked()
                        {
                            *close_clicked = true;
                        }
                    });
                });

                crate::theme::filigree_divider(ui, accent);
                ui.add_space(4.0);

                egui::ScrollArea::vertical()
                    .id_salt((id_tag, "body_scroll"))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        body(ui);
                    });
            });
        });
}

pub fn draw_overlays(
    ctx: &egui::Context,
    storage_dir: &Path,
    active_slot: usize,
    settings: &mut GameSettings,
    audio: &mut SoundEngine,
    assets: &AssetBank,
    overlay: &mut MenuOverlayState,
) -> MenuAction {
    use crate::theme::{AMETHYST, GOLD_DARK};

    let mut action = MenuAction::None;

    if overlay.show_new_campaign {
        let mut closed = false;
        draw_modal_shell(
            ctx,
            "Campaign Profiles & Starting Legion",
            "COMMANDER PROFILES & NEW EXPEDITION",
            "Close",
            Vec2::new(860.0, 620.0),
            GOLD,
            &mut closed,
            |ui| {
                section_ribbon(
                    ui,
                    "1. CAMPAIGN ARCHIVE SLOTS",
                    Some("Select an active chronicle or choose an archive slot to overwrite"),
                    GOLD,
                );
                ui.add_space(6.0);

                equal_columns(ui, 3, |ui, slot_idx| {
                    let meta = inspect_slot(storage_dir, slot_idx);
                    let is_sel = overlay.chosen_slot == slot_idx;
                    let is_curr = active_slot == slot_idx;
                    let acc = faction_color(meta.faction);
                    let border_acc = if is_sel {
                        GOLD
                    } else if meta.exists {
                        acc
                    } else {
                        GOLD_DARK
                    };

                    sculpted_card(ui, Some(border_acc), 10, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("SLOT {}", slot_idx + 1))
                                    .font(display_font(14.0))
                                    .color(if is_sel { GOLD_BRIGHT } else { TEXT_PRIMARY }),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if is_curr {
                                        draw_badge(ui, "ACTIVE", GOLD_DIM, GOLD_BRIGHT);
                                    } else if is_sel {
                                        draw_badge(ui, "TARGET", BG_WELL, EMERALD);
                                    }
                                },
                            );
                        });

                        ui.add_space(4.0);
                        if meta.exists {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(format!(
                                        "{}  ({})",
                                        meta.commander_name,
                                        meta.faction.code_name()
                                    ))
                                    .font(body_font(12.5))
                                    .color(acc),
                                )
                                .truncate(),
                            );
                            ui.add_space(4.0);
                            ui.horizontal_wrapped(|ui| {
                                draw_badge(
                                    ui,
                                    &format!("{} Stars", meta.stars),
                                    BG_WELL,
                                    GOLD_BRIGHT,
                                );
                                draw_badge(
                                    ui,
                                    &format!("{}G", meta.vault_gold),
                                    BG_WELL,
                                    TEXT_PRIMARY,
                                );
                                draw_badge(
                                    ui,
                                    &format!("{}/54 Units", meta.unlocked_units),
                                    BG_WELL,
                                    TEXT_SECONDARY,
                                );
                            });
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                if subtle_button(
                                    ui,
                                    if is_sel { "Targeted" } else { "Select" },
                                    is_sel,
                                    GOLD,
                                    Vec2::new(82.0, 26.0),
                                )
                                .clicked()
                                {
                                    audio.play_click();
                                    overlay.chosen_slot = slot_idx;
                                    overlay.overwrite_confirm_slot = None;
                                }
                                if !is_curr
                                    && subtle_button(
                                        ui,
                                        "Load Slot",
                                        false,
                                        EMERALD,
                                        Vec2::new(86.0, 26.0),
                                    )
                                    .clicked()
                                {
                                    audio.play_click();
                                    overlay.show_new_campaign = false;
                                    action = MenuAction::LoadSlot(slot_idx);
                                }
                            });
                        } else {
                            ui.label(
                                egui::RichText::new("Empty Campaign Slot")
                                    .font(body_font(12.5))
                                    .color(TEXT_MUTED),
                            );
                            ui.label(
                                egui::RichText::new("Unclaimed tactical ledger")
                                    .font(mono_font(10.5))
                                    .color(TEXT_MUTED),
                            );
                            ui.add_space(10.0);
                            if subtle_button(
                                ui,
                                if is_sel { "Targeted" } else { "Target Slot" },
                                is_sel,
                                GOLD,
                                Vec2::new(ui.available_width(), 26.0),
                            )
                            .clicked()
                            {
                                audio.play_click();
                                overlay.chosen_slot = slot_idx;
                                overlay.overwrite_confirm_slot = None;
                            }
                        }
                    });
                });

                ui.add_space(10.0);
                well_frame()
                    .inner_margin(Margin::symmetric(12, 8))
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                egui::RichText::new("2. COMMANDER CALLSIGN")
                                    .font(mono_font(11.0))
                                    .color(GOLD),
                            );
                            ui.add_space(6.0);
                            ui.add(
                                egui::TextEdit::singleline(&mut overlay.commander_name_buf)
                                    .desired_width(240.0)
                                    .char_limit(22),
                            );
                            draw_badge(ui, "MAX 22 CHARS", BG_CARD_DIM, TEXT_MUTED);
                        });
                    });

                ui.add_space(10.0);
                section_ribbon(
                    ui,
                    "3. CHOOSE STARTING LEGION",
                    Some("Defines initial starter deck, Fortress Q/E spells & 3-pc/6-pc bonus"),
                    GOLD,
                );
                ui.add_space(6.0);

                equal_columns(ui, 3, |ui, col| {
                    for row in 0..2 {
                        let fac = FactionId::ALL[col * 2 + row];
                        let fac_def = get_faction(fac);
                        let is_sel = overlay.chosen_faction == fac;
                        let acc = faction_color(fac);

                        sculpted_card(
                            ui,
                            Some(if is_sel { acc } else { GOLD_DARK }),
                            10,
                            |ui| {
                                ui.set_min_width(ui.available_width());

                                // Subtle faction arena horizon sliver behind header
                                if let Some(arena) = assets.get_arena(fac) {
                                    let w = ui.available_width();
                                    let (bg_rect, _) = ui.allocate_exact_size(
                                        Vec2::new(w, 40.0),
                                        egui::Sense::hover(),
                                    );
                                    ui.painter().image(
                                        arena.id(),
                                        bg_rect,
                                        Rect::from_min_max(
                                            Pos2::new(0.08, 0.25),
                                            Pos2::new(0.92, 0.62),
                                        ),
                                        Color32::from_rgba_unmultiplied(255, 255, 255, 110),
                                    );
                                    paint_vertical_gradient(
                                        ui.painter(),
                                        bg_rect,
                                        Color32::from_rgba_unmultiplied(12, 10, 9, 95),
                                        Color32::from_rgba_unmultiplied(16, 13, 11, 225),
                                    );
                                    let crest_r = Rect::from_min_size(
                                        Pos2::new(bg_rect.min.x + 6.0, bg_rect.min.y + 4.0),
                                        Vec2::splat(32.0),
                                    );
                                    if let Some(c) = assets.get_crest(fac) {
                                        crest_socket(ui.painter(), crest_r, c, acc);
                                    }
                                    ui.painter().text(
                                        Pos2::new(bg_rect.min.x + 46.0, bg_rect.center().y),
                                        egui::Align2::LEFT_CENTER,
                                        fac.display_name(),
                                        display_font(13.5),
                                        if is_sel { GOLD_BRIGHT } else { acc },
                                    );
                                } else {
                                    ui.horizontal(|ui| {
                                        let r = ui.allocate_space(Vec2::splat(32.0)).1;
                                        if let Some(c) = assets.get_crest(fac) {
                                            crest_socket(ui.painter(), r, c, acc);
                                        }
                                        ui.label(
                                            egui::RichText::new(fac.display_name())
                                                .font(display_font(13.5))
                                                .color(acc),
                                        );
                                    });
                                }

                                ui.add_space(4.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(fac_def.affinity_3pc_desc)
                                            .font(body_font(10.5))
                                            .color(TEXT_SECONDARY),
                                    )
                                    .wrap(),
                                );
                                ui.add_space(4.0);
                                ui.horizontal_wrapped(|ui| {
                                    for uid in starter_unit_ids(fac) {
                                        if let Some(u) = get_unit(uid) {
                                            draw_badge(ui, u.name, BG_WELL, TEXT_PRIMARY);
                                        }
                                    }
                                });
                                ui.add_space(6.0);
                                if subtle_button(
                                    ui,
                                    if is_sel { "SELECTED LEGION" } else { "Command Legion" },
                                    is_sel,
                                    acc,
                                    Vec2::new(ui.available_width(), 26.0),
                                )
                                .clicked()
                                {
                                    audio.play_click();
                                    overlay.chosen_faction = fac;
                                }
                            },
                        );
                        if row == 0 {
                            ui.add_space(6.0);
                        }
                    }
                });

                ui.add_space(10.0);
                let target_meta = inspect_slot(storage_dir, overlay.chosen_slot);
                sculpted_card(
                    ui,
                    Some(if target_meta.exists { RED_ENEMY } else { GOLD_DIM }),
                    10,
                    |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.horizontal_wrapped(|ui| {
                            let btn_w = ui.available_width().min(410.0);
                            if target_meta.exists
                                && overlay.overwrite_confirm_slot != Some(overlay.chosen_slot)
                            {
                                if cta_button(
                                    ui,
                                    &format!(
                                        "START NEW CAMPAIGN AS {} (SLOT {})",
                                        overlay.chosen_faction.code_name().to_uppercase(),
                                        overlay.chosen_slot + 1
                                    ),
                                    Vec2::new(btn_w, 38.0),
                                )
                                .clicked()
                                {
                                    overlay.overwrite_confirm_slot = Some(overlay.chosen_slot);
                                }
                            } else {
                                let label = if target_meta.exists {
                                    format!(
                                        "CONFIRM OVERWRITE SLOT {} & LAUNCH",
                                        overlay.chosen_slot + 1
                                    )
                                } else {
                                    format!(
                                        "LAUNCH {} CAMPAIGN IN SLOT {}",
                                        overlay.chosen_faction.code_name().to_uppercase(),
                                        overlay.chosen_slot + 1
                                    )
                                };
                                if cta_button(ui, &label, Vec2::new(btn_w, 38.0)).clicked() {
                                    audio.play_unlock();
                                    let name = overlay.commander_name_buf.trim().to_string();
                                    overlay.show_new_campaign = false;
                                    overlay.overwrite_confirm_slot = None;
                                    action = MenuAction::StartNewCampaign {
                                        slot: overlay.chosen_slot,
                                        faction: overlay.chosen_faction,
                                        commander_name: if name.is_empty() {
                                            "Commander".to_string()
                                        } else {
                                            name
                                        },
                                    };
                                }
                            }
                            if target_meta.exists {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(format!(
                                            "Warning: Slot {} currently holds {} Stars ({}).",
                                            overlay.chosen_slot + 1,
                                            target_meta.stars,
                                            target_meta.commander_name
                                        ))
                                        .font(body_font(12.0))
                                        .color(RED_ENEMY),
                                    )
                                    .wrap(),
                                );
                            }
                        });
                    },
                );
            },
        );
        if closed {
            overlay.show_new_campaign = false;
            overlay.overwrite_confirm_slot = None;
        }
    }

    if overlay.show_settings {
        let mut closed = false;
        let mut changed = false;
        draw_modal_shell(
            ctx,
            "Command Settings",
            "COMMAND SETTINGS",
            "Done",
            Vec2::new(660.0, 560.0),
            GOLD,
            &mut closed,
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    for (tab, lbl) in [
                        (SettingsTab::Audio, "Audio Mixer"),
                        (SettingsTab::Display, "Display & HUD"),
                        (SettingsTab::Gameplay, "Battle & Save"),
                    ] {
                        if nav_tab_button(
                            ui,
                            lbl,
                            overlay.settings_tab == tab,
                            GOLD,
                            Vec2::new(138.0, 32.0),
                        )
                        .clicked()
                        {
                            audio.play_click();
                            overlay.settings_tab = tab;
                        }
                    }
                });

                ui.add_space(10.0);
                sculpted_card(ui, Some(GOLD_DIM), 14, |ui| {
                    ui.set_min_width(ui.available_width());
                    match overlay.settings_tab {
                        SettingsTab::Audio => {
                            section_ribbon(
                                ui,
                                "ADAPTIVE SOUND ENGINE",
                                Some("Layered procedural synthesis shifts from War Room to Siege combat"),
                                GOLD,
                            );
                            ui.add_space(6.0);
                            if styled_toggle(ui, &mut settings.muted, "Mute All Audio", RED_ENEMY)
                                .changed()
                            {
                                changed = true;
                            }
                            ui.add_space(8.0);
                            let master_txt = format!("{}%", (settings.master_volume * 100.0).round() as i32);
                            changed |= styled_slider(
                                ui,
                                &mut settings.master_volume,
                                0.0..=1.0,
                                "Master Volume",
                                &master_txt,
                                GOLD,
                            );
                            ui.add_space(4.0);
                            let music_txt = format!("{}%", (settings.music_volume * 100.0).round() as i32);
                            changed |= styled_slider(
                                ui,
                                &mut settings.music_volume,
                                0.0..=1.0,
                                "Soundtrack Volume",
                                &music_txt,
                                CYAN,
                            );
                            ui.add_space(4.0);
                            let sfx_txt = format!("{}%", (settings.sfx_volume * 100.0).round() as i32);
                            changed |= styled_slider(
                                ui,
                                &mut settings.sfx_volume,
                                0.0..=1.0,
                                "Combat & UI SFX Volume",
                                &sfx_txt,
                                EMERALD,
                            );

                            ui.add_space(12.0);
                            well_frame().inner_margin(Margin::same(10)).show(ui, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.horizontal_wrapped(|ui| {
                                    if subtle_button(
                                        ui,
                                        "Test Battle Horn",
                                        false,
                                        GOLD,
                                        Vec2::new(146.0, 30.0),
                                    )
                                    .clicked()
                                    {
                                        audio.play_order_horn();
                                    }
                                    if subtle_button(
                                        ui,
                                        "Test Treasury Chime",
                                        false,
                                        EMERALD,
                                        Vec2::new(156.0, 30.0),
                                    )
                                    .clicked()
                                    {
                                        audio.play_unlock();
                                    }
                                });
                                ui.add_space(6.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(
                                            "Dynamic adaptive soundtrack shifts seamlessly between noble War-Room chords and high-tempo Martial Battle themes.",
                                        )
                                        .font(body_font(11.5))
                                        .color(TEXT_SECONDARY),
                                    )
                                    .wrap(),
                                );
                            });
                        }
                        SettingsTab::Display => {
                            section_ribbon(
                                ui,
                                "VIEWPORT & COMBAT TELEMETRY",
                                Some("Tune fullscreen presentation and battlefield visual indicators"),
                                GOLD,
                            );
                            ui.add_space(6.0);
                            let prev_fs = settings.fullscreen;
                            if styled_toggle(
                                ui,
                                &mut settings.fullscreen,
                                "Borderless Fullscreen Window",
                                GOLD,
                            )
                            .changed()
                            {
                                changed = true;
                                if settings.fullscreen != prev_fs {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(
                                        settings.fullscreen,
                                    ));
                                }
                            }
                            changed |= styled_toggle(
                                ui,
                                &mut settings.screen_shake,
                                "Cinematic Camera Shake (Ultimates & Boss Slams)",
                                GOLD,
                            )
                            .changed();
                            changed |= styled_toggle(
                                ui,
                                &mut settings.show_damage_numbers,
                                "Show Floating Combat Numbers",
                                CYAN,
                            )
                            .changed();
                            changed |= styled_toggle(
                                ui,
                                &mut settings.show_minimap,
                                "Show Frontline Tactical Minimap Strip",
                                EMERALD,
                            )
                            .changed();

                            ui.add_space(10.0);
                            section_ribbon(
                                ui,
                                "BATTLEFIELD HP BAR DECLUTTER",
                                None,
                                GOLD,
                            );
                            ui.add_space(4.0);
                            for mode in [HpBarMode::Smart, HpBarMode::Always, HpBarMode::Minimal] {
                                let selected = settings.hp_bar_mode == mode;
                                well_frame()
                                    .fill(if selected { BG_CARD } else { BG_WELL })
                                    .stroke(Stroke::new(
                                        if selected { 1.4 } else { 1.0 },
                                        if selected { GOLD } else { BORDER_SUBTLE },
                                    ))
                                    .inner_margin(Margin::symmetric(10, 7))
                                    .show(ui, |ui| {
                                        ui.set_min_width(ui.available_width());
                                        if ui.radio(selected, mode.label()).clicked() {
                                            settings.hp_bar_mode = mode;
                                            changed = true;
                                        }
                                        ui.add(
                                            egui::Label::new(
                                                egui::RichText::new(mode.desc())
                                                    .font(body_font(11.0))
                                                    .color(TEXT_MUTED),
                                            )
                                            .wrap(),
                                        );
                                    });
                                ui.add_space(4.0);
                            }
                        }
                        SettingsTab::Gameplay => {
                            section_ribbon(
                                ui,
                                "DEFAULT SIMULATION SPEED",
                                Some("Starting battle speed upon mission launch"),
                                GOLD,
                            );
                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                for spd in [0.5_f32, 1.0, 2.0] {
                                    let active = (settings.default_speed - spd).abs() < 0.05;
                                    if nav_tab_button(
                                        ui,
                                        &format!("{spd:.1}x"),
                                        active,
                                        GOLD,
                                        Vec2::new(72.0, 28.0),
                                    )
                                    .clicked()
                                    {
                                        settings.default_speed = spd;
                                        changed = true;
                                    }
                                }
                            });

                            ui.add_space(10.0);
                            changed |= styled_toggle(
                                ui,
                                &mut settings.auto_pause_unfocused,
                                "Auto-Pause Battle when window loses focus",
                                GOLD,
                            )
                            .changed();
                            changed |= styled_toggle(
                                ui,
                                &mut settings.edge_pan,
                                "Enable Mouse Edge-Panning in Tactical Camera",
                                CYAN,
                            )
                            .changed();

                            ui.add_space(10.0);
                            filigree_divider(ui, RED_ENEMY);
                            ui.add_space(6.0);
                            well_frame()
                                .stroke(Stroke::new(1.0, RED_ENEMY.gamma_multiply(0.55)))
                                .inner_margin(Margin::same(10))
                                .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "ARCHIVE DANGER ZONE — SLOT {}",
                                            active_slot + 1
                                        ))
                                        .font(mono_font(10.5))
                                        .color(RED_ENEMY),
                                    );
                                    ui.add_space(4.0);
                                    ui.horizontal_wrapped(|ui| {
                                        if !overlay.reset_slot_confirm {
                                            if subtle_button(
                                                ui,
                                                "Reset Active Slot Campaign",
                                                false,
                                                RED_ENEMY,
                                                Vec2::new(220.0, 30.0),
                                            )
                                            .clicked()
                                            {
                                                overlay.reset_slot_confirm = true;
                                            }
                                        } else {
                                            if subtle_button(
                                                ui,
                                                "CONFIRM WIPE ACTIVE SLOT",
                                                true,
                                                RED_ENEMY,
                                                Vec2::new(210.0, 30.0),
                                            )
                                            .clicked()
                                            {
                                                overlay.reset_slot_confirm = false;
                                                overlay.show_settings = false;
                                                action = MenuAction::ResetActiveSlot;
                                            }
                                            if subtle_button(
                                                ui,
                                                "Cancel",
                                                false,
                                                TEXT_SECONDARY,
                                                Vec2::new(76.0, 30.0),
                                            )
                                            .clicked()
                                            {
                                                overlay.reset_slot_confirm = false;
                                            }
                                        }
                                    });
                                });
                        }
                    }
                });
            },
        );

        if closed {
            audio.play_click();
            overlay.show_settings = false;
            overlay.reset_slot_confirm = false;
        }
        if changed {
            settings.clamp();
            audio.sync_settings(settings);
            let _ = settings.save_to_path(&grand_conquest_core::honors::settings_path(storage_dir));
        }
    }

    if overlay.show_manual {
        let mut closed = false;
        draw_modal_shell(
            ctx,
            "Commander's Field Manual & Credits",
            "COMMANDER FIELD MANUAL & CREDITS",
            "Close",
            Vec2::new(700.0, 560.0),
            GOLD,
            &mut closed,
            |ui| {
                section_ribbon(
                    ui,
                    "TACTICAL FUNDAMENTALS",
                    Some("Core rules of Economy, Orders, Role Matchups & Warmind AI"),
                    GOLD,
                );
                ui.add_space(6.0);

                for (num, title, body, badge_col) in [
                    (
                        "01",
                        "Economy vs Military Tempo",
                        "Upgrade [U] Gold Flow early if your front turret holds, or pool gold to deploy synchronized Tier B/A pushes.",
                        GOLD,
                    ),
                    (
                        "02",
                        "Orders [Z] / [X] / [C]",
                        "[Z] Attack marches forward; [X] Hold halts troops right where they stand to trap enemies under tower fire; [C] Retreat sprints back (+30% SPD).",
                        CYAN,
                    ),
                    (
                        "03",
                        "Frontline Role Triangle",
                        "Heavy Squares & Hexagons absorb damage; fast Circles rush backlines; ranged Triangles & Diamonds punish slow tanks; Cross units sustain sieges.",
                        EMERALD,
                    ),
                    (
                        "04",
                        "Legion Synergy (3-pc / 6-pc)",
                        "Build decks around 3 or 6 troops from one Legion to trigger massive global buffs & -10s spell cooldowns.",
                        AMETHYST,
                    ),
                    (
                        "05",
                        "Warmind Opposition",
                        "Enemy Commanders combine an evolved decision heuristic with a neural evaluation model and will bait over-extended armies near turrets.",
                        RED_ENEMY,
                    ),
                ] {
                    well_frame().inner_margin(Margin::same(10)).show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.horizontal(|ui| {
                            draw_badge(ui, num, BG_CARD, badge_col);
                            ui.label(
                                egui::RichText::new(title)
                                    .font(body_font(13.0))
                                    .color(TEXT_PRIMARY),
                            );
                        });
                        ui.add_space(3.0);
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(body)
                                    .font(body_font(11.8))
                                    .color(TEXT_SECONDARY),
                            )
                            .wrap(),
                        );
                    });
                    ui.add_space(5.0);
                }

                ui.add_space(6.0);
                sculpted_card(ui, Some(GOLD), 12, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("GRAND CONQUEST — SOVEREIGN EDITION")
                                .font(display_font(15.0))
                                .color(GOLD_BRIGHT),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            draw_badge(ui, "PURE RUST + EGUI", BG_WELL, GOLD);
                        });
                    });
                    ui.add_space(4.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(
                                "Built with custom adaptive procedural soundtrack synthesis, headless parallel tactical physics simulation, and self-play evolved + neural Warmind AI.",
                            )
                            .font(body_font(12.0))
                            .color(TEXT_SECONDARY),
                        )
                        .wrap(),
                    );
                });
            },
        );
        if closed {
            overlay.show_manual = false;
        }
    }

    action
}

fn next_campaign_mission(save: &ConquestSave) -> Option<&'static grand_conquest_core::missions::MissionDef> {
    MISSIONS
        .iter()
        .find(|m| save.is_mission_unlocked(m.id) && save.mission_stars(m.id) == 0)
        .or_else(|| MISSIONS.iter().rev().find(|m| save.is_mission_unlocked(m.id)))
}
