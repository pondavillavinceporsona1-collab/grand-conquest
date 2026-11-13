import os, re

BASE_DIR = r"C:\Users\Vince\.gemini\antigravity\scratch\epic_battle_2d"
ORIG_HTML_PATH = os.path.join(BASE_DIR, "index.html")

with open(ORIG_HTML_PATH, "r", encoding="utf-8") as f:
    orig_content = f.read()

# Extract the core JavaScript game logic between `<script src="src/conquest_engine.js"></script>\s*<script>`
# and the unified conquest controller
pattern = r'<script src="src/conquest_engine\.js"></script>\s*<script>(.*?)(?:// UNIFIED GRAND CONQUEST|// Global helper: Lookup any of the 54)'
match_js = re.search(pattern, orig_content, re.DOTALL)
assert match_js, "Could not extract core game JavaScript engine"
engine_simulation_js = match_js.group(1)

# Safe replace of old skirmish button bindings
old_btn_snippet = "document.getElementById('btnOpenGrimoire').onclick = openGrimoireModal;"
if old_btn_snippet in engine_simulation_js:
    # Replace the legacy block
    legacy_lines = [
        "document.getElementById('btnOpenGrimoire').onclick = openGrimoireModal;",
        "document.getElementById('btnCloseGrimoire').onclick = closeGrimoireModal;",
        "document.getElementById('btnCancelModal').onclick = closeGrimoireModal;",
        "document.getElementById('btnRestart').onclick = startNewMatch;",
        "document.getElementById('btnPause').onclick",
        "document.getElementById('btnSpeed').onclick",
        "document.getElementById('btnRenderMode').onclick",
        "document.getElementById('btnToggleLabels').onclick",
    ]
    # We can replace the whole button binding section
    start_btn = engine_simulation_js.find("// Button bindings")
    end_btn = engine_simulation_js.find("// =========================================================================", start_btn)
    if start_btn != -1 and end_btn != -1:
        safe_btns = """// Safe Combat Button bindings
    const bUp = document.getElementById('btnUpgradeIncome'); if (bUp) bUp.onclick = playerTryUpgradeIncome;
    const bTier = document.getElementById('btnUnlockTier'); if (bTier) bTier.onclick = playerTryUnlockTier;
    const bQ = document.getElementById('btnSpellQ'); if (bQ) bQ.onclick = () => matchState && castCommanderSpellQ(matchState.player);
    const bE = document.getElementById('btnSpellE'); if (bE) bE.onclick = () => matchState && castCommanderSpellE(matchState.player);
    """
        engine_simulation_js = engine_simulation_js[:start_btn] + safe_btns + engine_simulation_js[end_btn:]

# Modernized Safe updateHUD replacement
start_hud = engine_simulation_js.find("function updateHUD()")
end_hud = engine_simulation_js.find("// =========================================================================", start_hud)
if start_hud != -1 and end_hud != -1:
    safe_update_hud = """function updateHUD() {
      if (!matchState) return;
      const state = matchState;
      const p = state.player;

      // Clock
      const mins = String(Math.floor(state.time / 60)).padStart(2, '0');
      const secs = String(Math.floor(state.time % 60)).padStart(2, '0');
      const clockEl = document.getElementById('hudClock');
      if (clockEl) clockEl.textContent = `${mins}:${secs}`;

      // Counts
      const blueCount = state.units.filter(u => u.team === 'blue' && u.hp > 0).length;
      const redCount = state.units.filter(u => u.team === 'red' && u.hp > 0).length;
      const blueEl = document.getElementById('blueCountText');
      const redEl = document.getElementById('redCountText');
      if (blueEl) blueEl.textContent = `${blueCount} Blue`;
      if (redEl) redEl.textContent = `${redCount} Red`;

      // Economy & Tech Tier
      const goldEl = document.getElementById('hudGold');
      if (goldEl) goldEl.textContent = `${Math.floor(p.gold)}G`;
      const incRateEl = document.getElementById('hudIncomeRate');
      if (incRateEl) incRateEl.textContent = `+${getIncomeRate(p.incomeLvl)}G/s`;
      const tierBadgeEl = document.getElementById('hudTierBadge');
      if (tierBadgeEl) tierBadgeEl.textContent = `T${p.tier}`;
      const incTickEl = document.getElementById('incomeTickProgress');
      if (incTickEl) incTickEl.style.width = `${Math.min(100, state.incomeTimer * 100)}%`;

      const incCost = getIncomeUpgradeCost(p.incomeLvl);
      const incLvlEl = document.getElementById('hudIncomeLvl');
      if (incLvlEl) incLvlEl.textContent = `Lvl ${p.incomeLvl}`;
      const incCostEl = document.getElementById('hudIncomeCost');
      if (incCostEl) incCostEl.textContent = `${incCost}G`;

      const tierCost = getNextTierCost(p.tier);
      const nextTierEl = document.getElementById('hudNextTierLabel');
      const tierTitleEl = document.getElementById('hudTierTitle');
      const tierCostEl = document.getElementById('hudTierCost');
      if (tierCost) {
        if (nextTierEl) nextTierEl.textContent = `Tier ${p.tier + 1}`;
        if (tierTitleEl) tierTitleEl.textContent = p.tier === 1 ? 'Veterans' : 'Mythics';
        if (tierCostEl) tierCostEl.textContent = `${tierCost}G`;
      } else {
        if (nextTierEl) nextTierEl.textContent = 'MAX';
        if (tierTitleEl) tierTitleEl.textContent = 'All Tiers';
        if (tierCostEl) tierCostEl.textContent = 'MAX';
      }

      // Update 6 Card Affordability Highlights
      p.deck.forEach((u, idx) => {
        const reqTier = u.tier === 'C' ? 1 : u.tier === 'B' ? 2 : 3;
        const stEl = document.getElementById(`cardState_${idx}`);
        const cardBtn = document.getElementById(`cardBtn_${idx}`);
        if (!stEl || !cardBtn) return;
        if (p.tier < reqTier) {
          stEl.textContent = `LOCKED T${reqTier}`;
          stEl.className = 'text-[9px] font-bold text-rose-400';
        } else if (p.gold >= u.cost) {
          stEl.textContent = 'READY';
          stEl.className = 'text-[9px] font-bold text-amber-300';
          cardBtn.classList.add('border-amber-500/50');
        } else {
          stEl.textContent = `${u.cost}G`;
          stEl.className = 'text-[9px] font-bold text-slate-500';
          cardBtn.classList.remove('border-amber-500/50');
        }
      });

      // Commander Spells [Q] & [E]
      const qName = document.getElementById('qSpellName');
      if (qName) qName.textContent = p.faction.qSpell.name;
      const eName = document.getElementById('eSpellName');
      if (eName) eName.textContent = p.faction.eSpell.name;

      const qStatus = document.getElementById('qStatusText');
      if (qStatus) qStatus.textContent = p.qCd > 0 ? `${Math.ceil(p.qCd)}s` : 'READY';
      const eStatus = document.getElementById('eStatusText');
      if (eStatus) eStatus.textContent = p.eCd > 0 ? `${Math.ceil(p.eCd)}s` : 'READY';

      const qBar = document.getElementById('qCooldownBar');
      if (qBar) qBar.style.width = p.qCd <= 0 ? '100%' : `${Math.max(0, (1 - p.qCd / p.qCdMax) * 100)}%`;
      const eBar = document.getElementById('eCooldownBar');
      if (eBar) eBar.style.width = p.eCd <= 0 ? '100%' : `${Math.max(0, (1 - p.eCd / p.eCdMax) * 100)}%`;

      // Structures Telemetry
      const pBase = state.structures.find(s => s.id === 'p_base');
      const pT2 = state.structures.find(s => s.id === 'p_t2');
      const pT1 = state.structures.find(s => s.id === 'p_t1');
      const eT1 = state.structures.find(s => s.id === 'e_t1');
      const eT2 = state.structures.find(s => s.id === 'e_t2');
      const eBase = state.structures.find(s => s.id === 'e_base');

      if (pBase) {
        const hpEl = document.getElementById('pBaseHPText');
        const barEl = document.getElementById('pBaseBar');
        if (hpEl) hpEl.textContent = `${Math.max(0, Math.ceil(pBase.hp))} / ${pBase.maxHp}`;
        if (barEl) barEl.style.width = `${Math.max(0, (pBase.hp / pBase.maxHp) * 100)}%`;
      }
      if (pT2) {
        const hpEl = document.getElementById('pT2HPText');
        if (hpEl) hpEl.textContent = `${Math.max(0, Math.ceil(pT2.hp))}`;
      }
      if (pT1) {
        const hpEl = document.getElementById('pT1HPText');
        if (hpEl) hpEl.textContent = `${Math.max(0, Math.ceil(pT1.hp))}`;
      }
      if (eT1) {
        const hpEl = document.getElementById('eT1HPText');
        if (hpEl) hpEl.textContent = `${Math.max(0, Math.ceil(eT1.hp))}`;
      }
      if (eT2) {
        const hpEl = document.getElementById('eT2HPText');
        if (hpEl) hpEl.textContent = `${Math.max(0, Math.ceil(eT2.hp))}`;
      }
      if (eBase) {
        const hpEl = document.getElementById('eBaseHPText');
        const barEl = document.getElementById('eBaseBar');
        if (hpEl) hpEl.textContent = `${Math.max(0, Math.ceil(eBase.hp))} / ${eBase.maxHp}`;
        if (barEl) barEl.style.width = `${Math.max(0, (eBase.hp / eBase.maxHp) * 100)}%`;
      }
    }
    """
    engine_simulation_js = engine_simulation_js[:start_hud] + safe_update_hud + engine_simulation_js[end_hud:]

