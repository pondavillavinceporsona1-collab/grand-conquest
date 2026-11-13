"""
Batch 4x2 (8-frame progressive Walk + Attack) sprite-sheet extractor for Grand Conquest.
Takes any 16:9 white-background 4x2 sheet (<sprite_key>_sheet.jpg/png) generated from a
unit's reference portrait and outputs an aligned 512x256 RGBA sheet at:
  assets/sheets/<sprite_key>.png
"""
import argparse
import glob
import json
import os
import cv2
import numpy as np
from PIL import Image

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT_DIR = os.path.join(REPO_ROOT, "assets", "sheets")
PROMPTS_PATH = os.path.join(REPO_ROOT, "tools", "unit_sheet_prompts.json")


def load_unit_manifest():
    """Load the 54-unit prompt manifest keyed cleanly by sprite_key."""
    with open(PROMPTS_PATH, "r", encoding="utf-8") as f:
        entries = json.load(f)
    return {e["sprite_key"]: e for e in entries}


def build_generation_prompt(entry):
    """Build the standardized 16:9 4x2 white-background generation prompt for a unit."""
    return (
        f"Edit this game character into a 2D animation sprite sheet featuring the exact "
        f"{entry['faction']} unit '{entry['name']}' arranged in an exact 4 columns by 2 rows grid "
        f"(8 evenly-spaced sprite cells) on an entirely pure flat solid #FFFFFF white background. "
        f"No grid lines, no boxes, no borders, no labels, no text, no numbers, no arrows, and zero ground shadows.\n\n"
        f"STRICT CONSISTENCY & CLEANUP ACROSS ALL 8 FRAMES:\n"
        f"- Match the EXACT character silhouette, colors, armor/carapace, weapon, thick clean black ink outlines, "
        f"and soft cel-shaded art style of the reference image.\n"
        f"- IMPORTANT CLEANUP: Remove any pre-existing baked-in attack sprays, mist clouds, smoke, sparks, "
        f"or glow halos around the character in the reference image. Keep Row 1 (Walk frames 1-4) completely clean "
        f"with ONLY the character on pure #FFFFFF white; only draw attack spray/slash/projectile effects during "
        f"Row 2 Attack Frame 7.\n"
        f"- All 8 sprites MUST face RIGHT at identical scale and vertical ground line, with full feet, claws, "
        f"wheels, or legs completely visible and never cropped off.\n\n"
        f"ROW 1 (Top 4 cells left-to-right — Clean Walk / Movement Cycle, NO attack spray/effects):\n"
        f"{entry['walk_cue']}\n\n"
        f"ROW 2 (Bottom 4 cells left-to-right — Progressive Attack / Action Cycle):\n"
        f"{entry['attack_cue']}\n\n"
        f"Leave generous pure #FFFFFF white space between all 8 sprites so each cell slices cleanly."
    )


def extract_frame_on_white(cell_bgr):
    """Border-seeded flood-fill of #FFFFFF white background + enclosed pocket cleanup + de-fringe."""
    padded = cv2.copyMakeBorder(cell_bgr, 2, 2, 2, 2, cv2.BORDER_CONSTANT, value=(255, 255, 255))
    b, g, r = cv2.split(padded.astype(np.float32))
    luma = 0.114 * b + 0.587 * g + 0.299 * r
    max_c = np.maximum(np.maximum(r, g), b)
    min_c = np.minimum(np.minimum(r, g), b)
    sat = max_c - min_c

    is_bg = (luma >= 238.0) & (sat <= 18.0)
    _, labels = cv2.connectedComponents(is_bg.astype(np.uint8), connectivity=4)
    bg_mask = labels == labels[0, 0]

    # Clear enclosed white pockets inside arm/leg loops while preserving tiny highlight glints (<14px)
    is_pure_white = (luma >= 246.0) & (sat <= 10.0) & (~bg_mask)
    pw_num, pw_labels, pw_stats, _ = cv2.connectedComponentsWithStats(is_pure_white.astype(np.uint8), connectivity=4)
    for i in range(1, pw_num):
        area = pw_stats[i, cv2.CC_STAT_AREA]
        if 14 <= area <= 260:
            bg_mask[pw_labels == i] = True

    fg_mask = (~bg_mask).astype(np.uint8)
    fg_num, fg_labels, fg_stats, _ = cv2.connectedComponentsWithStats(fg_mask, connectivity=8)
    if fg_num > 1:
        areas = fg_stats[1:, cv2.CC_STAT_AREA]
        max_area = areas.max()
        keep = np.zeros_like(fg_mask)
        for idx, area in enumerate(areas, start=1):
            if area >= max(20, max_area * 0.008):
                keep[fg_labels == idx] = 1
        fg_mask = keep
        bg_mask = fg_mask == 0

    alpha = (fg_mask * 255).astype(np.float32)
    dilated = cv2.dilate(bg_mask.astype(np.uint8), np.ones((3, 3), np.uint8), iterations=1) > 0
    edge_ring = dilated & (fg_mask > 0) & (luma > 180.0) & (sat < 28.0)
    alpha[edge_ring] = np.clip((248.0 - luma[edge_ring]) / 68.0, 0.0, 1.0) * 255.0

    blurred = cv2.GaussianBlur(alpha, (3, 3), 0.7)
    alpha = np.where((alpha < 250.0) | edge_ring, blurred, 255.0)
    alpha = np.clip(alpha, 0, 255).astype(np.uint8)

    rgb = cv2.cvtColor(padded[2:-2, 2:-2], cv2.COLOR_BGR2RGB).copy()
    a = alpha[2:-2, 2:-2]
    fringe = (a > 10) & (a < 230) & (luma[2:-2, 2:-2] > 200)
    rgb[fringe] = np.clip(rgb[fringe].astype(np.float32) * 0.45, 0, 255).astype(np.uint8)
    return np.dstack([rgb, a])


