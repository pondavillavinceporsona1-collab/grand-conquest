import os, json, cv2, zipfile
import numpy as np
from PIL import Image

BASE_DIR = r"C:\Users\Vince\.gemini\antigravity\scratch\epic_battle_2d"
ORIG_SEP_ZIP = r"C:\Users\Vince\.gemini\antigravity\brain\088fd393-844c-4bb8-8089-3876457ef42b\.user_uploaded\media_1794192931827_dd5b5fa5.zip"
ORIG_STATIC_DIR = os.path.join(BASE_DIR, "assets", "static")

RAW_SEP_DIR = os.path.join(BASE_DIR, "assets", "raw_separated_parts")
os.makedirs(RAW_SEP_DIR, exist_ok=True)
with zipfile.ZipFile(ORIG_SEP_ZIP, 'r') as z:
    z.extractall(RAW_SEP_DIR)

# Original Colab masks for exact lossless inverse-mask reconstruction
COLAB_POLYS = {
    "head": [(0.22, 0.0), (0.78, 0.0), (0.76, 0.50), (0.24, 0.50)],
    "back_arm": [(0.0, 0.15), (0.36, 0.35), (0.36, 0.78), (0.0, 0.85)],
    "front_arm": [(0.64, 0.35), (1.0, 0.15), (1.0, 0.85), (0.64, 0.78)],
    "left_leg": [(0.20, 0.70), (0.50, 0.70), (0.50, 1.0), (0.15, 1.0)],
    "right_leg": [(0.50, 0.70), (0.80, 0.70), (0.85, 1.0), (0.50, 1.0)],
    "torso": [(0.32, 0.44), (0.68, 0.44), (0.68, 0.75), (0.32, 0.75)],
}
COLAB_BLUR = {"head": 2, "back_arm": 2, "front_arm": 2, "left_leg": 2, "right_leg": 2, "torso": 3}

w_sum = np.zeros((128, 128), dtype=np.float32)
for name, poly in COLAB_POLYS.items():
    m = np.zeros((128, 128), dtype=np.uint8)
    pts = np.array([[(int(px * 128), int(py * 128)) for px, py in poly]], dtype=np.int32)
    cv2.fillPoly(m, pts, 255)
    b = COLAB_BLUR[name]
    m = cv2.GaussianBlur(m, (b*2+1, b*2+1), 0)
    w_sum += m.astype(np.float32) / 255.0

def reconstruct_from_colab_parts(unit_dir):
    a_sum = np.zeros((128, 128), dtype=np.float32)
    rgb_num = np.zeros((128, 128, 3), dtype=np.float32)
    for name in COLAB_POLYS.keys():
        p = os.path.join(unit_dir, f"{name}.png")
        im = np.array(Image.open(p).convert("RGBA"), dtype=np.float32)
        a = im[:, :, 3]
        a_sum += a
        rgb_num += im[:, :, :3] * a[:, :, None]
    valid = w_sum > 0.02
    orig_a = np.zeros((128, 128), dtype=np.float32)
    orig_a[valid] = np.clip(a_sum[valid] / w_sum[valid], 0, 255)
    orig_rgb = np.zeros((128, 128, 3), dtype=np.float32)
    nz = a_sum > 1.0
    orig_rgb[nz] = np.clip(rgb_num[nz] / a_sum[nz, None], 0, 255)
    out = np.zeros((128, 128, 4), dtype=np.uint8)
    out[:, :, :3] = orig_rgb.astype(np.uint8)
    out[:, :, 3] = orig_a.astype(np.uint8)
    return out

# 14 spotless static sprites from epic_battle_2d_static.zip (excluding the 4 that had slate background boxes)
SPOTLESS_STATIC = {
    "abyssal_dark_sentinel", "abyssal_void_archer",
    "aegis_medic_bot", "aegis_police",
    "hellfire_imp",
    "hive_alien_grunt", "hive_alien_guard", "hive_plasma_shooter",
    "mechanica_assault_drone", "mechanica_combat_bot", "mechanica_shield_robot",
    "solaris_archer", "solaris_footman", "solaris_shield_man"
}

# Units in separated_parts that have upper-right text label remnants in the top-right corner
UNITS_WITH_TEXT_BANNER = {
    "abyssal_abyss_mage": (0.60, 0.56),
    "abyssal_abyssal_sorcerer": (0.55, 0.55),
    "abyssal_shadowling": (0.64, 0.58),
    "abyssal_soul_drainer": (0.68, 0.56),
    "abyssal_void_reaper": (0.68, 0.58),
    "abyssal_void_stalker": (0.68, 0.58),
    "abyssal_void_warden": (0.66, 0.58),
    "aegis_grenade_thrower": (0.62, 0.65),
    "aegis_riot_trooper": (0.66, 0.65),
    "aegis_shotgunner": (0.62, 0.55),
    "hellfire_blood_knight": (0.60, 0.54),
    "hellfire_demon_lord": (0.68, 0.56),
    "mechanica_cybernetic_commander": (0.64, 0.56),
    "solaris_high_priest": (0.66, 0.56),
}

