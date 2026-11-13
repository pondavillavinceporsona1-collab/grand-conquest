import os

BASE_DIR = r"C:\Users\Vince\.gemini\antigravity\scratch\epic_battle_2d"
HTML_PATH = os.path.join(BASE_DIR, "index.html")

with open(HTML_PATH, "r", encoding="utf-8") as f:
    content = f.read()

hud_html = """
      <!-- Conquest Battlefield Objective HUD -->
      <div id="conquestObjectiveHud" class="hidden pointer-events-none absolute top-3 left-3 px-3 py-1.5 rounded-xl bg-slate-950/90 border border-slate-700/80 shadow-2xl backdrop-blur-md z-20 flex flex-col gap-0.5">
        <div class="flex items-center gap-2">
          <span id="objHudTitle" class="text-[10px] font-mono-code font-black text-amber-400 tracking-wider">OBJECTIVE</span>
          <span id="objHudTimer" class="px-1.5 py-0.2 rounded bg-amber-500/20 text-amber-300 font-mono-code font-bold text-[10px]">--:--</span>
        </div>
        <div id="objHudSub" class="text-[9px] text-slate-300 font-medium"></div>
      </div>

      <!-- Conquest Battlefield Boss HUD -->
      <div id="conquestBossHud" class="hidden pointer-events-none absolute top-3 left-1/2 -translate-x-1/2 w-80 sm:w-96 px-3.5 py-2 rounded-2xl bg-slate-950/92 border border-rose-500/50 shadow-2xl backdrop-blur-md z-20 flex flex-col gap-1.5">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <span class="text-rose-400 text-sm">💀</span>
            <span id="bossHudName" class="text-xs font-black text-white tracking-wide uppercase">Titan Boss</span>
          </div>
          <span id="bossHudPhase" class="px-2 py-0.5 rounded-full text-[9px] font-black uppercase bg-amber-500/20 text-amber-300 border border-amber-400/40">PHASE 1</span>
        </div>
        <div class="space-y-1">
          <div class="flex items-center justify-between text-[9px] font-mono-code font-bold">
            <span class="text-slate-400">INTEGRITY</span>
            <span id="bossHudHpText" class="text-rose-300">3,500 / 3,500 HP (100%)</span>
          </div>
          <div class="w-full h-2 rounded-full bg-slate-800 overflow-hidden">
            <div id="bossHudHpBar" class="h-full bg-gradient-to-r from-rose-600 via-orange-500 to-amber-400 transition-all duration-100" style="width: 100%"></div>
          </div>
          <div id="bossHudShieldWrap" class="hidden">
            <div class="w-full h-1.5 rounded-full bg-slate-800 overflow-hidden mt-0.5">
              <div id="bossHudShieldBar" class="h-full bg-cyan-400 transition-all duration-100" style="width: 100%"></div>
            </div>
          </div>
        </div>
      </div>
"""

target_canvas = '<canvas id="battleCanvas"></canvas>'
if 'id="conquestObjectiveHud"' not in content:
    assert target_canvas in content, 'target_canvas not found'
    content = content.replace(target_canvas, target_canvas + hud_html, 1)

# Relic Vault Modal
relic_modal_html = """
  <!-- Conquest Relic Vault Modal -->
  <div id="conquestRelicVaultModal" class="hidden fixed inset-0 bg-slate-950/85 backdrop-blur-md z-50 flex items-center justify-center p-3 sm:p-6">
    <div class="max-w-4xl w-full max-h-[90vh] rounded-2xl border border-slate-700 bg-slate-900 shadow-2xl flex flex-col overflow-hidden">
      <div class="p-4 sm:px-6 border-b border-slate-800 flex items-center justify-between bg-slate-950/70">
        <div>
          <span class="text-[10px] font-mono-code font-bold uppercase text-amber-400">Sacred War Relics</span>
          <h3 class="text-lg font-black text-white">Relic Vault — Artifact Attunement</h3>
        </div>
        <div class="flex items-center gap-3">
          <div class="px-3 py-1.5 rounded-xl bg-slate-950 border border-slate-800 flex items-center gap-2">
            <span class="text-amber-400 text-xs">★</span>
            <span id="relicVaultStarsCount" class="text-xs font-mono-code font-black text-white">0 / 72 ★</span>
          </div>
          <button id="btnCloseRelicVault" class="w-8 h-8 rounded-xl bg-slate-800 hover:bg-slate-700 text-xs font-bold text-slate-300 flex items-center justify-center cursor-pointer">✕</button>
        </div>
      </div>
      
      <!-- Relic Power Budget Meter -->
      <div class="px-4 sm:px-6 py-2.5 bg-slate-950/40 border-b border-slate-800/80 flex flex-col sm:flex-row sm:items-center justify-between gap-2">
        <div class="flex items-center gap-2.5">
          <span class="text-xs font-black text-amber-300">ATTUNEMENT CAPACITY:</span>
          <span id="relicVaultRpBadge" class="px-2 py-0.5 rounded-full text-[10px] font-mono-code font-black bg-blue-500/20 text-blue-300 border border-blue-500/40">0 / 3 RP Equipped</span>
        </div>
        <div class="w-48 h-2 rounded-full bg-slate-800 overflow-hidden">
          <div id="relicVaultRpBar" class="h-full bg-gradient-to-r from-blue-500 to-cyan-400 transition-all duration-200" style="width: 0%"></div>
        </div>
      </div>

      <div class="p-4 sm:p-6 overflow-y-auto">
        <div id="relicVaultList" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3"></div>
      </div>

      <div class="p-3 sm:px-6 border-t border-slate-800 bg-slate-950/80 flex items-center justify-between text-xs text-slate-400">
        <span>Equipped relics apply universal tactical passives to your units and fortress structures across all campaign battles.</span>
      </div>
    </div>
  </div>
"""