def _find_min_ink_split(profile, start, end):
    """Find center of the cleanest empty/low-ink gutter inside [start, end)."""
    seg = profile[start:end]
    if len(seg) == 0:
        return (start + end) // 2
    min_val = int(seg.min())
    mins = np.where(seg <= min_val + max(1, int(seg.max() * 0.01)))[0]
    return int(start + np.median(mins))


def _cell_bounds_from_grid(bgr, rows=2, cols=4):
    """Find true whitespace gutter splits around nominal grid lines so shifted rows are never sliced in half."""
    h, w = bgr.shape[:2]
    b, g, r = cv2.split(bgr.astype(np.float32))
    luma = 0.114 * b + 0.587 * g + 0.299 * r
    sat = np.maximum(np.maximum(r, g), b) - np.minimum(np.minimum(r, g), b)
    ink = ((luma < 235.0) | (sat > 20.0)).astype(np.int32)

    if rows == 2:
        row_prof = ink.sum(axis=1)
        y_mid = _find_min_ink_split(row_prof, int(h * 0.28), int(h * 0.78))
        y_splits = [0, y_mid, h]
    else:
        y_splits = [0, h]

    cells = []
    for r_idx in range(rows):
        y0, y1 = y_splits[r_idx], y_splits[r_idx + 1]
        col_prof = ink[y0:y1, :].sum(axis=0)
        xs = [0]
        for c in range(1, cols):
            nom = int(w * (c / cols))
            pad = int(w * 0.10)
            xs.append(_find_min_ink_split(col_prof, max(1, nom - pad), min(w - 1, nom + pad)))
        xs.append(w)
        for c_idx in range(cols):
            cells.append((xs[c_idx], y0, xs[c_idx + 1], y1))
    return cells