def strip_top_right_banner(arr, x_thresh_norm, y_thresh_norm):
    """
    Removes white uppercase text and flat slate banner pixels in the top-right corner
    without touching the character body.
    """
    h, w = arr.shape[:2]
    rgb = arr[:, :, :3].astype(np.float32)
    a = arr[:, :, 3]
    luma = 0.299 * rgb[:, :, 0] + 0.587 * rgb[:, :, 1] + 0.114 * rgb[:, :, 2]
    sat = np.max(rgb, axis=2) - np.min(rgb, axis=2)

    # 1. Remove bright neutral-grey/white text components in top-right (cx > 69, cy < 84, msat < 14)
    text_mask = ((luma > 115) & (sat < 25) & (a > 10)).astype(np.uint8)
    num_l, labels, stats, centroids = cv2.connectedComponentsWithStats(text_mask, connectivity=8)
    for i in range(1, num_l):
        cx, cy = centroids[i]
        x0, y0, bw, bh, area = stats[i]
        msat = sat[labels == i].mean()
        if cx > 69 and cy < 84 and area < 400 and msat < 14.0:
            cm = (labels == i).astype(np.uint8)
            # Dilate by 2px to also erase the dark anti-aliased text border around each letter
            cm = cv2.dilate(cm, np.ones((5, 5), np.uint8), iterations=1)
            arr[cm > 0, 3] = 0

    # 2. Remove flat dark-slate background banner in top-right corner (y < 85)
    yy, xx = np.mgrid[0:h, 0:w]
    in_tr = (xx > int(w * x_thresh_norm)) & (yy < 85)
    slate = in_tr & (a > 0) & (sat < 28) & (luma < 140)
    arr[slate, 3] = 0

    # 3. Clean isolated specks (< 65px) floating in the upper-right banner zone
    fg = (arr[:, :, 3] > 15).astype(np.uint8)
    num_l, labels, stats, centroids = cv2.connectedComponentsWithStats(fg, connectivity=8)
    for i in range(1, num_l):
        cx, cy = centroids[i]
        area = stats[i, cv2.CC_STAT_AREA]
        if area < 65 and cx > 68 and cy < 85:
            arr[labels == i, 3] = 0

    return arr

