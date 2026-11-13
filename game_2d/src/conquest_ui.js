// =========================================================================
// EPIC BATTLE! — GRAND CONQUEST: MODALS & DECK WORKSHOP UI CONTROLLER
// Imperial War Table & Tactical Dispatch Styling
// =========================================================================

window.ConquestUI = {
  currentMission: null,

  // 1. Mission Briefing Modal (Imperial War Dispatch)
  showBriefingModal(mission, onDeployClicked) {
    this.currentMission = mission;
    const modal = document.getElementById("conquestBriefingModal");
    if (!modal) return;

    document.getElementById("briefingChapterTag").textContent = `PROVINCE ${mission.chapter} • ${mission.territoryName.toUpperCase()}`;
    document.getElementById("briefingTitle").textContent = mission.name;
    document.getElementById("briefingNarrative").textContent = mission.briefing;

    // Archetype Badge
    const archBadge = document.getElementById("briefingArchetypeBadge");
    archBadge.textContent = mission.archetype.toUpperCase().replace("_", " ");
    archBadge.className = `px-3 py-1 rounded-full text-[10px] font-mono-code font-black uppercase tracking-wider ${
      mission.archetype === "titan_slayer"
        ? "bg-amber-500/15 text-amber-300 border border-amber-500/40"
        : mission.archetype === "hold_the_line"
        ? "bg-blue-500/15 text-blue-300 border border-blue-500/40"
        : mission.archetype === "blitzkrieg"
        ? "bg-rose-500/15 text-rose-300 border border-rose-500/40"
        : mission.archetype === "mutator"
        ? "bg-purple-500/15 text-purple-300 border border-purple-500/40"
        : "bg-emerald-500/15 text-emerald-300 border border-emerald-500/40"
    }`;

    // Objective & Mutator Notes
    const notesEl = document.getElementById("briefingObjectiveNotes");
    if (mission.archetype === "hold_the_line") {
      notesEl.innerHTML = `<span class="text-amber-400 font-bold">Strategic Objective:</span> Hold the line and survive continuous assault waves for <strong>${Math.floor(mission.timeLimitSeconds / 60)}m ${mission.timeLimitSeconds % 60}s</strong>.`;
    } else if (mission.archetype === "blitzkrieg") {
      notesEl.innerHTML = `<span class="text-rose-400 font-bold">Blitzkrieg Order:</span> Destroy the enemy base fortress before the <strong>${Math.floor(mission.timeLimitSeconds / 60)}m ${mission.timeLimitSeconds % 60}s</strong> orbital strike timer concludes!`;
    } else if (mission.archetype === "titan_slayer") {
      notesEl.innerHTML = `<span class="text-amber-400 font-bold">Colossal Threat:</span> Slay <strong>${mission.bossProps.name}</strong> (${mission.bossProps.maxHp} HP, Scale ${mission.bossProps.scale}x).`;
    } else if (mission.mutator) {
      notesEl.innerHTML = `<span class="text-purple-300 font-bold">Environmental Hazard:</span> <strong>${mission.mutator.name}</strong> — ${mission.mutator.desc}`;
    } else {
      notesEl.innerHTML = `<span class="text-amber-400 font-bold">Siege Objective:</span> Destroy Outer Turret 1 &rarr; Inner Turret 2 &rarr; 1,500 HP Stronghold Base.`;
    }

    // 3 Stars Criteria List
    const starListEl = document.getElementById("briefingStarList");
    starListEl.innerHTML = "";
    mission.starCriteria.forEach((crit) => {
      const li = document.createElement("li");
      li.className = "flex items-center gap-2.5 text-xs text-slate-300";
      li.innerHTML = `<span class="text-amber-400 font-black text-sm">★ ${crit.stars}</span> <span>${crit.desc}</span>`;
      starListEl.appendChild(li);
    });

    // Enemy Vanguard Preview
    const enemyDeckEl = document.getElementById("briefingEnemyDeckPreview");
    enemyDeckEl.innerHTML = "";
    (mission.enemyDeck || []).forEach((uId) => {
      const uDef = window.getUnitDefById ? window.getUnitDefById(uId) : null;
      if (uDef) {
        const item = document.createElement("div");
        item.className = "flex items-center gap-2 p-1.5 rounded-xl bg-[#07090e] border border-slate-800 text-[10px]";
        item.innerHTML = `
          <img src="assets/static_all_54/${uDef.spriteKey}.png" class="w-6 h-6 object-contain" />
          <div class="truncate">
            <span class="font-bold text-white block truncate">${uDef.name}</span>
            <span class="text-[9px] text-slate-400">${uDef.tier}-Tier • ${uDef.shape.toUpperCase()}</span>
          </div>
        `;
        enemyDeckEl.appendChild(item);
      }
    });

    // Deploy Button Binding
    document.getElementById("btnBriefingDeploy").onclick = () => {
      modal.classList.add("hidden");
      this.showDeckWorkshopModal(mission, onDeployClicked);
    };
    document.getElementById("btnBriefingCancel").onclick = () => {
      modal.classList.add("hidden");
    };

    modal.classList.remove("hidden");
  },

  // 2. Deck Workshop Modal (Inscribing 2C, 2B, 2A + Sockets)
  showDeckWorkshopModal(mission, onStartBattle) {
    const modal = document.getElementById("conquestDeckModal");
    if (!modal) return;
    const save = window.loadConquestSave();

    const renderDeckSlots = () => {
      const deckContainer = document.getElementById("workshopActiveDeckSlots");
      deckContainer.innerHTML = "";

      save.equippedDeckUnitIds.forEach((uId, idx) => {
        const uDef = window.getUnitDefById(uId);
        const reqTier = idx < 2 ? "C" : idx < 4 ? "B" : "A";
        const slotEl = document.createElement("div");
        slotEl.className = "p-2.5 rounded-xl bg-[#07090e] border border-amber-500/30 flex flex-col justify-between";
        slotEl.innerHTML = `
          <div class="flex items-center justify-between text-[10px]">
            <span class="font-bold text-amber-400">Slot [${idx + 1}]</span>
            <span class="px-1.5 py-0.2 rounded bg-amber-500/10 text-amber-300 font-mono-code font-bold">${reqTier}-Tier</span>
          </div>
          <div class="flex items-center gap-2 my-1.5">
            <img src="assets/static_all_54/${uDef.spriteKey}.png" class="w-8 h-8 object-contain" />
            <div class="min-w-0">
              <div class="text-xs font-black text-white truncate">${uDef.name}</div>
              <div class="text-[9px] text-amber-300 font-mono-code">${uDef.cost}G • ${uDef.shape.toUpperCase()}</div>
            </div>
          </div>
          <div class="text-[8.5px] text-slate-400 truncate">${uDef.passiveDesc}</div>
        `;
        deckContainer.appendChild(slotEl);
      });
    };

    const renderBlueprintPool = () => {
      const poolContainer = document.getElementById("workshopBlueprintPool");
      poolContainer.innerHTML = "";

      save.unlockedBlueprintIds.forEach((uId) => {
        const uDef = window.getUnitDefById(uId);
        if (!uDef) return;

        const card = document.createElement("button");
        card.className = "p-2 rounded-xl bg-[#0a0e17] hover:bg-[#121927] border border-slate-800 hover:border-amber-500/30 text-left flex items-center justify-between gap-2 cursor-pointer transition-all";
        card.onclick = () => {
          // Swap into appropriate tier slot
          const targetIndices = uDef.tier === "C" ? [0, 1] : uDef.tier === "B" ? [2, 3] : [4, 5];
          const currSlot = targetIndices[0];
          save.equippedDeckUnitIds[currSlot] = uDef.id;
          window.saveConquestProgress(save);
          renderDeckSlots();
        };

        card.innerHTML = `
          <div class="flex items-center gap-2">
            <img src="assets/static_all_54/${uDef.spriteKey}.png" class="w-7 h-7 object-contain" />
            <div>
              <div class="text-xs font-bold text-white">${uDef.name} <span class="text-amber-400 font-mono-code">(${uDef.cost}G)</span></div>
              <div class="text-[9px] text-slate-400">${uDef.tier}-Tier • ${uDef.passiveDesc}</div>
            </div>
          </div>
          <span class="text-[9px] font-bold text-amber-300 px-2 py-0.5 rounded bg-amber-500/10 border border-amber-500/20">EQUIP</span>
        `;
        poolContainer.appendChild(card);
      });
    };

    renderDeckSlots();
    renderBlueprintPool();

    document.getElementById("btnWorkshopCommence").onclick = () => {
      modal.classList.add("hidden");
      if (onStartBattle) onStartBattle(mission || this.currentMission, save);
    };
    document.getElementById("btnWorkshopBack").onclick = () => {
      modal.classList.add("hidden");
      if (mission) {
        this.showBriefingModal(mission, onStartBattle);
      }
    };

    modal.classList.remove("hidden");
  },

  // 3. Relic Vault Modal (15 Artifacts, RP Budget Management)
  showRelicVaultModal(onClose) {
    const modal = document.getElementById("conquestRelicVaultModal");
    if (!modal) return;
    const save = window.loadConquestSave();

    const renderVault = () => {
      const listEl = document.getElementById("relicVaultList");
      if (!listEl) return;
      listEl.innerHTML = "";

      const currentStars = save.totalConquestStars || 0;
      const usedRP = window.getUsedRelicPower ? window.getUsedRelicPower(save) : 0;
      const maxRP = 3;

      const starsCountEl = document.getElementById("relicVaultStarsCount");
      if (starsCountEl) starsCountEl.textContent = `${currentStars} / 72 ★`;

      const rpBadgeEl = document.getElementById("relicVaultRpBadge");
      if (rpBadgeEl) rpBadgeEl.textContent = `${usedRP} / ${maxRP} RP Equipped`;

      const rpBarEl = document.getElementById("relicVaultRpBar");
      if (rpBarEl) {
        rpBarEl.style.width = `${Math.min(100, (usedRP / maxRP) * 100)}%`;
      }

      (window.CONQUEST_RELICS || []).forEach((relic) => {
        const isUnlocked = (save.unlockedRelicIds || []).includes(relic.id) || currentStars >= relic.starsReq;
        const isEquipped = (save.equippedRelicIds || []).includes(relic.id);

        const card = document.createElement("div");
        card.className = `p-3.5 rounded-2xl border transition-all flex flex-col justify-between ${
          isEquipped
            ? "bg-[#181f2f] border-amber-500 shadow-lg shadow-amber-950/20"
            : isUnlocked
            ? "bg-[#0d121c] border-slate-800 hover:border-amber-500/30"
            : "bg-[#07090e] border-slate-900 opacity-55"
        }`;

        card.innerHTML = `
          <div>
            <div class="flex items-center justify-between mb-2">
              <div class="flex items-center gap-2.5">
                <span class="text-2xl">${relic.icon}</span>
                <div>
                  <h4 class="text-xs font-black text-white">${relic.name}</h4>
                  <div class="text-[9px] font-mono-code ${isUnlocked ? 'text-amber-400' : 'text-slate-500'} font-bold">
                    ${isUnlocked ? 'ATTUNED' : `Requires ${relic.starsReq} ★`}
                  </div>
                </div>
              </div>
              <span class="px-2 py-0.5 rounded-full text-[9px] font-mono-code font-black ${
                relic.rpCost === 1 ? 'bg-blue-500/15 text-blue-300 border border-blue-500/30' : 'bg-purple-500/15 text-purple-300 border border-purple-500/30'
              }">${relic.rpCost} RP</span>
            </div>
            <p class="text-[10px] text-slate-300 leading-relaxed mb-3">${relic.desc}</p>
          </div>
          <div>
            ${
              !isUnlocked
                ? `<button disabled class="w-full py-2 rounded-xl bg-slate-900/60 text-[10px] font-bold text-slate-600 cursor-not-allowed">Locked (${relic.starsReq - currentStars} ★ needed)</button>`
                : isEquipped
                ? `<button class="btn-unequip w-full py-2 rounded-xl bg-amber-500/20 hover:bg-amber-500/30 border border-amber-500/40 text-[10px] font-black text-amber-300 uppercase tracking-wider cursor-pointer">Attuned (Click to Remove)</button>`
                : usedRP + relic.rpCost > maxRP
                ? `<button disabled class="w-full py-2 rounded-xl bg-slate-900/60 text-[10px] font-bold text-slate-500 cursor-not-allowed">RP Capacity Full (+${relic.rpCost} RP)</button>`
                : `<button class="btn-equip w-full py-2 rounded-xl btn-gold text-[10px] font-black uppercase tracking-wider cursor-pointer">Attune (+${relic.rpCost} RP)</button>`
            }
          </div>
        `;

        const equipBtn = card.querySelector(".btn-equip");
        if (equipBtn) {
          equipBtn.onclick = () => {
            if (usedRP + relic.rpCost <= maxRP) {
              if (!save.equippedRelicIds) save.equippedRelicIds = [];
              save.equippedRelicIds.push(relic.id);
              window.saveConquestProgress(save);
              renderVault();
              window.updateConquestMapHeader && window.updateConquestMapHeader();
            }
          };
        }

        const unequipBtn = card.querySelector(".btn-unequip");
        if (unequipBtn) {
          unequipBtn.onclick = () => {
            save.equippedRelicIds = (save.equippedRelicIds || []).filter((id) => id !== relic.id);
            window.saveConquestProgress(save);
            renderVault();
            window.updateConquestMapHeader && window.updateConquestMapHeader();
          };
        }

        listEl.appendChild(card);
      });
    };

    renderVault();

    const btnClose = document.getElementById("btnCloseRelicVault");
    if (btnClose) {
      btnClose.onclick = () => {
        modal.classList.add("hidden");
        if (onClose) onClose();
      };
    }

    modal.classList.remove("hidden");
  },

  // 4. Battlefield Boss HUD Overlay
  updateBossHud(boss) {
    const bossHud = document.getElementById("conquestBossHud");
    if (!bossHud) return;

    if (!boss || boss.hp <= 0) {
      bossHud.classList.add("hidden");
      return;
    }

    bossHud.classList.remove("hidden");
    const nameEl = document.getElementById("bossHudName");
    const phaseEl = document.getElementById("bossHudPhase");
    const hpTextEl = document.getElementById("bossHudHpText");
    const hpBarEl = document.getElementById("bossHudHpBar");
    const shieldBarEl = document.getElementById("bossHudShieldBar");
    const shieldWrapEl = document.getElementById("bossHudShieldWrap");

    if (nameEl) nameEl.textContent = boss.name;
    if (phaseEl) {
      phaseEl.textContent = boss.phase === 3 ? "PHASE 3: ENRAGED" : boss.phase === 2 ? "PHASE 2: SHIELDED" : "PHASE 1";
      phaseEl.className = `px-2.5 py-0.5 rounded-full text-[9px] font-black uppercase ${
        boss.phase === 3
          ? "bg-rose-600 text-white animate-pulse"
          : boss.phase === 2
          ? "bg-blue-500/20 text-blue-300 border border-blue-400/40"
          : "bg-amber-500/20 text-amber-300 border border-amber-400/40"
      }`;
    }

    const hpPct = Math.max(0, Math.min(100, (boss.hp / boss.maxHp) * 100));
    if (hpTextEl) hpTextEl.textContent = `${Math.ceil(boss.hp)} / ${boss.maxHp} HP (${Math.round(hpPct)}%)`;
    if (hpBarEl) hpBarEl.style.width = `${hpPct}%`;

    if (boss.shieldHp && boss.shieldHp > 0) {
      if (shieldWrapEl) shieldWrapEl.classList.remove("hidden");
      if (shieldBarEl) shieldBarEl.style.width = `${Math.min(100, (boss.shieldHp / (boss.maxHp * 0.4)) * 100)}%`;
    } else {
      if (shieldWrapEl) shieldWrapEl.classList.add("hidden");
    }
  },

  // 5. Battlefield Objective & Hazard HUD Overlay
  updateObjectiveHud(state, mission) {
    const objHud = document.getElementById("conquestObjectiveHud");
    if (!objHud) return;

    if (!mission || !state || !state.isConquest) {
      objHud.classList.add("hidden");
      return;
    }

    objHud.classList.remove("hidden");
    const titleEl = document.getElementById("objHudTitle");
    const timerEl = document.getElementById("objHudTimer");
    const subEl = document.getElementById("objHudSub");

    if (mission.archetype === "hold_the_line") {
      const remaining = Math.max(0, Math.ceil(state.defenseTimer || 0));
      const mins = Math.floor(remaining / 60);
      const secs = remaining % 60;
      if (titleEl) titleEl.textContent = "HOLD THE LINE";
      if (timerEl) timerEl.textContent = `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
      const wavesRemaining = (state.waveQueue || []).filter((w) => !w.fired).length;
      if (subEl) subEl.textContent = `Incoming Waves: ${wavesRemaining} remaining`;
    } else if (mission.archetype === "blitzkrieg") {
      const remaining = Math.max(0, Math.ceil(state.blitzTimer || 0));
      const mins = Math.floor(remaining / 60);
      const secs = remaining % 60;
      if (titleEl) titleEl.textContent = "BLITZKRIEG TIME";
      if (timerEl) timerEl.textContent = `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
      if (subEl) subEl.textContent = "Demolish enemy base before orbital beam strikes!";
    } else if (mission.archetype === "titan_slayer") {
      if (titleEl) titleEl.textContent = "TITAN SLAYER";
      if (timerEl) timerEl.textContent = "COLOSSUS ENGAGED";
      if (subEl) subEl.textContent = "Slay the titan before our frontline collapses!";
    } else if (mission.mutator) {
      if (titleEl) titleEl.textContent = `MUTATOR: ${mission.mutator.name.toUpperCase()}`;
      if (timerEl) timerEl.textContent = "HAZARD ACTIVE";
      if (subEl) subEl.textContent = mission.mutator.desc;
    } else {
      if (titleEl) titleEl.textContent = "CAMPAIGN SIEGE";
      if (timerEl) timerEl.textContent = "BREAK DEFENSES";
      if (subEl) subEl.textContent = "Destroy Outer Turret 1 → Inner Turret 2 → Base Tower";
    }
  },

  hideConquestHuds() {
    const bossHud = document.getElementById("conquestBossHud");
    const objHud = document.getElementById("conquestObjectiveHud");
    if (bossHud) bossHud.classList.add("hidden");
    if (objHud) objHud.classList.add("hidden");
  },

  // 6. Victory / Defeat Debrief Modal
  showDebriefModal(isVictory, stars, mission, matchTime, onContinue, onReplay) {
    const modal = document.getElementById("conquestDebriefModal");
    if (!modal) return;

    const badge = document.getElementById("debriefBadge");
    const title = document.getElementById("debriefTitle");
    const starsContainer = document.getElementById("debriefStarsContainer");
    const rewardsContainer = document.getElementById("debriefRewardsContainer");

    if (isVictory) {
      badge.textContent = `TERRITORY LIBERATED • ${stars} STARS`;
      badge.className = "inline-block px-3 py-1 rounded-full text-xs font-mono-code font-black uppercase tracking-widest bg-amber-500/15 text-amber-300 border border-amber-500/40";
      title.textContent = "Victory Acclaimed!";
      
      starsContainer.innerHTML = `
        <span class="text-3xl ${stars >= 1 ? "text-amber-400" : "text-slate-700"}">★</span>
        <span class="text-3xl ${stars >= 2 ? "text-amber-400" : "text-slate-700"}">★</span>
        <span class="text-3xl ${stars >= 3 ? "text-amber-400" : "text-slate-700"}">★</span>
      `;

      rewardsContainer.innerHTML = `
        <div class="p-3 rounded-xl bg-[#07090e] border border-amber-500/30 text-xs text-left space-y-1">
          <div class="text-[10px] text-slate-400 uppercase font-bold">Campaign Commendations:</div>
          <div class="text-amber-300 font-bold">+${mission.rewards.goldBounty || 100}G Vault Treasury Bounty</div>
          ${(mission.rewards.blueprints || []).map(b => `<div class="text-emerald-300 font-bold">Unlocked Blueprint: ${b}</div>`).join("")}
        </div>
      `;
    } else {
      badge.textContent = "EXPEDITION REPELLED";
      badge.className = "inline-block px-3 py-1 rounded-full text-xs font-mono-code font-black uppercase tracking-widest bg-rose-500/15 text-rose-300 border border-rose-500/40";
      title.textContent = "Vanguard Shattered!";
      starsContainer.innerHTML = `<span class="text-sm font-bold text-slate-400">0 Stars Awarded</span>`;
      rewardsContainer.innerHTML = `<div class="text-xs text-slate-400">Re-evaluate your 6-card Grimoire loadout or attune defense relics in the Vault.</div>`;
    }

    document.getElementById("btnDebriefContinue").onclick = () => {
      modal.classList.add("hidden");
      if (onContinue) onContinue();
    };
    document.getElementById("btnDebriefReplay").onclick = () => {
      modal.classList.add("hidden");
      if (onReplay) onReplay();
    };

    modal.classList.remove("hidden");
  }
};