def _pack_8_frames_rgba(raw_frames, out_path, frame_sz=128):
    """Align and normalize 8 extracted RGBA frames into a 512x256 sheet."""
    bboxes, body_centers_x, walk_heights = [], [], []
    for idx, rgba in enumerate(raw_frames):
        ys, xs = np.where(rgba[:, :, 3] > 20)
        if len(xs) == 0:
            bboxes.append((0, 0, 1, 1))
            body_centers_x.append(rgba.shape[1] * 0.5)
        else:
            fx0, fx1 = int(xs.min()), int(xs.max()) + 1
            fy0, fy1 = int(ys.min()), int(ys.max()) + 1
            bboxes.append((fx0, fy0, fx1, fy1))
            foot_band_y = fy0 + int((fy1 - fy0) * 0.45)
            _, low_xs = np.where(rgba[foot_band_y:fy1, :, 3] > 40)
            bx = float(np.median(low_xs)) if len(low_xs) > 0 else (fx0 + fx1) * 0.5
            body_centers_x.append(bx)
            if idx < 4:
                walk_heights.append(fy1 - fy0)

    ref_h = float(np.median(walk_heights)) if walk_heights else 200.0
    base_scale = (frame_sz * 0.68) / max(1.0, ref_h)

    sheet = Image.new("RGBA", (frame_sz * 4, frame_sz * 2), (0, 0, 0, 0))
    anchor_y_bottom = int(round(frame_sz * 0.94))
    anchor_x_center = frame_sz // 2

    for idx, rgba in enumerate(raw_frames):
        fx0, fy0, fx1, fy1 = bboxes[idx]
        crop = rgba[fy0:fy1, fx0:fx1]
        pil_crop = Image.fromarray(crop)
        cw_px, ch_px = pil_crop.size
        max_w_scale = (frame_sz * 0.94) / max(1, cw_px)
        max_h_scale = (frame_sz * 0.88) / max(1, ch_px)
        sc = min(base_scale, max_w_scale, max_h_scale) if idx >= 4 else base_scale

        nw, nh = max(1, int(round(cw_px * sc))), max(1, int(round(ch_px * sc)))
        resized = pil_crop.resize((nw, nh), Image.Resampling.LANCZOS)
        bx_local = (body_centers_x[idx] - fx0) * sc
        dest_x = max(2, min(frame_sz - nw - 2, int(round(anchor_x_center - bx_local))))
        dest_y = max(2, anchor_y_bottom - nh)
        sheet.paste(resized, ((idx % 4) * frame_sz + dest_x, (idx // 4) * frame_sz + dest_y), resized)

    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    sheet.save(out_path, optimize=True)
    print(f"Saved {out_path} ({os.path.getsize(out_path) // 1024} KB)")


def slice_sheet_4x2(src_path, out_path, frame_sz=128):
    bgr = cv2.imread(src_path)
    if bgr is None:
        raise RuntimeError(f"Could not read image: {src_path}")
    cells = _cell_bounds_from_grid(bgr, rows=2, cols=4)
    raw_frames = [extract_frame_on_white(bgr[y0:y1, x0:x1]) for (x0, y0, x1, y1) in cells]
    _pack_8_frames_rgba(raw_frames, out_path, frame_sz=frame_sz)


def assemble_strips_4x1(walk_src, atk_src, out_path, frame_sz=128):
    """Combine separate 4x1 Walk and 4x1 Attack horizontal strips into one aligned 512x256 sheet."""
    w_bgr = cv2.imread(walk_src)
    a_bgr = cv2.imread(atk_src)
    if w_bgr is None or a_bgr is None:
        raise RuntimeError(f"Could not read strips: {walk_src}, {atk_src}")
    raw_frames = []
    for bgr in (w_bgr, a_bgr):
        for (x0, y0, x1, y1) in _cell_bounds_from_grid(bgr, rows=1, cols=4):
            raw_frames.append(extract_frame_on_white(bgr[y0:y1, x0:x1]))
    _pack_8_frames_rgba(raw_frames, out_path, frame_sz=frame_sz)


def validate_sheet(path, frame_sz=128):
    """Verify an output sprite sheet is 512x256 RGBA with all 8 non-empty cells."""
    if not os.path.isfile(path):
        return False, "missing"
    with Image.open(path) as im:
        if im.size != (frame_sz * 4, frame_sz * 2) or im.mode != "RGBA":
            return False, f"bad format {im.size} {im.mode}"
        arr = np.array(im)
    empty = [
        idx
        for idx in range(8)
        if np.count_nonzero(
            arr[(idx // 4) * frame_sz : (idx // 4 + 1) * frame_sz, (idx % 4) * frame_sz : (idx % 4 + 1) * frame_sz, 3] > 20
        )
        < 25
    ]
    if empty:
        return False, f"empty cells {empty}"
    return True, "ok"


def batch_slice(raw_dir, overwrite=False):
    """Slice all matching raw 4x2 sheets inside `raw_dir` into assets/sheets/."""
    manifest = load_unit_manifest()
    processed, skipped = 0, 0
    for key in manifest:
        out_path = os.path.join(OUT_DIR, f"{key}.png")
        if os.path.isfile(out_path) and not overwrite:
            skipped += 1
            continue
        candidates = []
        for pat in (f"{key}.*", f"{key}_sheet.*"):
            candidates.extend(glob.glob(os.path.join(raw_dir, pat)))
        src = next((p for p in candidates if os.path.splitext(p)[1].lower() in (".png", ".jpg", ".jpeg", ".webp")), None)
        if src:
            slice_sheet_4x2(src, out_path)
            processed += 1
    print(f"Batch complete: {processed} sliced, {skipped} already existed.")


def audit_sheets():
    """Report valid vs missing sprite sheets across all 54 units."""
    manifest = load_unit_manifest()
    valid, bad_or_missing = [], []
    for key in manifest:
        ok, reason = validate_sheet(os.path.join(OUT_DIR, f"{key}.png"))
        (valid if ok else bad_or_missing).append((key, reason))
    print(f"Sprite Sheet Coverage: {len(valid)}/{len(manifest)} ready")
    if bad_or_missing:
        print("Pending units:", ", ".join(k for k, _ in bad_or_missing))
    return len(bad_or_missing) == 0


def main():
    parser = argparse.ArgumentParser(description="Slice raw 4x2 unit animation sheets into assets/sheets/<sprite_key>.png")
    parser.add_argument("--input", help="Single raw 16:9 4x2 sheet image path")
    parser.add_argument("--unit", help="Unit sprite_key when using --input (e.g. solaris_footman)")
    parser.add_argument("--batch-dir", help="Directory containing <sprite_key>.* or <sprite_key>_sheet.* images")
    parser.add_argument("--overwrite", action="store_true", help="Re-slice existing assets/sheets/<key>.png files")
    parser.add_argument("--audit", action="store_true", help="Check 54-unit sheet readiness and print coverage")
    args = parser.parse_args()

    if args.input:
        if not args.unit:
            parser.error("--unit is required with --input")
        slice_sheet_4x2(args.input, os.path.join(OUT_DIR, f"{args.unit}.png"))
    elif args.batch_dir:
        batch_slice(args.batch_dir, overwrite=args.overwrite)
        audit_sheets()
    elif args.audit:
        audit_sheets()
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