def partition_of_unity_rig_v3(clean_rgba):
    if clean_rgba.size != (128, 128):
        canvas = Image.new("RGBA", (128, 128), (0, 0, 0, 0))
        im = clean_rgba.copy()
        im.thumbnail((96, 96), Image.Resampling.LANCZOS)
        ox = (128 - im.width) // 2
        oy = (128 - im.height) - 10
        canvas.paste(im, (ox, oy), im)
    else:
        canvas = clean_rgba.copy()

    arr = np.array(canvas)
    alpha = arr[:, :, 3]
    ys, xs = np.where(alpha > 20)
    if len(xs) == 0:
        return canvas, {}, {}

    x0, x1 = int(xs.min()), int(xs.max())
    y0, y1 = int(ys.min()), int(ys.max())
    bw = max(16, x1 - x0)
    bh = max(16, y1 - y0)

    def to_px(poly):
        pts = []
        for nx, ny in poly:
            if nx <= 0.0:
                px = 0
            elif nx >= 1.0:
                px = 127
            else:
                px = int(round(x0 + nx * bw))
            if ny <= 0.0:
                py = 0
            elif ny >= 1.0:
                py = 127
            else:
                py = int(round(y0 + ny * bh))
            pts.append((px, py))
        return np.array([pts], dtype=np.int32)

    head_poly = [
        (0.24, 0.0), (0.74, 0.0),
        (0.73, 0.40), (0.60, 0.52),
        (0.36, 0.52), (0.25, 0.40)
    ]
    torso_poly = [
        (0.36, 0.52), (0.60, 0.52),
        (0.58, 0.77), (0.49, 0.77), (0.36, 0.77)
    ]
    back_arm_poly = [
        (0.0, 0.0), (0.24, 0.0),
        (0.25, 0.40), (0.36, 0.52),
        (0.36, 0.77), (0.22, 1.0), (0.0, 1.0)
    ]
    front_arm_poly = [
        (0.74, 0.0), (1.0, 0.0),
        (1.0, 1.0), (0.74, 1.0),
        (0.58, 0.77), (0.60, 0.52), (0.73, 0.40)
    ]
    left_leg_poly = [
        (0.36, 0.77), (0.49, 0.77),
        (0.49, 1.0), (0.22, 1.0)
    ]
    right_leg_poly = [
        (0.49, 0.77), (0.58, 0.77),
        (0.74, 1.0), (0.49, 1.0)
    ]

    polys = {
        "back_arm": back_arm_poly,
        "left_leg": left_leg_poly,
        "right_leg": right_leg_poly,
        "torso": torso_poly,
        "head": head_poly,
        "front_arm": front_arm_poly,
    }

    raw_masks = {}
    mask_sum = np.zeros((128, 128), dtype=np.float32)
    for name, poly in polys.items():
        m = np.zeros((128, 128), dtype=np.uint8)
        cv2.fillPoly(m, to_px(poly), 255)
        mf = cv2.GaussianBlur(m.astype(np.float32), (5, 5), 0)
        raw_masks[name] = mf
        mask_sum += mf

    mask_sum = np.maximum(mask_sum, 1e-5)
    parts = {}
    for name, mf in raw_masks.items():
        w = mf / mask_sum
        part = arr.copy()
        part[:, :, 3] = np.clip(arr[:, :, 3].astype(np.float32) * w, 0, 255).astype(np.uint8)
        parts[name] = part

    # Shadowed joint-cushion under-layer on `torso` from `torso` + `head` interior
    core_rgb = np.maximum(parts["torso"][:, :, :3], parts["head"][:, :, :3])
    core_a = np.maximum(parts["torso"][:, :, 3], parts["head"][:, :, 3])
    k_joint = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (13, 13))
    dil_rgb = cv2.dilate(core_rgb, k_joint, iterations=1)
    dil_a = cv2.dilate(core_a, k_joint, iterations=1)
    dil_a = cv2.GaussianBlur(dil_a.astype(np.float32), (7, 7), 0)
    sil_a = (arr[:, :, 3].astype(np.float32) / 255.0)
    cushion_a = np.clip(dil_a * sil_a * 0.92, 0, 255).astype(np.uint8)
    cushion = np.zeros_like(parts["torso"])
    cushion[:, :, :3] = (dil_rgb.astype(np.float32) * 0.68).astype(np.uint8)
    cushion[:, :, 3] = cushion_a
    cushion_im = Image.fromarray(cushion)
    cushion_im.alpha_composite(Image.fromarray(parts["torso"]))
    parts["torso"] = cushion_im

    for name in parts:
        if not isinstance(parts[name], Image.Image):
            parts[name] = Image.fromarray(parts[name])

    pivots = {
        "head": (int(round(x0 + 0.48 * bw)), int(round(y0 + 0.50 * bh))),
        "torso": (int(round(x0 + 0.48 * bw)), int(round(y0 + 0.65 * bh))),
        "back_arm": (int(round(x0 + 0.34 * bw)), int(round(y0 + 0.52 * bh))),
        "front_arm": (int(round(x0 + 0.60 * bw)), int(round(y0 + 0.54 * bh))),
        "left_leg": (int(round(x0 + 0.42 * bw)), int(round(y0 + 0.77 * bh))),
        "right_leg": (int(round(x0 + 0.54 * bw)), int(round(y0 + 0.77 * bh))),
    }
    return canvas, parts, pivots

def rotate_around_pivot(img, angle_deg, pivot, offset=(0, 0)):
    px, py = pivot
    dx, dy = offset
    arr = np.array(img)
    M = cv2.getRotationMatrix2D((float(px), float(py)), float(angle_deg), 1.0)
    M[0, 2] += dx
    M[1, 2] += dy
    rotated = cv2.warpAffine(arr, M, (img.width, img.height), flags=cv2.INTER_LINEAR, borderMode=cv2.BORDER_CONSTANT, borderValue=(0,0,0,0))
    return Image.fromarray(rotated)

