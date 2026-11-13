use egui::epaint::Shape;
use egui::{Color32, CornerRadius, Margin, Painter, Pos2, Rect, ScrollArea, Sense, Stroke, StrokeKind, Ui, Vec2};
use grand_conquest_core::missions::{get_mission, MissionDef, MISSIONS};
use grand_conquest_core::model::{FactionId, MissionArchetype};
use grand_conquest_core::save::ConquestSave;
use grand_conquest_core::units::get_unit;

use crate::assets::{image_rect_tinted, AssetBank};
use crate::theme::{
    archetype_color, body_font, crest_socket, cta_button, display_font, equal_columns,
    faction_color, mono_font, paint_archetype_emblem, paint_parchment_backdrop, paint_star,
    paint_unit_shape, portrait_pedestal, sculpted_card, subtle_button, tier_color, BG_BAR,
    BG_PANEL, BG_VOID, BORDER_SUBTLE, EMERALD, GOLD, GOLD_BRIGHT, GOLD_DIM, RED_ENEMY, TEXT_MUTED,
    TEXT_PRIMARY, TEXT_SECONDARY,
};

pub const WORLD_MAP_W: f32 = 2000.0;
pub const WORLD_MAP_H: f32 = 1200.0;

pub enum MapAction {
    None,
    LaunchMission(&'static str),
    EditDeckForMission(&'static str),
}

pub struct MapViewState {
    pub cam_x: f32,
    pub cam_y: f32,
    pub target_cam_x: f32,
    pub target_cam_y: f32,
    pub zoom: f32,
    pub selected_mission_id: Option<&'static str>,
    pub hovered_mission_id: Option<&'static str>,
    pub active_chapter_tab: u8,
}

impl Default for MapViewState {
    fn default() -> Self {
        Self {
            cam_x: 30.0,
            cam_y: 410.0,
            target_cam_x: 30.0,
            target_cam_y: 410.0,
            zoom: 0.92,
            selected_mission_id: Some("M1_1"),
            hovered_mission_id: None,
            active_chapter_tab: 1,
        }
    }
}

impl MapViewState {
    pub fn focus_chapter(&mut self, chapter: u8, view_size: Vec2) {
        self.active_chapter_tab = chapter;
        // Exact chapter centroids matching MISSIONS pin_coords
        let center = match chapter {
            1 => Pos2::new(540.0, 690.0),   // Solaris West-Mid
            2 => Pos2::new(1505.0, 690.0),  // Aegis East-Mid
            3 => Pos2::new(555.0, 1015.0),  // Hive South-West
            4 => Pos2::new(555.0, 235.0),   // Void North-West
            5 => Pos2::new(1505.0, 1015.0), // Hellfire South-East
            _ => Pos2::new(1505.0, 235.0),  // Mechanica North-East
        };
        let vw = view_size.x.max(400.0) / self.zoom;
        let vh = view_size.y.max(300.0) / self.zoom;
        self.target_cam_x = (center.x - vw * 0.5).clamp(0.0, (WORLD_MAP_W - vw).max(0.0));
        self.target_cam_y = (center.y - vh * 0.5).clamp(0.0, (WORLD_MAP_H - vh).max(0.0));
    }

    pub fn draw(
        &mut self,
        ui: &mut Ui,
        save: &ConquestSave,
        assets: &AssetBank,
        dt: f32,
        t_anim: f32,
    ) -> MapAction {
        let mut action = MapAction::None;

        let vp_w = ui.ctx().viewport_rect().width();
        let compact_dock = vp_w < 1220.0;

        // Bottom Province Navigator Bar
        egui::Panel::bottom("map_chapter_dock")
            .exact_size(46.0)
            .frame(
                egui::Frame::new()
                    .fill(BG_BAR)
                    .inner_margin(Margin::symmetric(12, 6)),
            )
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    let chapters: [(u8, &str, &str, FactionId); 6] = [
                        (1, "I  Solaris Plains", "I Solaris", FactionId::Solaris),
                        (2, "II  Neo-Veridia", "II Veridia", FactionId::Aegis),
                        (3, "III  Chitin Wastes", "III Chitin", FactionId::Hive),
                        (4, "IV  Abyssal Rift", "IV Abyssal", FactionId::Void),
                        (5, "V  Brimstone Crucible", "V Brimstone", FactionId::Hellfire),
                        (6, "VI  Sector Zero", "VI Sector 0", FactionId::Mechanica),
                    ];
                    let btn_w = ((ui.available_width() - 24.0) / 6.0).clamp(112.0, 184.0);
                    let est_view = Vec2::new((vp_w - 400.0).max(520.0), 650.0);
                    ScrollArea::horizontal()
                        .id_salt("map_chapter_dock_scroll")
                        .show(ui, |ui| {
                            ui.horizontal_centered(|ui| {
                                for (ch, full_lbl, short_lbl, fac) in chapters {
                                    let stars = save.chapter_stars(ch);
                                    let active = self.active_chapter_tab == ch;
                                    let col = faction_color(fac);
                                    let label = if compact_dock {
                                        format!("{short_lbl}  {stars}/12")
                                    } else {
                                        format!("{full_lbl}  {stars}/12")
                                    };
                                    if crate::theme::nav_tab_button(
                                        ui,
                                        &label,
                                        active,
                                        col,
                                        Vec2::new(btn_w, 30.0),
                                    )
                                    .clicked()
                                    {
                                        self.focus_chapter(ch, est_view);
                                    }
                                }
                            });
                        });
                });
            });

        // Slide-Over Right Tactical Dossier Drawer (only when a mission is selected)
        if let Some(sel_id) = self.selected_mission_id {
            if let Some(m) = get_mission(sel_id) {
                let drawer_w = (vp_w * 0.34).clamp(340.0, 412.0);
                egui::Panel::right("map_mission_dossier")
                    .exact_size(drawer_w)
                    .resizable(false)
                    .frame(
                        egui::Frame::new()
                            .fill(crate::theme::BG_PANEL)
                            .inner_margin(Margin::same(14)),
                    )
                    .show(ui, |ui| {
                        let inner_w = (drawer_w - 28.0).max(290.0);
                        ui.set_min_width(inner_w);
                        ui.set_max_width(inner_w);

                        let full = ui.max_rect().expand(14.0);
                        crate::theme::paint_vertical_gradient(
                            ui.painter(),
                            full,
                            crate::theme::BG_CARD_TOP,
                            BG_PANEL,
                        );
                        if let Some(parch) = assets.parchment_dark.as_ref() {
                            paint_parchment_backdrop(
                                ui.painter(),
                                full,
                                parch,
                                Color32::from_rgba_unmultiplied(235, 222, 200, 62),
                            );
                        }
                        ui.painter().line_segment(
                            [full.left_top(), full.left_bottom()],
                            Stroke::new(1.0, BORDER_SUBTLE),
                        );
                        action = draw_dossier_drawer(
                            ui,
                            m,
                            save,
                            assets,
                            &mut self.selected_mission_id,
                        );
                    });
            }
        }

        // Main Interactive Cartographic Map Viewport
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG_VOID))
            .show(ui, |ui| {
                let rect = ui.available_rect_before_wrap();
                let resp = ui.allocate_rect(rect, Sense::click_and_drag());

                // Smooth camera lerp toward target
                let alpha = (dt * 10.0).min(1.0);
                self.cam_x += (self.target_cam_x - self.cam_x) * alpha;
                self.cam_y += (self.target_cam_y - self.cam_y) * alpha;

                // WASD / Arrow keyboard panning
                let pan_spd = 650.0 / self.zoom * dt;
                ui.input(|i| {
                    if i.key_down(egui::Key::A) || i.key_down(egui::Key::ArrowLeft) {
                        self.target_cam_x -= pan_spd;
                    }
                    if i.key_down(egui::Key::D) || i.key_down(egui::Key::ArrowRight) {
                        self.target_cam_x += pan_spd;
                    }
                    if i.key_down(egui::Key::W) || i.key_down(egui::Key::ArrowUp) {
                        self.target_cam_y -= pan_spd;
                    }
                    if i.key_down(egui::Key::S) || i.key_down(egui::Key::ArrowDown) {
                        self.target_cam_y += pan_spd;
                    }
                });

                // Mouse drag pan
                if resp.dragged() {
                    let delta = resp.drag_delta() / self.zoom;
                    self.cam_x -= delta.x;
                    self.cam_y -= delta.y;
                    self.target_cam_x = self.cam_x;
                    self.target_cam_y = self.cam_y;
                }

                // Wheel cursor-centered zoom
                if resp.hovered() {
                    let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                    if scroll.abs() > 0.1 {
                        let factor = if scroll > 0.0 { 1.12 } else { 0.89 };
                        let old_zoom = self.zoom;
                        let new_zoom = (old_zoom * factor).clamp(0.55, 1.85);
                        if let Some(mpos) = resp.hover_pos() {
                            let local = mpos - rect.min;
                            let world_before = Pos2::new(
                                self.cam_x + local.x / old_zoom,
                                self.cam_y + local.y / old_zoom,
                            );
                            self.zoom = new_zoom;
                            self.cam_x = world_before.x - local.x / new_zoom;
                            self.cam_y = world_before.y - local.y / new_zoom;
                            self.target_cam_x = self.cam_x;
                            self.target_cam_y = self.cam_y;
                        }
                    }
                }

                self.clamp_camera(rect.size());

                // Hover pin hit-test
                self.hovered_mission_id = None;
                if let Some(mpos) = resp.hover_pos() {
                    let wx = self.cam_x + (mpos.x - rect.min.x) / self.zoom;
                    let wy = self.cam_y + (mpos.y - rect.min.y) / self.zoom;
                    for m in MISSIONS {
                        let d = (wx - m.pin_coord[0]).hypot(wy - m.pin_coord[1]);
                        if d <= 34.0 {
                            self.hovered_mission_id = Some(m.id);
                            break;
                        }
                    }
                }

                if self.hovered_mission_id.is_some() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                } else if resp.dragged() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                } else if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
                }

                // Click pin to select mission; double-click unlocked pin to quick launch!
                if resp.clicked() {
                    if let Some(hid) = self.hovered_mission_id {
                        self.selected_mission_id = Some(hid);
                    }
                }
                if resp.double_clicked() {
                    if let Some(hid) = self.hovered_mission_id {
                        if save.is_mission_unlocked(hid) {
                            action = MapAction::LaunchMission(hid);
                        }
                    }
                }

                // Draw World Map Canvas
                let painter = ui.painter_at(rect);
                self.paint_world_atlas(&painter, rect, save, assets, t_anim);

                // Floating Overlay Controls (Top-Left Legend + Top-Right Zoom Pod)
                let zoom_rect = Rect::from_min_size(rect.right_top() + Vec2::new(-48.0, 14.0), Vec2::new(36.0, 110.0));
                painter.rect_filled(
                    zoom_rect,
                    CornerRadius::same(10),
                    Color32::from_rgba_unmultiplied(16, 14, 15, 195),
                );

                egui::Area::new(egui::Id::unique("map_zoom_buttons"))
                    .fixed_pos(zoom_rect.min + Vec2::new(3.0, 6.0))
                    .fade_in(false)
                    .show(ui.ctx(), |ui| {
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(0.0, 4.0);
                            if subtle_button(ui, "+", false, GOLD, Vec2::splat(30.0)).clicked() {
                                self.zoom = (self.zoom * 1.18).min(1.85);
                                self.clamp_camera(rect.size());
                            }
                            if subtle_button(ui, "-", false, GOLD, Vec2::splat(30.0)).clicked() {
                                self.zoom = (self.zoom * 0.85).max(0.55);
                                self.clamp_camera(rect.size());
                            }
                            if subtle_button(ui, "O", false, GOLD, Vec2::splat(30.0))
                                .on_hover_text("Recenter Active Province")
                                .clicked()
                            {
                                self.focus_chapter(self.active_chapter_tab, rect.size());
                            }
                        });
                    });

                // Guided Beginner "Next Campaign Objective" Banner at Top-Left
                let next_mission = MISSIONS
                    .iter()
                    .find(|m| save.is_mission_unlocked(m.id) && save.mission_stars(m.id) == 0);
                let legend_pos = rect.left_top() + Vec2::new(14.0, 14.0);
                let max_pill_w = (rect.width() - 82.0).max(240.0);
                let narrow_map = rect.width() < 780.0;
                egui::Area::new(egui::Id::unique("map_legend_pill"))
                    .fixed_pos(legend_pos)
                    .fade_in(false)
                    .show(ui.ctx(), |ui| {
                        ui.set_max_width(max_pill_w);
                        egui::Frame::new()
                            .fill(Color32::from_rgba_unmultiplied(16, 14, 15, 200))
                            .corner_radius(CornerRadius::same(10))
                            .inner_margin(Margin::symmetric(12, 7))
                            .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                if let Some(nm) = next_mission {
                                    ui.label(
                                        egui::RichText::new("Next")
                                            .font(mono_font(10.0))
                                            .color(GOLD_BRIGHT),
                                    );
                                    ui.label(
                                        egui::RichText::new(format!("{} — {}", nm.id, nm.name))
                                            .font(body_font(12.2))
                                            .color(TEXT_PRIMARY),
                                    );
                                    if subtle_button(ui, "Focus", false, GOLD, Vec2::new(62.0, 22.0)).clicked() {
                                        self.selected_mission_id = Some(nm.id);
                                        self.focus_chapter(nm.chapter, rect.size());
                                    }
                                } else {
                                    ui.label(
                                        egui::RichText::new("Campaign")
                                            .font(display_font(11.5))
                                            .color(GOLD),
                                    );
                                }
                                if !narrow_map {
                                    ui.separator();
                                    ui.label(
                                        egui::RichText::new("Drag · Scroll · Click Pin")
                                            .font(body_font(11.5))
                                            .color(TEXT_MUTED),
                                    );
                                }
                            });
                        });
                    });
            });

        action
    }

    fn clamp_camera(&mut self, view_size: Vec2) {
        let vw = view_size.x.max(200.0) / self.zoom;
        let vh = view_size.y.max(200.0) / self.zoom;
        let max_x = (WORLD_MAP_W - vw).max(0.0);
        let max_y = (WORLD_MAP_H - vh).max(0.0);
        self.cam_x = self.cam_x.clamp(0.0, max_x);
        self.cam_y = self.cam_y.clamp(0.0, max_y);
        self.target_cam_x = self.target_cam_x.clamp(0.0, max_x);
        self.target_cam_y = self.target_cam_y.clamp(0.0, max_y);
    }

    fn world_to_screen(&self, rect: Rect, wx: f32, wy: f32) -> Pos2 {
        Pos2::new(
            rect.min.x + (wx - self.cam_x) * self.zoom,
            rect.min.y + (wy - self.cam_y) * self.zoom,
        )
    }

    fn paint_world_atlas(
        &self,
        painter: &Painter,
        rect: Rect,
        save: &ConquestSave,
        assets: &AssetBank,
        t_anim: f32,
    ) {
        // 1. Charred Ebony Backdrop & Painted Geo-Terra Parchment World Map
        painter.rect_filled(rect, CornerRadius::ZERO, Color32::from_rgb(13, 11, 10));

        let map_p0 = self.world_to_screen(rect, 0.0, 0.0);
        let map_p1 = self.world_to_screen(rect, WORLD_MAP_W, WORLD_MAP_H);
        let map_rect = Rect::from_min_max(map_p0, map_p1);

        if let Some(world_tex) = assets.world_map.as_ref() {
            image_rect_tinted(
                painter,
                world_tex,
                map_rect,
                Color32::from_rgb(236, 224, 206),
            );
            // Subtle warm sepia wash for effortless gold road & stronghold contrast
            painter.rect_filled(
                map_rect,
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(16, 12, 9, 68),
            );
            painter.rect_stroke(
                map_rect,
                CornerRadius::same(4),
                Stroke::new(3.0, GOLD_DIM),
                StrokeKind::Inside,
            );
        } else {
            let grid_stroke = Stroke::new(1.0, Color32::from_rgba_unmultiplied(220, 176, 84, 16));
            for gx in (0..=2000).step_by(100) {
                let p0 = self.world_to_screen(rect, gx as f32, 0.0);
                let p1 = self.world_to_screen(rect, gx as f32, WORLD_MAP_H);
                painter.line_segment([p0, p1], grid_stroke);
            }
            for gy in (0..=1200).step_by(100) {
                let p0 = self.world_to_screen(rect, 0.0, gy as f32);
                let p1 = self.world_to_screen(rect, WORLD_MAP_W, gy as f32);
                painter.line_segment([p0, p1], grid_stroke);
            }
        }

        // Subtle Cartographic Graticule over parchment
        let graticule = Stroke::new(0.8, Color32::from_rgba_unmultiplied(220, 176, 84, 16));
        for gx in (200..2000).step_by(200) {
            let p0 = self.world_to_screen(rect, gx as f32, 0.0);
            let p1 = self.world_to_screen(rect, gx as f32, WORLD_MAP_H);
            painter.line_segment([p0, p1], graticule);
        }
        for gy in (200..1200).step_by(200) {
            let p0 = self.world_to_screen(rect, 0.0, gy as f32);
            let p1 = self.world_to_screen(rect, WORLD_MAP_W, gy as f32);
            painter.line_segment([p0, p1], graticule);
        }

        // 2. Six Styled Biome Province Boundaries & Heraldic Crest Badges
        struct Zone {
            num: &'static str,
            title: &'static str,
            bounds: [f32; 4],
            fac: FactionId,
            chapter: u8,
        }
        let zones = [
            Zone { num: "IV", title: "ABYSSAL RIFT  •  VOID", bounds: [50.0, 50.0, 900.0, 440.0], fac: FactionId::Void, chapter: 4 },
            Zone { num: "VI", title: "SECTOR ZERO  •  MECHANICA", bounds: [1050.0, 50.0, 900.0, 440.0], fac: FactionId::Mechanica, chapter: 6 },
            Zone { num: "I", title: "SUN-GILDED PLAINS  •  SOLARIS", bounds: [50.0, 525.0, 900.0, 295.0], fac: FactionId::Solaris, chapter: 1 },
            Zone { num: "II", title: "NEO-VERIDIA SECTOR  •  AEGIS", bounds: [1050.0, 525.0, 900.0, 295.0], fac: FactionId::Aegis, chapter: 2 },
            Zone { num: "III", title: "THE CHITIN WASTES  •  HIVE MIND", bounds: [50.0, 855.0, 900.0, 295.0], fac: FactionId::Hive, chapter: 3 },
            Zone { num: "V", title: "BRIMSTONE CRUCIBLE  •  HELLFIRE", bounds: [1050.0, 855.0, 900.0, 295.0], fac: FactionId::Hellfire, chapter: 5 },
        ];

        for z in zones {
            let p0 = self.world_to_screen(rect, z.bounds[0], z.bounds[1]);
            let p1 = self.world_to_screen(rect, z.bounds[0] + z.bounds[2], z.bounds[1] + z.bounds[3]);
            let zr = Rect::from_min_max(p0, p1);
            let accent = faction_color(z.fac);
            let [ar, ag, ab] = z.fac.accent_rgb();
            let liberated = save.chapter_stars(z.chapter) == 12;

            painter.rect_filled(
                zr,
                CornerRadius::same((14.0 * self.zoom) as u8),
                Color32::from_rgba_unmultiplied(ar, ag, ab, if liberated { 16 } else { 6 }),
            );

            // Province Banner Pill + Heraldic Crest
            let crest_sz = 30.0 * self.zoom;
            let banner_galley = painter.layout_no_wrap(
                format!("{} · {}", z.num, z.title),
                display_font((11.5 * self.zoom).clamp(8.5, 18.0)),
                accent,
            );
            let banner_w = banner_galley.size().x + crest_sz + 22.0 * self.zoom;
            let banner_h = (crest_sz + 6.0 * self.zoom).max(banner_galley.size().y + 8.0 * self.zoom);
            let banner_rect = Rect::from_min_size(
                zr.left_top() + Vec2::new(14.0 * self.zoom, 12.0 * self.zoom),
                Vec2::new(banner_w, banner_h),
            );
            painter.rect_filled(
                banner_rect,
                CornerRadius::same(8),
                Color32::from_rgba_unmultiplied(18, 15, 13, 200),
            );

            if let Some(crest_tex) = assets.get_crest(z.fac) {
                let cr = Rect::from_min_size(
                    Pos2::new(banner_rect.min.x + 6.0 * self.zoom, banner_rect.center().y - crest_sz * 0.5),
                    Vec2::splat(crest_sz),
                );
                crest_socket(painter, cr, crest_tex, accent);
            }
            painter.galley(
                Pos2::new(
                    banner_rect.min.x + crest_sz + 14.0 * self.zoom,
                    banner_rect.center().y - banner_galley.size().y * 0.5,
                ),
                banner_galley,
                accent,
            );
        }

        // 3. Quadratic Bezier Road Connections with Animated Expedition Chevrons
        for m in MISSIONS {
            let unlocked = save.is_mission_unlocked(m.id);
            let completed = save.mission_stars(m.id) > 0;
            let dst = Pos2::new(m.pin_coord[0], m.pin_coord[1]);

            for pre_id in m.prerequisites {
                let Some(src_m) = get_mission(pre_id) else {
                    continue;
                };
                let src = Pos2::new(src_m.pin_coord[0], src_m.pin_coord[1]);
                let ctrl = Pos2::new((src.x + dst.x) * 0.5, (src.y + dst.y) * 0.5 - 32.0);

                let pts: Vec<Pos2> = (0..=24)
                    .map(|step| {
                        let t = step as f32 / 24.0;
                        let omt = 1.0 - t;
                        let wx = omt * omt * src.x + 2.0 * omt * t * ctrl.x + t * t * dst.x;
                        let wy = omt * omt * src.y + 2.0 * omt * t * ctrl.y + t * t * dst.y;
                        self.world_to_screen(rect, wx, wy)
                    })
                    .collect();

                if completed {
                    painter.add(Shape::line(pts.clone(), Stroke::new(3.2 * self.zoom, GOLD)));
                } else if unlocked {
                    painter.add(Shape::line(
                        pts.clone(),
                        Stroke::new(2.4 * self.zoom, Color32::from_rgba_unmultiplied(228, 184, 92, 130)),
                    ));
                    // Moving golden expedition pip along unlocked route
                    let phase = (t_anim * 0.45) % 1.0;
                    let omt = 1.0 - phase;
                    let px = omt * omt * src.x + 2.0 * omt * phase * ctrl.x + phase * phase * dst.x;
                    let py = omt * omt * src.y + 2.0 * omt * phase * ctrl.y + phase * phase * dst.y;
                    painter.circle_filled(
                        self.world_to_screen(rect, px, py),
                        4.2 * self.zoom,
                        GOLD_BRIGHT,
                    );
                } else {
                    for seg in pts.chunks(2) {
                        if seg.len() == 2 {
                            painter.line_segment(
                                [seg[0], seg[1]],
                                Stroke::new(1.5 * self.zoom, Color32::from_rgba_unmultiplied(100, 116, 139, 75)),
                            );
                        }
                    }
                }
            }
        }

        // 4. Stronghold Pins + Next Target Beacon
        let next_pin_id = MISSIONS
            .iter()
            .find(|m| save.is_mission_unlocked(m.id) && save.mission_stars(m.id) == 0)
            .map(|m| m.id);

        for m in MISSIONS {
            let center = self.world_to_screen(rect, m.pin_coord[0], m.pin_coord[1]);
            let unlocked = save.is_mission_unlocked(m.id);
            let stars = save.mission_stars(m.id);
            let is_sel = self.selected_mission_id == Some(m.id);
            let is_hov = self.hovered_mission_id == Some(m.id);
            let r = 24.0 * self.zoom;
            let fac_col = faction_color(m.enemy_faction);
            let arch_col = archetype_color(m.archetype);

            // Pulse ring + clear "NEXT" chip on the recommended next battle
            if next_pin_id == Some(m.id) {
                let pulse = 1.0 + (t_anim * 3.2).sin() * 0.14;
                painter.circle_stroke(
                    center,
                    r * 1.36 * pulse,
                    Stroke::new(2.2 * self.zoom, GOLD_BRIGHT),
                );
                let tag_gal = painter.layout_no_wrap(
                    if save.total_conquest_stars == 0 { "START HERE" } else { "NEXT" }.into(),
                    mono_font((9.5 * self.zoom).clamp(7.5, 14.0)),
                    BG_VOID,
                );
                let tag_rect = Rect::from_center_size(
                    Pos2::new(center.x, center.y - r * 1.65 - 6.0 * self.zoom),
                    tag_gal.size() + Vec2::new(12.0 * self.zoom, 5.0 * self.zoom),
                );
                painter.rect(tag_rect, CornerRadius::same(4), GOLD_BRIGHT, Stroke::NONE, StrokeKind::Inside);
                painter.galley(tag_rect.center() - tag_gal.size() * 0.5, tag_gal, BG_VOID);
            } else if unlocked && stars == 0 {
                let pulse = 1.0 + (t_anim * 3.0).sin() * 0.12;
                painter.circle_stroke(
                    center,
                    r * 1.30 * pulse,
                    Stroke::new(1.8 * self.zoom, GOLD.linear_multiply(0.75)),
                );
            }

            if is_sel {
                painter.circle_stroke(
                    center,
                    r * 1.46,
                    Stroke::new(2.4 * self.zoom, GOLD_BRIGHT),
                );
            }

            let body_fill = if unlocked {
                Color32::from_rgb(31, 24, 19)
            } else {
                Color32::from_rgb(19, 15, 13)
            };
            let rim_col = if !unlocked {
                Color32::from_rgba_unmultiplied(110, 96, 78, 120)
            } else if is_hov || is_sel {
                GOLD_BRIGHT
            } else if stars == 3 {
                GOLD
            } else {
                fac_col
            };
            painter.circle(
                center,
                r + 2.5 * self.zoom,
                Color32::from_rgba_unmultiplied(8, 6, 5, 180),
                Stroke::NONE,
            );
            painter.circle(
                center,
                r,
                body_fill,
                Stroke::new(if stars == 3 || is_sel { 2.8 } else { 2.0 }, rim_col),
            );

            if unlocked {
                paint_archetype_emblem(painter, m.archetype, center, r * 0.56, arch_col);
            } else {
                // Padlock glyph
                painter.rect_stroke(
                    Rect::from_center_size(center + Vec2::new(0.0, 2.5 * self.zoom), Vec2::new(r * 0.52, r * 0.42)),
                    CornerRadius::same(2),
                    Stroke::new(1.5, TEXT_MUTED),
                    StrokeKind::Inside,
                );
                painter.circle_stroke(
                    center - Vec2::new(0.0, 3.5 * self.zoom),
                    r * 0.18,
                    Stroke::new(1.5, TEXT_MUTED),
                );
            }

            // 3 Vector Stars beneath Pin
            if unlocked {
                for s_idx in 0..3 {
                    let sx = (s_idx as f32 - 1.0) * 13.0 * self.zoom;
                    let sy = r + 14.5 * self.zoom;
                    let earned = stars > s_idx;
                    paint_star(
                        painter,
                        center + Vec2::new(sx, sy),
                        4.8 * self.zoom,
                        earned,
                        if earned { GOLD_BRIGHT } else { Color32::from_rgb(54, 64, 82) },
                    );
                }
            }

            // Crisp Territory Nameplate
            let label_y = center.y + r + (if unlocked { 28.0 } else { 15.0 }) * self.zoom;
            let text_galley = painter.layout_no_wrap(
                m.name.to_string(),
                body_font((11.0 * self.zoom).clamp(8.5, 16.0)),
                if unlocked { TEXT_PRIMARY } else { TEXT_MUTED },
            );
            let pill_rect = Rect::from_center_size(
                Pos2::new(center.x, label_y + text_galley.size().y * 0.5),
                text_galley.size() + Vec2::new(14.0 * self.zoom, 6.0 * self.zoom),
            );
            painter.rect(
                pill_rect,
                CornerRadius::same(5),
                Color32::from_rgba_unmultiplied(20, 16, 14, 215),
                if is_sel || is_hov { Stroke::new(1.0, GOLD) } else { Stroke::NONE },
                StrokeKind::Inside,
            );
            painter.galley(
                pill_rect.center() - text_galley.size() * 0.5,
                text_galley,
                TEXT_PRIMARY,
            );
        }
    }
}

