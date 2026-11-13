import re

with open(r"C:\Users\Vince\.gemini\antigravity\scratch\epic_battle_2d\index.html", "r", encoding="utf-8") as f:
    text = f.read()

idx = text.find("function updateHUD()")
end_idx = text.find("function playerTrySpawnSlot")
hud_code = text[idx:end_idx]

ids = re.findall(r'document\.getElementById\([\'"](.*?)[\'"]\)', hud_code)
unique_ids = sorted(list(set(ids)))

missing = []
for el_id in unique_ids:
    if f'id="{el_id}"' not in text:
        missing.append(el_id)

print("Total IDs in updateHUD/renderUnitCardsBar:", len(unique_ids))
print("Missing IDs:", missing)