def render_skeletal_spritesheet(parts, pivots):
    sheet = Image.new("RGBA", (128 * 8, 128), (0, 0, 0, 0))
    # Note: In OpenCV, +angle is counter-clockwise (raises right/front arm, lowers left/back arm)
    # and -angle is clockwise (lowers right/front arm, raises left/back arm)
    pose_frames = [
        # Frame 0: Walk Pass 1 (Left leg steps forward, Right leg back, Arms counter-swing)
        {"left_leg": (18, -1, -1), "right_leg": (-18, 1, -1), "back_arm": (-15, 0, 0), "front_arm": (15, 0, 0), "torso": (-2, 0, -1), "head": (-2, 0, -2)},
        # Frame 1: Walk Contact
        {"left_leg": (6, 0, 0), "right_leg": (-6, 0, 0), "back_arm": (-5, 0, 1), "front_arm": (5, 0, 1), "torso": (0, 0, 1), "head": (0, 0, 1)},
        # Frame 2: Walk Pass 2 (Right leg steps forward, Left leg back, Arms swap swing)
        {"left_leg": (-18, 1, -1), "right_leg": (18, -1, -1), "back_arm": (15, 0, 0), "front_arm": (-15, 0, 0), "torso": (2, 0, -1), "head": (2, 0, -2)},
        # Frame 3: Walk Contact 2
        {"left_leg": (-6, 0, 0), "right_leg": (6, 0, 0), "back_arm": (5, 0, 1), "front_arm": (-5, 0, 1), "torso": (0, 0, 1), "head": (0, 0, 1)},

        # Frame 4: Attack Wind-Up (Raise BOTH weapon/shield arms high, brace legs)
        {"left_leg": (-12, -2, 0), "right_leg": (14, -1, 0), "back_arm": (-22, -2, -2), "front_arm": (26, -2, -3), "torso": (6, -2, 0), "head": (5, -2, 0)},
        # Frame 5: Attack Strike! (Slam BOTH weapon/shield arms forward & down, lunge torso)
        {"left_leg": (20, 3, 0), "right_leg": (-18, 4, -1), "back_arm": (26, 4, 2), "front_arm": (-30, 5, 3), "torso": (-8, 4, 1), "head": (-6, 4, 1)},
        # Frame 6: Attack Follow-through
        {"left_leg": (10, 2, 0), "right_leg": (-10, 2, 0), "back_arm": (13, 2, 1), "front_arm": (-15, 3, 2), "torso": (-4, 2, 1), "head": (-3, 2, 1)},
        # Frame 7: Return to Combat Ready
        {"left_leg": (0, 0, 0), "right_leg": (0, 0, 0), "back_arm": (0, 0, 0), "front_arm": (0, 0, 0), "torso": (0, 0, 0), "head": (0, 0, 0)},
    ]
    z_order = ["back_arm", "left_leg", "right_leg", "torso", "head", "front_arm"]
    for f_idx, pose in enumerate(pose_frames):
        frame_canvas = Image.new("RGBA", (128, 128), (0, 0, 0, 0))
        for part_name in z_order:
            angle, dx, dy = pose[part_name]
            transformed_part = rotate_around_pivot(parts[part_name], angle, pivots[part_name], (dx, dy))
            frame_canvas.alpha_composite(transformed_part)
        sheet.paste(frame_canvas, (f_idx * 128, 0))
    return sheet

OUT_STATIC_DIR = os.path.join(BASE_DIR, "assets", "static_all_54")
OUT_SEP_DIR = os.path.join(BASE_DIR, "assets", "separated_parts")
OUT_SHEETS_DIR = os.path.join(BASE_DIR, "assets", "limb_sheets")
os.makedirs(OUT_STATIC_DIR, exist_ok=True)
os.makedirs(OUT_SEP_DIR, exist_ok=True)
os.makedirs(OUT_SHEETS_DIR, exist_ok=True)

units = sorted([d for d in os.listdir(RAW_SEP_DIR) if os.path.isdir(os.path.join(RAW_SEP_DIR, d))])
print(f"Processing {len(units)} units...")

contact = Image.new("RGBA", (9 * 128, 6 * 128), (20, 24, 32, 255))
all_pivots = {}

for idx, u in enumerate(units):
    if u in SPOTLESS_STATIC:
        src_img = Image.open(os.path.join(ORIG_STATIC_DIR, f"{u}.png")).convert("RGBA")
    else:
        arr = reconstruct_from_colab_parts(os.path.join(RAW_SEP_DIR, u))
        if u in UNITS_WITH_TEXT_BANNER:
            xt, yt = UNITS_WITH_TEXT_BANNER[u]
            arr = strip_top_right_banner(arr, xt, yt)
        src_img = Image.fromarray(arr)

    norm_canvas, parts, pivots = partition_of_unity_rig_v3(src_img)
    all_pivots[u] = pivots

    # Save static 128x128 sprite
    norm_canvas.save(os.path.join(OUT_STATIC_DIR, f"{u}.png"))

    # Save 6 separated parts
    u_sep_dir = os.path.join(OUT_SEP_DIR, u)
    os.makedirs(u_sep_dir, exist_ok=True)
    for pname, pimg in parts.items():
        pimg.save(os.path.join(u_sep_dir, f"{pname}.png"))

    # Save 8-frame skeletal sprite sheet
    sheet = render_skeletal_spritesheet(parts, pivots)
    sheet.save(os.path.join(OUT_SHEETS_DIR, f"{u}_limbs_sheet.png"))

    r, c = idx // 9, idx % 9
    contact.paste(norm_canvas, (c * 128, r * 128), norm_canvas)

with open(os.path.join(BASE_DIR, "assets", "pivots.json"), "w") as f:
    json.dump(all_pivots, f, indent=2)

contact.save(os.path.join(BASE_DIR, "all_54_final_check.png"))
print("Finished generating all 54 static portraits, 324 separated body parts, and 54 8-frame limb sprite sheets!")