fn draw_dossier_drawer(
    ui: &mut Ui,
    m: &'static MissionDef,
    save: &ConquestSave,
    assets: &AssetBank,
    selected_id: &mut Option<&'static str>,
) -> MapAction {
    let mut action = MapAction::None;
    let unlocked = save.is_mission_unlocked(m.id);
    let rec = save.missions.get(m.id);
    let best_stars = rec.map(|r| r.stars).unwrap_or(0);
    let fac_col = faction_color(m.enemy_faction);
    let arch_col = archetype_color(m.archetype);
    let drawer_inner_w = ui.available_width();

    ui.set_max_width(drawer_inner_w);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("PROVINCE {}  •  {}", m.chapter, m.id))
                .font(mono_font(10.5))
                .color(GOLD),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if subtle_button(ui, "X", false, TEXT_MUTED, Vec2::splat(24.0)).clicked() {
                *selected_id = None;
            }
            ui.label(
                egui::RichText::new(m.archetype.label())
                    .font(mono_font(10.0))
                    .color(arch_col),
            );
        });
    });

    ui.add_space(2.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(m.name)
                .font(display_font(16.5))
                .color(GOLD_BRIGHT),
        )
        .wrap(),
    );
    ui.add(
        egui::Label::new(
            egui::RichText::new(m.territory_name)
                .font(body_font(11.5))
                .color(TEXT_SECONDARY),
        )
        .wrap(),
    );
    ui.add_space(6.0);

    let footer_h = if unlocked { 44.0 } else { 54.0 };
    let scroll_h = (ui.available_height() - footer_h).max(160.0);

    ScrollArea::vertical()
        .id_salt(("dossier_scroll", m.id))
        .auto_shrink([false, false])
        .max_height(scroll_h)
        .show(ui, |ui| {
            let content_w = ui.available_width();
            ui.set_min_width(content_w);
            ui.set_max_width(content_w);

            ui.add(
                egui::Label::new(
                    egui::RichText::new(m.briefing)
                        .font(body_font(11.8))
                        .color(TEXT_SECONDARY),
                )
                .wrap(),
            );

            ui.add_space(8.0);

            // Tactical Parameters / Mutator / Boss Info
            sculpted_card(ui, None, 10, |ui| {
                let cw = ui.available_width();
                ui.set_min_width(cw);
                ui.set_max_width(cw);
                ui.horizontal(|ui| {
                    if let Some(crest) = assets.get_crest(m.enemy_faction) {
                        let (cr, _) = ui.allocate_exact_size(Vec2::splat(26.0), Sense::hover());
                        crest_socket(ui.painter(), cr, crest, fac_col);
                    }
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.label(
                            egui::RichText::new("Enemy")
                                .font(mono_font(9.2))
                                .color(TEXT_MUTED),
                        );
                        ui.label(
                            egui::RichText::new(m.enemy_faction.display_name())
                                .font(body_font(12.2))
                                .color(fac_col),
                        );
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(format!("AI · {}", m.ai_difficulty.name()))
                                .font(mono_font(10.0))
                                .color(RED_ENEMY),
                        )
                        .on_hover_text(m.ai_difficulty.desc());
                    });
                });

                if let Some(tlim) = m.time_limit_seconds {
                    let mins = (tlim as u32) / 60;
                    let secs = (tlim as u32) % 60;
                    let msg = if m.archetype == MissionArchetype::HoldTheLine {
                        format!(
                            "Survive {mins:02}:{secs:02} against {} waves",
                            m.waves.len()
                        )
                    } else {
                        format!("Win under {mins:02}:{secs:02}")
                    };
                    ui.add_space(4.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(msg)
                                .font(body_font(11.5))
                                .color(GOLD_BRIGHT),
                        )
                        .wrap(),
                    );
                }

                if let Some(mut_id) = m.mutator {
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(format!("Rule · {}", mut_id.name()))
                            .font(mono_font(9.8))
                            .color(crate::theme::AMETHYST),
                    );
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(mut_id.desc())
                                .font(body_font(11.2))
                                .color(TEXT_SECONDARY),
                        )
                        .wrap(),
                    );
                }

                if let Some(boss) = &m.boss {
                    ui.add_space(4.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!(
                                "Boss: {} ({:.0} HP · {:.0} DMG)",
                                boss.name, boss.max_hp, boss.dmg
                            ))
                            .font(body_font(11.8))
                            .color(RED_ENEMY),
                        )
                        .wrap(),
                    );
                    for p in boss.passive_summary {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(format!("• {p}"))
                                    .font(body_font(11.0))
                                    .color(TEXT_SECONDARY),
                            )
                            .wrap(),
                        );
                    }
                }
            });

            ui.add_space(4.0);

            // Enemy Troops Preview (Unique units only for clean readability!)
            sculpted_card(ui, None, 10, |ui| {
                let cw = ui.available_width();
                ui.set_min_width(cw);
                ui.set_max_width(cw);
                ui.label(
                    egui::RichText::new("Enemy Troops")
                        .font(mono_font(10.0))
                        .color(fac_col),
                );
                ui.add_space(4.0);
                let mut seen_ids: Vec<&'static str> = Vec::new();
                for uid in m.enemy_deck {
                    if !seen_ids.contains(&uid) {
                        seen_ids.push(uid);
                    }
                }
                for row in seen_ids.chunks(2) {
                    equal_columns(ui, 2, |ui, ci| {
                        if let Some(uid) = row.get(ci) {
                            if let Some(udef) = get_unit(uid) {
                                let uw = ui.available_width();
                                ui.vertical(|ui| {
                                    ui.set_min_width(uw);
                                    ui.set_max_width(uw);
                                    ui.horizontal(|ui| {
                                        let (p_rect, _) = ui
                                            .allocate_exact_size(Vec2::splat(26.0), Sense::hover());
                                        portrait_pedestal(
                                            ui.painter(),
                                            p_rect,
                                            assets.get_portrait(udef.sprite_key),
                                            RED_ENEMY,
                                            false,
                                        );
                                        if assets.get_portrait(udef.sprite_key).is_none() {
                                            paint_unit_shape(
                                                ui.painter(),
                                                udef.shape,
                                                p_rect.center(),
                                                9.0,
                                                0.0,
                                                RED_ENEMY,
                                                Color32::WHITE,
                                                1.2,
                                            );
                                        }
                                        ui.vertical(|ui| {
                                            ui.spacing_mut().item_spacing.y = 0.0;
                                            ui.add(
                                                egui::Label::new(
                                                    egui::RichText::new(udef.name)
                                                        .font(body_font(11.4))
                                                        .color(TEXT_PRIMARY),
                                                )
                                                .truncate(),
                                            );
                                            ui.add(
                                                egui::Label::new(
                                                    egui::RichText::new(format!(
                                                        "T{} · {}",
                                                        udef.tier.code(),
                                                        udef.shape.short_role()
                                                    ))
                                                    .font(body_font(10.0))
                                                    .color(tier_color(udef.tier)),
                                                )
                                                .truncate(),
                                            );
                                        });
                                    });
                                });
                            }
                        }
                    });
                    ui.add_space(4.0);
                }
            });

            ui.add_space(4.0);

            // 3-Star Commendations
            sculpted_card(ui, None, 10, |ui| {
                let cw = ui.available_width();
                ui.set_min_width(cw);
                ui.set_max_width(cw);
                ui.label(
                    egui::RichText::new("Mission Goals")
                        .font(mono_font(10.0))
                        .color(GOLD),
                );
                ui.add_space(3.0);
                for crit in m.star_criteria {
                    let earned = best_stars >= crit.stars;
                    ui.horizontal_top(|ui| {
                        let (s_rect, _) =
                            ui.allocate_exact_size(Vec2::new(15.0, 16.0), Sense::hover());
                        paint_star(
                            ui.painter(),
                            s_rect.center(),
                            5.4,
                            earned,
                            if earned { GOLD_BRIGHT } else { TEXT_MUTED },
                        );
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(crit.desc)
                                    .font(body_font(11.3))
                                    .color(if earned { TEXT_PRIMARY } else { TEXT_SECONDARY }),
                            )
                            .wrap(),
                        );
                    });
                    ui.add_space(2.0);
                }
            });

            ui.add_space(4.0);

            // Rewards Strip
            sculpted_card(ui, None, 10, |ui| {
                let cw = ui.available_width();
                ui.set_min_width(cw);
                ui.set_max_width(cw);
                ui.label(
                    egui::RichText::new("Rewards")
                        .font(mono_font(10.0))
                        .color(EMERALD),
                );
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        egui::RichText::new(format!("+{}G", m.gold_bounty))
                            .font(mono_font(11.0))
                            .color(GOLD_BRIGHT),
                    );
                    for bp in m.reward_blueprints {
                        if let Some(udef) = get_unit(bp) {
                            ui.label(
                                egui::RichText::new(format!("·  {} (T{})", udef.name, udef.tier.code()))
                                    .font(body_font(11.5))
                                    .color(EMERALD),
                            );
                        }
                    }
                });
                if m.unlocks_tier2 {
                    ui.add_space(2.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("+ Unlocks Tier 2 Veterans")
                                .font(body_font(11.2))
                                .color(crate::theme::BLUE_LEGION),
                        )
                        .wrap(),
                    );
                }
                if m.unlocks_tier3 {
                    ui.add_space(2.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("+ Unlocks Tier 3 Elites")
                                .font(body_font(11.2))
                                .color(crate::theme::AMETHYST),
                        )
                        .wrap(),
                    );
                }
            });
        });

    ui.add_space(6.0);
    if unlocked {
        let total_w = ui.available_width();
        let edit_w = (total_w * 0.34).clamp(92.0, 120.0);
        let deploy_w = (total_w - edit_w - 8.0).max(120.0);
        ui.horizontal(|ui| {
            if subtle_button(ui, "Edit Deck", false, GOLD, Vec2::new(edit_w, 38.0)).clicked() {
                action = MapAction::EditDeckForMission(m.id);
            }
            if cta_button(ui, "DEPLOY", Vec2::new(deploy_w, 38.0)).clicked() {
                action = MapAction::LaunchMission(m.id);
            }
        });
    } else {
        let prereqs = m.prerequisites.join(", ");
        ui.add(
            egui::Label::new(
                egui::RichText::new(format!("Locked — clear {prereqs} first"))
                    .font(mono_font(10.8))
                    .color(RED_ENEMY),
            )
            .wrap(),
        );
    }

    action
}
