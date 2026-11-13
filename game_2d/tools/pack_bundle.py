import os, io, json, base64, zipfile
from PIL import Image

BASE_DIR = r"C:\Users\Vince\.gemini\antigravity\scratch\epic_battle_2d"
STATIC_DIR = os.path.join(BASE_DIR, "assets", "static_all_54")
SEP_DIR = os.path.join(BASE_DIR, "assets", "separated_parts")
SHEETS_DIR = os.path.join(BASE_DIR, "assets", "limb_sheets")
PIVOTS_PATH = os.path.join(BASE_DIR, "assets", "pivots.json")

with open(PIVOTS_PATH, "r") as f:
    pivots = json.load(f)

z_order = ["back_arm", "left_leg", "right_leg", "torso", "head", "front_arm"]
portraits_b64 = {}
strips_b64 = {}

units = sorted(pivots.keys())
for u in units:
    # 1. Static portrait (128x128 optimized PNG)
    p_im = Image.open(os.path.join(STATIC_DIR, f"{u}.png"))
    buf_p = io.BytesIO()
    p_im.save(buf_p, format="PNG", optimize=True)
    portraits_b64[u] = "data:image/png;base64," + base64.b64encode(buf_p.getvalue()).decode("ascii")

    # 2. 6-part horizontal rig strip (768x128: back_arm, left_leg, right_leg, torso, head, front_arm)
    strip = Image.new("RGBA", (128 * 6, 128), (0, 0, 0, 0))
    for idx, pname in enumerate(z_order):
        part_im = Image.open(os.path.join(SEP_DIR, u, f"{pname}.png"))
        strip.paste(part_im, (idx * 128, 0))
    buf_s = io.BytesIO()
    strip.save(buf_s, format="PNG", optimize=True)
    strips_b64[u] = "data:image/png;base64," + base64.b64encode(buf_s.getvalue()).decode("ascii")

# Write JS bundle file that can be loaded via <script src="epic_battle_2d_assets_bundle.js"> or inlined
bundle_js_path = os.path.join(BASE_DIR, "epic_battle_2d_assets_bundle.js")
with open(bundle_js_path, "w", encoding="utf-8") as f:
    f.write("// Auto-generated 54-Unit 2D Skeletal Rig & Portrait Asset Bundle\n")
    f.write("window.EB2D_PART_ORDER = " + json.dumps(z_order) + ";\n")
    f.write("window.EB2D_PIVOTS = " + json.dumps(pivots) + ";\n")
    f.write("window.EB2D_PORTRAITS = " + json.dumps(portraits_b64) + ";\n")
    f.write("window.EB2D_RIG_STRIPS = " + json.dumps(strips_b64) + ";\n")

print(f"Wrote {bundle_js_path} ({os.path.getsize(bundle_js_path)/1024:.1f} KB)")

# Also create updated ZIP archives of the cleaned separated_parts and limb_sheets
sep_zip_path = os.path.join(BASE_DIR, "epic_battle_separated_bodyparts_fixed.zip")
with zipfile.ZipFile(sep_zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
    for u in units:
        for pname in z_order:
            full_p = os.path.join(SEP_DIR, u, f"{pname}.png")
            zf.write(full_p, f"{u}/{pname}.png")
    zf.write(PIVOTS_PATH, "pivots.json")

sheets_zip_path = os.path.join(BASE_DIR, "epic_battle_limb_spritesheets_fixed.zip")
with zipfile.ZipFile(sheets_zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
    for u in units:
        full_p = os.path.join(SHEETS_DIR, f"{u}_limbs_sheet.png")
        zf.write(full_p, f"{u}_limbs_sheet.png")

print(f"Created {sep_zip_path} and {sheets_zip_path}")
