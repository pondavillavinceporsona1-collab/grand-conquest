// =========================================================================
// EPIC BATTLE! — GRAND CONQUEST: COMBAT SIMULATION EXTENSION & STAR AUDITOR
// =========================================================================

window.ConquestEngine = {
  activeMission: null,
  activeSaveState: null,
  activeBossEntity: null,

  initMatch(mission, saveState) {
    this.activeMission = mission;
    this.activeSaveState = saveState;
    this.activeBossEntity = null;

    // Build player deck from equipped blueprints
    const pDeck = saveState.equippedDeckUnitIds.map((uId) => window.getUnitDefById(uId)).filter(Boolean);
    const eDeck = (mission.enemyDeck || []).map((uId) => window.getUnitDefById(uId)).filter(Boolean);

    // Initial match state overrides
    const state = {
      isConquest: true,
      missionId: mission.id,
      missionArchetype: mission.archetype,
      time: 0,
      incomeTimer: 0,
      over: false,
      winner: null,
      bannerText: `Conquest Mission: ${mission.name}`,
      stats: { unitsSpawned: 0, goldEarned: mission.startingGold || 40, spellsCast: 0, enemyKills: 0 },
      activeMutator: mission.mutator || null,

      player: {
        team: "blue",
        faction: window.FACTIONS[saveState.playerCommanderFaction || "solaris"],
        deck: pDeck,
        gold: mission.startingGold || 40,
        incomeLvl: 0,
        tier: 1,
        qCd: 0,
        eCd: 0,
        qCdMax: 60.0,
        eCdMax: 60.0,
        surgeTimer: 0
      },

      ai: {
        team: "red",
        faction: window.FACTIONS[mission.enemyFaction || "solaris"],
        deck: eDeck,
        gold: mission.startingGold || 40,
        incomeLvl: 0,
        tier: 1,
        qCd: 15,
        eCd: 20,
        surgeTimer: 0,
        decisionTimer: 1.0
      },

      structures: [
        { id: "p_base", team: "blue", kind: "base", order: 3, name: "Blue Base", x: 95, y: 265, radius: 38, hp: 1500, maxHp: 1500, dmg: 28, rng: 220, cd: 1.1, atkTimer: 0 },
        { id: "p_t2", team: "blue", kind: "turret2", order: 2, name: "Blue T2", x: 245, y: 265, radius: 25, hp: 600, maxHp: 600, dmg: 20, rng: 195, cd: 1.0, atkTimer: 0 },
        { id: "p_t1", team: "blue", kind: "turret1", order: 1, name: "Blue T1", x: 410, y: 265, radius: 22, hp: 450, maxHp: 450, dmg: 16, rng: 180, cd: 0.95, atkTimer: 0 },
        { id: "e_t1", team: "red", kind: "turret1", order: 1, name: "Red T1", x: 1400 - 410, y: 265, radius: 22, hp: 450, maxHp: 450, dmg: 16, rng: 180, cd: 0.95, atkTimer: 0 },
        { id: "e_t2", team: "red", kind: "turret2", order: 2, name: "Red T2", x: 1400 - 245, y: 265, radius: 25, hp: 600, maxHp: 600, dmg: 20, rng: 195, cd: 1.0, atkTimer: 0 },
        { id: "e_base", team: "red", kind: "base", order: 3, name: "Red Base", x: 1400 - 95, y: 265, radius: 38, hp: 1500, maxHp: 1500, dmg: 28, rng: 220, cd: 1.1, atkTimer: 0 }
      ],

      units: [],
      projectiles: [],
      effects: [],
      floatingTexts: []
    };

    // Apply Active Relics (onMatchInit)
    state.activeRelics = (saveState.equippedRelicIds || [])
      .map((rId) => window.getRelicDef(rId))
      .filter(Boolean);
    state.activeRelics.forEach((relic) => {
      if (relic.onMatchInit) {
        relic.onMatchInit(state);
      }
    });

    // Hold the Line Archetype Setup
    if (mission.archetype === "hold_the_line") {
      state.defenseTimer = mission.timeLimitSeconds || 150.0;
      state.waveQueue = JSON.parse(JSON.stringify(mission.waves || []));
      const eBase = state.structures.find((s) => s.id === "e_base");
      if (eBase) eBase.isInvulnerable = true;
    }

    // Blitzkrieg Archetype Setup
    if (mission.archetype === "blitzkrieg") {
      state.blitzTimer = mission.timeLimitSeconds || 135.0;
    }

    // Titan Slayer Boss Spawn
    if (mission.archetype === "titan_slayer" && mission.bossProps) {
      const bProps = mission.bossProps;
      const baseDef = window.getUnitDefById(bProps.baseUnitId) || window.FACTIONS.solaris.units[6];
      const boss = {
        uid: 99999,
        isBoss: true,
        team: "red",
        def: baseDef,
        name: bProps.name,
        tier: "A",
        shape: "hexagon",
        x: 1400 - 150,
        y: 265,
        vx: -1,
        vy: 0,
        aimAngle: Math.PI,
        radius: 20 * (bProps.scale || 2.0),
        hp: bProps.maxHp,
        maxHp: bProps.maxHp,
        shieldHp: 0,
        phase: 1,
        dmg: bProps.dmg || 45,
        baseRngPx: (bProps.rng || 7.5) * 8.5,
        cd: bProps.cd || 1.0,
        spd: (bProps.spd || 10.0) * 3.1,
        passiveKey: "none",
        atkTimer: 0,
        shotsFired: 0,
        abilityTimer: 8.0,
        lungeOffset: 0,
        hitFlash: 0,
        walkPhase: 0,
        idlePhase: 0,
        atkAnimTimer: 0,
        atkAnimDuration: 0.45,
        isMoving: true
      };
      state.units.push(boss);
      this.activeBossEntity = boss;
    }

    if (window.ConquestUI) {
      window.ConquestUI.hideConquestHuds();
      window.ConquestUI.updateObjectiveHud(state, mission);
      if (this.activeBossEntity) {
        window.ConquestUI.updateBossHud(this.activeBossEntity);
      }
    }

    return state;
  },

  updateSimulation(state, dt) {
    if (!state || state.over) return;
    const mission = this.activeMission;
    if (!mission) return;

    // HUD Update
    if (window.ConquestUI) {
      window.ConquestUI.updateBossHud(this.activeBossEntity);
      window.ConquestUI.updateObjectiveHud(state, mission);
    }

    // 1. Hold the Line: Wave Spawner & Timer
    if (mission.archetype === "hold_the_line") {
      state.defenseTimer = Math.max(0, state.defenseTimer - dt);

      (state.waveQueue || []).forEach((wave) => {
        if (state.time >= wave.triggerSec && !wave.fired) {
          wave.fired = true;
          window.setBanner && window.setBanner(`WARNING: ${wave.alertName}!`);
          (wave.spawns || []).forEach((sp) => {
            const uDef = window.getUnitDefById(sp.unitId);
            if (uDef) {
              for (let i = 0; i < sp.count; i++) {
                window.spawnUnitEntity(state, "red", uDef, true, {
                  x: 1400 - 120 - i * 18,
                  y: sp.yOffset + (Math.random() * 20 - 10)
                });
              }
            }
          });
        }
      });

      // Win Condition: Timer expired & player base alive
      const pBase = state.structures.find((s) => s.id === "p_base");
      if (state.defenseTimer <= 0 && pBase && pBase.hp > 0) {
        state.over = true;
        state.winner = "blue";
        this.onMatchEnded(true, state);
        return;
      }
    }

    // 2. Blitzkrieg: Timer Expiration Trigger
    if (mission.archetype === "blitzkrieg") {
      state.blitzTimer = Math.max(0, state.blitzTimer - dt);
      if (state.blitzTimer <= 0) {
        state.over = true;
        state.winner = "red";
        window.setBanner && window.setBanner("BLITZKRIEG TIME EXPIRED! Satellite strike destroyed our base!");
        this.onMatchEnded(false, state);
        return;
      }
    }

    // 3. Titan Slayer: Boss Phase Controller & Ability Ticker
    if (this.activeBossEntity && this.activeBossEntity.hp > 0) {
      const boss = this.activeBossEntity;
      const hpPct = boss.hp / boss.maxHp;

      // Phase 2 Shield Trigger (at 50% HP)
      if (hpPct <= 0.50 && boss.phase === 1) {
        boss.phase = 2;
        if (mission.bossProps.hasPhase2Shield) {
          boss.shieldHp = mission.bossProps.phase2ShieldAmount || 1200;
          window.setBanner && window.setBanner(`${boss.name} ACTIVATED EMERGENCY SHIELD (+${boss.shieldHp} HP)!`);
        }
      }

      // Phase 3 Enrage Trigger (at 25% HP)
      if (hpPct <= 0.25 && boss.phase === 2) {
        boss.phase = 3;
        boss.cd = boss.cd * 0.70;
        boss.spd = boss.spd * 1.25;
        window.setBanner && window.setBanner(`${boss.name} IS ENRAGED (+35% ATK SPEED)!`);
      }

      // Boss Ability Ticker
      boss.abilityTimer -= dt;
      if (boss.abilityTimer <= 0 && mission.bossProps.telegraphedAbility) {
        const ab = mission.bossProps.telegraphedAbility;
        boss.abilityTimer = ab.cooldown || 15.0;

        // Deal AoE Damage to all blue units within radius
        state.units.forEach((u) => {
          if (u.team === "blue" && Math.hypot(u.x - boss.x, u.y - boss.y) <= (ab.radius || 150)) {
            window.applyDamageToUnit(state, u, ab.aoeDmg || 80, boss, true);
          }
        });
        window.setBanner && window.setBanner(`${boss.name} CAST ${ab.name}!`);
      }
    }

    // 4. Mutator Periodic Ticks
    if (state.activeMutator) {
      state.mutatorTickAccum = (state.mutatorTickAccum || 0) + dt;
      if (state.activeMutator.id === "corrosive_miasma" && state.mutatorTickAccum >= 1.5) {
        state.mutatorTickAccum = 0;
        state.units.forEach((u) => {
          if (u.hp > 0) {
            // Check if near healer or structure
            const nearStructure = state.structures.some((s) => s.team === u.team && s.hp > 0 && Math.hypot(s.x - u.x, s.y - u.y) <= 120);
            const nearHealer = state.units.some((h) => h.team === u.team && h.shape === "cross" && h.hp > 0 && Math.hypot(h.x - u.x, h.y - u.y) <= 110);
            if (!nearStructure && !nearHealer) {
              window.applyDamageToUnit(state, u, 2, null, false);
            }
          }
        });
      } else if (state.activeMutator.id === "blood_frenzy" && state.mutatorTickAccum >= 1.0) {
        state.mutatorTickAccum = 0;
        state.units.forEach((u) => {
          if (u.hp > 0) {
            window.applyDamageToUnit(state, u, Math.max(1, u.maxHp * 0.01), null, false);
          }
        });
      } else if (state.activeMutator.id === "nanite_regeneration" && state.mutatorTickAccum >= 3.0) {
        state.mutatorTickAccum = 0;
        state.units.forEach((u) => {
          if (u.hp > 0 && (u.shape === "square" || u.shape === "hexagon")) {
            u.hp = Math.min(u.maxHp, u.hp + u.maxHp * 0.05);
          }
        });
      }
    }

    // Standard Win/Loss Checks
    const blueBase = state.structures.find((s) => s.id === "p_base");
    const redBase = state.structures.find((s) => s.id === "e_base");

    if (blueBase && blueBase.hp <= 0 && !state.over) {
      state.over = true;
      state.winner = "red";
      this.onMatchEnded(false, state);
    } else if (redBase && redBase.hp <= 0 && !state.over) {
      if (mission.archetype === "titan_slayer" && this.activeBossEntity && this.activeBossEntity.hp > 0) {
        // Boss must also be dead
        return;
      }
      state.over = true;
      state.winner = "blue";
      this.onMatchEnded(true, state);
    }
  },

  onMatchEnded(isVictory, state) {
    if (window.ConquestUI) {
      window.ConquestUI.hideConquestHuds();
    }
    const mission = this.activeMission;
    const save = this.activeSaveState;
    if (!mission || !save) return;

    let starsEarned = 0;
    if (isVictory) {
      starsEarned = 1; // Star 1: Victory

      // Star 2: Tactical Metric
      const pT1 = state.structures.find((s) => s.id === "p_t1");
      if (pT1 && pT1.hp > 0) {
        starsEarned += 1;
      }

      // Star 3: Mastery Time Metric
      const maxTime = mission.chapter === 1 ? 180 : mission.chapter === 2 ? 165 : 195;
      if (state.time <= maxTime) {
        starsEarned += 1;
      }

      // Update Save Progress
      if (!save.missions[mission.id] || starsEarned > save.missions[mission.id].stars) {
        save.missions[mission.id] = {
          stars: starsEarned,
          bestTimeSeconds: state.time,
          completedAtIso: new Date().toISOString()
        };
      }

      // Unlock Blueprints
      (mission.rewards.blueprints || []).forEach((bId) => {
        if (!save.unlockedBlueprintIds.includes(bId)) {
          save.unlockedBlueprintIds.push(bId);
        }
      });

      // Award Bounty
      save.accountGoldVault = (save.accountGoldVault || 0) + (mission.rewards.goldBounty || 100);

      window.saveConquestProgress(save);
    }

    // Display Debrief Modal
    setTimeout(() => {
      window.ConquestUI.showDebriefModal(
        isVictory,
        starsEarned,
        mission,
        state.time,
        () => {
          // Return to World Map
          if (window.showCampaignScreen) {
            window.showCampaignScreen();
          } else if (window.openConquestWorldMap) {
            window.openConquestWorldMap();
          }
        },
        () => {
          // Replay
          window.startConquestMission(mission, save);
        }
      );
    }, 700);
  }
};
