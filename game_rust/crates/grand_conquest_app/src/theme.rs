use std::sync::Arc;

use egui::epaint::{Mesh, TextureHandle};
use egui::{
    Align2, Color32, Context, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Margin,
    Painter, Pos2, Rect, Response, Sense, Shape, Stroke, StrokeKind, Ui, Vec2,
};
use grand_conquest_core::model::{FactionId, MissionArchetype, UnitShape, UnitTier};

pub const BG_VOID: Color32 = Color32::from_rgb(11, 10, 12);
pub const BG_WELL: Color32 = Color32::from_rgb(15, 13, 15);
pub const BG_BAR: Color32 = Color32::from_rgb(20, 17, 17);
pub const BG_PANEL: Color32 = Color32::from_rgb(26, 22, 20);
pub const BG_CARD: Color32 = Color32::from_rgb(34, 28, 25);
pub const BG_CARD_TOP: Color32 = Color32::from_rgb(40, 33, 29);
pub const BG_CARD_HOVER: Color32 = Color32::from_rgb(52, 43, 37);
pub const BG_CARD_DIM: Color32 = Color32::from_rgb(18, 15, 14);

pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(70, 58, 46);
pub const BORDER_STRONG: Color32 = Color32::from_rgb(148, 118, 64);

pub const GOLD: Color32 = Color32::from_rgb(220, 178, 90);
pub const GOLD_BRIGHT: Color32 = Color32::from_rgb(248, 220, 144);
pub const GOLD_DIM: Color32 = Color32::from_rgb(138, 106, 48);
pub const GOLD_DARK: Color32 = Color32::from_rgb(94, 68, 26);

pub const RIM_SHADOW: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 150);

pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(244, 236, 222);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(198, 184, 160);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(142, 128, 108);

pub const BLUE_LEGION: Color32 = Color32::from_rgb(88, 164, 240);
pub const RED_ENEMY: Color32 = Color32::from_rgb(224, 76, 82);
pub const EMERALD: Color32 = Color32::from_rgb(86, 196, 132);
pub const AMETHYST: Color32 = Color32::from_rgb(176, 118, 234);
pub const CYAN: Color32 = Color32::from_rgb(88, 204, 214);

pub fn display_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("display".into()))
}

pub fn body_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Proportional)
}

pub fn mono_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Monospace)
}

pub fn install_theme_and_fonts(ctx: &Context, assets_dir: &'static include_dir::Dir<'static>) {
    let mut fonts = FontDefinitions::default();

    let load_font = |fonts: &mut FontDefinitions, key: &str, file: &str| {
        if let Some(f) = assets_dir.get_file(format!("fonts/{file}")) {
            fonts
                .font_data
                .insert(key.to_owned(), Arc::new(FontData::from_static(f.contents())));
            true
        } else {
            false
        }
    };

    let has_cinzel = load_font(&mut fonts, "cinzel", "Cinzel.ttf");
    let has_segoe = load_font(&mut fonts, "segoe_ui", "SegoeUI.ttf");
    let has_segoe_bold = load_font(&mut fonts, "segoe_bold", "SegoeUI-Bold.ttf");
    let has_consolas_bold = load_font(&mut fonts, "consolas_bold", "Consolas-Bold.ttf");
    let has_jb = load_font(&mut fonts, "jb_mono", "JetBrainsMono.ttf");

    if has_segoe {
        if let Some(fam) = fonts.families.get_mut(&FontFamily::Proportional) {
            fam.insert(0, "segoe_ui".to_owned());
        }
    }
    if has_segoe_bold {
        fonts.families.insert(
            FontFamily::Name("bold".into()),
            vec!["segoe_bold".to_owned()],
        );
    }
    if has_cinzel {
        let mut disp = vec!["cinzel".to_owned()];
        if has_segoe_bold {
            disp.push("segoe_bold".to_owned());
        }
        fonts.families.insert(FontFamily::Name("display".into()), disp);
    }
    if let Some(fam) = fonts.families.get_mut(&FontFamily::Monospace) {
        if has_consolas_bold {
            fam.insert(0, "consolas_bold".to_owned());
        } else if has_jb {
            fam.insert(0, "jb_mono".to_owned());
        }
    }
    ctx.set_fonts(fonts);

    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = BG_PANEL;
    visuals.panel_fill = BG_VOID;
    visuals.extreme_bg_color = BG_WELL;
    visuals.faint_bg_color = BG_BAR;
    visuals.widgets.noninteractive.bg_fill = BG_CARD;
    visuals.widgets.noninteractive.bg_stroke = Stroke::NONE;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(32, 28, 26);
    visuals.widgets.inactive.bg_stroke = Stroke::new(0.8, Color32::from_rgba_unmultiplied(180, 150, 105, 26));
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.1, GOLD);
    visuals.widgets.hovered.bg_fill = BG_CARD_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgba_unmultiplied(220, 178, 90, 95));
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.1, GOLD_BRIGHT);
    visuals.widgets.active.bg_fill = Color32::from_rgb(52, 42, 33);
    visuals.widgets.active.bg_stroke = Stroke::new(1.2, GOLD);
    visuals.widgets.active.fg_stroke = Stroke::new(1.2, GOLD_BRIGHT);
    visuals.slider_trailing_fill = true;
    visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(220, 178, 90, 88);
    visuals.selection.stroke = Stroke::new(1.0, GOLD);
    visuals.window_corner_radius = CornerRadius::same(12);
    visuals.window_stroke = Stroke::new(1.0, Color32::from_rgba_unmultiplied(180, 150, 100, 48));
    ctx.set_visuals(visuals);

    let mut style = (*ctx.global_style()).clone();
    style.spacing.item_spacing = Vec2::new(8.0, 8.0);
    style.spacing.button_padding = Vec2::new(12.0, 7.0);
    style.spacing.scroll.bar_width = 7.0;
    style.spacing.scroll.bar_inner_margin = 3.0;
    style.spacing.scroll.bar_outer_margin = 2.0;
    style.interaction.tooltip_delay = 0.22;
    ctx.set_global_style(style);
}

