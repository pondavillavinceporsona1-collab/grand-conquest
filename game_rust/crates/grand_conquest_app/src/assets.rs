use std::collections::HashMap;

use egui::epaint::{ColorImage, Mesh, TextureHandle, Vertex};
use egui::{Color32, Context, Painter, Pos2, Rect, TextureOptions};
use grand_conquest_core::model::{FactionId, RelicId, StructureKind, Team, UnitTier};
use grand_conquest_core::relics::RELICS;
use grand_conquest_core::sim::UnitEntity;
use grand_conquest_core::units::UNITS;
use include_dir::{include_dir, Dir};

pub static ASSETS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../assets");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewPose {
    Idle,
    Walk,
    Attack,
}

#[derive(Debug, Clone, Copy)]
pub struct AnimTiming {
    pub atk_anim_timer: f32,
    pub atk_anim_duration: f32,
    pub is_moving: bool,
    pub walk_phase: f32,
    pub idle_phase: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SheetSample {
    /// Index in a 4x2 grid: 0..4 = walk progression, 4..8 = attack sequence.
    pub frame_idx: usize,
    pub squash_x: f32,
    pub squash_y: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct IntactMotion {
    pub squash_x: f32,
    pub squash_y: f32,
    pub lean: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

/// Maps continuous combat/movement timers onto the 8 progressive frames of a 4x2 sprite sheet.
pub fn compute_sheet_frame(anim: AnimTiming) -> SheetSample {
    if anim.atk_anim_timer > 0.0 && anim.atk_anim_duration > 0.0 {
        let p = (1.0 - anim.atk_anim_timer / anim.atk_anim_duration).clamp(0.0, 1.0);
        if p < 0.16 {
            return SheetSample {
                frame_idx: 4,
                squash_x: 1.03,
                squash_y: 0.96,
                offset_x: -0.02,
                offset_y: 0.0,
            };
        } else if p < 0.42 {
            return SheetSample {
                frame_idx: 5,
                squash_x: 0.98,
                squash_y: 1.03,
                offset_x: -0.03,
                offset_y: -0.01,
            };
        } else if p < 0.76 {
            return SheetSample {
                frame_idx: 6,
                squash_x: 1.05,
                squash_y: 0.96,
                offset_x: 0.06,
                offset_y: 0.0,
            };
        } else {
            let t = 1.0 - (p - 0.76) / 0.24;
            return SheetSample {
                frame_idx: 7,
                squash_x: 1.0 + 0.02 * t,
                squash_y: 1.0 - 0.01 * t,
                offset_x: 0.03 * t,
                offset_y: 0.0,
            };
        }
    }

    if anim.is_moving {
        let phase = anim.walk_phase.rem_euclid(std::f32::consts::TAU) / std::f32::consts::TAU;
        let frame_idx = ((phase * 4.0).floor() as usize).min(3);
        let is_passing = frame_idx == 1 || frame_idx == 3;
        return SheetSample {
            frame_idx,
            squash_x: if is_passing { 0.99 } else { 1.01 },
            squash_y: if is_passing { 1.015 } else { 0.99 },
            offset_x: 0.0,
            offset_y: if is_passing { -0.015 } else { 0.0 },
        };
    }

    let br = anim.idle_phase.sin();
    SheetSample {
        frame_idx: 0,
        squash_x: 1.0 - 0.010 * br,
        squash_y: 1.0 + 0.016 * br,
        offset_x: 0.0,
        offset_y: 0.0,
    }
}

/// Cohesive intact whole-body animation used for units awaiting their bespoke 4x2 sprite sheet.
pub fn compute_intact_motion(anim: AnimTiming) -> IntactMotion {
    if anim.atk_anim_timer > 0.0 && anim.atk_anim_duration > 0.0 {
        let p = (1.0 - anim.atk_anim_timer / anim.atk_anim_duration).clamp(0.0, 1.0);
        if p < 0.35 {
            let t = p / 0.35;
            return IntactMotion {
                squash_x: 1.0 - 0.05 * t,
                squash_y: 1.0 + 0.04 * t,
                lean: -0.10 * t,
                offset_x: -0.06 * t,
                offset_y: -0.01 * t,
            };
        } else if p < 0.65 {
            let t = (p - 0.35) / 0.30;
            let lerp = |a: f32, b: f32| a + (b - a) * t;
            return IntactMotion {
                squash_x: lerp(0.95, 1.08),
                squash_y: lerp(1.04, 0.93),
                lean: lerp(-0.10, 0.16),
                offset_x: lerp(-0.06, 0.14),
                offset_y: lerp(-0.01, 0.01),
            };
        } else {
            let t = 1.0 - (p - 0.65) / 0.35;
            return IntactMotion {
                squash_x: 1.0 + 0.08 * t,
                squash_y: 1.0 - 0.07 * t,
                lean: 0.16 * t,
                offset_x: 0.14 * t,
                offset_y: 0.01 * t,
            };
        }
    }

    if anim.is_moving {
        let hop = anim.walk_phase.sin().abs();
        let rock = anim.walk_phase.cos() * 0.04;
        return IntactMotion {
            squash_x: 1.03 - 0.05 * hop,
            squash_y: 0.96 + 0.06 * hop,
            lean: 0.06 + rock,
            offset_x: 0.0,
            offset_y: -0.055 * hop,
        };
    }

    let br = anim.idle_phase.sin();
    IntactMotion {
        squash_x: 1.0 - 0.014 * br,
        squash_y: 1.0 + 0.024 * br,
        lean: 0.0,
        offset_x: 0.0,
        offset_y: 0.0,
    }
}

pub struct AssetBank {
    pub portraits: HashMap<&'static str, TextureHandle>,
    pub sheets: HashMap<&'static str, TextureHandle>,
    pub battle_atlas: Option<TextureHandle>,
    pub atlas_uvs: HashMap<&'static str, Rect>,
    pub world_map: Option<TextureHandle>,
    pub parchment_dark: Option<TextureHandle>,
    pub arenas: HashMap<FactionId, TextureHandle>,
    pub crests: HashMap<FactionId, TextureHandle>,
    pub relic_icons: HashMap<RelicId, TextureHandle>,
    pub fortresses: [Option<TextureHandle>; 4],
}

impl AssetBank {
    pub fn load(ctx: &Context) -> Self {
        let mut portraits = HashMap::new();
        let mut sheets = HashMap::new();
        let mut atlas_entries: Vec<(&'static str, usize, usize, Vec<u8>)> = Vec::with_capacity(UNITS.len());

        for u in UNITS {
            let key = u.sprite_key;
            if !portraits.contains_key(key) {
                if let Some(img_file) = ASSETS_DIR.get_file(format!("static_all_54/{key}.png")) {
                    if let Some((w, h, raw)) = decode_rgba(img_file.contents()) {
                        let color_img = ColorImage::from_rgba_unmultiplied([w, h], &raw);
                        portraits.insert(
                            key,
                            ctx.load_texture(
                                &format!("p_{key}"),
                                color_img,
                                TextureOptions::LINEAR,
                            ),
                        );
                        atlas_entries.push((key, w, h, raw));
                    }
                }
            }

            if !sheets.contains_key(key) {
                if let Some(sheet_file) = ASSETS_DIR.get_file(format!("sheets/{key}.png")) {
                    if let Some(tex) = decode_image_texture(ctx, &format!("sheet_{key}"), sheet_file.contents()) {
                        sheets.insert(key, tex);
                    }
                }
            }
        }

        let (battle_atlas, atlas_uvs) = pack_battle_atlas(ctx, &atlas_entries);

        let load_art = |rel: &str, name: &str| -> Option<TextureHandle> {
            let file = ASSETS_DIR.get_file(rel)?;
            decode_image_texture(ctx, name, file.contents())
        };

        let world_map = load_art("art/world_map.jpg", "art_world_map");
        let parchment_dark = load_art("art/parchment_dark.jpg", "art_parchment_dark");

        let mut arenas = HashMap::new();
        let mut crests = HashMap::new();
        for fid in FactionId::ALL {
            let fstr = fid.id_str();
            if let Some(tex) = load_art(&format!("art/arenas/arena_{fstr}.jpg"), &format!("arena_{fstr}")) {
                arenas.insert(fid, tex);
            }
            if let Some(tex) = load_art(&format!("art/crests/crest_{fstr}.png"), &format!("crest_{fstr}")) {
                crests.insert(fid, tex);
            }
        }

        let mut relic_icons = HashMap::new();
        for r in RELICS {
            let rstr = r.id.as_str();
            if let Some(tex) = load_art(&format!("art/relics/relic_{rstr}.png"), &format!("relic_{rstr}")) {
                relic_icons.insert(r.id, tex);
            }
        }

        let fortresses = [
            load_art("art/fortresses/fort_blue_turret.png", "fort_blue_turret"),
            load_art("art/fortresses/fort_blue_base.png", "fort_blue_base"),
            load_art("art/fortresses/fort_red_turret.png", "fort_red_turret"),
            load_art("art/fortresses/fort_red_base.png", "fort_red_base"),
        ];

        Self {
            portraits,
            sheets,
            battle_atlas,
            atlas_uvs,
            world_map,
            parchment_dark,
            arenas,
            crests,
            relic_icons,
            fortresses,
        }
    }

    pub fn get_portrait(&self, sprite_key: &str) -> Option<&TextureHandle> {
        self.portraits.get(sprite_key)
    }

    pub fn has_sheet(&self, sprite_key: &str) -> bool {
        self.sheets.contains_key(sprite_key)
    }

    pub fn get_sheet(&self, sprite_key: &str) -> Option<&TextureHandle> {
        self.sheets.get(sprite_key)
    }

    pub fn get_crest(&self, faction: FactionId) -> Option<&TextureHandle> {
        self.crests.get(&faction)
    }

    pub fn get_arena(&self, faction: FactionId) -> Option<&TextureHandle> {
        self.arenas.get(&faction)
    }

    pub fn get_relic_icon(&self, relic: RelicId) -> Option<&TextureHandle> {
        self.relic_icons.get(&relic)
    }

    pub fn get_fortress(&self, team: Team, kind: StructureKind) -> Option<&TextureHandle> {
        let idx = match (team, kind) {
            (Team::Blue, StructureKind::Base) => 1,
            (Team::Blue, _) => 0,
            (Team::Red, StructureKind::Base) => 3,
            (Team::Red, _) => 2,
        };
        self.fortresses[idx].as_ref()
    }

    /// Renders a single frame cell from a 4x2 sprite sheet inside `rect`.
    pub fn paint_sheet_cell(
        &self,
        painter: &Painter,
        tex: &TextureHandle,
        rect: Rect,
        frame_idx: usize,
        facing: f32,
        tint: Color32,
    ) {
        let idx = frame_idx.min(7);
        let col = (idx % 4) as f32;
        let row = (idx / 4) as f32;
        let u_min = col * 0.25;
        let v_min = row * 0.5;
        paint_tex_quad(
            painter,
            tex,
            rect,
            Pos2::new(u_min, v_min),
            Pos2::new(u_min + 0.25, v_min + 0.5),
            facing,
            0.0,
            tint,
        );
    }

    /// Draws an animated unit anchored around `center`: prefers bespoke 4x2 sprite sheets
    /// and falls back cleanly to intact animated whole-body artwork (never torn limb chunks).
    pub fn paint_unit_sprite(
        &self,
        painter: &Painter,
        sprite_key: &str,
        center: Pos2,
        draw_size: f32,
        facing: f32,
        anim: AnimTiming,
        tint: Color32,
    ) -> bool {
        if let Some(sheet) = self.sheets.get(sprite_key) {
            let sample = compute_sheet_frame(anim);
            let feet_y = center.y + 0.17 * draw_size + sample.offset_y * draw_size;
            let cell = draw_size * 1.06;
            let w = cell * sample.squash_x;
            let h = cell * sample.squash_y;
            let cx = center.x + facing * sample.offset_x * draw_size;
            let bottom_y = feet_y + 0.06 * h;
            let rect = Rect::from_min_max(
                Pos2::new(cx - w * 0.5, bottom_y - h),
                Pos2::new(cx + w * 0.5, bottom_y),
            );
            self.paint_sheet_cell(painter, sheet, rect, sample.frame_idx, facing, tint);
            return true;
        }

        let Some(portrait) = self.portraits.get(sprite_key) else {
            return false;
        };
        let (rect, top_skew_x) = intact_sprite_rect_and_lean(center, draw_size, facing, anim);
        paint_tex_quad(
            painter,
            portrait,
            rect,
            Pos2::ZERO,
            Pos2::new(1.0, 1.0),
            facing,
            top_skew_x,
            tint,
        );
        true
    }

    /// Pushes an animated unit sprite quad into an existing `battle_atlas`-bound `Mesh`.
    /// Returns `false` if the unit uses a bespoke 4x2 sheet or lacks an atlas UV entry.
    pub fn push_soldier_atlas_sprite(
        &self,
        mesh: &mut Mesh,
        u: &UnitEntity,
        screen_pos: Pos2,
        draw_size: f32,
        facing: f32,
        tint: Color32,
    ) -> bool {
        if self.has_sheet(u.def.sprite_key) {
            return false;
        }
        let Some(&uv) = self.atlas_uvs.get(u.def.sprite_key) else {
            return false;
        };
        let anim = AnimTiming {
            atk_anim_timer: u.atk_anim_timer,
            atk_anim_duration: u.atk_anim_duration,
            is_moving: u.is_moving,
            walk_phase: u.walk_phase,
            idle_phase: u.idle_phase,
        };
        let (rect, top_skew_x) = intact_sprite_rect_and_lean(screen_pos, draw_size, facing, anim);
        push_tex_quad_to_mesh(mesh, rect, uv.min, uv.max, facing, top_skew_x, tint);
        true
    }

    /// Pushes a melee slash crescent ribbon for intact sprites into `mesh`.
    pub fn push_soldier_slash_mesh(
        &self,
        mesh: &mut Mesh,
        u: &UnitEntity,
        screen_pos: Pos2,
        draw_size: f32,
        facing: f32,
        is_blue: bool,
    ) {
        if self.has_sheet(u.def.sprite_key)
            || u.atk_anim_timer <= 0.0
            || (u.def.rng_studs > 7.5 && !u.is_boss)
            || u.atk_anim_duration <= 0.0
        {
            return;
        }
        let p = 1.0 - u.atk_anim_timer / u.atk_anim_duration;
        if !(0.32..=0.68).contains(&p) {
            return;
        }
        let alpha = ((1.0 - (p - 0.5).abs() / 0.2) * 230.0).clamp(0.0, 255.0) as u8;
        let sc = if is_blue {
            Color32::from_rgba_unmultiplied(147, 197, 253, alpha)
        } else {
            Color32::from_rgba_unmultiplied(253, 164, 175, alpha)
        };
        let slash_r = 34.0 * (draw_size / 128.0);
        let ox = facing * 12.0 * (draw_size / 128.0);
        let oy = -18.0 * (draw_size / 128.0);
        let mut arc = [Pos2::ZERO; 10];
        for (k, pt) in arc.iter_mut().enumerate() {
            let t = k as f32 / 9.0;
            let a = (-0.38 * std::f32::consts::PI) + t * (0.70 * std::f32::consts::PI);
            *pt = Pos2::new(
                screen_pos.x + ox + facing * a.cos() * slash_r,
                screen_pos.y + oy + a.sin() * slash_r,
            );
        }
        push_polyline_ribbon_mesh(mesh, &arc, 3.5, sc);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SoldierDrawMeta {
    pub draw_size: f32,
    pub facing: f32,
    pub tint: Color32,
}

pub fn compute_soldier_draw_params(u: &UnitEntity, scale_factor: f32) -> SoldierDrawMeta {
    let base_draw = match (u.is_boss, u.boss_def, u.tier) {
        (true, Some(bd), _) => 54.0 * bd.scale * 0.85,
        (_, _, UnitTier::A) => 78.0,
        (_, _, UnitTier::B) => 64.0,
        (_, _, UnitTier::C) => 54.0,
    };
    let draw_size = base_draw * scale_factor;
    let facing = if u.aim_angle.cos() >= -0.15 { 1.0 } else { -1.0 };
    let tint = if u.hit_flash > 0.0 {
        Color32::from_rgb(255, 210, 210)
    } else {
        Color32::WHITE
    };
    SoldierDrawMeta {
        draw_size,
        facing,
        tint,
    }
}

pub fn push_soldier_ground_mesh(
    mesh: &mut Mesh,
    u: &UnitEntity,
    screen_pos: Pos2,
    scale_factor: f32,
    is_blue: bool,
) {
    let foot_y = screen_pos.y + u.radius * 0.65 * scale_factor;
    let ring_col = if u.is_boss {
        Color32::from_rgb(248, 113, 113)
    } else if u.tier == UnitTier::A {
        Color32::from_rgb(253, 224, 71)
    } else if is_blue {
        Color32::from_rgb(96, 165, 250)
    } else {
        Color32::from_rgb(251, 113, 133)
    };
    let fill_col = if is_blue {
        Color32::from_rgba_unmultiplied(37, 99, 235, 95)
    } else {
        Color32::from_rgba_unmultiplied(225, 29, 72, 95)
    };
    let rw = u.radius * 1.28 * scale_factor;
    let rh = rw * 0.44;
    let stroke_w = if u.tier == UnitTier::A || u.is_boss { 2.4 } else { 1.6 };
    push_ellipse_fill_and_ring_mesh(mesh, Pos2::new(screen_pos.x, foot_y), rw, rh, fill_col, stroke_w, ring_col);
}

fn intact_sprite_rect_and_lean(
    center: Pos2,
    draw_size: f32,
    facing: f32,
    anim: AnimTiming,
) -> (Rect, f32) {
    let motion = compute_intact_motion(anim);
    let feet_y = center.y + 0.17 * draw_size + motion.offset_y * draw_size;
    let w = draw_size * motion.squash_x;
    let h = draw_size * motion.squash_y;
    let cx = center.x + facing * motion.offset_x * draw_size;
    let bottom_y = feet_y + 0.11 * h;
    let rect = Rect::from_min_max(
        Pos2::new(cx - w * 0.5, bottom_y - h),
        Pos2::new(cx + w * 0.5, bottom_y),
    );
    (rect, facing * motion.lean * draw_size)
}

const C22_5: f32 = 0.9238795;
const S22_5: f32 = 0.38268343;
const D45: f32 = std::f32::consts::FRAC_1_SQRT_2;

const CIRCLE16: [(f32, f32); 16] = [
    (1.0, 0.0),
    (C22_5, S22_5),
    (D45, D45),
    (S22_5, C22_5),
    (0.0, 1.0),
    (-S22_5, C22_5),
    (-D45, D45),
    (-C22_5, S22_5),
    (-1.0, 0.0),
    (-C22_5, -S22_5),
    (-D45, -D45),
    (-S22_5, -C22_5),
    (0.0, -1.0),
    (S22_5, -C22_5),
    (D45, -D45),
    (C22_5, -S22_5),
];

const OCT8: [(f32, f32); 8] = [
    (1.0, 0.0),
    (D45, D45),
    (0.0, 1.0),
    (-D45, D45),
    (-1.0, 0.0),
    (-D45, -D45),
    (0.0, -1.0),
    (D45, -D45),
];

pub fn flush_mesh_if_large(painter: &Painter, mesh: &mut Mesh) {
    if mesh.vertices.len() > 40_000 {
        let tid = mesh.texture_id;
        let full = std::mem::replace(mesh, Mesh::with_texture(tid));
        painter.add(full);
    }
}

pub fn push_solid_rect_mesh(mesh: &mut Mesh, rect: Rect, color: Color32) {
    let base = mesh.vertices.len() as u32;
    mesh.colored_vertex(rect.min, color);
    mesh.colored_vertex(Pos2::new(rect.max.x, rect.min.y), color);
    mesh.colored_vertex(rect.max, color);
    mesh.colored_vertex(Pos2::new(rect.min.x, rect.max.y), color);
    mesh.add_triangle(base, base + 1, base + 2);
    mesh.add_triangle(base, base + 2, base + 3);
}

pub fn push_rect_outline_mesh(mesh: &mut Mesh, rect: Rect, thickness: f32, color: Color32) {
    let t = thickness.max(0.5);
    let inner_top = (rect.min.y + t).min(rect.max.y);
    let inner_bot = (rect.max.y - t).max(inner_top);
    push_solid_rect_mesh(mesh, Rect::from_min_max(rect.min, Pos2::new(rect.max.x, inner_top)), color);
    push_solid_rect_mesh(mesh, Rect::from_min_max(Pos2::new(rect.min.x, inner_bot), rect.max), color);
    if inner_bot > inner_top {
        let left_r = (rect.min.x + t).min(rect.max.x);
        let right_l = (rect.max.x - t).max(left_r);
        push_solid_rect_mesh(
            mesh,
            Rect::from_min_max(Pos2::new(rect.min.x, inner_top), Pos2::new(left_r, inner_bot)),
            color,
        );
        push_solid_rect_mesh(
            mesh,
            Rect::from_min_max(Pos2::new(right_l, inner_top), Pos2::new(rect.max.x, inner_bot)),
            color,
        );
    }
}

pub fn push_ellipse_fill_and_ring_mesh(
    mesh: &mut Mesh,
    center: Pos2,
    rx: f32,
    ry: f32,
    fill: Color32,
    ring_width: f32,
    ring_color: Color32,
) {
    if fill.a() > 0 {
        let c_idx = mesh.vertices.len() as u32;
        mesh.colored_vertex(center, fill);
        for &(cx, cy) in &CIRCLE16 {
            mesh.colored_vertex(Pos2::new(center.x + cx * rx, center.y + cy * ry), fill);
        }
        let ring0 = c_idx + 1;
        for i in 0..16u32 {
            let next = (i + 1) & 15;
            mesh.add_triangle(c_idx, ring0 + i, ring0 + next);
        }
    }
    if ring_width > 0.0 && ring_color.a() > 0 {
        let hw = ring_width * 0.5;
        let irx = (rx - hw).max(0.0);
        let iry = (ry - hw).max(0.0);
        let orx = rx + hw;
        let ory = ry + hw;
        let base = mesh.vertices.len() as u32;
        for &(cx, cy) in &CIRCLE16 {
            mesh.colored_vertex(Pos2::new(center.x + cx * irx, center.y + cy * iry), ring_color);
            mesh.colored_vertex(Pos2::new(center.x + cx * orx, center.y + cy * ory), ring_color);
        }
        for i in 0..16u32 {
            let next = (i + 1) & 15;
            let i0 = base + i * 2;
            let o0 = i0 + 1;
            let i1 = base + next * 2;
            let o1 = i1 + 1;
            mesh.add_triangle(i0, o0, o1);
            mesh.add_triangle(i0, o1, i1);
        }
    }
}

pub fn push_ring_stroke_mesh(
    mesh: &mut Mesh,
    center: Pos2,
    radius: f32,
    stroke_width: f32,
    color: Color32,
) {
    push_ellipse_fill_and_ring_mesh(
        mesh,
        center,
        radius,
        radius,
        Color32::TRANSPARENT,
        stroke_width,
        color,
    );
}

pub fn push_filled_circle_lut_mesh(
    mesh: &mut Mesh,
    center: Pos2,
    radius: f32,
    color: Color32,
    hi_detail: bool,
) {
    if radius <= 0.0 || color.a() == 0 {
        return;
    }
    let pts: &[(f32, f32)] = if hi_detail { &CIRCLE16 } else { &OCT8 };
    let n = pts.len() as u32;
    let c_idx = mesh.vertices.len() as u32;
    mesh.colored_vertex(center, color);
    for &(cx, cy) in pts {
        mesh.colored_vertex(Pos2::new(center.x + cx * radius, center.y + cy * radius), color);
    }
    let ring0 = c_idx + 1;
    for i in 0..n {
        let next = if i + 1 < n { i + 1 } else { 0 };
        mesh.add_triangle(c_idx, ring0 + i, ring0 + next);
    }
}

pub fn push_line_ribbon_mesh(mesh: &mut Mesh, p0: Pos2, p1: Pos2, width: f32, color: Color32) {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    let len_sq = dx * dx + dy * dy;
    if len_sq <= 1e-6 || color.a() == 0 {
        return;
    }
    let inv = (width * 0.5) / len_sq.sqrt();
    let nx = -dy * inv;
    let ny = dx * inv;
    let base = mesh.vertices.len() as u32;
    mesh.colored_vertex(Pos2::new(p0.x + nx, p0.y + ny), color);
    mesh.colored_vertex(Pos2::new(p1.x + nx, p1.y + ny), color);
    mesh.colored_vertex(Pos2::new(p1.x - nx, p1.y - ny), color);
    mesh.colored_vertex(Pos2::new(p0.x - nx, p0.y - ny), color);
    mesh.add_triangle(base, base + 1, base + 2);
    mesh.add_triangle(base, base + 2, base + 3);
}

pub fn push_polyline_ribbon_mesh(mesh: &mut Mesh, pts: &[Pos2], width: f32, color: Color32) {
    if pts.len() < 2 || color.a() == 0 {
        return;
    }
    let hw = width * 0.5;
    let base = mesh.vertices.len() as u32;
    for i in 0..pts.len() {
        let (dx, dy) = if i == 0 {
            (pts[1].x - pts[0].x, pts[1].y - pts[0].y)
        } else if i + 1 == pts.len() {
            (pts[i].x - pts[i - 1].x, pts[i].y - pts[i - 1].y)
        } else {
            (pts[i + 1].x - pts[i - 1].x, pts[i + 1].y - pts[i - 1].y)
        };
        let len_sq = dx * dx + dy * dy;
        let (nx, ny) = if len_sq > 1e-6 {
            let inv = hw / len_sq.sqrt();
            (-dy * inv, dx * inv)
        } else {
            (0.0, hw)
        };
        mesh.colored_vertex(Pos2::new(pts[i].x + nx, pts[i].y + ny), color);
        mesh.colored_vertex(Pos2::new(pts[i].x - nx, pts[i].y - ny), color);
    }
    for i in 0..(pts.len() as u32 - 1) {
        let a0 = base + i * 2;
        let b0 = a0 + 1;
        let a1 = a0 + 2;
        let b1 = a0 + 3;
        mesh.add_triangle(a0, a1, b1);
        mesh.add_triangle(a0, b1, b0);
    }
}

pub fn push_tex_quad_to_mesh(
    mesh: &mut Mesh,
    rect: Rect,
    uv_min: Pos2,
    uv_max: Pos2,
    facing: f32,
    top_skew_x: f32,
    tint: Color32,
) {
    let (u_left, u_right) = if facing >= 0.0 {
        (uv_min.x, uv_max.x)
    } else {
        (uv_max.x, uv_min.x)
    };
    let base = mesh.vertices.len() as u32;
    mesh.vertices.extend_from_slice(&[
        Vertex {
            pos: Pos2::new(rect.min.x + top_skew_x, rect.min.y),
            uv: Pos2::new(u_left, uv_min.y),
            color: tint,
        },
        Vertex {
            pos: Pos2::new(rect.max.x + top_skew_x, rect.min.y),
            uv: Pos2::new(u_right, uv_min.y),
            color: tint,
        },
        Vertex {
            pos: Pos2::new(rect.max.x, rect.max.y),
            uv: Pos2::new(u_right, uv_max.y),
            color: tint,
        },
        Vertex {
            pos: Pos2::new(rect.min.x, rect.max.y),
            uv: Pos2::new(u_left, uv_max.y),
            color: tint,
        },
    ]);
    mesh.add_triangle(base, base + 1, base + 2);
    mesh.add_triangle(base, base + 2, base + 3);
}

fn paint_tex_quad(
    painter: &Painter,
    tex: &TextureHandle,
    rect: Rect,
    uv_min: Pos2,
    uv_max: Pos2,
    facing: f32,
    top_skew_x: f32,
    tint: Color32,
) {
    let mut mesh = Mesh::with_texture(tex.id());
    push_tex_quad_to_mesh(&mut mesh, rect, uv_min, uv_max, facing, top_skew_x, tint);
    painter.add(mesh);
}

fn decode_rgba(bytes: &[u8]) -> Option<(usize, usize, Vec<u8>)> {
    let img = image::load_from_memory(bytes).ok()?;
    let rgba = img.to_rgba8();
    let w = rgba.width() as usize;
    let h = rgba.height() as usize;
    Some((w, h, rgba.into_raw()))
}

fn decode_image_texture(ctx: &Context, name: &str, bytes: &[u8]) -> Option<TextureHandle> {
    let (w, h, raw) = decode_rgba(bytes)?;
    let color_img = ColorImage::from_rgba_unmultiplied([w, h], &raw);
    Some(ctx.load_texture(name, color_img, TextureOptions::LINEAR))
}

fn pack_battle_atlas(
    ctx: &Context,
    entries: &[(&'static str, usize, usize, Vec<u8>)],
) -> (Option<TextureHandle>, HashMap<&'static str, Rect>) {
    let mut uvs = HashMap::new();
    if entries.is_empty() {
        return (None, uvs);
    }
    const ATLAS_COLS: usize = 8;
    const GUTTER: usize = 2;
    let tile_w = entries.iter().map(|(_, w, _, _)| *w).max().unwrap_or(128).max(1);
    let tile_h = entries.iter().map(|(_, _, h, _)| *h).max().unwrap_or(128).max(1);
    let cell_w = tile_w + GUTTER * 2;
    let cell_h = tile_h + GUTTER * 2;
    let rows = (entries.len() + ATLAS_COLS - 1) / ATLAS_COLS;
    let atlas_w = ATLAS_COLS * cell_w;
    let atlas_h = rows * cell_h;
    let mut atlas_raw = vec![0u8; atlas_w * atlas_h * 4];
    uvs.reserve(entries.len());

    for (idx, (key, w, h, raw)) in entries.iter().enumerate() {
        let col = idx % ATLAS_COLS;
        let row = idx / ATLAS_COLS;
        let dx = col * cell_w + GUTTER;
        let dy = row * cell_h + GUTTER;
        let src_stride = w * 4;
        let dst_stride = atlas_w * 4;
        for y in 0..*h {
            let src_start = y * src_stride;
            let dst_start = (dy + y) * dst_stride + dx * 4;
            atlas_raw[dst_start..dst_start + src_stride]
                .copy_from_slice(&raw[src_start..src_start + src_stride]);
        }
        let uv = Rect::from_min_max(
            Pos2::new(dx as f32 / atlas_w as f32, dy as f32 / atlas_h as f32),
            Pos2::new((dx + w) as f32 / atlas_w as f32, (dy + h) as f32 / atlas_h as f32),
        );
        uvs.insert(*key, uv);
    }

    let color_img = ColorImage::from_rgba_unmultiplied([atlas_w, atlas_h], &atlas_raw);
    let tex = ctx.load_texture("battle_unit_atlas", color_img, TextureOptions::LINEAR);
    (Some(tex), uvs)
}

pub fn save_color_image_png(img: &egui::ColorImage, path: &std::path::Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let w = img.width() as u32;
    let h = img.height() as u32;
    let mut raw = Vec::with_capacity((w * h * 4) as usize);
    for px in &img.pixels {
        let [r, g, b, a] = px.to_srgba_unmultiplied();
        raw.extend_from_slice(&[r, g, b, a]);
    }
    image::save_buffer(path, &raw, w, h, image::ColorType::Rgba8)
        .map_err(std::io::Error::other)
}

pub fn portrait_rect(painter: &Painter, tex: &TextureHandle, rect: Rect) {
    painter.image(
        tex.id(),
        rect,
        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
        Color32::WHITE,
    );
}

pub fn image_rect_tinted(painter: &Painter, tex: &TextureHandle, rect: Rect, tint: Color32) {
    painter.image(
        tex.id(),
        rect,
        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
        tint,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sheet_walk_cycles_across_first_row_frames() {
        let seen: Vec<usize> = (0..4)
            .map(|k| {
                let anim = AnimTiming {
                    atk_anim_timer: 0.0,
                    atk_anim_duration: 0.5,
                    is_moving: true,
                    walk_phase: (k as f32 + 0.5) * (std::f32::consts::TAU / 4.0),
                    idle_phase: 0.0,
                };
                compute_sheet_frame(anim).frame_idx
            })
            .collect();
        assert_eq!(seen, vec![0, 1, 2, 3]);
    }

    #[test]
    fn sheet_attack_progresses_across_second_row_frames() {
        let mut frames = Vec::new();
        for p in [0.05_f32, 0.25, 0.55, 0.90] {
            let dur = 1.0;
            let anim = AnimTiming {
                atk_anim_timer: dur * (1.0 - p),
                atk_anim_duration: dur,
                is_moving: true,
                walk_phase: 1.2,
                idle_phase: 0.0,
            };
            frames.push(compute_sheet_frame(anim).frame_idx);
        }
        assert_eq!(frames, vec![4, 5, 6, 7]);
    }

    #[test]
    fn original_unit_artwork_is_embedded_in_assets() {
        assert!(ASSETS_DIR.get_file("static_all_54/solaris_footman.png").is_some());
        assert!(ASSETS_DIR.get_file("static_all_54/hive_acid_spitter.png").is_some());
    }
}
