// =========================================================================
// EPIC BATTLE! — GRAND CONQUEST: 15 WAR RELICS CATALOG & MATH HOOKS
// =========================================================================

window.CONQUEST_RELICS = [
  {
    id: "treasury_seal",
    name: "Treasury Seal",
    starsReq: 5,
    rpCost: 1,
    icon: "💰",
    desc: "Starting gold increased from 40G to 65G (+25G initial vault).",
    onMatchInit: (state) => {
      state.player.gold += 25;
      state.stats.goldEarned += 25;
    }
  },
  {
    id: "rapid_banner",
    name: "Rapid Deployment Banner",
    starsReq: 10,
    rpCost: 1,
    icon: "🚩",
    desc: "Unit movement speed across all tiers increased by +12%.",
    onSpawnUnit: (unit) => {
      unit.spd = unit.spd * 1.12;
    }
  },
  {
    id: "overcharged_conduits",
    name: "Overcharged Conduits",
    starsReq: 15,
    rpCost: 2,
    icon: "⚡",
    desc: "Commander Spells [Q] and [E] cooldowns reduced from 60s to 48s (-20%).",
    onMatchInit: (state) => {
      state.player.qCdMax = 48.0;
      state.player.eCdMax = 48.0;
    }
  },
  {
    id: "titanium_fort",
    name: "Titanium Fortification",
    starsReq: 20,
    rpCost: 2,
    icon: "🏰",
    desc: "Base Tower HP increased from 1500 to 1850; Turrets gain +15% firing range.",
    onMatchInit: (state) => {
      const pBase = state.structures.find((s) => s.id === "p_base");
      if (pBase) {
        pBase.maxHp = 1850;
        pBase.hp = 1850;
      }
      state.structures.forEach((s) => {
        if (s.team === "blue" && (s.kind === "turret1" || s.kind === "turret2")) {
          s.rng = s.rng * 1.15;
        }
      });
    }
  },
  {
    id: "vampiric_stone",
    name: "Vampiric Bloodstone",
    starsReq: 25,
    rpCost: 2,
    icon: "🩸",
    desc: "All friendly melee units (<= 7.5 studs) restore +10 HP on strike.",
    onMeleeStrike: (attacker) => {
      attacker.hp = Math.min(attacker.maxHp, attacker.hp + 10);
    }
  },
  {
    id: "arcane_magnifier",
    name: "Arcane Magnifier",
    starsReq: 30,
    rpCost: 1,
    icon: "🔮",
    desc: "All Diamond shapes (Mages & Artillery) deal +15% damage.",
    onComputeDamage: (attacker, baseDmg) => {
      if (attacker.shape === "diamond") {
        return Math.round(baseDmg * 1.15);
      }
      return baseDmg;
    }
  },
  {
    id: "ballistic_lens",
    name: "Ballistic Compensator",
    starsReq: 35,
    rpCost: 1,
    icon: "🎯",
    desc: "All Triangle shapes (Marksmen) gain +4.0 studs (+34px) firing range.",
    onSpawnUnit: (unit) => {
      if (unit.shape === "triangle") {
        unit.baseRngPx += 34;
      }
    }
  },
  {
    id: "sacred_phylactery",
    name: "Sacred Phylactery",
    starsReq: 40,
    rpCost: 2,
    icon: "✨",
    desc: "All Cross shape (Healer) wave heals grant a temporary +25 HP barrier for 4s.",
    onHealAlly: (target) => {
      target.shieldHp = (target.shieldHp || 0) + 25;
      target.shieldTimer = 4.0;
    }
  },
  {
    id: "chitin_carapace",
    name: "Chitin-Plated Carapace",
    starsReq: 45,
    rpCost: 2,
    icon: "🛡️",
    desc: "All Square shapes (Tanks & Blockers) take 3 less damage from every hit.",
    onTakeDamage: (target, incomingDmg) => {
      if (target.shape === "square") {
        return Math.max(1, incomingDmg - 3);
      }
      return incomingDmg;
    }
  },
  {
    id: "void_singularity",
    name: "Singularity Heart",
    starsReq: 50,
    rpCost: 2,
    icon: "🌌",
    desc: "Enemies below 15% Max HP are instantly executed (excluding Titans).",
    onDamageEnemy: (target) => {
      if (!target.isBoss && target.hp > 0 && target.hp / target.maxHp <= 0.15) {
        target.hp = 0;
      }
    }
  },
  {
    id: "nanite_injector",
    name: "Nanite Emergency Injector",
    starsReq: 55,
    rpCost: 2,
    icon: "💉",
    desc: "Friendly units dropping below 25% HP instantly restore 30% Max HP (once per life).",
    onTakeDamage: (target, incomingDmg) => {
      const remainingHp = target.hp - incomingDmg;
      if (remainingHp > 0 && remainingHp <= target.maxHp * 0.25 && !target.naniteUsed) {
        target.naniteUsed = true;
        const healAmt = Math.round(target.maxHp * 0.30);
        target.hp = Math.min(target.maxHp, target.hp + healAmt);
        return 0; // absorb remainder of this hit
      }
      return incomingDmg;
    }
  },
  {
    id: "economic_monopoly",
    name: "Economic Monopoly",
    starsReq: 60,
    rpCost: 1,
    icon: "📈",
    desc: "Permanent 25% discount on [U] Income Upgrade costs.",
    onGetIncomeCost: (baseCost) => {
      return Math.round(baseCost * 0.75);
    }
  },
  {
    id: "orbital_lens",
    name: "Targeting Satellite Matrix",
    starsReq: 64,
    rpCost: 1,
    icon: "🛰️",
    desc: "Commander Spell [Q] AoE radius increased by +25% (170px radius blast).",
    onCastQRadius: (baseRadius) => {
      return baseRadius * 1.25;
    }
  },
  {
    id: "surge_amplifier",
    name: "Tactical Surge Amplifier",
    starsReq: 68,
    rpCost: 2,
    icon: "🔥",
    desc: "Commander Spell [E] heals +100 HP (up from 75) and surges +65% speed (up from +45%).",
    onCastE: (spellParams) => {
      spellParams.heal = 100;
      spellParams.surgeSpeed = 1.65;
    }
  },
  {
    id: "crown_geo_terra",
    name: "Crown of Geo-Terra",
    starsReq: 72,
    rpCost: 2,
    icon: "👑",
    desc: "Ultimate 100% Mastery: All friendly units gain +10% HP, +10% DMG, and +10% Speed!",
    onSpawnUnit: (unit) => {
      unit.maxHp = Math.round(unit.maxHp * 1.10);
      unit.hp = unit.maxHp;
      unit.dmg = Math.round(unit.dmg * 1.10);
      unit.spd = unit.spd * 1.10;
    }
  }
];

window.getRelicDef = function (id) {
  return window.CONQUEST_RELICS.find((r) => r.id === id) || null;
};