/// Paints a smooth vertical linear gradient inside `rect` using vertex shading.
pub fn paint_vertical_gradient(painter: &Painter, rect: Rect, top_col: Color32, bot_col: Color32) {
    if rect.width() <= 0.5 || rect.height() <= 0.5 {
        return;
    }
    let mut mesh = Mesh::default();
    let i0 = mesh.vertices.len() as u32;
    mesh.colored_vertex(rect.left_top(), top_col);
    mesh.colored_vertex(rect.right_top(), top_col);
    mesh.colored_vertex(rect.right_bottom(), bot_col);
    mesh.colored_vertex(rect.left_bottom(), bot_col);
    mesh.add_triangle(i0, i0 + 1, i0 + 2);
    mesh.add_triangle(i0, i0 + 2, i0 + 3);
    painter.add(Shape::mesh(mesh));
}

/// Paints a sculpted beveled plaque with vertical metallic gradient, top gleam rim, bottom shadow, and outer frame.
pub fn paint_beveled_box(
    painter: &Painter,
    rect: Rect,
    radius: u8,
    top_col: Color32,
    bot_col: Color32,
    border_col: Color32,
    border_w: f32,
    accent_bar_top: Option<Color32>,
) {
    if rect.width() <= 1.0 || rect.height() <= 1.0 {
        return;
    }
    painter.rect_filled(rect, CornerRadius::same(radius), bot_col);
    let inner = rect.shrink(1.0);
    let clipped = painter.with_clip_rect(inner);
    paint_vertical_gradient(&clipped, inner, top_col, bot_col);
    if let Some(ab) = accent_bar_top {
        let strip = Rect::from_min_size(inner.min, Vec2::new(inner.width(), 2.0));
        clipped.rect_filled(strip, CornerRadius::ZERO, ab.linear_multiply(0.75));
    }
    if border_w > 0.0 && border_col.a() > 0 {
        painter.rect_stroke(
            rect,
            CornerRadius::same(radius),
            Stroke::new(border_w, border_col.linear_multiply(0.65)),
            StrokeKind::Inside,
        );
    }
}

#[allow(dead_code)]
pub fn well_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(Color32::from_rgba_unmultiplied(15, 13, 14, 155))
        .stroke(Stroke::NONE)
        .corner_radius(CornerRadius::same(10))
        .inner_margin(Margin::same(10))
}

