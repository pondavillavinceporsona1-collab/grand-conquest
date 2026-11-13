with open(r"C:\Users\Vince\.gemini\antigravity\scratch\epic_battle_2d\index.html", "r", encoding="utf-8") as f:
    for idx, line in enumerate(f):
        s = line.strip()
        tags = ["<header", "<main", "<section", 'id="battleCanvas"', 'id="conquestMap', 'id="conquestBriefing', 'id="conquestDeck', 'id="conquestRelic', 'id="conquestDebrief', 'id="grimoireModal']
        for k in tags:
            if k in s:
                print(f"{idx+1}: {s[:90]}")
                break
