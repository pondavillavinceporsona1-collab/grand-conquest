// =========================================================================
// EPIC BATTLE! — GRAND CONQUEST: 24-MISSION MASTER SPECIFICATION & QUEUES
// =========================================================================

window.CONQUEST_MISSIONS = [
  // -----------------------------------------------------------------------
  // CHAPTER 1: THE SUN-GILDED PLAINS (SOLARIS EMPIRE)
  // -----------------------------------------------------------------------
  {
    id: "M1_1",
    chapter: 1,
    name: "The Sun-Gilded Border",
    territoryName: "Solaris Outpost Alpha",
    pinCoord: [220, 720],
    prerequisites: [],
    archetype: "siege",
    enemyFaction: "solaris",
    aiDifficulty: "Easy",
    startingGold: 40,
    enemyDeck: ["sol_1", "sol_2", "sol_3", "sol_1", "sol_2", "sol_3"],
    briefing: "Our expedition has made landfall on the golden shores of the Solaris Empire. Their border sentries have raised their banners and refuse our parley. Secure the border gate and demonstrate the strength of our legion.",
    starCriteria: [
      { stars: 1, desc: "Demolish the enemy Base Tower." },
      { stars: 2, desc: "Outer Turret 1 (T1) remains standing with >= 50% HP (225+ HP)." },
      { stars: 3, desc: "Conclude the match in under 3:00 minutes (180.0s)." }
    ],
    rewards: {
      blueprints: ["sol_4"], // Unlocks Solaris Knight
      goldBounty: 100,
      unlocksMissions: ["M1_2"]
    }
  },
  {
    id: "M1_2",
    chapter: 1,
    name: "The Gilded Abbey",
    territoryName: "Sanctuary of Saint Lyra",
    pinCoord: [410, 680],
    prerequisites: ["M1_1"],
    archetype: "hold_the_line",
    enemyFaction: "solaris",
    aiDifficulty: "Normal",
    startingGold: 50,
    enemyDeck: ["sol_1", "sol_2", "sol_3", "sol_4", "sol_6", "sol_6"],
    briefing: "The Solaris Clergy has rallied their knights to reclaim the Abbey. Our forward outpost is cut off from reinforcements! Dig in and hold the perimeter for 2 minutes and 30 seconds until the supply zeppelin arrives!",
    timeLimitSeconds: 150.0,
    playerIncomeBonusRate: 3.0,
    waves: [
      { triggerSec: 10, alertName: "Footman Vanguard", spawns: [{ unitId: "sol_1", count: 4, yOffset: 150 }] },
      { triggerSec: 35, alertName: "Shield & Archer Volley", spawns: [{ unitId: "sol_3", count: 2, yOffset: 200 }, { unitId: "sol_2", count: 3, yOffset: 320 }] },
      { triggerSec: 65, alertName: "Knight Holy Retinue", spawns: [{ unitId: "sol_4", count: 3, yOffset: 220 }, { unitId: "sol_6", count: 2, yOffset: 280 }] },
      { triggerSec: 100, alertName: "Mass Footman Charge", spawns: [{ unitId: "sol_1", count: 6, yOffset: 180 }, { unitId: "sol_2", count: 4, yOffset: 340 }] },
      { triggerSec: 130, alertName: "FINAL CRUSADE", spawns: [{ unitId: "sol_4", count: 4, yOffset: 210 }, { unitId: "sol_6", count: 2, yOffset: 265 }, { unitId: "sol_3", count: 4, yOffset: 320 }] }
    ],
    starCriteria: [
      { stars: 1, desc: "Survive the full 2:30 minutes without Base Tower falling." },
      { stars: 2, desc: "Outer Turret 1 (T1) remains standing at 00:00." },
      { stars: 3, desc: "Eliminate at least 35 enemy invaders during the siege." }
    ],
    rewards: {
      blueprints: ["sol_6"], // Unlocks Solaris Priest + Tier 2 Tech
      featureUnlock: "tier2",
      goldBounty: 150,
      unlocksMissions: ["M1_3"]
    }
  },
  {
    id: "M1_3",
    chapter: 1,
    name: "Ramparts of the Sun",
    territoryName: "The Sunken Bastion",
    pinCoord: [630, 750],
    prerequisites: ["M1_2"],
    archetype: "mutator",
    enemyFaction: "solaris",
    aiDifficulty: "Normal",
    startingGold: 60,
    enemyDeck: ["sol_1", "sol_3", "sol_4", "sol_5", "sol_6", "sol_6"],
    briefing: "Sacred sunlight beams bathe the Sunken Bastion. The glare weakens ranged projectile accuracy, but divine holy energies empower all healers on the field!",
    mutator: {
      id: "consecrated_solar",
      name: "Consecrated Solar Conduits",
      desc: "All Cross shape heals pulse +50% extra healing. All Triangle shape ranged damage reduced by -20%."
    },
    starCriteria: [
      { stars: 1, desc: "Destroy the Sunken Bastion Base Tower." },
      { stars: 2, desc: "Reach and maintain an Income Level >= 5 before winning." },
      { stars: 3, desc: "Win without deploying any Triangle (Ranged) units." }
    ],
    rewards: {
      blueprints: ["sol_5"], // Unlocks Solaris Mage
      goldBounty: 175,
      unlocksMissions: ["M1_4_BOSS"]
    }
  },
  {
    id: "M1_4_BOSS",
    chapter: 1,
    name: "High Cathedral of Radiance",
    territoryName: "Grand Cathedral of High Solaris",
    pinCoord: [850, 710],
    prerequisites: ["M1_3"],
    archetype: "titan_slayer",
    enemyFaction: "solaris",
    aiDifficulty: "Hard",
    startingGold: 70,
    enemyDeck: ["sol_1", "sol_3", "sol_4", "sol_6", "sol_7", "sol_7"],
    briefing: "Grand Paladin Aurelius, Champion of the Sun, stands upon the Cathedral gates. His sacred blade cleaves legions and his holy aura inspires his soldiers to fight beyond death!",
    bossProps: {
      name: "Grand Paladin Aurelius",
      baseUnitId: "sol_7",
      scale: 1.95,
      maxHp: 2800,
      dmg: 45,
      rng: 7.5,
      cd: 0.95,
      spd: 10.5,
      passives: ["-30% Melee DMG Taken", "Aura: +15% ATK Spd & 25 HP heal / 6s (28 studs)"],
      telegraphedAbility: {
        name: "Judgement of the Sun",
        cooldown: 16.0,
        castTime: 1.8,
        aoeDmg: 95,
        radius: 140,
        color: "#fbbf24"
      }
    },
    starCriteria: [
      { stars: 1, desc: "Slay Grand Paladin Aurelius and topple the Cathedral Base." },
      { stars: 2, desc: "Inner Turret 2 (T2) never takes damage." },
      { stars: 3, desc: "Slay the Grand Paladin in under 3:15 minutes (195.0s)." }
    ],
    rewards: {
      blueprints: ["sol_7"], // Unlocks Solaris Elite Knight + Tier 3 Tech
      featureUnlock: "tier3",
      trophy: "solaris_suncrest",
      goldBounty: 250,
      unlocksMissions: ["M2_1", "M3_1"]
    }
  },

  // -----------------------------------------------------------------------
  // CHAPTER 2: NEO-VERIDIA SECTOR (AEGIS COALITION)
  // -----------------------------------------------------------------------
  {
    id: "M2_1",
    chapter: 2,
    name: "Checkpoint Alpha: Neon Gates",
    territoryName: "Neo-Veridia Transit Gate 01",
    pinCoord: [1150, 720],
    prerequisites: ["M1_4_BOSS"],
    archetype: "siege",
    enemyFaction: "aegis",
    aiDifficulty: "Normal",
    startingGold: 50,
    enemyDeck: ["aeg_1", "aeg_2", "aeg_3", "aeg_1", "aeg_5", "aeg_5"],
    briefing: "Unidentified armored division, halt immediately. Neo-Veridia airspace and land routes are sealed by executive order. Form a tight vanguard and breach the security gates!",
    starCriteria: [
      { stars: 1, desc: "Breach Checkpoint Alpha and demolish the Security Bastion." },
      { stars: 2, desc: "Win without any friendly unit being struck by enemy Commander [Q] Airstrike." },
      { stars: 3, desc: "Finish the match in under 2:45 minutes (165.0s)." }
    ],
    rewards: {
      blueprints: ["aeg_1", "aeg_2"], // Unlocks Riot Trooper & Police
      goldBounty: 180,
      unlocksMissions: ["M2_2"]
    }
  },
  {
    id: "M2_2",
    chapter: 2,
    name: "Cyber-Plaza Blitz",
    territoryName: "Neon Stockade Plaza",
    pinCoord: [1380, 670],
    prerequisites: ["M2_1"],
    archetype: "blitzkrieg",
    enemyFaction: "aegis",
    aiDifficulty: "Hard",
    startingGold: 80,
    enemyDeck: ["aeg_4", "aeg_5", "aeg_6", "aeg_4", "aeg_5", "aeg_6"],
    briefing: "Aegis defense satellites are aligning overhead! We have exactly 2 minutes and 15 seconds to overrun the plaza and destroy their uplink base before an orbital kinetic strike obliterates us!",
    timeLimitSeconds: 135.0,
    starCriteria: [
      { stars: 1, desc: "Demolish the Plaza Base Tower before the 02:15 timer expires." },
      { stars: 2, desc: "Cast player Commander Spell [Q] at least twice during the blitz." },
      { stars: 3, desc: "Destroy the enemy Base Tower in under 1:45 minutes (105.0s)." }
    ],
    rewards: {
      blueprints: ["aeg_4", "aeg_5"], // Unlocks Rifleman & Riot Shield
      goldBounty: 220,
      unlocksMissions: ["M2_3"]
    }
  },
  {
    id: "M2_3",
    chapter: 2,
    name: "Neon Grid Jammer",
    territoryName: "Sub-Level Power Conduits",
    pinCoord: [1620, 760],
    prerequisites: ["M2_2"],
    archetype: "mutator",
    enemyFaction: "aegis",
    aiDifficulty: "Normal",
    startingGold: 70,
    enemyDeck: ["aeg_1", "aeg_3", "aeg_5", "aeg_6", "aeg_5", "aeg_6"],
    briefing: "High-voltage conduit leaks overload optical targeting sensors. All ranged projectile attack cooldowns are increased by +40%, while melee units gain +25% move speed from motor overclocking!",
    mutator: {
      id: "emp_scrambler",
      name: "Electromagnetic Interference",
      desc: "Ranged Triangle & Diamond attack cooldowns +40%. Melee Circle & Square movement speed +25%."
    },
    starCriteria: [
      { stars: 1, desc: "Destroy the Power Conduit Base Tower." },
      { stars: 2, desc: "Win with at least 5 melee units (Square or Circle) alive on field." },
      { stars: 3, desc: "Upgrade player income to Level 6 before completing the mission." }
    ],
    rewards: {
      blueprints: ["aeg_3", "aeg_6"], // Unlocks Medic Bot & Grenade Thrower
      goldBounty: 240,
      unlocksMissions: ["M2_4_BOSS"]
    }
  },
  {
    id: "M2_4_BOSS",
    chapter: 2,
    name: "Citadel of Law: The Juggernaut Enforcer",
    territoryName: "Central Aegis High Command",
    pinCoord: [1860, 700],
    prerequisites: ["M2_3"],
    archetype: "titan_slayer",
    enemyFaction: "aegis",
    aiDifficulty: "Hard",
    startingGold: 85,
    enemyDeck: ["aeg_1", "aeg_3", "aeg_4", "aeg_5", "aeg_7", "aeg_8"],
    briefing: "Enforcer Unit Omega commands the Central Citadel. Wrapped in reactive titanium armor, its suppression pulse disables entire squadrons and its tactical aura crushes enemy defense!",
    bossProps: {
      name: "Enforcer Unit Omega",
      baseUnitId: "aeg_7",
      scale: 2.05,
      maxHp: 3400,
      dmg: 35,
      rng: 7.0,
      cd: 0.90,
      spd: 9.8,
      passives: ["-25% Damage Taken", "Aura: -20% Enemy Armor/Defense (24 studs)"],
      telegraphedAbility: {
        name: "Suppression Pulse",
        cooldown: 12.0,
        castTime: 1.5,
        aoeDmg: 65,
        radius: 200,
        slowPercent: 0.50,
        slowDuration: 4.0,
        color: "#38bdf8"
      }
    },
    starCriteria: [
      { stars: 1, desc: "Demolish Enforcer Unit Omega and raze the Central Citadel." },
      { stars: 2, desc: "Player Base Tower suffers zero damage throughout the encounter." },
      { stars: 3, desc: "Slay Enforcer Unit Omega in under 3:15 minutes (195.0s)." }
    ],
    rewards: {
      blueprints: ["aeg_7", "aeg_8"], // Unlocks Elite Riot & Shotgunner
      trophy: "aegis_star",
      goldBounty: 280,
      unlocksMissions: ["M4_1"]
    }
  },

  // -----------------------------------------------------------------------
  // CHAPTER 3: THE CHITIN WASTES (HIVE MIND)
  // -----------------------------------------------------------------------
  {
    id: "M3_1",
    chapter: 3,
    name: "Fungal Dunes Outpost",
    territoryName: "Spore Trench Perimeter",
    pinCoord: [220, 1020],
    prerequisites: ["M1_4_BOSS"],
    archetype: "siege",
    enemyFaction: "hive",
    aiDifficulty: "Normal",
    startingGold: 40,
    enemyDeck: ["hiv_1", "hiv_2", "hiv_3", "hiv_1", "hiv_2", "hiv_3"],
    briefing: "The southern continent is choking under a virulent fungal creep. The Alien Hive senses our presence and is spawning thousands of grunts beneath the dunes. Maintain firing lines and purge the tunnels!",
    starCriteria: [
      { stars: 1, desc: "Destroy the Spore Trench Base Spire." },
      { stars: 2, desc: "Slay at least 40 alien units during the match." },
      { stars: 3, desc: "Complete the mission in under 2:45 minutes (165.0s)." }
    ],
    rewards: {
      blueprints: ["hiv_1", "hiv_2"], // Unlocks Alien Grunt & Plasma Shooter
      goldBounty: 180,
      unlocksMissions: ["M3_2"]
    }
  },
  {
    id: "M3_2",
    chapter: 3,
    name: "Acid Marsh Survival",
    territoryName: "Caustic Quagmire",
    pinCoord: [450, 970],
    prerequisites: ["M3_1"],
    archetype: "hold_the_line",
    enemyFaction: "hive",
    aiDifficulty: "Hard",
    startingGold: 50,
    enemyDeck: ["hiv_1", "hiv_3", "hiv_4", "hiv_6", "hiv_4", "hiv_6"],
    briefing: "Our artillery wagons are bogged down in the caustic quagmire! Alien bio-forms are converging from all flanks. Hold the perimeter for 3 minutes while the recovery team unhitches the treads!",
    timeLimitSeconds: 180.0,
    playerIncomeBonusRate: 2.25,
    waves: [
      { triggerSec: 15, alertName: "Grunt Swarm", spawns: [{ unitId: "hiv_1", count: 6, yOffset: 160 }] },
      { triggerSec: 45, alertName: "Carapace Vanguard", spawns: [{ unitId: "hiv_3", count: 3, yOffset: 200 }, { unitId: "hiv_2", count: 3, yOffset: 320 }] },
      { triggerSec: 80, alertName: "Mutated Warriors & Spitters", spawns: [{ unitId: "hiv_4", count: 4, yOffset: 220 }, { unitId: "hiv_6", count: 3, yOffset: 290 }] },
      { triggerSec: 120, alertName: "Mass Bio-Infestation", spawns: [{ unitId: "hiv_1", count: 8, yOffset: 170 }, { unitId: "hiv_4", count: 4, yOffset: 330 }] },
      { triggerSec: 155, alertName: "APEX BIO-HORDE", spawns: [{ unitId: "hiv_7", count: 2, yOffset: 220 }, { unitId: "hiv_6", count: 6, yOffset: 280 }, { unitId: "hiv_3", count: 4, yOffset: 340 }] }
    ],
    starCriteria: [
      { stars: 1, desc: "Survive the full 3 minutes without Base Tower falling." },
      { stars: 2, desc: "Outer Turret 1 (T1) remains alive at 00:00." },
      { stars: 3, desc: "Never allow more than 2 friendly units to be poisoned simultaneously." }
    ],
    rewards: {
      blueprints: ["hiv_4", "hiv_6"], // Unlocks Mutated Warrior & Acid Spitter
      goldBounty: 220,
      unlocksMissions: ["M3_3"]
    }
  },
  {
    id: "M3_3",
    chapter: 3,
    name: "Caustic Grotto",
    territoryName: "The Bile Chambers",
    pinCoord: [680, 1050],
    prerequisites: ["M3_2"],
    archetype: "mutator",
    enemyFaction: "hive",
    aiDifficulty: "Hard",
    startingGold: 75,
    enemyDeck: ["hiv_1", "hiv_3", "hiv_4", "hiv_5", "hiv_6", "hiv_6"],
    briefing: "The cavern atmosphere is dense with digestive acid fumes. Every unit takes 2 DMG every 1.5 seconds, UNLESS they are standing within 110px of an allied structure or an active Cross healer!",
    mutator: {
      id: "corrosive_miasma",
      name: "Corrosive Miasma",
      desc: "All units suffer 2 DMG every 1.5s unless within 110px of an allied structure or healer shape."
    },
    starCriteria: [
      { stars: 1, desc: "Destroy the Bile Chamber Core." },
      { stars: 2, desc: "Win with all 3 of your defense structures alive." },
      { stars: 3, desc: "Cast player Commander Spell [E] (Healing/Surge) at least 3 times." }
    ],
    rewards: {
      blueprints: ["hiv_3", "hiv_5"], // Unlocks Alien Guard & Plasma Specialist
      goldBounty: 250,
      unlocksMissions: ["M3_4_BOSS"]
    }
  },
  {
    id: "M3_4_BOSS",
    chapter: 3,
    name: "The Broodmother's Hive Core",
    territoryName: "Spire of the Apex Broodmother",
    pinCoord: [890, 1000],
    prerequisites: ["M3_3"],
    archetype: "titan_slayer",
    enemyFaction: "hive",
    aiDifficulty: "Hard",
    startingGold: 85,
    enemyDeck: ["hiv_1", "hiv_4", "hiv_5", "hiv_6", "hiv_7", "hiv_9"],
    briefing: "The Gorgon Leviathan guards the Hive Spire. Its colossal scythes tear through steel, it spawns endless bio-grunts from its carapace, and its toxic vomit melts flesh in seconds!",
    bossProps: {
      name: "Gorgon Leviathan",
      baseUnitId: "hiv_7",
      scale: 2.15,
      maxHp: 3600,
      dmg: 55,
      rng: 7.5,
      cd: 1.05,
      spd: 9.5,
      passives: ["Brood Spawner: Summons 2 Grunts / 8s", "Thick Carapace: Ignores 5 DMG from all hits"],
      telegraphedAbility: {
        name: "Caustic Acid Vomit",
        cooldown: 14.0,
        castTime: 1.6,
        aoeDmg: 85,
        lineLength: 280,
        poisonDps: 8,
        poisonDuration: 5.0,
        color: "#4ade80"
      }
    },
    starCriteria: [
      { stars: 1, desc: "Slay the Gorgon Leviathan and destroy the Hive Core." },
      { stars: 2, desc: "Player Base Tower sustains zero damage." },
      { stars: 3, desc: "Slay the Leviathan in under 3:15 minutes (195.0s)." }
    ],
    rewards: {
      blueprints: ["hiv_7", "hiv_9"], // Unlocks Alien Juggernaut & Hive Commander
      trophy: "chitin_carapace",
      goldBounty: 280,
      unlocksMissions: ["M5_1"]
    }
  },

  // -----------------------------------------------------------------------
  // CHAPTER 4: THE ABYSSAL RIFT (ABYSSAL VOID)
  // -----------------------------------------------------------------------
  {
    id: "M4_1",
    chapter: 4,
    name: "Event Horizon Chasm",
    territoryName: "Void Outpost Obsidian",
    pinCoord: [220, 240],
    prerequisites: ["M2_4_BOSS"],
    archetype: "siege",
    enemyFaction: "void",
    aiDifficulty: "Normal",
    startingGold: 50,
    enemyDeck: ["voi_1", "voi_2", "voi_3", "voi_1", "voi_2", "voi_3"],
    briefing: "We have climbed into the high atmospheric rift where reality bends. The Abyssal Void guards their dark-matter portals with cloaked assassins. Establish our beacon and collapse their forward vortex!",
    starCriteria: [
      { stars: 1, desc: "Collapse the Obsidian Base Vortex." },
      { stars: 2, desc: "Outer Turret 1 (T1) retains >= 60% HP at victory." },
      { stars: 3, desc: "Conclude the match in under 2:30 minutes (150.0s)." }
    ],
    rewards: {
      blueprints: ["voi_1", "voi_2"], // Unlocks Shadowling & Void Archer
      goldBounty: 260,
      unlocksMissions: ["M4_2"]
    }
  },
  {
    id: "M4_2",
    chapter: 4,
    name: "The Phantom Corridor",
    territoryName: "Corridor of Phantoms",
    pinCoord: [450, 180],
    prerequisites: ["M4_1"],
    archetype: "blitzkrieg",
    enemyFaction: "void",
    aiDifficulty: "Hard",
    startingGold: 85,
    enemyDeck: ["voi_3", "voi_4", "voi_5", "voi_3", "voi_4", "voi_5"],
    briefing: "At 02:00, the corridor completely implodes into a singularity, destroying all forces. Deploy high-speed strikers and break their nexus before reality shatters!",
    timeLimitSeconds: 120.0,
    starCriteria: [
      { stars: 1, desc: "Destroy the Corridor Nexus before 02:00." },
      { stars: 2, desc: "Suffer zero deaths on friendly A-Tier Mythic units." },
      { stars: 3, desc: "Destroy the enemy Base in under 1:35 minutes (95.0s)." }
    ],
    rewards: {
      blueprints: ["voi_4", "voi_5"], // Unlocks Void Stalker & Abyss Mage
      goldBounty: 290,
      unlocksMissions: ["M4_3"]
    }
  },
  {
    id: "M4_3",
    chapter: 4,
    name: "Null-Space Singularity",
    territoryName: "Gravity Distortion Well",
    pinCoord: [680, 270],
    prerequisites: ["M4_2"],
    archetype: "mutator",
    enemyFaction: "void",
    aiDifficulty: "Hard",
    startingGold: 90,
    enemyDeck: ["voi_1", "voi_3", "voi_4", "voi_6", "voi_8", "voi_6"],
    briefing: "The gravitational well bends spacetime. Soul Drainers' Defy Death immunity duration is doubled from 1.0s to 2.0 seconds! Trigger your own death immunities to turn the tide!",
    mutator: {
      id: "gravity_distortion",
      name: "Gravity Inversion & Chrono Stasis",
      desc: "Death Immunity duration doubled to 2.0s. Projectiles curve inward toward lane center."
    },
    starCriteria: [
      { stars: 1, desc: "Destroy the Gravitational Well Generator." },
      { stars: 2, desc: "Successfully trigger a friendly unit's Death Immunity passive at least 3 times." },
      { stars: 3, desc: "Win without losing either Turret 1 or Turret 2." }
    ],
    rewards: {
      blueprints: ["voi_3", "voi_6"], // Unlocks Dark Sentinel & Soul Drainer
      goldBounty: 310,
      unlocksMissions: ["M4_4_BOSS"]
    }
  },
  {
    id: "M4_4_BOSS",
    chapter: 4,
    name: "The Void Nexus: Herald of Entropy",
    territoryName: "Sanctum of Sovereign Malakor",
    pinCoord: [890, 210],
    prerequisites: ["M4_3"],
    archetype: "titan_slayer",
    enemyFaction: "void",
    aiDifficulty: "Hard",
    startingGold: 95,
    enemyDeck: ["voi_4", "voi_6", "voi_7", "voi_8", "voi_9", "voi_9"],
    briefing: "Sovereign Malakor wields the scythe of absolute entropy. His dark aura erodes armor by 35% and his Black Hole pulls entire legions into a crushing singularity!",
    bossProps: {
      name: "Sovereign Malakor",
      baseUnitId: "voi_9",
      scale: 2.10,
      maxHp: 3500,
      dmg: 48,
      rng: 7.5,
      cd: 0.95,
      spd: 10.0,
      passives: ["Entropy Aura: -35% Enemy Armor (28 studs)", "Phase Shift: Immune to ranged hits for 1.5s / 10s"],
      telegraphedAbility: {
        name: "Black Hole Singularity",
        cooldown: 16.0,
        castTime: 1.5,
        aoeDmg: 90,
        radius: 220,
        color: "#a855f7"
      }
    },
    starCriteria: [
      { stars: 1, desc: "Slay Sovereign Malakor and shatter the Void Nexus." },
      { stars: 2, desc: "Inner Turret 2 (T2) never falls below 50% HP." },
      { stars: 3, desc: "Slay Sovereign Malakor in under 3:00 minutes (180.0s)." }
    ],
    rewards: {
      blueprints: ["voi_7", "voi_8", "voi_9"], // Unlocks Void Reaper, Sorcerer, Warden
      trophy: "void_singularity_heart",
      goldBounty: 330,
      unlocksMissions: ["M5_1", "M6_1"]
    }
  },

  // -----------------------------------------------------------------------
  // CHAPTER 5: THE BRIMSTONE CRUCIBLE (HELLFIRE LEGION)
  // -----------------------------------------------------------------------
  {
    id: "M5_1",
    chapter: 5,
    name: "Magma Trench Incursion",
    territoryName: "Charred Redoubt 01",
    pinCoord: [1150, 1020],
    prerequisites: ["M3_4_BOSS"],
    archetype: "siege",
    enemyFaction: "hellfire",
    aiDifficulty: "Normal",
    startingGold: 50,
    enemyDeck: ["hel_1", "hel_2", "hel_3", "hel_1", "hel_2", "hel_3"],
    briefing: "We march across volcanic bridges above rivers of boiling magma. Hell hounds sprint at 16.5 speed—the fastest beasts on Geo-Terra. Hold rank and do not let their speed fracture our formation!",
    starCriteria: [
      { stars: 1, desc: "Destroy the Charred Redoubt Base." },
      { stars: 2, desc: "Outer Turret 1 (T1) remains standing with >= 75% HP." },
      { stars: 3, desc: "Conclude the match in under 2:30 minutes (150.0s)." }
    ],
    rewards: {
      blueprints: ["hel_1", "hel_2"], // Unlocks Imp & Hell Hound
      goldBounty: 270,
      unlocksMissions: ["M5_2"]
    }
  },
  {
    id: "M5_2",
    chapter: 5,
    name: "Blood Gate Siegetrain",
    territoryName: "The Crimson Bastion",
    pinCoord: [1380, 970],
    prerequisites: ["M5_1"],
    archetype: "hold_the_line",
    enemyFaction: "hellfire",
    aiDifficulty: "Hard",
    startingGold: 60,
    enemyDeck: ["hel_2", "hel_3", "hel_4", "hel_5", "hel_4", "hel_6"],
    briefing: "Armored Blood Knights drink the blood of our soldiers to heal their wounds. Anchor our shield wall and repel the invasion waves for 3 minutes!",
    timeLimitSeconds: 180.0,
    playerIncomeBonusRate: 2.4,
    waves: [
      { triggerSec: 15, alertName: "Hell Hound Sprint", spawns: [{ unitId: "hel_2", count: 6, yOffset: 160 }] },
      { triggerSec: 45, alertName: "Soldiers & Flame Casters", spawns: [{ unitId: "hel_3", count: 4, yOffset: 200 }, { unitId: "hel_5", count: 3, yOffset: 330 }] },
      { triggerSec: 85, alertName: "Blood Knight Phalanx", spawns: [{ unitId: "hel_4", count: 4, yOffset: 220 }, { unitId: "hel_6", count: 3, yOffset: 290 }] },
      { triggerSec: 125, alertName: "Imp Swarm Rush", spawns: [{ unitId: "hel_1", count: 8, yOffset: 170 }, { unitId: "hel_4", count: 4, yOffset: 340 }] },
      { triggerSec: 160, alertName: "DEMONIC SIEGETRAIN", spawns: [{ unitId: "hel_7", count: 2, yOffset: 220 }, { unitId: "hel_4", count: 4, yOffset: 270 }, { unitId: "hel_5", count: 4, yOffset: 330 }] }
    ],
    starCriteria: [
      { stars: 1, desc: "Hold the Crimson Bastion for the full 3 minutes without Base Tower falling." },
      { stars: 2, desc: "Slay at least 50 enemy demons during the defense." },
      { stars: 3, desc: "Win without losing any defense structure (T1, T2, Base all alive)." }
    ],
    rewards: {
      blueprints: ["hel_3", "hel_4"], // Unlocks Demon Soldier & Blood Knight
      goldBounty: 300,
      unlocksMissions: ["M5_3"]
    }
  },
  {
    id: "M5_3",
    chapter: 5,
    name: "The Infernal Caldera",
    territoryName: "Caldera of Molten Souls",
    pinCoord: [1620, 1050],
    prerequisites: ["M5_2"],
    archetype: "mutator",
    enemyFaction: "hellfire",
    aiDifficulty: "Hard",
    startingGold: 80,
    enemyDeck: ["hel_1", "hel_3", "hel_4", "hel_5", "hel_6", "hel_6"],
    briefing: "The searing heat boils the blood of all combatants. ALL units gain +35% Attack Speed and +25% Move Speed, but suffer 1.0% Max HP burn per second!",
    mutator: {
      id: "blood_frenzy",
      name: "Blood Frenzy & Magma Heat",
      desc: "All units gain +35% ATK speed and +25% move speed, but take 1.0% Max HP burn damage per second."
    },
    starCriteria: [
      { stars: 1, desc: "Shatter the Caldera Base Tower." },
      { stars: 2, desc: "Upgrade player income to Level 6 before completing the match." },
      { stars: 3, desc: "Destroy the enemy Base in under 2:15 minutes (135.0s)." }
    ],
    rewards: {
      blueprints: ["hel_5", "hel_6"], // Unlocks Flame Caster & Soul Reaper
      goldBounty: 330,
      unlocksMissions: ["M5_4_BOSS"]
    }
  },
  {
    id: "M5_4_BOSS",
    chapter: 5,
    name: "Throne of the Archfiend: Lord Malgok",
    territoryName: "The Obsidian Skull Citadel",
    pinCoord: [1860, 1000],
    prerequisites: ["M5_3"],
    archetype: "titan_slayer",
    enemyFaction: "hellfire",
    aiDifficulty: "Hard",
    startingGold: 90,
    enemyDeck: ["hel_4", "hel_6", "hel_7", "hel_8", "hel_9", "hel_9"],
    briefing: "Lord Malgok towers over the obsidian throne. His flaming sword heals him for 50 HP on every swing and his Meteor Rain burns entire armies to ash!",
    bossProps: {
      name: "Arch-Demon Lord Malgok",
      baseUnitId: "hel_9",
      scale: 2.20,
      maxHp: 4200,
      dmg: 65,
      rng: 7.5,
      cd: 0.90,
      spd: 10.5,
      passives: ["Greater Vampirism: +50 HP Lifesteal on strike", "-20% Ranged DMG Taken"],
      telegraphedAbility: {
        name: "Cataclysmic Meteor Rain",
        cooldown: 14.0,
        castTime: 1.8,
        aoeDmg: 115,
        radius: 120,
        meteorCount: 3,
        color: "#f43f5e"
      }
    },
    starCriteria: [
      { stars: 1, desc: "Slay Arch-Demon Lord Malgok and raze the Skull Citadel." },
      { stars: 2, desc: "Player Base Tower sustains at least 1,200 HP at match conclusion." },
      { stars: 3, desc: "Slay Lord Malgok in under 3:30 minutes (210.0s)." }
    ],
    rewards: {
      blueprints: ["hel_7", "hel_8", "hel_9"], // Unlocks Demon Brute, Archmage, Demon Lord
      trophy: "horn_of_malgok",
      goldBounty: 360,
      unlocksMissions: ["M6_1"]
    }
  },

  // -----------------------------------------------------------------------
  // CHAPTER 6: SECTOR ZERO: THE CORE (MECHANICA CORPS)
  // -----------------------------------------------------------------------
  {
    id: "M6_1",
    chapter: 6,
    name: "Automated Perimeter Breaker",
    territoryName: "Cyber-Citadel Trench 01",
    pinCoord: [1150, 240],
    prerequisites: ["M4_4_BOSS", "M5_4_BOSS"],
    archetype: "siege",
    enemyFaction: "mechanica",
    aiDifficulty: "Hard",
    startingGold: 60,
    enemyDeck: ["mec_1", "mec_2", "mec_3", "mec_1", "mec_2", "mec_3"],
    briefing: "We have reached Sector Zero, the central brain of Geo-Terra. The Mechanica Corps builds war machines that feel no pain or fear. Shatter their sentries!",
    starCriteria: [
      { stars: 1, desc: "Destroy the Cyber-Citadel Base Tower." },
      { stars: 2, desc: "Suffer fewer than 10 total friendly unit casualties during the entire battle." },
      { stars: 3, desc: "Finish the match in under 2:45 minutes (165.0s)." }
    ],
    rewards: {
      blueprints: ["mec_1", "mec_2"], // Unlocks Combat Bot & Assault Drone
      goldBounty: 350,
      unlocksMissions: ["M6_2"]
    }
  },
  {
    id: "M6_2",
    chapter: 6,
    name: "Siege Engine Blitzkrieg",
    territoryName: "Railgun Assembly Foundry",
    pinCoord: [1380, 180],
    prerequisites: ["M6_1"],
    archetype: "blitzkrieg",
    enemyFaction: "mechanica",
    aiDifficulty: "Hard",
    startingGold: 90,
    enemyDeck: ["mec_3", "mec_4", "mec_5", "mec_3", "mec_4", "mec_5"],
    briefing: "At 02:20, the foundry superlasers complete charging and vaporize our army. Push forward with maximum tempo and destroy their power couplings!",
    timeLimitSeconds: 140.0,
    starCriteria: [
      { stars: 1, desc: "Destroy the Assembly Foundry Base before 02:20." },
      { stars: 2, desc: "Cast Commander Spell [Q] at least twice during the siege." },
      { stars: 3, desc: "Destroy the enemy Base in under 1:45 minutes (105.0s)." }
    ],
    rewards: {
      blueprints: ["mec_4", "mec_5"], // Unlocks Warbot & Plasma Robot
      goldBounty: 380,
      unlocksMissions: ["M6_3"]
    }
  },
  {
    id: "M6_3",
    chapter: 6,
    name: "Overclocked Factory Core",
    territoryName: "Nanite Fusion Reactor",
    pinCoord: [1620, 270],
    prerequisites: ["M6_2"],
    archetype: "mutator",
    enemyFaction: "mechanica",
    aiDifficulty: "Extreme",
    startingGold: 100,
    enemyDeck: ["mec_1", "mec_3", "mec_4", "mec_6", "mec_8", "mec_8"],
    briefing: "Microscopic repair nanites infuse the atmosphere. All mechanical units (Square & Hexagon) regenerate 5% Max HP every 3 seconds and gain +20% move speed!",
    mutator: {
      id: "nanite_regeneration",
      name: "Autonomous Nanite Matrix",
      desc: "All Square & Hexagon units regenerate 5% Max HP every 3s and gain +20% move speed."
    },
    starCriteria: [
      { stars: 1, desc: "Destroy the Nanite Fusion Reactor Base." },
      { stars: 2, desc: "Destroy all three enemy defense structures (T1, T2, and Base)." },
      { stars: 3, desc: "Upgrade player income to Level 7 before match completion." }
    ],
    rewards: {
      blueprints: ["mec_3", "mec_6"], // Unlocks Shield Robot & Repair Bot
      goldBounty: 420,
      unlocksMissions: ["M6_4_FINAL_BOSS"]
    }
  },
  {
    id: "M6_4_FINAL_BOSS",
    chapter: 6,
    name: "The Core Prime: Cybernetic Colossus",
    territoryName: "Central Processing Throne: Geo-Terra Core",
    pinCoord: [1860, 210],
    prerequisites: ["M6_3"],
    archetype: "titan_slayer",
    enemyFaction: "mechanica",
    aiDifficulty: "Extreme",
    startingGold: 110,
    enemyDeck: ["mec_4", "mec_6", "mec_7", "mec_8", "mec_9", "mec_9"],
    briefing: "Colossus Prime, the autonomous superintelligence of Geo-Terra, activates. Dual autocannons, orbital ion strikes, and an emergency nanite barrier stand between us and planetary liberation!",
    bossProps: {
      name: "Colossus Prime",
      baseUnitId: "mec_7",
      scale: 2.40,
      maxHp: 5500,
      dmg: 75,
      rng: 8.5,
      cd: 0.95,
      spd: 9.0,
      passives: ["Cyber Speed Aura: +20% ATK Spd (28 studs)", "Enrage at 25% HP: +35% ATK Spd & +25% Speed"],
      hasPhase2Shield: true,
      phase2ShieldAmount: 1200,
      telegraphedAbility: {
        name: "Orbital Ion Cannon Death-Ray",
        cooldown: 18.0,
        castTime: 2.0,
        aoeDmg: 160,
        radius: 150,
        color: "#38bdf8"
      }
    },
    starCriteria: [
      { stars: 1, desc: "Slay Colossus Prime and liberate all of Geo-Terra!" },
      { stars: 2, desc: "Player Outer Turret 1 (T1) survives the final boss encounter." },
      { stars: 3, desc: "Defeat Colossus Prime in under 4:00 minutes (240.0s)." }
    ],
    rewards: {
      blueprints: ["mec_7", "mec_8", "mec_9"], // Unlocks Destroyer Mech, Artillery Bot, Cyber Commander
      trophy: "crown_of_geo_terra",
      goldBounty: 1000,
      unlocksMissions: []
    }
  }
];

window.getMissionById = function (id) {
  return window.CONQUEST_MISSIONS.find((m) => m.id === id) || null;
};