/// Renders a calm, low-clutter dark-fantasy surface (borderless by default; subtle glow when `accent` is set).
pub fn sculpted_card<R>(
    ui: &mut Ui,
    accent: Option<Color32>,
    margin: i8,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let bg_slot = ui.painter().add(Shape::Noop);
    let inner_resp = egui::Frame::new()
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::NONE)
        .corner_radius(CornerRadius::same(10))
        .inner_margin(Margin::same(margin))
        .show(ui, add_contents);
    let r = inner_resp.response.rect;
    let bg_painter = ui.painter().clone();
    let mut shapes = Vec::<Shape>::new();
    let radius = 10u8;
    let top_col = if let Some(ac) = accent {
        Color32::from_rgb(
            (32u16 + ac.r() as u16 / 14).min(255) as u8,
            (27u16 + ac.g() as u16 / 14).min(255) as u8,
            (25u16 + ac.b() as u16 / 14).min(255) as u8,
        )
    } else {
        Color32::from_rgb(30, 26, 24)
    };
    let bot_col = Color32::from_rgb(21, 18, 18);
    let mut mesh = Mesh::default();
    let ir = r.shrink(1.0);
    if ir.width() > 1.0 && ir.height() > 1.0 {
        let i0 = mesh.vertices.len() as u32;
        mesh.colored_vertex(ir.left_top(), top_col);
        mesh.colored_vertex(ir.right_top(), top_col);
        mesh.colored_vertex(ir.right_bottom(), bot_col);
        mesh.colored_vertex(ir.left_bottom(), bot_col);
        mesh.add_triangle(i0, i0 + 1, i0 + 2);
        mesh.add_triangle(i0, i0 + 2, i0 + 3);
    }
    shapes.push(Shape::rect_filled(r, CornerRadius::same(radius), bot_col));
    shapes.push(Shape::mesh(mesh));
    if let Some(ac) = accent {
        shapes.push(Shape::rect_stroke(
            r,
            CornerRadius::same(radius),
            Stroke::new(1.0, ac.linear_multiply(0.42)),
            StrokeKind::Inside,
        ));
    }
    bg_painter.set(bg_slot, Shape::Vec(shapes));
    inner_resp.inner
}

/// Renders an open, calm obsidian-glass panel without heavy multi-layered square frames.
pub fn sculpted_panel<R>(
    ui: &mut Ui,
    corner_accent: Color32,
    margin: i8,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let bg_slot = ui.painter().add(Shape::Noop);
    let inner_resp = egui::Frame::new()
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::NONE)
        .corner_radius(CornerRadius::same(14))
        .inner_margin(Margin::same(margin))
        .show(ui, add_contents);
    let r = inner_resp.response.rect;
    let mut shapes = Vec::<Shape>::new();
    let mut mesh = Mesh::default();
    let ir = r.shrink(1.0);
    if ir.width() > 1.0 && ir.height() > 1.0 {
        let top = Color32::from_rgba_unmultiplied(26, 22, 20, 228);
        let bot = Color32::from_rgba_unmultiplied(16, 14, 15, 236);
        let i0 = mesh.vertices.len() as u32;
        mesh.colored_vertex(ir.left_top(), top);
        mesh.colored_vertex(ir.right_top(), top);
        mesh.colored_vertex(ir.right_bottom(), bot);
        mesh.colored_vertex(ir.left_bottom(), bot);
        mesh.add_triangle(i0, i0 + 1, i0 + 2);
        mesh.add_triangle(i0, i0 + 2, i0 + 3);
    }
    shapes.push(Shape::rect_filled(
        r,
        CornerRadius::same(14),
        Color32::from_rgba_unmultiplied(16, 14, 15, 232),
    ));
    shapes.push(Shape::mesh(mesh));
    shapes.push(Shape::rect_stroke(
        r,
        CornerRadius::same(14),
        Stroke::new(0.9, corner_accent.linear_multiply(0.24)),
        StrokeKind::Inside,
    ));
    ui.painter().set(bg_slot, Shape::Vec(shapes));
    inner_resp.inner
}