old_card_styling = "bg-slate-900 border-blue-500/40 hover:border-blue-400"
new_card_styling = "bg-[#101624] border-amber-500/25 hover:border-amber-400/60"
engine_simulation_js = engine_simulation_js.replace(old_card_styling, new_card_styling)


# Define the new unified Screen & Conquest Controller JS
new_conquest_controller_js = """
    // =========================================================================
    // UNIFIED GRAND CONQUEST CAMPAIGN CONTROLLER & DUAL-SCREEN LIFECYCLE
    // =========================================================================
    window.getUnitDefById = function(uId) {
      for (const fac of Object.values(FACTIONS)) {
        const found = fac.units.find(u => u.id === uId);
        if (found) return found;
      }
      return null;
    };

    let conquestMapInstance = null;
    let currentAppScreen = "campaign"; // "campaign" or "battle"
    let isConquestMapOpen = true;

    window.showCampaignScreen = function() {
      currentAppScreen = "campaign";
      isConquestMapOpen = true;
      isPaused = true;

      const mapScreen = document.getElementById("screenCampaignMap");
      const battleScreen = document.getElementById("screenBattlefield");
      if (mapScreen) mapScreen.classList.remove("hidden");
      if (battleScreen) battleScreen.classList.add("hidden");

      const canvas = document.getElementById("conquestMapCanvas");
      if (canvas) {
        const rect = canvas.parentElement.getBoundingClientRect();
        canvas.width = rect.width || window.innerWidth;
        canvas.height = rect.height || (window.innerHeight - 120);
      }

      if (!conquestMapInstance && canvas) {
        conquestMapInstance = new window.ConquestMapEngine(canvas, (selectedMission) => {
          window.ConquestUI.showBriefingModal(selectedMission, (mission, saveState) => {
            window.startConquestMission(mission, saveState);
          });
        });
      }

      window.updateConquestMapHeader();
    };

    window.showBattlefieldScreen = function() {
      currentAppScreen = "battle";
      isConquestMapOpen = false;

      const mapScreen = document.getElementById("screenCampaignMap");
      const battleScreen = document.getElementById("screenBattlefield");
      if (mapScreen) mapScreen.classList.add("hidden");
      if (battleScreen) battleScreen.classList.remove("hidden");

      resizeCanvas();
      isPaused = false;
    };

    window.startConquestMission = function(mission, saveState) {
      window.showBattlefieldScreen();
      const endOverlay = document.getElementById("matchEndOverlay");
      if (endOverlay) endOverlay.classList.add("hidden");

      matchState = window.ConquestEngine.initMatch(mission, saveState);
      isPaused = false;
      renderUnitCardsBar();
      updateHUD();
    };

    window.retreatToCampaign = function() {
      if (window.ConquestUI) window.ConquestUI.hideConquestHuds();
      window.showCampaignScreen();
    };

    window.updateConquestMapHeader = function() {
      const save = window.loadConquestSave ? window.loadConquestSave() : null;
      if (!save) return;
      const starsEl = document.getElementById("mapTotalStarsText");
      const goldEl = document.getElementById("mapVaultGoldText");
      const relicsEl = document.getElementById("mapRelicsCount");

      if (starsEl) starsEl.textContent = `${save.totalConquestStars} / 72`;
      if (goldEl) goldEl.textContent = `${save.accountGoldVault || 0}G`;
      const usedRP = window.getUsedRelicPower ? window.getUsedRelicPower(save) : 0;
      if (relicsEl) relicsEl.textContent = `${usedRP} / 3 RP`;
    };

    window.initConquestDomBindings = function() {
      // Relic Vault Modal button
      const btnRelic = document.getElementById("btnOpenRelicVault");
      if (btnRelic) {
        btnRelic.onclick = () => {
          if (window.ConquestUI) {
            window.ConquestUI.showRelicVaultModal(() => {
              window.updateConquestMapHeader();
            });
          }
        };
      }

      // Deck Workshop Modal button
      const btnWorkshop = document.getElementById("btnOpenWorkshopDeck");
      if (btnWorkshop) {
        btnWorkshop.onclick = () => {
          if (window.ConquestUI) {
            window.ConquestUI.showDeckWorkshopModal(null, (m, s) => {
              if (m) window.startConquestMission(m, s);
            });
          }
        };
      }

      // Reset Campaign button
      const btnReset = document.getElementById("btnResetCampaign");
      if (btnReset) {
        btnReset.onclick = () => {
          if (confirm("Reset Campaign progress? All conquered territories and attunements will be restored to Chapter 1.")) {
            localStorage.removeItem("EB2D_GRAND_CONQUEST_SAVE_v2");
            window.showCampaignScreen();
          }
        };
      }

      // Map Zoom / Nav Controls
      const btnZoomIn = document.getElementById("btnMapZoomIn");
      if (btnZoomIn) btnZoomIn.onclick = () => conquestMapInstance && (conquestMapInstance.zoom = Math.min(1.85, conquestMapInstance.zoom * 1.2));
      const btnZoomOut = document.getElementById("btnMapZoomOut");
      if (btnZoomOut) btnZoomOut.onclick = () => conquestMapInstance && (conquestMapInstance.zoom = Math.max(0.65, conquestMapInstance.zoom * 0.8));
      const btnCenter = document.getElementById("btnMapCenter");
      if (btnCenter) btnCenter.onclick = () => conquestMapInstance && conquestMapInstance.focusChapter(1);

      // Chapter Carousel Buttons (1 to 6)
      for (let i = 1; i <= 6; i++) {
        const btnCh = document.getElementById(`btnChapterNav_${i}`);
        if (btnCh) {
          btnCh.onclick = () => conquestMapInstance && conquestMapInstance.focusChapter(i);
        }
      }

      // Retreat to War Table Button from battlefield
      const btnRetreat = document.getElementById("btnRetreatToMap");
      if (btnRetreat) btnRetreat.onclick = window.retreatToCampaign;

      const btnEndRetreat = document.getElementById("btnChangeDeckEnd");
      if (btnEndRetreat) btnEndRetreat.onclick = window.retreatToCampaign;

      const btnPlayAgain = document.getElementById("btnPlayAgain");
      if (btnPlayAgain) {
        btnPlayAgain.onclick = () => {
          const save = window.loadConquestSave();
          const currMission = window.ConquestEngine.activeMission || window.getMissionById("M1_1");
          window.startConquestMission(currMission, save);
        };
      }
    };

    window.spawnUnitEntity = spawnUnitEntity;
    window.applyDamageToUnit = applyDamageToUnit;
    window.computeUnitAttackDamage = computeUnitAttackDamage;
    window.playerTrySpawnSlot = playerTrySpawnSlot;
    window.setBanner = setBanner;
    window.updateHUD = updateHUD;
    window.renderUnitCardsBar = renderUnitCardsBar;

    window.onload = function () {
      preloadAll2DSprites();
      window.showCampaignScreen();
      if (window.initConquestDomBindings) window.initConquestDomBindings();
      requestAnimationFrame(gameLoop);
    };
"""