target_body = '</body>'
if 'id="conquestRelicVaultModal"' not in content:
    assert target_body in content, 'target_body not found'
    content = content.replace(target_body, relic_modal_html + '\n</body>', 1)

# Bindings and header updates
if 'window.updateConquestMapHeader = function()' not in content:
    old_bind = 'window.initConquestDomBindings = function() {\n      const btnOpen = document.getElementById("btnOpenConquest");'
    new_bind = '''window.updateConquestMapHeader = function() {
      const save = window.loadConquestSave ? window.loadConquestSave() : null;
      if (!save) return;
      const starsEl = document.getElementById("mapTotalStarsText");
      const badgeEl = document.getElementById("headerStarsBadge");
      const relicsEl = document.getElementById("mapRelicsCount");
      if (starsEl) starsEl.textContent = `${save.totalConquestStars} / 72`;
      if (badgeEl) badgeEl.textContent = `${save.totalConquestStars}★`;
      if (relicsEl) relicsEl.textContent = `(${save.unlockedRelicIds.length}/15)`;
    };

    window.initConquestDomBindings = function() {
      const btnRelicVault = document.getElementById("btnOpenRelicVault");
      if (btnRelicVault) {
        btnRelicVault.onclick = () => {
          if (window.ConquestUI) {
            window.ConquestUI.showRelicVaultModal(() => {
              if (window.updateConquestMapHeader) window.updateConquestMapHeader();
            });
          }
        };
      }
      const btnOpen = document.getElementById("btnOpenConquest");'''

    if old_bind in content:
        content = content.replace(old_bind, new_bind, 1)
    elif old_bind.replace('\n', '\r\n') in content:
        content = content.replace(old_bind.replace('\n', '\r\n'), new_bind.replace('\n', '\r\n'), 1)
    else:
        print("Warning: old_bind pattern not matched")

with open(HTML_PATH, "w", encoding="utf-8") as f:
    f.write(content)
print(f"Updated {HTML_PATH} successfully!")

# Now build the standalone file epic_battle_2d_animated_soldiers.html
BUNDLE_JS = os.path.join(BASE_DIR, "epic_battle_2d_assets_bundle.js")
with open(BUNDLE_JS, "r", encoding="utf-8") as f:
    bundle_js_code = f.read()

# Read the 6 conquest src files
src_files = [
    "conquest_relics.js",
    "conquest_missions.js",
    "conquest_storage.js",
    "conquest_map.js",
    "conquest_ui.js",
    "conquest_engine.js"
]

standalone_html = content
# Inline assets bundle
standalone_html = standalone_html.replace(
    '<script src="epic_battle_2d_assets_bundle.js"></script>',
    f"<script>\n{bundle_js_code}\n</script>",
    1
)

# Inline the 6 src files
for sf in src_files:
    sf_path = os.path.join(BASE_DIR, "src", sf)
    with open(sf_path, "r", encoding="utf-8") as f:
        sf_code = f.read()
    standalone_html = standalone_html.replace(
        f'<script src="src/{sf}"></script>',
        f"<script>\n// --- INLINED: src/{sf} ---\n{sf_code}\n</script>",
        1
    )

STANDALONE_PATH = os.path.join(BASE_DIR, "epic_battle_2d_animated_soldiers.html")
with open(STANDALONE_PATH, "w", encoding="utf-8") as f:
    f.write(standalone_html)
print(f"Updated {STANDALONE_PATH} successfully! ({os.path.getsize(STANDALONE_PATH)/(1024*1024):.2f} MB)")