/// Ornamental filigree horizontal divider with centered diamond stud (`━━ ◆ ━━`).
pub fn filigree_divider(ui: &mut Ui, accent: Color32) {
    let w = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(Vec2::new(w, 12.0), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    let p = ui.painter();
    let cy = rect.center().y;
    let cx = rect.center().x;
    let line_col = accent.linear_multiply(0.32);
    let half = (w * 0.5 - 12.0).max(8.0);
    p.line_segment([Pos2::new(rect.left() + 4.0, cy), Pos2::new(cx - 10.0, cy)], Stroke::new(1.0, line_col));
    p.line_segment([Pos2::new(cx + 10.0, cy), Pos2::new(rect.right() - 4.0, cy)], Stroke::new(1.0, line_col));
    if half > 20.0 {
        let dia = vec![
            Pos2::new(cx, cy - 4.0),
            Pos2::new(cx + 4.5, cy),
            Pos2::new(cx, cy + 4.0),
            Pos2::new(cx - 4.5, cy),
        ];
        p.add(Shape::convex_polygon(dia, accent.linear_multiply(0.85), Stroke::new(1.0, GOLD_BRIGHT)));
    }
}

/// Styled section ribbon header with small ornamental diamond pip and crisp Cinzel title.
pub fn section_ribbon(ui: &mut Ui, title: &str, subtitle: Option<&str>, accent: Color32) {
    ui.horizontal(|ui| {
        let (pip, _) = ui.allocate_exact_size(Vec2::new(10.0, 16.0), Sense::hover());
        let c = pip.center();
        let dia = vec![
            Pos2::new(c.x, c.y - 4.5),
            Pos2::new(c.x + 4.5, c.y),
            Pos2::new(c.x, c.y + 4.5),
            Pos2::new(c.x - 4.5, c.y),
        ];
        ui.painter().add(Shape::convex_polygon(dia, accent, Stroke::NONE));
        ui.label(egui::RichText::new(title).font(display_font(12.5)).color(accent));
        if let Some(sub) = subtitle {
            ui.label(egui::RichText::new(sub).font(body_font(11.5)).color(TEXT_MUTED));
        }
    });
}

/// Sculpted slider with recessed dark groove, luminous gradient track fill, brass thumb knob & value pill.
pub fn styled_slider(
    ui: &mut Ui,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    label: &str,
    value_text: &str,
    accent: Color32,
) -> bool {
    let mut changed = false;
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(label).font(body_font(13.0)).color(TEXT_PRIMARY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                draw_badge(ui, value_text, BG_WELL, GOLD_BRIGHT);
            });
        });

        let w = ui.available_width().max(120.0);
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 22.0), Sense::click_and_drag());
        let min_v = *range.start();
        let max_v = *range.end();
        let span = (max_v - min_v).max(1e-5);

        let track_pad = 9.0;
        let track_rect = Rect::from_min_max(
            Pos2::new(rect.left() + track_pad, rect.center().y - 3.5),
            Pos2::new(rect.right() - track_pad, rect.center().y + 3.5),
        );

        if resp.dragged() || resp.clicked() {
            if let Some(p) = resp.interact_pointer_pos() {
                let t = ((p.x - track_rect.left()) / track_rect.width().max(1.0)).clamp(0.0, 1.0);
                let new_v = min_v + t * span;
                if (new_v - *value).abs() > 1e-4 {
                    *value = new_v;
                    changed = true;
                }
            }
        }

        if ui.is_rect_visible(rect) {
            let t = ((*value - min_v) / span).clamp(0.0, 1.0);
            let p = ui.painter();
            // Recessed groove well
            p.rect(
                track_rect,
                CornerRadius::same(4),
                BG_WELL,
                Stroke::new(1.0, Color32::from_rgb(66, 56, 46)),
                StrokeKind::Inside,
            );
            // Active gradient fill bar
            let fill_w = (track_rect.width() * t).max(0.0);
            if fill_w > 2.0 {
                let fill_rect = Rect::from_min_size(track_rect.min + Vec2::splat(1.0), Vec2::new((fill_w - 2.0).max(1.0), track_rect.height() - 2.0));
                p.rect_filled(fill_rect, CornerRadius::same(3), accent);
                p.line_segment(
                    [Pos2::new(fill_rect.left() + 1.0, fill_rect.top() + 1.0), Pos2::new(fill_rect.right() - 1.0, fill_rect.top() + 1.0)],
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 245, 210, 110)),
                );
            }
            // Brass thumb medallion
            let kx = track_rect.left() + fill_w;
            let ky = track_rect.center().y;
            let kpos = Pos2::new(kx, ky);
            let kr = if resp.hovered() || resp.dragged() { 8.5 } else { 7.5 };
            p.circle_filled(Pos2::new(kx, ky + 1.5), kr, RIM_SHADOW);
            p.circle(
                kpos,
                kr,
                if resp.hovered() || resp.dragged() { GOLD_BRIGHT } else { GOLD },
                Stroke::new(1.5, BG_VOID),
            );
            p.circle_filled(kpos, kr * 0.36, BG_PANEL);
        }
        resp.on_hover_cursor(egui::CursorIcon::PointingHand);
    });
    changed
}

/// Custom brass pill toggle switch with status dot and label.
pub fn styled_toggle(ui: &mut Ui, checked: &mut bool, label: &str, accent: Color32) -> Response {
    let mut clicked = false;
    let resp = ui.horizontal(|ui| {
        let (sw_rect, sw_resp) = ui.allocate_exact_size(Vec2::new(40.0, 20.0), Sense::click());
        if ui.is_rect_visible(sw_rect) {
            let p = ui.painter();
            let bg = if *checked {
                accent.linear_multiply(0.32)
            } else {
                BG_WELL
            };
            let st = if *checked {
                Stroke::new(1.2, accent)
            } else {
                Stroke::new(1.0, BORDER_SUBTLE)
            };
            p.rect(sw_rect, CornerRadius::same(10), bg, st, StrokeKind::Inside);
            let kx = if *checked { sw_rect.right() - 10.0 } else { sw_rect.left() + 10.0 };
            let kpos = Pos2::new(kx, sw_rect.center().y);
            p.circle_filled(kpos, 6.5, if *checked { accent } else { TEXT_MUTED });
        }
        let lbl_resp = ui.add(
            egui::Label::new(
                egui::RichText::new(label)
                    .font(body_font(13.0))
                    .color(if *checked { TEXT_PRIMARY } else { TEXT_SECONDARY }),
            )
            .sense(Sense::click()),
        );
        if sw_resp.clicked() || lbl_resp.clicked() {
            *checked = !*checked;
            clicked = true;
        }
        sw_resp.on_hover_cursor(egui::CursorIcon::PointingHand) | lbl_resp.on_hover_cursor(egui::CursorIcon::PointingHand)
    });
    let _ = clicked;
    resp.inner
}