# Update gameLoop in engine_simulation_js so it only updates battlefield when currentAppScreen === 'battle'
old_gameloop = """    function gameLoop(now) {
      const rawDt = Math.min(0.05, (now - lastFrameTime) / 1000);
      lastFrameTime = now;
      const dt = rawDt * gameSpeed;

      updateSimulation(dt);
      renderBattlefield();

      if (isConquestMapOpen && conquestMapInstance) {
        conquestMapInstance.render(window.loadConquestSave(), rawDt);
      }
      hudAccum += rawDt;
      if (hudAccum >= 0.12) {
        hudAccum = 0;
        updateHUD();
      }

      requestAnimationFrame(gameLoop);
    }"""

new_gameloop = """    function gameLoop(now) {
      const rawDt = Math.min(0.05, (now - lastFrameTime) / 1000);
      lastFrameTime = now;
      const dt = rawDt * gameSpeed;

      if (currentAppScreen === "battle") {
        updateSimulation(dt);
        renderBattlefield();
        hudAccum += rawDt;
        if (hudAccum >= 0.12) {
          hudAccum = 0;
          updateHUD();
        }
      } else if (currentAppScreen === "campaign" && conquestMapInstance) {
        conquestMapInstance.render(window.loadConquestSave(), rawDt);
      }

      requestAnimationFrame(gameLoop);
    }"""

if old_gameloop in engine_simulation_js:
    engine_simulation_js = engine_simulation_js.replace(old_gameloop, new_gameloop, 1)
elif old_gameloop.replace('\n', '\r\n') in engine_simulation_js:
    engine_simulation_js = engine_simulation_js.replace(old_gameloop.replace('\n', '\r\n'), new_gameloop.replace('\n', '\r\n'), 1)
else:
    print("Notice: manual replace of gameLoop")