/// Recessed progress bar with glossy metallic fill and optional centered readout.
pub fn beveled_progress_bar(ui: &mut Ui, frac: f32, size: Vec2, fill: Color32, text: Option<&str>) {
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    let p = ui.painter();
    p.rect(
        rect,
        CornerRadius::same(4),
        BG_WELL,
        Stroke::new(1.0, Color32::from_rgb(66, 56, 46)),
        StrokeKind::Inside,
    );
    let f = frac.clamp(0.0, 1.0);
    if f > 0.01 {
        let ir = rect.shrink(1.5);
        let fill_rect = Rect::from_min_size(ir.min, Vec2::new((ir.width() * f).max(2.0), ir.height()));
        let clipped = p.with_clip_rect(fill_rect);
        paint_vertical_gradient(
            &clipped,
            fill_rect,
            Color32::from_rgb(
                fill.r().saturating_add(28),
                fill.g().saturating_add(24),
                fill.b().saturating_add(16),
            ),
            fill.linear_multiply(0.72),
        );
    }
    if let Some(txt) = text {
        p.text(rect.center(), Align2::CENTER_CENTER, txt, mono_font(10.5), TEXT_PRIMARY);
    }
}

/// Renders a unit/relic sprite on an open circular aura (no heavy square box cage).
pub fn portrait_pedestal(
    painter: &Painter,
    rect: Rect,
    tex: Option<&TextureHandle>,
    accent: Color32,
    active: bool,
) {
    let rad = rect.width().min(rect.height()) * 0.46;
    painter.circle_filled(
        rect.center(),
        rad,
        accent.linear_multiply(if active { 0.16 } else { 0.08 }),
    );
    painter.circle_filled(
        rect.center(),
        rad * 0.68,
        accent.linear_multiply(if active { 0.18 } else { 0.07 }),
    );
    if active {
        painter.circle_stroke(
            rect.center(),
            rad,
            Stroke::new(1.0, accent.linear_multiply(0.35)),
        );
    }
    if let Some(t) = tex {
        crate::assets::portrait_rect(painter, t, rect.shrink(2.0));
    }
}

/// Compact char-safe ellipsis helper so labels never clip or spill past narrow slots.
pub fn clamp_chars(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else if max_chars > 3 {
        let cut: String = s.chars().take(max_chars - 3).collect();
        format!("{cut}...")
    } else {
        s.chars().take(max_chars).collect()
    }
}

/// Paints dark smoked-leather / parchment texture inside a panel rect (aspect-preserving cover fit).
pub fn paint_parchment_backdrop(painter: &Painter, rect: Rect, tex: &TextureHandle, tint: Color32) {
    let ts = tex.size_vec2();
    if ts.x <= 1.0 || ts.y <= 1.0 || rect.width() <= 1.0 || rect.height() <= 1.0 {
        return;
    }
    let scale = (rect.width() / ts.x).max(rect.height() / ts.y);
    let u_span = (rect.width() / (ts.x * scale)).clamp(0.1, 1.0);
    let v_span = (rect.height() / (ts.y * scale)).clamp(0.1, 1.0);
    let u0 = (1.0 - u_span) * 0.5;
    let v0 = (1.0 - v_span) * 0.5;
    painter.image(
        tex.id(),
        rect,
        Rect::from_min_max(Pos2::new(u0, v0), Pos2::new(u0 + u_span, v0 + v_span)),
        tint,
    );
}

pub fn faction_color(id: FactionId) -> Color32 {
    let [r, g, b] = id.accent_rgb();
    Color32::from_rgb(r, g, b)
}

pub fn tier_color(tier: UnitTier) -> Color32 {
    match tier {
        UnitTier::C => EMERALD,
        UnitTier::B => BLUE_LEGION,
        UnitTier::A => AMETHYST,
    }
}