# Assemble the new complete HTML page
new_html = f"""<!DOCTYPE html>
<html lang="en" class="dark">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>GRAND CONQUEST — Tactical 2D Skeletal Warfare</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <link href="https://fonts.googleapis.com/css2?family=Cinzel:wght@600;700;800;900&family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@500;700;800&display=swap" rel="stylesheet">
  <style>
    * {{
      box-sizing: border-box;
      user-select: none;
      -webkit-user-select: none;
    }}
    body {{
      font-family: 'Inter', sans-serif;
      background-color: #07090e;
      color: #f1f5f9;
      overflow: hidden;
    }}
    .font-serif-title {{
      font-family: 'Cinzel', serif;
      letter-spacing: 0.05em;
    }}
    .font-mono-code {{
      font-family: 'JetBrains Mono', monospace;
    }}
    canvas {{
      display: block;
      width: 100%;
      height: 100%;
    }}
    .gold-gradient-text {{
      background: linear-gradient(135deg, #fce99f 0%, #d4af37 50%, #aa8420 100%);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }}
    .panel-brass {{
      background: #0d121c;
      border: 1px solid rgba(212, 175, 55, 0.22);
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.65), inset 0 1px 0 rgba(255, 255, 255, 0.04);
    }}
    .btn-gold {{
      background: linear-gradient(180deg, #d4af37 0%, #b89327 100%);
      color: #07090e;
      font-weight: 800;
      border: 1px solid #f5d77f;
      box-shadow: 0 4px 14px rgba(0, 0, 0, 0.4), inset 0 1px 0 rgba(255, 255, 255, 0.3);
      transition: all 0.15s ease;
    }}
    .btn-gold:hover:not(:disabled) {{
      background: linear-gradient(180deg, #e5c158 0%, #c8a335 100%);
      transform: translateY(-1px);
      box-shadow: 0 6px 20px rgba(184, 147, 39, 0.35);
    }}
    .btn-brass-dark {{
      background: #111724;
      border: 1px solid rgba(212, 175, 55, 0.25);
      color: #f1f5f9;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
      transition: all 0.15s ease;
    }}
    .btn-brass-dark:hover:not(:disabled) {{
      background: #192336;
      border-color: rgba(212, 175, 55, 0.5);
      color: #fce99f;
    }}
    .card-btn {{
      transition: transform 0.12s cubic-bezier(0.2, 0.8, 0.2, 1), box-shadow 0.15s ease, border-color 0.15s ease;
    }}
    .card-btn:hover:not(:disabled) {{
      transform: translateY(-2px);
    }}
    ::-webkit-scrollbar {{
      width: 6px;
      height: 6px;
    }}
    ::-webkit-scrollbar-track {{
      background: #07090e;
    }}
    ::-webkit-scrollbar-thumb {{
      background: #1e293b;
      border-radius: 9999px;
    }}
    ::-webkit-scrollbar-thumb:hover {{
      background: #334155;
    }}
  </style>
</head>
<body class="w-screen h-screen overflow-hidden bg-[#07090e] text-slate-100 flex flex-col">

  <!-- =========================================================================
       VIEW 1: GRAND CONQUEST CONTINENTAL WAR ROOM (PRIMARY ENTRY POINT)
       ========================================================================= -->
  <div id="screenCampaignMap" class="w-full h-full flex flex-col bg-[#07090e] select-none overflow-hidden">
    <!-- Top Imperial Command Bar -->
    <header class="h-16 px-4 sm:px-6 border-b border-amber-500/20 bg-[#0d121c]/95 backdrop-blur-md flex items-center justify-between z-20 shadow-xl shrink-0">
      <!-- Brand & Emblem -->
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-b from-amber-400 via-amber-600 to-amber-800 p-0.5 shadow-md shadow-amber-950/40 flex items-center justify-center">
          <span class="text-base">⚔️</span>
        </div>
        <div>
          <div class="flex items-center gap-2">
            <h1 class="font-serif-title text-base sm:text-lg font-black gold-gradient-text tracking-wider">
              GRAND CONQUEST
            </h1>
            <span class="text-[9px] font-mono-code font-bold px-2 py-0.5 rounded bg-amber-500/10 text-amber-300 border border-amber-500/30">
              GEO-TERRA WAR TABLE
            </span>
          </div>
          <div class="text-[11px] text-slate-400 hidden sm:block">
            Campaign of Liberation • 6 Sovereign Provinces • 24 Strategic Strongholds
          </div>
        </div>
      </div>

      <!-- Campaign Telemetry: Stars, Gold Vault, Relic Power -->
      <div class="flex items-center gap-2 sm:gap-3">
        <!-- Campaign Stars -->
        <div class="px-3 py-1 rounded-xl bg-[#101624] border border-amber-500/30 flex items-center gap-2 shadow-sm">
          <span class="text-amber-400 text-sm">★</span>
          <div>
            <div class="text-[8px] uppercase tracking-wider font-bold text-slate-400 leading-none">Campaign Stars</div>
            <div id="mapTotalStarsText" class="text-xs sm:text-sm font-mono-code font-black text-amber-300 leading-none mt-0.5">0 / 72</div>
          </div>
        </div>

        <!-- Gold Treasury -->
        <div class="px-3 py-1 rounded-xl bg-[#101624] border border-amber-500/30 flex items-center gap-2 shadow-sm">
          <span class="text-amber-400 text-sm">💰</span>
          <div>
            <div class="text-[8px] uppercase tracking-wider font-bold text-slate-400 leading-none">War Treasury</div>
            <div id="mapVaultGoldText" class="text-xs sm:text-sm font-mono-code font-black text-amber-300 leading-none mt-0.5">0G</div>
          </div>
        </div>

        <!-- Relic Attunement -->
        <div class="px-3 py-1 rounded-xl bg-[#101624] border border-blue-500/30 flex items-center gap-2 shadow-sm">
          <span class="text-blue-400 text-sm">⚡</span>
          <div>
            <div class="text-[8px] uppercase tracking-wider font-bold text-slate-400 leading-none">Attunement</div>
            <div id="mapRelicsCount" class="text-xs sm:text-sm font-mono-code font-black text-blue-300 leading-none mt-0.5">0 / 3 RP</div>
          </div>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-2">
        <button id="btnOpenRelicVault" class="btn-brass-dark px-3 py-1.5 rounded-xl text-xs font-bold cursor-pointer flex items-center gap-1.5">
          <span>👑</span> <span>Relic Vault</span>
        </button>
        <button id="btnOpenWorkshopDeck" class="btn-brass-dark px-3 py-1.5 rounded-xl text-xs font-bold cursor-pointer flex items-center gap-1.5">
          <span>📜</span> <span>War Workshop</span>
        </button>
        <button id="btnResetCampaign" class="p-2 rounded-xl bg-[#101624] border border-slate-800 hover:border-rose-500/40 text-slate-400 hover:text-rose-400 transition-colors text-xs cursor-pointer" title="Reset Campaign Save">
          ↺
        </button>
      </div>
    </header>

    <!-- World Map Canvas Viewport -->
    <div class="relative flex-1 w-full h-full overflow-hidden bg-[#07090e]">
      <canvas id="conquestMapCanvas" class="w-full h-full block cursor-grab"></canvas>

      <!-- Map Zoom / Nav Controls (Top Right) -->
      <div class="absolute top-4 right-4 flex flex-col gap-2 z-10">
        <button id="btnMapZoomIn" class="btn-brass-dark w-9 h-9 rounded-xl font-mono-code text-sm font-black flex items-center justify-center cursor-pointer shadow-lg">+</button>
        <button id="btnMapZoomOut" class="btn-brass-dark w-9 h-9 rounded-xl font-mono-code text-sm font-black flex items-center justify-center cursor-pointer shadow-lg">-</button>
        <button id="btnMapCenter" class="btn-brass-dark w-9 h-9 rounded-xl text-xs flex items-center justify-center cursor-pointer shadow-lg" title="Center on Active Province">🎯</button>
      </div>

      <!-- Archetype Intel Legend (Top Left) -->
      <div class="absolute top-4 left-4 p-3 rounded-2xl bg-[#0d121c]/90 border border-amber-500/20 text-[11px] space-y-1 backdrop-blur-md hidden sm:block z-10 shadow-xl">
        <div class="font-serif-title text-[9px] font-black uppercase text-amber-400 tracking-wider">Mission Archetypes</div>
        <div class="flex items-center gap-2"><span class="text-sm">⚔️</span> <span><strong>Siege:</strong> Breach Outer Outposts &rarr; Citadel</span></div>
        <div class="flex items-center gap-2"><span class="text-sm">🛡️</span> <span><strong>Hold the Line:</strong> 3-Minute Wave Defense</span></div>
        <div class="flex items-center gap-2"><span class="text-sm">⚡</span> <span><strong>Blitzkrieg:</strong> Destroy Base Before Strike</span></div>
        <div class="flex items-center gap-2"><span class="text-sm">👑</span> <span><strong>Titan Slayer:</strong> Slay Colossal Boss</span></div>
        <div class="flex items-center gap-2"><span class="text-sm">☣️</span> <span><strong>Mutator:</strong> Environmental Hazard Zone</span></div>
      </div>
    </div>

    <!-- Bottom Chapter Carousel Dock -->
    <footer class="h-14 px-4 border-t border-amber-500/20 bg-[#0d121c]/95 backdrop-blur-md flex items-center justify-between gap-2 z-20 shrink-0">
      <div class="text-[10px] font-serif-title font-bold text-amber-400 uppercase tracking-widest whitespace-nowrap mr-2 hidden md:block">
        PROVINCES:
      </div>
      <div class="flex items-center gap-2 flex-1 justify-center overflow-x-auto py-1">
        <button id="btnChapterNav_1" class="btn-brass-dark px-3 py-1.5 rounded-xl text-xs font-bold whitespace-nowrap flex items-center gap-1.5">
          <span class="text-amber-400">I</span> <span>Solaris Empire</span>
        </button>
        <button id="btnChapterNav_2" class="btn-brass-dark px-3 py-1.5 rounded-xl text-xs font-bold whitespace-nowrap flex items-center gap-1.5">
          <span class="text-blue-400">II</span> <span>Aegis Coalition</span>
        </button>
        <button id="btnChapterNav_3" class="btn-brass-dark px-3 py-1.5 rounded-xl text-xs font-bold whitespace-nowrap flex items-center gap-1.5">
          <span class="text-lime-400">III</span> <span>Hive Swarm</span>
        </button>
        <button id="btnChapterNav_4" class="btn-brass-dark px-3 py-1.5 rounded-xl text-xs font-bold whitespace-nowrap flex items-center gap-1.5">
          <span class="text-rose-400">IV</span> <span>Hellfire Crucible</span>
        </button>
        <button id="btnChapterNav_5" class="btn-brass-dark px-3 py-1.5 rounded-xl text-xs font-bold whitespace-nowrap flex items-center gap-1.5">
          <span class="text-purple-400">V</span> <span>Abyssal Void</span>
        </button>
        <button id="btnChapterNav_6" class="btn-brass-dark px-3 py-1.5 rounded-xl text-xs font-bold whitespace-nowrap flex items-center gap-1.5">
          <span class="text-teal-400">VI</span> <span>Sector Zero</span>
        </button>
      </div>
    </footer>
  </div>

  <!-- =========================================================================
       VIEW 2: CINEMATIC BATTLEFIELD THEATER
       ========================================================================= -->
  <div id="screenBattlefield" class="hidden w-full h-full flex flex-col bg-[#07090e] select-none overflow-hidden">
    <!-- Top Fortress & Objective Telemetry Bar -->
    <header class="h-16 px-3 sm:px-5 border-b border-amber-500/20 bg-[#0d121c]/95 backdrop-blur-md flex items-center justify-between gap-3 z-20 shrink-0 shadow-lg">
      <!-- Player Legion Fortification -->
      <div class="flex items-center gap-2 sm:gap-3 flex-1 min-w-0 max-w-[420px]">
        <div class="p-1.5 rounded-xl bg-blue-950/40 border border-blue-500/30 flex-1 min-w-0">
          <div class="flex items-center justify-between text-[10px] font-bold">
            <span class="text-blue-400 truncate">BLUE BASTION</span>
            <span id="pBaseStatus" class="text-[9px] font-mono-code text-blue-300">SECURE</span>
          </div>
          <div id="pBaseHPText" class="text-xs font-mono-code font-black text-white">1500 / 1500</div>
          <div class="w-full h-1.5 rounded-full bg-slate-800 overflow-hidden mt-0.5">
            <div id="pBaseBar" class="h-full bg-blue-500 transition-all" style="width: 100%"></div>
          </div>
        </div>
        <div class="hidden sm:flex gap-1.5 text-[9px] font-mono-code font-bold">
          <div class="p-1 rounded bg-[#101624] border border-blue-500/20">
            <span class="text-slate-400">T2:</span> <span id="pT2HPText" class="text-blue-300">600</span>
          </div>
          <div class="p-1 rounded bg-[#101624] border border-blue-500/20">
            <span class="text-slate-400">T1:</span> <span id="pT1HPText" class="text-blue-300">450</span>
          </div>
        </div>
      </div>

      <!-- Center Objective & Event Message -->
      <div class="flex flex-col items-center justify-center text-center px-2 flex-1 max-w-[480px]">
        <div class="flex items-center gap-2">
          <span id="liveBannerMsg" class="text-xs font-mono-code font-bold text-amber-300 truncate max-w-[320px]">
            Battle Commenced! Breach enemy defensive line!
          </span>
        </div>
        <div class="flex items-center gap-2 text-[10px] font-mono-code text-slate-400 mt-0.5">
          <span id="blueCountText" class="text-blue-400 font-bold">0 Blue</span>
          <span>vs</span>
          <span id="redCountText" class="text-rose-400 font-bold">0 Red</span>
          <span>•</span>
          <span id="hudClock" class="text-white font-bold">00:00</span>
        </div>
      </div>

      <!-- Enemy Fortress & Retreat Button -->
      <div class="flex items-center gap-2 sm:gap-3 flex-1 min-w-0 max-w-[420px] justify-end">
        <div class="hidden sm:flex gap-1.5 text-[9px] font-mono-code font-bold">
          <div class="p-1 rounded bg-[#101624] border border-rose-500/20">
            <span class="text-slate-400">T1:</span> <span id="eT1HPText" class="text-rose-300">450</span>
          </div>
          <div class="p-1 rounded bg-[#101624] border border-rose-500/20">
            <span class="text-slate-400">T2:</span> <span id="eT2HPText" class="text-rose-300">600</span>
          </div>
        </div>
        <div class="p-1.5 rounded-xl bg-rose-950/40 border border-rose-500/30 flex-1 min-w-0 max-w-[140px] sm:max-w-none">
          <div class="flex items-center justify-between text-[10px] font-bold">
            <span class="text-rose-400 truncate">RED STRONGHOLD</span>
            <span id="eBaseStatus" class="text-[9px] font-mono-code text-rose-300">ACTIVE</span>
          </div>
          <div id="eBaseHPText" class="text-xs font-mono-code font-black text-white">1500 / 1500</div>
          <div class="w-full h-1.5 rounded-full bg-slate-800 overflow-hidden mt-0.5">
            <div id="eBaseBar" class="h-full bg-rose-500 transition-all" style="width: 100%"></div>
          </div>
        </div>

        <!-- Retreat to Map Button -->
        <button id="btnRetreatToMap" class="btn-brass-dark px-2.5 py-1.5 rounded-xl text-xs font-bold text-amber-300 flex items-center gap-1 cursor-pointer" title="Retreat to War Table">
          <span>⮌</span> <span class="hidden md:inline">Retreat</span>
        </button>
      </div>
    </header>

    <!-- Center Battlefield Canvas -->
    <main class="relative flex-1 w-full h-full overflow-hidden bg-[#07090e]">
      <canvas id="battleCanvas" class="w-full h-full block"></canvas>

      <!-- Objective Telemetry Overlay (Top Left) -->
      <div id="conquestObjectiveHud" class="hidden pointer-events-none absolute top-3 left-3 px-3 py-1.5 rounded-xl bg-[#0d121c]/90 border border-amber-500/30 shadow-2xl backdrop-blur-md z-20 flex flex-col gap-0.5">
        <div class="flex items-center gap-2">
          <span id="objHudTitle" class="text-[10px] font-mono-code font-black text-amber-400 tracking-wider">OBJECTIVE</span>
          <span id="objHudTimer" class="px-1.5 py-0.2 rounded bg-amber-500/15 text-amber-300 font-mono-code font-bold text-[10px]">--:--</span>
        </div>
        <div id="objHudSub" class="text-[9px] text-slate-300 font-medium"></div>
      </div>

      <!-- Boss Bar Overlay (Top Center) -->
      <div id="conquestBossHud" class="hidden pointer-events-none absolute top-3 left-1/2 -translate-x-1/2 w-80 sm:w-96 px-3.5 py-2 rounded-2xl bg-[#0d121c]/95 border border-rose-500/50 shadow-2xl backdrop-blur-md z-20 flex flex-col gap-1.5">
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
            <span id="bossHudHpText" class="text-rose-300">3,500 / 3,500 HP</span>
          </div>
          <div class="w-full h-2 rounded-full bg-slate-800 overflow-hidden">
            <div id="bossHudHpBar" class="h-full bg-gradient-to-r from-rose-600 to-amber-400 transition-all duration-100" style="width: 100%"></div>
          </div>
          <div id="bossHudShieldWrap" class="hidden">
            <div class="w-full h-1.5 rounded-full bg-slate-800 overflow-hidden mt-0.5">
              <div id="bossHudShieldBar" class="h-full bg-blue-400 transition-all duration-100" style="width: 100%"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- Floating Tooltip -->
      <div id="canvasTooltip" class="hidden pointer-events-none absolute top-3 left-1/2 -translate-x-1/2 px-3.5 py-2 rounded-xl bg-[#0d121c]/95 border border-amber-500/30 text-xs shadow-xl backdrop-blur-md z-20"></div>

      <!-- Match End Modal -->
      <div id="matchEndOverlay" class="hidden absolute inset-0 bg-[#07090e]/85 backdrop-blur-md z-30 flex items-center justify-center p-4">
        <div class="max-w-md w-full rounded-2xl border border-amber-500/40 bg-[#0d121c] p-6 text-center shadow-2xl space-y-4">
          <div id="matchEndBadge" class="inline-block px-3 py-1 rounded-full text-xs font-mono-code font-black uppercase tracking-widest bg-amber-500/20 text-amber-300 border border-amber-500/40">
            VICTORY
          </div>
          <h2 id="matchEndTitle" class="font-serif-title text-2xl font-black text-white">Enemy Stronghold Toppled!</h2>
          <p id="matchEndDesc" class="text-xs text-slate-300"></p>
          <div id="matchEndStats" class="grid grid-cols-3 gap-2 p-3 rounded-xl bg-[#07090e] border border-slate-800 text-xs font-mono-code"></div>
          <div class="flex items-center justify-center gap-3 pt-2">
            <button id="btnPlayAgain" class="btn-gold px-5 py-2 rounded-xl text-xs uppercase cursor-pointer">Play Again</button>
            <button id="btnChangeDeckEnd" class="btn-brass-dark px-4 py-2 rounded-xl text-xs font-bold cursor-pointer">Return to Map</button>
          </div>
        </div>
      </div>
    </main>

    <!-- Bottom Commander Console -->
    <footer class="h-28 sm:h-32 px-3 sm:px-5 py-2 border-t border-amber-500/20 bg-[#0d121c]/95 backdrop-blur-md z-20 shrink-0">
      <div class="max-w-[1440px] mx-auto h-full grid grid-cols-12 gap-2 sm:gap-3 items-center">
        <!-- Economy Module (Left 3 cols) -->
        <div class="col-span-3 grid grid-cols-3 gap-1.5 sm:gap-2 h-full">
          <!-- Gold Vault -->
          <div class="p-2 rounded-xl bg-[#07090e] border border-amber-500/30 flex flex-col justify-between">
            <div class="flex items-center justify-between">
              <span class="text-[9px] font-bold uppercase text-amber-400">Gold</span>
              <span id="hudTierBadge" class="text-[9px] font-mono-code font-bold px-1 py-0.2 rounded bg-amber-500/10 text-amber-300">T1</span>
            </div>
            <div>
              <div id="hudGold" class="text-lg sm:text-xl font-mono-code font-black text-amber-400 leading-none">40G</div>
              <div id="hudIncomeRate" class="text-[10px] font-mono-code font-bold text-emerald-400 mt-0.5">+3G/s</div>
            </div>
            <div class="w-full h-1 rounded-full bg-slate-800 overflow-hidden">
              <div id="incomeTickProgress" class="h-full bg-emerald-400 transition-all duration-100" style="width: 0%"></div>
            </div>
          </div>

          <!-- Income Upgrade [U] -->
          <button id="btnUpgradeIncome" class="p-2 rounded-xl bg-[#101624] hover:bg-[#182136] border border-emerald-500/30 text-left flex flex-col justify-between cursor-pointer transition-all">
            <div class="flex items-center justify-between w-full">
              <kbd class="px-1 py-0.2 rounded bg-emerald-600 text-slate-950 font-mono-code text-[9px] font-black">[U]</kbd>
              <span id="hudIncomeLvl" class="text-[9px] font-mono-code font-bold text-emerald-300">Lvl 0</span>
            </div>
            <div>
              <div class="text-[11px] font-black text-white leading-tight">+2G/s Flow</div>
            </div>
            <div class="w-full flex items-center justify-between text-[10px] font-mono-code">
              <span id="hudIncomeCost" class="font-black text-amber-400">20G</span>
              <span class="text-[8px] font-bold text-emerald-400">UPG</span>
            </div>
          </button>

          <!-- Tech Tier [T] -->
          <button id="btnUnlockTier" class="p-2 rounded-xl bg-[#101624] hover:bg-[#182136] border border-amber-500/30 text-left flex flex-col justify-between cursor-pointer transition-all">
            <div class="flex items-center justify-between w-full">
              <kbd class="px-1 py-0.2 rounded bg-amber-500 text-slate-950 font-mono-code text-[9px] font-black">[T]</kbd>
              <span id="hudNextTierLabel" class="text-[9px] font-mono-code font-bold text-amber-300">Tier 2</span>
            </div>
            <div>
              <div id="hudTierTitle" class="text-[11px] font-black text-white leading-tight">Veterans</div>
            </div>
            <div class="w-full flex items-center justify-between text-[10px] font-mono-code">
              <span id="hudTierCost" class="font-black text-amber-400">100G</span>
              <span class="text-[8px] font-bold text-amber-300">TECH</span>
            </div>
          </button>
        </div>

        <!-- 6 Inscribed Champion Deployment Cards (Center 6 cols) -->
        <div id="unitCardsContainer" class="col-span-6 grid grid-cols-6 gap-1.5 sm:gap-2 h-full">
          <!-- Populated dynamically by JS -->
        </div>

        <!-- Commander Spells [Q] & [E] (Right 3 cols) -->
        <div class="col-span-3 grid grid-cols-2 gap-1.5 sm:gap-2 h-full">
          <!-- Spell [Q] -->
          <button id="btnSpellQ" class="p-2 rounded-xl bg-[#140b12] hover:bg-[#20101d] border border-rose-500/30 text-left flex flex-col justify-between cursor-pointer transition-all">
            <div class="flex items-center justify-between w-full">
              <kbd class="px-1 py-0.2 rounded bg-rose-600 text-white font-mono-code text-[9px] font-black">[Q]</kbd>
              <span id="qStatusText" class="text-[9px] font-mono-code font-bold text-emerald-400">READY</span>
            </div>
            <div>
              <div id="qSpellName" class="text-[11px] font-black text-white truncate">Sky-Strike</div>
              <div class="text-[9px] text-slate-400">115 AoE DMG</div>
            </div>
            <div class="w-full h-1 rounded-full bg-slate-800 overflow-hidden">
              <div id="qCooldownBar" class="h-full bg-rose-500 transition-all" style="width: 100%"></div>
            </div>
          </button>

          <!-- Spell [E] -->
          <button id="btnSpellE" class="p-2 rounded-xl bg-[#091518] hover:bg-[#102227] border border-blue-500/30 text-left flex flex-col justify-between cursor-pointer transition-all">
            <div class="flex items-center justify-between w-full">
              <kbd class="px-1 py-0.2 rounded bg-blue-600 text-white font-mono-code text-[9px] font-black">[E]</kbd>
              <span id="eStatusText" class="text-[9px] font-mono-code font-bold text-emerald-400">READY</span>
            </div>
            <div>
              <div id="eSpellName" class="text-[11px] font-black text-white truncate">Rally Surge</div>
              <div class="text-[9px] text-slate-400">+75 HP / +45% SPD</div>
            </div>
            <div class="w-full h-1 rounded-full bg-slate-800 overflow-hidden">
              <div id="eCooldownBar" class="h-full bg-blue-500 transition-all" style="width: 100%"></div>
            </div>
          </button>
        </div>
      </div>
    </footer>
  </div>

  <!-- =========================================================================
       MODALS: BRIEFING, WORKSHOP, RELIC VAULT, DEBRIEF (IMPERIAL STYLING)
       ========================================================================= -->

  <!-- 1. Conquest Briefing Modal -->
  <div id="conquestBriefingModal" class="hidden fixed inset-0 bg-[#07090e]/85 backdrop-blur-md z-50 flex items-center justify-center p-3 sm:p-6">
    <div class="max-w-xl w-full rounded-2xl border border-amber-500/30 bg-[#0d121c] shadow-2xl p-5 space-y-4">
      <div class="flex items-center justify-between">
        <span id="briefingChapterTag" class="text-[11px] font-mono-code font-bold text-amber-400 uppercase tracking-wider">CHAPTER 1</span>
        <span id="briefingArchetypeBadge" class="px-2.5 py-1 rounded-full text-[10px] font-black uppercase">SIEGE</span>
      </div>
      <div>
        <h3 id="briefingTitle" class="font-serif-title text-xl font-black text-white">The Sun-Gilded Border</h3>
        <p id="briefingNarrative" class="text-xs text-slate-300 mt-1 leading-relaxed"></p>
      </div>
      <div id="briefingObjectiveNotes" class="p-3 rounded-xl bg-[#07090e] border border-amber-500/20 text-xs text-slate-200"></div>

      <!-- 3-Star Criteria -->
      <div>
        <div class="font-serif-title text-[10px] font-black uppercase text-amber-400 tracking-wider mb-1.5">Conquest Star Commendations:</div>
        <ul id="briefingStarList" class="space-y-1"></ul>
      </div>

      <!-- Enemy Vanguard Preview -->
      <div>
        <div class="font-serif-title text-[10px] font-black uppercase text-slate-400 tracking-wider mb-1.5">Enemy Vanguard Detected:</div>
        <div id="briefingEnemyDeckPreview" class="grid grid-cols-3 gap-2"></div>
      </div>

      <!-- Buttons -->
      <div class="pt-2 border-t border-slate-800 flex items-center justify-between gap-3">
        <button id="btnBriefingCancel" class="btn-brass-dark px-4 py-2 rounded-xl text-xs font-bold text-slate-300 cursor-pointer">Cancel</button>
        <button id="btnBriefingDeploy" class="btn-gold px-5 py-2.5 rounded-xl text-slate-950 font-black text-xs uppercase tracking-wider cursor-pointer">
          Inscribe Deck &amp; Engage &rarr;
        </button>
      </div>
    </div>
  </div>

  <!-- 2. Conquest Deck Workshop Modal -->
  <div id="conquestDeckModal" class="hidden fixed inset-0 bg-[#07090e]/85 backdrop-blur-md z-50 flex items-center justify-center p-3 sm:p-6">
    <div class="max-w-4xl w-full max-h-[90vh] rounded-2xl border border-amber-500/30 bg-[#0d121c] shadow-2xl flex flex-col overflow-hidden">
      <div class="p-4 sm:px-6 border-b border-amber-500/20 flex items-center justify-between bg-[#07090e]/70">
        <div>
          <span class="text-[10px] font-mono-code font-bold uppercase text-amber-400">War Workshop: Campaign Loadout</span>
          <h3 class="font-serif-title text-lg font-black text-white">Inscribe 6 Champions (2 C-Tier, 2 B-Tier, 2 A-Tier)</h3>
        </div>
        <button id="btnWorkshopBack" class="btn-brass-dark px-3 py-1.5 rounded-xl text-xs font-bold text-slate-300 cursor-pointer">&larr; Back</button>
      </div>
      <div class="p-4 sm:p-6 overflow-y-auto space-y-5">
        <div>
          <div class="font-serif-title text-xs font-black uppercase text-amber-400 mb-2">1. Active Expeditionary Deck (6 Slots)</div>
          <div id="workshopActiveDeckSlots" class="grid grid-cols-2 sm:grid-cols-6 gap-2"></div>
        </div>
        <div>
          <div class="font-serif-title text-xs font-black uppercase text-slate-400 mb-2">2. Unlocked Blueprints Library (Click to Inscribe)</div>
          <div id="workshopBlueprintPool" class="grid grid-cols-1 sm:grid-cols-3 gap-2"></div>
        </div>
      </div>
      <div class="p-4 sm:px-6 border-t border-amber-500/20 bg-[#07090e]/80 flex items-center justify-between">
        <div class="text-xs text-slate-400">Equip cross-faction mercenaries unlocked from conquered provinces!</div>
        <button id="btnWorkshopCommence" class="btn-gold px-6 py-2.5 rounded-xl text-slate-950 font-black text-xs uppercase tracking-wider cursor-pointer">
          Deploy Formation &rarr;
        </button>
      </div>
    </div>
  </div>

  <!-- 3. Conquest Relic Vault Modal -->
  <div id="conquestRelicVaultModal" class="hidden fixed inset-0 bg-[#07090e]/85 backdrop-blur-md z-50 flex items-center justify-center p-3 sm:p-6">
    <div class="max-w-4xl w-full max-h-[90vh] rounded-2xl border border-amber-500/30 bg-[#0d121c] shadow-2xl flex flex-col overflow-hidden">
      <div class="p-4 sm:px-6 border-b border-amber-500/20 flex items-center justify-between bg-[#07090e]/70">
        <div>
          <span class="text-[10px] font-mono-code font-bold uppercase text-amber-400">Sacred War Relics</span>
          <h3 class="font-serif-title text-lg font-black text-white">Relic Vault — Artifact Attunement</h3>
        </div>
        <div class="flex items-center gap-3">
          <div class="px-3 py-1 rounded-xl bg-[#101624] border border-amber-500/30 flex items-center gap-2">
            <span class="text-amber-400 text-xs">★</span>
            <span id="relicVaultStarsCount" class="text-xs font-mono-code font-black text-white">0 / 72 ★</span>
          </div>
          <button id="btnCloseRelicVault" class="btn-brass-dark w-8 h-8 rounded-xl text-xs font-bold flex items-center justify-center cursor-pointer">✕</button>
        </div>
      </div>
      
      <!-- Relic Power Budget Meter -->
      <div class="px-4 sm:px-6 py-2.5 bg-[#07090e]/60 border-b border-amber-500/20 flex flex-col sm:flex-row sm:items-center justify-between gap-2">
        <div class="flex items-center gap-2.5">
          <span class="text-xs font-serif-title font-bold text-amber-300">ATTUNEMENT CAPACITY:</span>
          <span id="relicVaultRpBadge" class="px-2 py-0.5 rounded-full text-[10px] font-mono-code font-black bg-blue-500/15 text-blue-300 border border-blue-500/30">0 / 3 RP Equipped</span>
        </div>
        <div class="w-48 h-2 rounded-full bg-slate-800 overflow-hidden">
          <div id="relicVaultRpBar" class="h-full bg-gradient-to-r from-blue-500 to-amber-400 transition-all duration-200" style="width: 0%"></div>
        </div>
      </div>

      <div class="p-4 sm:p-6 overflow-y-auto">
        <div id="relicVaultList" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3"></div>
      </div>

      <div class="p-3 sm:px-6 border-t border-amber-500/20 bg-[#07090e]/80 flex items-center justify-between text-xs text-slate-400">
        <span>Equipped relics apply universal tactical passives to your units and fortress structures across all campaign battles.</span>
      </div>
    </div>
  </div>

  <!-- 4. Conquest Debrief Modal -->
  <div id="conquestDebriefModal" class="hidden fixed inset-0 bg-[#07090e]/85 backdrop-blur-md z-50 flex items-center justify-center p-3 sm:p-6">
    <div class="max-w-md w-full rounded-2xl border border-amber-500/30 bg-[#0d121c] shadow-2xl p-6 text-center space-y-4">
      <div id="debriefBadge" class="inline-block px-3 py-1 rounded-full text-xs font-mono-code font-black uppercase tracking-widest"></div>
      <h3 id="debriefTitle" class="font-serif-title text-2xl font-black text-white"></h3>
      <div id="debriefStarsContainer" class="flex items-center justify-center gap-2"></div>
      <div id="debriefRewardsContainer"></div>
      <div class="flex items-center justify-center gap-3 pt-2">
        <button id="btnDebriefReplay" class="btn-brass-dark px-4 py-2 rounded-xl text-xs font-bold text-white cursor-pointer">Replay</button>
        <button id="btnDebriefContinue" class="btn-gold px-5 py-2.5 rounded-xl text-slate-950 font-black text-xs uppercase tracking-wider cursor-pointer">
          Return to World Map &rarr;
        </button>
      </div>
    </div>
  </div>

  <!-- Scripts -->
  <script src="epic_battle_2d_assets_bundle.js"></script>
  <script src="src/conquest_relics.js"></script>
  <script src="src/conquest_missions.js"></script>
  <script src="src/conquest_storage.js"></script>
  <script src="src/conquest_map.js"></script>
  <script src="src/conquest_ui.js"></script>
  <script src="src/conquest_engine.js"></script>
  <script>
{engine_simulation_js}
{new_conquest_controller_js}
  </script>
</body>
</html>
"""

# Save updated index.html
with open(ORIG_HTML_PATH, "w", encoding="utf-8") as f:
    f.write(new_html)
print(f"Saved updated {ORIG_HTML_PATH} ({os.path.getsize(ORIG_HTML_PATH)/1024:.1f} KB)")

# Now build the standalone file epic_battle_2d_animated_soldiers.html
BUNDLE_JS = os.path.join(BASE_DIR, "epic_battle_2d_assets_bundle.js")
with open(BUNDLE_JS, "r", encoding="utf-8") as f:
    bundle_js_code = f.read()

src_files = [
    "conquest_relics.js",
    "conquest_missions.js",
    "conquest_storage.js",
    "conquest_map.js",
    "conquest_ui.js",
    "conquest_engine.js"
]

standalone_html = new_html
standalone_html = standalone_html.replace(
    '<script src="epic_battle_2d_assets_bundle.js"></script>',
    f"<script>\n{bundle_js_code}\n</script>",
    1
)

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
print(f"Saved updated standalone {STANDALONE_PATH} ({os.path.getsize(STANDALONE_PATH)/(1024*1024):.2f} MB)")