pub fn archetype_color(arch: MissionArchetype) -> Color32 {
    match arch {
        MissionArchetype::Siege => EMERALD,
        MissionArchetype::HoldTheLine => BLUE_LEGION,
        MissionArchetype::Blitzkrieg => RED_ENEMY,
        MissionArchetype::TitanSlayer => GOLD,
        MissionArchetype::Mutator => AMETHYST,
    }
}

pub fn draw_badge(ui: &mut Ui, text: &str, fill: Color32, fg: Color32) {
    let pill_fill = if fill == BG_WELL || fill == BG_CARD_TOP {
        fg.linear_multiply(0.12)
    } else {
        fill
    };
    let frame = egui::Frame::new()
        .fill(pill_fill)
        .stroke(Stroke::NONE)
        .corner_radius(CornerRadius::same(11))
        .inner_margin(Margin::symmetric(8, 2));
    frame.show(ui, |ui| {
        ui.label(egui::RichText::new(text).font(mono_font(10.0)).color(fg));
    });
}

pub fn cta_button(ui: &mut Ui, text: &str, min_size: Vec2) -> Response {
    let (rect, resp) = ui.allocate_exact_size(min_size, Sense::click());
    if ui.is_rect_visible(rect) {
        let down = resp.is_pointer_button_down_on();
        let draw_r = if down { rect.translate(Vec2::new(0.0, 1.0)) } else { rect };
        let (top_c, bot_c, rim_c) = if down {
            (
                Color32::from_rgb(160, 114, 38),
                Color32::from_rgb(198, 150, 62),
                GOLD,
            )
        } else if resp.hovered() {
            (
                Color32::from_rgb(254, 232, 162),
                Color32::from_rgb(196, 148, 56),
                Color32::from_rgb(255, 242, 198),
            )
        } else {
            (
                Color32::from_rgb(240, 200, 112),
                Color32::from_rgb(166, 120, 42),
                Color32::from_rgb(245, 218, 148),
            )
        };
        let p = ui.painter();
        p.rect_filled(draw_r, CornerRadius::same(8), bot_c);
        let inner = draw_r.shrink(1.0);
        let clipped = p.with_clip_rect(inner);
        paint_vertical_gradient(&clipped, inner, top_c, bot_c);
        p.rect_stroke(draw_r, CornerRadius::same(8), Stroke::new(1.0, rim_c), StrokeKind::Inside);
        p.text(
            draw_r.center(),
            Align2::CENTER_CENTER,
            text,
            display_font(12.5),
            BG_VOID,
        );
    }
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn subtle_button(ui: &mut Ui, text: &str, active: bool, accent: Color32, min_size: Vec2) -> Response {
    let (rect, resp) = ui.allocate_exact_size(min_size, Sense::click());
    if ui.is_rect_visible(rect) {
        let down = resp.is_pointer_button_down_on();
        let draw_r = if down { rect.translate(Vec2::new(0.0, 0.5)) } else { rect };
        let (fill_c, border) = if active {
            (
                Color32::from_rgba_unmultiplied(
                    24u8.saturating_add((accent.r() / 5).min(56)),
                    20u8.saturating_add((accent.g() / 5).min(50)),
                    19u8.saturating_add((accent.b() / 5).min(56)),
                    220,
                ),
                Stroke::new(1.0, accent.linear_multiply(0.55)),
            )
        } else if resp.hovered() {
            (
                Color32::from_rgba_unmultiplied(50, 42, 36, 215),
                Stroke::new(0.8, Color32::from_rgba_unmultiplied(210, 178, 120, 65)),
            )
        } else {
            (
                Color32::from_rgba_unmultiplied(33, 28, 26, 185),
                Stroke::NONE,
            )
        };
        let p = ui.painter();
        p.rect_filled(draw_r, CornerRadius::same(8), fill_c);
        if border != Stroke::NONE {
            p.rect_stroke(draw_r, CornerRadius::same(8), border, StrokeKind::Inside);
        }
        let text_col = if active {
            Color32::from_rgb(
                accent.r().saturating_add(24),
                accent.g().saturating_add(24),
                accent.b().saturating_add(24),
            )
        } else if resp.hovered() {
            GOLD_BRIGHT
        } else {
            TEXT_PRIMARY
        };
        p.text(
            draw_r.center(),
            Align2::CENTER_CENTER,
            text,
            body_font(12.5),
            text_col,
        );
    }
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Clean borderless command/filter tab with soft hover wash and bottom accent underline when active.
pub fn nav_tab_button(ui: &mut Ui, text: &str, active: bool, accent: Color32, min_size: Vec2) -> Response {
    let (rect, resp) = ui.allocate_exact_size(min_size, Sense::click());
    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        if active {
            p.rect_filled(rect, CornerRadius::same(8), accent.linear_multiply(0.14));
            let under = Rect::from_min_size(
                Pos2::new(rect.left() + 8.0, rect.bottom() - 2.0),
                Vec2::new((rect.width() - 16.0).max(4.0), 2.0),
            );
            p.rect_filled(under, CornerRadius::same(1), accent);
        } else if resp.hovered() {
            p.rect_filled(
                rect,
                CornerRadius::same(8),
                Color32::from_rgba_unmultiplied(255, 240, 210, 14),
            );
        }
        p.text(
            rect.center(),
            Align2::CENTER_CENTER,
            text,
            body_font(12.4),
            if active {
                GOLD_BRIGHT
            } else if resp.hovered() {
                TEXT_PRIMARY
            } else {
                TEXT_SECONDARY
            },
        );
    }
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn equal_columns(ui: &mut Ui, n: usize, mut draw_col: impl FnMut(&mut Ui, usize)) {
    if n == 0 {
        return;
    }
    let spacing = ui.spacing().item_spacing.x;
    let avail = ui.available_width();
    let col_w = ((avail - spacing * (n.saturating_sub(1) as f32)) / n as f32).max(24.0);
    ui.horizontal_top(|ui| {
        for idx in 0..n {
            ui.allocate_ui_with_layout(
                Vec2::new(col_w, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    ui.set_min_width(col_w);
                    ui.set_max_width(col_w);
                    draw_col(ui, idx);
                },
            );
        }
    });
}

/// Renders a dark-fantasy crest on an open radial glow halo without any square cage.
pub fn crest_socket(painter: &Painter, rect: Rect, tex: &TextureHandle, accent: Color32) {
    let rad = rect.width().min(rect.height()) * 0.48;
    painter.circle_filled(rect.center(), rad, accent.linear_multiply(0.16));
    painter.image(
        tex.id(),
        rect.shrink(2.0),
        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
        Color32::WHITE,
    );
}

pub fn paint_unit_shape(
    painter: &Painter,
    shape: UnitShape,
    center: Pos2,
    radius: f32,
    angle: f32,
    fill: Color32,
    stroke_col: Color32,
    stroke_w: f32,
) {
    let rot = |px: f32, py: f32| -> Pos2 {
        let c = angle.cos();
        let s = angle.sin();
        Pos2::new(center.x + px * c - py * s, center.y + px * s + py * c)
    };

    match shape {
        UnitShape::Circle => {
            painter.circle(center, radius, fill, Stroke::new(stroke_w, stroke_col));
            painter.circle_filled(rot(radius * 0.45, 0.0), radius * 0.25, stroke_col);
        }
        UnitShape::Square => {
            let s = radius * 0.85;
            let pts = vec![rot(-s, -s), rot(s, -s), rot(s, s), rot(-s, s)];
            painter.add(Shape::convex_polygon(pts, fill, Stroke::new(stroke_w, stroke_col)));
            let inner = s * 0.46;
            painter.rect_stroke(
                Rect::from_center_size(center, Vec2::splat(inner * 2.0)),
                CornerRadius::same(1),
                Stroke::new(1.2, Color32::from_rgba_unmultiplied(255, 255, 255, 110)),
                StrokeKind::Inside,
            );
        }
        UnitShape::Triangle => {
            let pts = vec![
                rot(radius * 1.2, 0.0),
                rot(-radius * 0.9, -radius * 0.9),
                rot(-radius * 0.9, radius * 0.9),
            ];
            painter.add(Shape::convex_polygon(pts, fill, Stroke::new(stroke_w, stroke_col)));
        }
        UnitShape::Diamond => {
            let pts = vec![
                rot(radius * 1.25, 0.0),
                rot(0.0, -radius * 0.9),
                rot(-radius * 1.25, 0.0),
                rot(0.0, radius * 0.9),
            ];
            painter.add(Shape::convex_polygon(pts, fill, Stroke::new(stroke_w, stroke_col)));
            painter.circle_filled(center, radius * 0.26, Color32::WHITE);
        }
        UnitShape::Cross => {
            let a = radius * 0.40;
            let l = radius * 1.05;
            painter.rect(
                Rect::from_center_size(center, Vec2::new(a * 2.0, l * 2.0)),
                CornerRadius::same(2),
                fill,
                Stroke::new(stroke_w, stroke_col),
                StrokeKind::Outside,
            );
            painter.rect(
                Rect::from_center_size(center, Vec2::new(l * 2.0, a * 2.0)),
                CornerRadius::same(2),
                fill,
                Stroke::new(stroke_w, stroke_col),
                StrokeKind::Outside,
            );
            painter.rect_filled(
                Rect::from_center_size(center, Vec2::new(a * 1.8, l * 1.9)),
                CornerRadius::same(1),
                fill,
            );
        }
        UnitShape::Hexagon => {
            let pts: Vec<Pos2> = (0..6)
                .map(|i| {
                    let a = angle + (std::f32::consts::TAU / 6.0) * (i as f32);
                    Pos2::new(center.x + a.cos() * radius * 1.15, center.y + a.sin() * radius * 1.15)
                })
                .collect();
            painter.add(Shape::convex_polygon(pts, fill, Stroke::new(stroke_w, stroke_col)));
            painter.circle_filled(center, radius * 0.34, Color32::from_rgb(253, 230, 138));
        }
    }
}

pub fn paint_archetype_emblem(
    painter: &Painter,
    arch: MissionArchetype,
    center: Pos2,
    r: f32,
    color: Color32,
) {
    let s = Stroke::new(1.8, color);
    match arch {
        MissionArchetype::Siege => {
            // Crossed blades
            painter.line_segment([center + Vec2::new(-r * 0.65, r * 0.65), center + Vec2::new(r * 0.65, -r * 0.65)], s);
            painter.line_segment([center + Vec2::new(-r * 0.65, -r * 0.65), center + Vec2::new(r * 0.65, r * 0.65)], s);
        }
        MissionArchetype::HoldTheLine => {
            // Crest shield
            let pts = vec![
                center + Vec2::new(-r * 0.65, -r * 0.7),
                center + Vec2::new(r * 0.65, -r * 0.7),
                center + Vec2::new(r * 0.65, 0.0),
                center + Vec2::new(0.0, r * 0.8),
                center + Vec2::new(-r * 0.65, 0.0),
            ];
            painter.add(Shape::closed_line(pts, s));
        }
        MissionArchetype::Blitzkrieg => {
            // Lightning bolt
            let pts = vec![
                center + Vec2::new(r * 0.15, -r * 0.8),
                center + Vec2::new(-r * 0.45, r * 0.05),
                center + Vec2::new(r * 0.05, r * 0.05),
                center + Vec2::new(-r * 0.2, r * 0.85),
                center + Vec2::new(r * 0.48, -r * 0.08),
                center + Vec2::new(-r * 0.05, -r * 0.08),
            ];
            painter.add(Shape::convex_polygon(pts, color, Stroke::NONE));
        }
        MissionArchetype::TitanSlayer => {
            // 3-peak Imperial Crown
            let pts = vec![
                center + Vec2::new(-r * 0.72, r * 0.52),
                center + Vec2::new(-r * 0.75, -r * 0.35),
                center + Vec2::new(-r * 0.3, r * 0.05),
                center + Vec2::new(0.0, -r * 0.65),
                center + Vec2::new(r * 0.3, r * 0.05),
                center + Vec2::new(r * 0.75, -r * 0.35),
                center + Vec2::new(r * 0.72, r * 0.52),
            ];
            painter.add(Shape::convex_polygon(pts, color, Stroke::NONE));
        }
        MissionArchetype::Mutator => {
            // Biohazard / Atom trefoil rings
            for k in 0..3 {
                let a = (std::f32::consts::TAU / 3.0) * (k as f32) - std::f32::consts::FRAC_PI_2;
                let c = center + Vec2::new(a.cos(), a.sin()) * (r * 0.38);
                painter.circle_stroke(c, r * 0.36, s);
            }
            painter.circle_filled(center, r * 0.18, color);
        }
    }
}

pub fn paint_star(painter: &Painter, center: Pos2, outer_r: f32, filled: bool, color: Color32) {
    let inner_r = outer_r * 0.45;
    let mut pts = Vec::with_capacity(10);
    for i in 0..10 {
        let a = -std::f32::consts::FRAC_PI_2 + (std::f32::consts::PI / 5.0) * (i as f32);
        let r = if i % 2 == 0 { outer_r } else { inner_r };
        pts.push(center + Vec2::new(a.cos() * r, a.sin() * r));
    }
    if filled {
        painter.add(Shape::convex_polygon(
            pts.iter().step_by(2).copied().collect(),
            color,
            Stroke::NONE,
        ));
        // Add crisp points
        for i in 0..5 {
            let p_tip = pts[i * 2];
            let p_l = pts[(i * 2 + 9) % 10];
            let p_r = pts[i * 2 + 1];
            painter.add(Shape::convex_polygon(vec![p_tip, p_r, center, p_l], color, Stroke::NONE));
        }
    } else {
        painter.add(Shape::closed_line(pts, Stroke::new(1.2, color)));
    }
}
