-- =========================================================================
-- EPIC BATTLE! — GRAND CONQUEST CONFIGURATION & MISSION REGISTRY
-- Production Luau Port: 6 Chapters, 24 Missions, 15 War Relics
-- =========================================================================

local ConquestConfig = {}

-- -------------------------------------------------------------------------
-- 15 WAR RELICS CATALOG & COSTS
-- -------------------------------------------------------------------------
ConquestConfig.Relics = {
  ["treasury_seal"] = {
    Id = "treasury_seal",
    Name = "Treasury Seal",
    StarsReq = 5,
    RPCost = 1,
    Icon = "💰",
    Description = "Starting gold increased from 40G to 65G (+25G initial vault).",
  },
  ["rapid_banner"] = {
    Id = "rapid_banner",
    Name = "Rapid Deployment Banner",
    StarsReq = 10,
    RPCost = 1,
    Icon = "🚩",
    Description = "Unit movement speed across all tiers increased by +12%.",
  },
  ["overcharged_conduits"] = {
    Id = "overcharged_conduits",
    Name = "Overcharged Conduits",
    StarsReq = 15,
    RPCost = 2,
    Icon = "⚡",
    Description = "Commander Spells [Q] and [E] cooldowns reduced from 60s to 48s (-20%).",
  },
  ["titanium_fort"] = {
    Id = "titanium_fort",
    Name = "Titanium Fortification",
    StarsReq = 20,
    RPCost = 2,
    Icon = "🏰",
    Description = "Base Tower HP increased from 1500 to 1850; Turrets gain +15% firing range.",
  },
  ["vampiric_stone"] = {
    Id = "vampiric_stone",
    Name = "Vampiric Bloodstone",
    StarsReq = 25,
    RPCost = 2,
    Icon = "🩸",
    Description = "All friendly melee units (<= 7.5 studs) restore +10 HP on strike.",
  },
  ["arcane_magnifier"] = {
    Id = "arcane_magnifier",
    Name = "Arcane Magnifier",
    StarsReq = 30,
    RPCost = 1,
    Icon = "🔮",
    Description = "All Diamond shapes (Mages & Artillery) deal +15% damage.",
  },
  ["ballistic_lens"] = {
    Id = "ballistic_lens",
    Name = "Ballistic Compensator",
    StarsReq = 35,
    RPCost = 1,
    Icon = "🎯",
    Description = "All Triangle shapes (Marksmen) gain +4.0 studs (+34px) firing range.",
  },
  ["sacred_phylactery"] = {
    Id = "sacred_phylactery",
    Name = "Sacred Phylactery",
    StarsReq = 40,
    RPCost = 2,
    Icon = "✨",
    Description = "All Cross shape (Healer) wave heals grant a temporary +25 HP barrier for 4s.",
  },
  ["chitin_carapace"] = {
    Id = "chitin_carapace",
    Name = "Chitin-Plated Carapace",
    StarsReq = 45,
    RPCost = 2,
    Icon = "🛡️",
    Description = "All Square shapes (Tanks & Blockers) take 3 less damage from every hit.",
  },
  ["void_singularity"] = {
    Id = "void_singularity",
    Name = "Singularity Heart",
    StarsReq = 50,
    RPCost = 2,
    Icon = "🌌",
    Description = "Enemies below 15% Max HP are instantly executed (excluding Titans).",
  },
  ["nanite_injector"] = {
    Id = "nanite_injector",
    Name = "Nanite Emergency Injector",
    StarsReq = 55,
    RPCost = 2,
    Icon = "💉",
    Description = "Friendly units dropping below 25% HP instantly restore 30% Max HP (once per life).",
  },
  ["economic_monopoly"] = {
    Id = "economic_monopoly",
    Name = "Economic Monopoly",
    StarsReq = 60,
    RPCost = 1,
    Icon = "📈",
    Description = "Permanent 25% discount on [U] Income Upgrade costs.",
  },
  ["orbital_lens"] = {
    Id = "orbital_lens",
    Name = "Targeting Satellite Matrix",
    StarsReq = 64,
    RPCost = 1,
    Icon = "🛰️",
    Description = "Commander Spell [Q] AoE radius increased by +25% (170px radius blast).",
  },
  ["surge_amplifier"] = {
    Id = "surge_amplifier",
    Name = "Tactical Surge Amplifier",
    StarsReq = 68,
    RPCost = 2,
    Icon = "🔥",
    Description = "Commander Spell [E] heals +100 HP (up from 75) and surges +65% speed (up from +45%).",
  },
  ["crown_geo_terra"] = {
    Id = "crown_geo_terra",
    Name = "Crown of Geo-Terra",
    StarsReq = 72,
    RPCost = 2,
    Icon = "👑",
    Description = "Ultimate 100% Mastery: All friendly units gain +10% HP, +10% DMG, and +10% Speed!",
  },
}

-- -------------------------------------------------------------------------
-- 24 CAMPAIGN MISSIONS (CHAPTERS 1 - 6)
-- -------------------------------------------------------------------------
ConquestConfig.Missions = {
  ["M1_1"] = {
    Id = "M1_1",
    Chapter = 1,
    Name = "The Sun-Gilded Border",
    TerritoryName = "Solaris Outpost Alpha",
    PinCoord = Vector2.new(220, 720),
    Prerequisites = {},
    Archetype = "siege",
    EnemyFaction = "solaris",
    AIDifficulty = "Easy",
    StartingGold = 40,
    EnemyDeck = { "sol_1", "sol_2", "sol_3", "sol_1", "sol_2", "sol_3" },
    Briefing = "Our expedition has made landfall on the golden shores of the Solaris Empire. Their border sentries have raised their banners and refuse our parley. Secure the border gate and demonstrate the strength of our legion.",
    StarCriteria = {
      {
        stars = 1,
        desc = "Demolish the enemy Base Tower.",
      },
      {
        stars = 2,
        desc = "Outer Turret 1 (T1) remains standing with >= 50% HP (225+ HP).",
      },
      {
        stars = 3,
        desc = "Conclude the match in under 3:00 minutes (180.0s).",
      },
    },
    Rewards = {
      blueprints = { "sol_4" },
      goldBounty = 100,
      unlocksMissions = { "M1_2" },
    },
  },
  ["M1_2"] = {
    Id = "M1_2",
    Chapter = 1,
    Name = "The Gilded Abbey",
    TerritoryName = "Sanctuary of Saint Lyra",
    PinCoord = Vector2.new(410, 680),
    Prerequisites = { "M1_1" },
    Archetype = "hold_the_line",
    EnemyFaction = "solaris",
    AIDifficulty = "Normal",
    StartingGold = 50,
    EnemyDeck = { "sol_1", "sol_2", "sol_3", "sol_4", "sol_6", "sol_6" },
    Briefing = "The Solaris Clergy has rallied their knights to reclaim the Abbey. Our forward outpost is cut off from reinforcements! Dig in and hold the perimeter for 2 minutes and 30 seconds until the supply zeppelin arrives!",
    TimeLimitSeconds = 150,
    Waves = {
      {
        triggerSec = 10,
        alertName = "Footman Vanguard",
        spawns = {
          {
            unitId = "sol_1",
            count = 4,
            yOffset = 150,
          },
        },
      },
      {
        triggerSec = 35,
        alertName = "Shield & Archer Volley",
        spawns = {
          {
            unitId = "sol_3",
            count = 2,
            yOffset = 200,
          },
          {
            unitId = "sol_2",
            count = 3,
            yOffset = 320,
          },
        },
      },
      {
        triggerSec = 65,
        alertName = "Knight Holy Retinue",
        spawns = {
          {
            unitId = "sol_4",
            count = 3,
            yOffset = 220,
          },
          {
            unitId = "sol_6",
            count = 2,
            yOffset = 280,
          },
        },
      },
      {
        triggerSec = 100,
        alertName = "Mass Footman Charge",
        spawns = {
          {
            unitId = "sol_1",
            count = 6,
            yOffset = 180,
          },
          {
            unitId = "sol_2",
            count = 4,
            yOffset = 340,
          },
        },
      },
      {
        triggerSec = 130,
        alertName = "FINAL CRUSADE",
        spawns = {
          {
            unitId = "sol_4",
            count = 4,
            yOffset = 210,
          },
          {
            unitId = "sol_6",
            count = 2,
            yOffset = 265,
          },
          {
            unitId = "sol_3",
            count = 4,
            yOffset = 320,
          },
        },
      },
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Survive the full 2:30 minutes without Base Tower falling.",
      },
      {
        stars = 2,
        desc = "Outer Turret 1 (T1) remains standing at 00:00.",
      },
      {
        stars = 3,
        desc = "Eliminate at least 35 enemy invaders during the siege.",
      },
    },
    Rewards = {
      blueprints = { "sol_6" },
      featureUnlock = "tier2",
      goldBounty = 150,
      unlocksMissions = { "M1_3" },
    },
  },
  ["M1_3"] = {
    Id = "M1_3",
    Chapter = 1,
    Name = "Ramparts of the Sun",
    TerritoryName = "The Sunken Bastion",
    PinCoord = Vector2.new(630, 750),
    Prerequisites = { "M1_2" },
    Archetype = "mutator",
    EnemyFaction = "solaris",
    AIDifficulty = "Normal",
    StartingGold = 60,
    EnemyDeck = { "sol_1", "sol_3", "sol_4", "sol_5", "sol_6", "sol_6" },
    Briefing = "Sacred sunlight beams bathe the Sunken Bastion. The glare weakens ranged projectile accuracy, but divine holy energies empower all healers on the field!",
    Mutator = {
      id = "consecrated_solar",
      name = "Consecrated Solar Conduits",
      desc = "All Cross shape heals pulse +50% extra healing. All Triangle shape ranged damage reduced by -20%.",
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Destroy the Sunken Bastion Base Tower.",
      },
      {
        stars = 2,
        desc = "Reach and maintain an Income Level >= 5 before winning.",
      },
      {
        stars = 3,
        desc = "Win without deploying any Triangle (Ranged) units.",
      },
    },
    Rewards = {
      blueprints = { "sol_5" },
      goldBounty = 175,
      unlocksMissions = { "M1_4_BOSS" },
    },
  },
  ["M1_4_BOSS"] = {
    Id = "M1_4_BOSS",
    Chapter = 1,
    Name = "High Cathedral of Radiance",
    TerritoryName = "Grand Cathedral of High Solaris",
    PinCoord = Vector2.new(850, 710),
    Prerequisites = { "M1_3" },
    Archetype = "titan_slayer",
    EnemyFaction = "solaris",
    AIDifficulty = "Hard",
    StartingGold = 70,
    EnemyDeck = { "sol_1", "sol_3", "sol_4", "sol_6", "sol_7", "sol_7" },
    Briefing = "Grand Paladin Aurelius, Champion of the Sun, stands upon the Cathedral gates. His sacred blade cleaves legions and his holy aura inspires his soldiers to fight beyond death!",
    BossProps = {
      name = "Grand Paladin Aurelius",
      baseUnitId = "sol_7",
      scale = 1.95,
      maxHp = 2800,
      dmg = 45,
      rng = 7.5,
      cd = 0.95,
      spd = 10.5,
      passives = { "-30% Melee DMG Taken", "Aura: +15% ATK Spd & 25 HP heal / 6s (28 studs)" },
      telegraphedAbility = {
        name = "Judgement of the Sun",
        cooldown = 16,
        castTime = 1.8,
        aoeDmg = 95,
        radius = 140,
        color = "#fbbf24",
      },
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Slay Grand Paladin Aurelius and topple the Cathedral Base.",
      },
      {
        stars = 2,
        desc = "Inner Turret 2 (T2) never takes damage.",
      },
      {
        stars = 3,
        desc = "Slay the Grand Paladin in under 3:15 minutes (195.0s).",
      },
    },
    Rewards = {
      blueprints = { "sol_7" },
      featureUnlock = "tier3",
      trophy = "solaris_suncrest",
      goldBounty = 250,
      unlocksMissions = { "M2_1", "M3_1" },
    },
  },
  ["M2_1"] = {
    Id = "M2_1",
    Chapter = 2,
    Name = "Checkpoint Alpha: Neon Gates",
    TerritoryName = "Neo-Veridia Transit Gate 01",
    PinCoord = Vector2.new(1150, 720),
    Prerequisites = { "M1_4_BOSS" },
    Archetype = "siege",
    EnemyFaction = "aegis",
    AIDifficulty = "Normal",
    StartingGold = 50,
    EnemyDeck = { "aeg_1", "aeg_2", "aeg_3", "aeg_1", "aeg_5", "aeg_5" },
    Briefing = "Unidentified armored division, halt immediately. Neo-Veridia airspace and land routes are sealed by executive order. Form a tight vanguard and breach the security gates!",
    StarCriteria = {
      {
        stars = 1,
        desc = "Breach Checkpoint Alpha and demolish the Security Bastion.",
      },
      {
        stars = 2,
        desc = "Win without any friendly unit being struck by enemy Commander [Q] Airstrike.",
      },
      {
        stars = 3,
        desc = "Finish the match in under 2:45 minutes (165.0s).",
      },
    },
    Rewards = {
      blueprints = { "aeg_1", "aeg_2" },
      goldBounty = 180,
      unlocksMissions = { "M2_2" },
    },
  },
  ["M2_2"] = {
    Id = "M2_2",
    Chapter = 2,
    Name = "Cyber-Plaza Blitz",
    TerritoryName = "Neon Stockade Plaza",
    PinCoord = Vector2.new(1380, 670),
    Prerequisites = { "M2_1" },
    Archetype = "blitzkrieg",
    EnemyFaction = "aegis",
    AIDifficulty = "Hard",
    StartingGold = 80,
    EnemyDeck = { "aeg_4", "aeg_5", "aeg_6", "aeg_4", "aeg_5", "aeg_6" },
    Briefing = "Aegis defense satellites are aligning overhead! We have exactly 2 minutes and 15 seconds to overrun the plaza and destroy their uplink base before an orbital kinetic strike obliterates us!",
    TimeLimitSeconds = 135,
    StarCriteria = {
      {
        stars = 1,
        desc = "Demolish the Plaza Base Tower before the 02:15 timer expires.",
      },
      {
        stars = 2,
        desc = "Cast player Commander Spell [Q] at least twice during the blitz.",
      },
      {
        stars = 3,
        desc = "Destroy the enemy Base Tower in under 1:45 minutes (105.0s).",
      },
    },
    Rewards = {
      blueprints = { "aeg_4", "aeg_5" },
      goldBounty = 220,
      unlocksMissions = { "M2_3" },
    },
  },
  ["M2_3"] = {
    Id = "M2_3",
    Chapter = 2,
    Name = "Neon Grid Jammer",
    TerritoryName = "Sub-Level Power Conduits",
    PinCoord = Vector2.new(1620, 760),
    Prerequisites = { "M2_2" },
    Archetype = "mutator",
    EnemyFaction = "aegis",
    AIDifficulty = "Normal",
    StartingGold = 70,
    EnemyDeck = { "aeg_1", "aeg_3", "aeg_5", "aeg_6", "aeg_5", "aeg_6" },
    Briefing = "High-voltage conduit leaks overload optical targeting sensors. All ranged projectile attack cooldowns are increased by +40%, while melee units gain +25% move speed from motor overclocking!",
    Mutator = {
      id = "emp_scrambler",
      name = "Electromagnetic Interference",
      desc = "Ranged Triangle & Diamond attack cooldowns +40%. Melee Circle & Square movement speed +25%.",
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Destroy the Power Conduit Base Tower.",
      },
      {
        stars = 2,
        desc = "Win with at least 5 melee units (Square or Circle) alive on field.",
      },
      {
        stars = 3,
        desc = "Upgrade player income to Level 6 before completing the mission.",
      },
    },
    Rewards = {
      blueprints = { "aeg_3", "aeg_6" },
      goldBounty = 240,
      unlocksMissions = { "M2_4_BOSS" },
    },
  },
  ["M2_4_BOSS"] = {
    Id = "M2_4_BOSS",
    Chapter = 2,
    Name = "Citadel of Law: The Juggernaut Enforcer",
    TerritoryName = "Central Aegis High Command",
    PinCoord = Vector2.new(1860, 700),
    Prerequisites = { "M2_3" },
    Archetype = "titan_slayer",
    EnemyFaction = "aegis",
    AIDifficulty = "Hard",
    StartingGold = 85,
    EnemyDeck = { "aeg_1", "aeg_3", "aeg_4", "aeg_5", "aeg_7", "aeg_8" },
    Briefing = "Enforcer Unit Omega commands the Central Citadel. Wrapped in reactive titanium armor, its suppression pulse disables entire squadrons and its tactical aura crushes enemy defense!",
    BossProps = {
      name = "Enforcer Unit Omega",
      baseUnitId = "aeg_7",
      scale = 2.05,
      maxHp = 3400,
      dmg = 35,
      rng = 7,
      cd = 0.9,
      spd = 9.8,
      passives = { "-25% Damage Taken", "Aura: -20% Enemy Armor/Defense (24 studs)" },
      telegraphedAbility = {
        name = "Suppression Pulse",
        cooldown = 12,
        castTime = 1.5,
        aoeDmg = 65,
        radius = 200,
        slowPercent = 0.5,
        slowDuration = 4,
        color = "#38bdf8",
      },
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Demolish Enforcer Unit Omega and raze the Central Citadel.",
      },
      {
        stars = 2,
        desc = "Player Base Tower suffers zero damage throughout the encounter.",
      },
      {
        stars = 3,
        desc = "Slay Enforcer Unit Omega in under 3:15 minutes (195.0s).",
      },
    },
    Rewards = {
      blueprints = { "aeg_7", "aeg_8" },
      trophy = "aegis_star",
      goldBounty = 280,
      unlocksMissions = { "M4_1" },
    },
  },
  ["M3_1"] = {
    Id = "M3_1",
    Chapter = 3,
    Name = "Fungal Dunes Outpost",
    TerritoryName = "Spore Trench Perimeter",
    PinCoord = Vector2.new(220, 1020),
    Prerequisites = { "M1_4_BOSS" },
    Archetype = "siege",
    EnemyFaction = "hive",
    AIDifficulty = "Normal",
    StartingGold = 40,
    EnemyDeck = { "hiv_1", "hiv_2", "hiv_3", "hiv_1", "hiv_2", "hiv_3" },
    Briefing = "The southern continent is choking under a virulent fungal creep. The Alien Hive senses our presence and is spawning thousands of grunts beneath the dunes. Maintain firing lines and purge the tunnels!",
    StarCriteria = {
      {
        stars = 1,
        desc = "Destroy the Spore Trench Base Spire.",
      },
      {
        stars = 2,
        desc = "Slay at least 40 alien units during the match.",
      },
      {
        stars = 3,
        desc = "Complete the mission in under 2:45 minutes (165.0s).",
      },
    },
    Rewards = {
      blueprints = { "hiv_1", "hiv_2" },
      goldBounty = 180,
      unlocksMissions = { "M3_2" },
    },
  },
  ["M3_2"] = {
    Id = "M3_2",
    Chapter = 3,
    Name = "Acid Marsh Survival",
    TerritoryName = "Caustic Quagmire",
    PinCoord = Vector2.new(450, 970),
    Prerequisites = { "M3_1" },
    Archetype = "hold_the_line",
    EnemyFaction = "hive",
    AIDifficulty = "Hard",
    StartingGold = 50,
    EnemyDeck = { "hiv_1", "hiv_3", "hiv_4", "hiv_6", "hiv_4", "hiv_6" },
    Briefing = "Our artillery wagons are bogged down in the caustic quagmire! Alien bio-forms are converging from all flanks. Hold the perimeter for 3 minutes while the recovery team unhitches the treads!",
    TimeLimitSeconds = 180,
    Waves = {
      {
        triggerSec = 15,
        alertName = "Grunt Swarm",
        spawns = {
          {
            unitId = "hiv_1",
            count = 6,
            yOffset = 160,
          },
        },
      },
      {
        triggerSec = 45,
        alertName = "Carapace Vanguard",
        spawns = {
          {
            unitId = "hiv_3",
            count = 3,
            yOffset = 200,
          },
          {
            unitId = "hiv_2",
            count = 3,
            yOffset = 320,
          },
        },
      },
      {
        triggerSec = 80,
        alertName = "Mutated Warriors & Spitters",
        spawns = {
          {
            unitId = "hiv_4",
            count = 4,
            yOffset = 220,
          },
          {
            unitId = "hiv_6",
            count = 3,
            yOffset = 290,
          },
        },
      },
      {
        triggerSec = 120,
        alertName = "Mass Bio-Infestation",
        spawns = {
          {
            unitId = "hiv_1",
            count = 8,
            yOffset = 170,
          },
          {
            unitId = "hiv_4",
            count = 4,
            yOffset = 330,
          },
        },
      },
      {
        triggerSec = 155,
        alertName = "APEX BIO-HORDE",
        spawns = {
          {
            unitId = "hiv_7",
            count = 2,
            yOffset = 220,
          },
          {
            unitId = "hiv_6",
            count = 6,
            yOffset = 280,
          },
          {
            unitId = "hiv_3",
            count = 4,
            yOffset = 340,
          },
        },
      },
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Survive the full 3 minutes without Base Tower falling.",
      },
      {
        stars = 2,
        desc = "Outer Turret 1 (T1) remains alive at 00:00.",
      },
      {
        stars = 3,
        desc = "Never allow more than 2 friendly units to be poisoned simultaneously.",
      },
    },
    Rewards = {
      blueprints = { "hiv_4", "hiv_6" },
      goldBounty = 220,
      unlocksMissions = { "M3_3" },
    },
  },
  ["M3_3"] = {
    Id = "M3_3",
    Chapter = 3,
    Name = "Caustic Grotto",
    TerritoryName = "The Bile Chambers",
    PinCoord = Vector2.new(680, 1050),
    Prerequisites = { "M3_2" },
    Archetype = "mutator",
    EnemyFaction = "hive",
    AIDifficulty = "Hard",
    StartingGold = 75,
    EnemyDeck = { "hiv_1", "hiv_3", "hiv_4", "hiv_5", "hiv_6", "hiv_6" },
    Briefing = "The cavern atmosphere is dense with digestive acid fumes. Every unit takes 2 DMG every 1.5 seconds, UNLESS they are standing within 110px of an allied structure or an active Cross healer!",
    Mutator = {
      id = "corrosive_miasma",
      name = "Corrosive Miasma",
      desc = "All units suffer 2 DMG every 1.5s unless within 110px of an allied structure or healer shape.",
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Destroy the Bile Chamber Core.",
      },
      {
        stars = 2,
        desc = "Win with all 3 of your defense structures alive.",
      },
      {
        stars = 3,
        desc = "Cast player Commander Spell [E] (Healing/Surge) at least 3 times.",
      },
    },
    Rewards = {
      blueprints = { "hiv_3", "hiv_5" },
      goldBounty = 250,
      unlocksMissions = { "M3_4_BOSS" },
    },
  },
  ["M3_4_BOSS"] = {
    Id = "M3_4_BOSS",
    Chapter = 3,
    Name = "The Broodmother's Hive Core",
    TerritoryName = "Spire of the Apex Broodmother",
    PinCoord = Vector2.new(890, 1000),
    Prerequisites = { "M3_3" },
    Archetype = "titan_slayer",
    EnemyFaction = "hive",
    AIDifficulty = "Hard",
    StartingGold = 85,
    EnemyDeck = { "hiv_1", "hiv_4", "hiv_5", "hiv_6", "hiv_7", "hiv_9" },
    Briefing = "The Gorgon Leviathan guards the Hive Spire. Its colossal scythes tear through steel, it spawns endless bio-grunts from its carapace, and its toxic vomit melts flesh in seconds!",
    BossProps = {
      name = "Gorgon Leviathan",
      baseUnitId = "hiv_7",
      scale = 2.15,
      maxHp = 3600,
      dmg = 55,
      rng = 7.5,
      cd = 1.05,
      spd = 9.5,
      passives = { "Brood Spawner: Summons 2 Grunts / 8s", "Thick Carapace: Ignores 5 DMG from all hits" },
      telegraphedAbility = {
        name = "Caustic Acid Vomit",
        cooldown = 14,
        castTime = 1.6,
        aoeDmg = 85,
        lineLength = 280,
        poisonDps = 8,
        poisonDuration = 5,
        color = "#4ade80",
      },
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Slay the Gorgon Leviathan and destroy the Hive Core.",
      },
      {
        stars = 2,
        desc = "Player Base Tower sustains zero damage.",
      },
      {
        stars = 3,
        desc = "Slay the Leviathan in under 3:15 minutes (195.0s).",
      },
    },
    Rewards = {
      blueprints = { "hiv_7", "hiv_9" },
      trophy = "chitin_carapace",
      goldBounty = 280,
      unlocksMissions = { "M5_1" },
    },
  },
  ["M4_1"] = {
    Id = "M4_1",
    Chapter = 4,
    Name = "Event Horizon Chasm",
    TerritoryName = "Void Outpost Obsidian",
    PinCoord = Vector2.new(220, 240),
    Prerequisites = { "M2_4_BOSS" },
    Archetype = "siege",
    EnemyFaction = "void",
    AIDifficulty = "Normal",
    StartingGold = 50,
    EnemyDeck = { "voi_1", "voi_2", "voi_3", "voi_1", "voi_2", "voi_3" },
    Briefing = "We have climbed into the high atmospheric rift where reality bends. The Abyssal Void guards their dark-matter portals with cloaked assassins. Establish our beacon and collapse their forward vortex!",
    StarCriteria = {
      {
        stars = 1,
        desc = "Collapse the Obsidian Base Vortex.",
      },
      {
        stars = 2,
        desc = "Outer Turret 1 (T1) retains >= 60% HP at victory.",
      },
      {
        stars = 3,
        desc = "Conclude the match in under 2:30 minutes (150.0s).",
      },
    },
    Rewards = {
      blueprints = { "voi_1", "voi_2" },
      goldBounty = 260,
      unlocksMissions = { "M4_2" },
    },
  },
  ["M4_2"] = {
    Id = "M4_2",
    Chapter = 4,
    Name = "The Phantom Corridor",
    TerritoryName = "Corridor of Phantoms",
    PinCoord = Vector2.new(450, 180),
    Prerequisites = { "M4_1" },
    Archetype = "blitzkrieg",
    EnemyFaction = "void",
    AIDifficulty = "Hard",
    StartingGold = 85,
    EnemyDeck = { "voi_3", "voi_4", "voi_5", "voi_3", "voi_4", "voi_5" },
    Briefing = "At 02:00, the corridor completely implodes into a singularity, destroying all forces. Deploy high-speed strikers and break their nexus before reality shatters!",
    TimeLimitSeconds = 120,
    StarCriteria = {
      {
        stars = 1,
        desc = "Destroy the Corridor Nexus before 02:00.",
      },
      {
        stars = 2,
        desc = "Suffer zero deaths on friendly A-Tier Mythic units.",
      },
      {
        stars = 3,
        desc = "Destroy the enemy Base in under 1:35 minutes (95.0s).",
      },
    },
    Rewards = {
      blueprints = { "voi_4", "voi_5" },
      goldBounty = 290,
      unlocksMissions = { "M4_3" },
    },
  },
  ["M4_3"] = {
    Id = "M4_3",
    Chapter = 4,
    Name = "Null-Space Singularity",
    TerritoryName = "Gravity Distortion Well",
    PinCoord = Vector2.new(680, 270),
    Prerequisites = { "M4_2" },
    Archetype = "mutator",
    EnemyFaction = "void",
    AIDifficulty = "Hard",
    StartingGold = 90,
    EnemyDeck = { "voi_1", "voi_3", "voi_4", "voi_6", "voi_8", "voi_6" },
    Briefing = "The gravitational well bends spacetime. Soul Drainers' Defy Death immunity duration is doubled from 1.0s to 2.0 seconds! Trigger your own death immunities to turn the tide!",
    Mutator = {
      id = "gravity_distortion",
      name = "Gravity Inversion & Chrono Stasis",
      desc = "Death Immunity duration doubled to 2.0s. Projectiles curve inward toward lane center.",
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Destroy the Gravitational Well Generator.",
      },
      {
        stars = 2,
        desc = "Successfully trigger a friendly unit's Death Immunity passive at least 3 times.",
      },
      {
        stars = 3,
        desc = "Win without losing either Turret 1 or Turret 2.",
      },
    },
    Rewards = {
      blueprints = { "voi_3", "voi_6" },
      goldBounty = 310,
      unlocksMissions = { "M4_4_BOSS" },
    },
  },
  ["M4_4_BOSS"] = {
    Id = "M4_4_BOSS",
    Chapter = 4,
    Name = "The Void Nexus: Herald of Entropy",
    TerritoryName = "Sanctum of Sovereign Malakor",
    PinCoord = Vector2.new(890, 210),
    Prerequisites = { "M4_3" },
    Archetype = "titan_slayer",
    EnemyFaction = "void",
    AIDifficulty = "Hard",
    StartingGold = 95,
    EnemyDeck = { "voi_4", "voi_6", "voi_7", "voi_8", "voi_9", "voi_9" },
    Briefing = "Sovereign Malakor wields the scythe of absolute entropy. His dark aura erodes armor by 35% and his Black Hole pulls entire legions into a crushing singularity!",
    BossProps = {
      name = "Sovereign Malakor",
      baseUnitId = "voi_9",
      scale = 2.1,
      maxHp = 3500,
      dmg = 48,
      rng = 7.5,
      cd = 0.95,
      spd = 10,
      passives = { "Entropy Aura: -35% Enemy Armor (28 studs)", "Phase Shift: Immune to ranged hits for 1.5s / 10s" },
      telegraphedAbility = {
        name = "Black Hole Singularity",
        cooldown = 16,
        castTime = 1.5,
        aoeDmg = 90,
        radius = 220,
        color = "#a855f7",
      },
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Slay Sovereign Malakor and shatter the Void Nexus.",
      },
      {
        stars = 2,
        desc = "Inner Turret 2 (T2) never falls below 50% HP.",
      },
      {
        stars = 3,
        desc = "Slay Sovereign Malakor in under 3:00 minutes (180.0s).",
      },
    },
    Rewards = {
      blueprints = { "voi_7", "voi_8", "voi_9" },
      trophy = "void_singularity_heart",
      goldBounty = 330,
      unlocksMissions = { "M5_1", "M6_1" },
    },
  },
  ["M5_1"] = {
    Id = "M5_1",
    Chapter = 5,
    Name = "Magma Trench Incursion",
    TerritoryName = "Charred Redoubt 01",
    PinCoord = Vector2.new(1150, 1020),
    Prerequisites = { "M3_4_BOSS" },
    Archetype = "siege",
    EnemyFaction = "hellfire",
    AIDifficulty = "Normal",
    StartingGold = 50,
    EnemyDeck = { "hel_1", "hel_2", "hel_3", "hel_1", "hel_2", "hel_3" },
    Briefing = "We march across volcanic bridges above rivers of boiling magma. Hell hounds sprint at 16.5 speed—the fastest beasts on Geo-Terra. Hold rank and do not let their speed fracture our formation!",
    StarCriteria = {
      {
        stars = 1,
        desc = "Destroy the Charred Redoubt Base.",
      },
      {
        stars = 2,
        desc = "Outer Turret 1 (T1) remains standing with >= 75% HP.",
      },
      {
        stars = 3,
        desc = "Conclude the match in under 2:30 minutes (150.0s).",
      },
    },
    Rewards = {
      blueprints = { "hel_1", "hel_2" },
      goldBounty = 270,
      unlocksMissions = { "M5_2" },
    },
  },
  ["M5_2"] = {
    Id = "M5_2",
    Chapter = 5,
    Name = "Blood Gate Siegetrain",
    TerritoryName = "The Crimson Bastion",
    PinCoord = Vector2.new(1380, 970),
    Prerequisites = { "M5_1" },
    Archetype = "hold_the_line",
    EnemyFaction = "hellfire",
    AIDifficulty = "Hard",
    StartingGold = 60,
    EnemyDeck = { "hel_2", "hel_3", "hel_4", "hel_5", "hel_4", "hel_6" },
    Briefing = "Armored Blood Knights drink the blood of our soldiers to heal their wounds. Anchor our shield wall and repel the invasion waves for 3 minutes!",
    TimeLimitSeconds = 180,
    Waves = {
      {
        triggerSec = 15,
        alertName = "Hell Hound Sprint",
        spawns = {
          {
            unitId = "hel_2",
            count = 6,
            yOffset = 160,
          },
        },
      },
      {
        triggerSec = 45,
        alertName = "Soldiers & Flame Casters",
        spawns = {
          {
            unitId = "hel_3",
            count = 4,
            yOffset = 200,
          },
          {
            unitId = "hel_5",
            count = 3,
            yOffset = 330,
          },
        },
      },
      {
        triggerSec = 85,
        alertName = "Blood Knight Phalanx",
        spawns = {
          {
            unitId = "hel_4",
            count = 4,
            yOffset = 220,
          },
          {
            unitId = "hel_6",
            count = 3,
            yOffset = 290,
          },
        },
      },
      {
        triggerSec = 125,
        alertName = "Imp Swarm Rush",
        spawns = {
          {
            unitId = "hel_1",
            count = 8,
            yOffset = 170,
          },
          {
            unitId = "hel_4",
            count = 4,
            yOffset = 340,
          },
        },
      },
      {
        triggerSec = 160,
        alertName = "DEMONIC SIEGETRAIN",
        spawns = {
          {
            unitId = "hel_7",
            count = 2,
            yOffset = 220,
          },
          {
            unitId = "hel_4",
            count = 4,
            yOffset = 270,
          },
          {
            unitId = "hel_5",
            count = 4,
            yOffset = 330,
          },
        },
      },
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Hold the Crimson Bastion for the full 3 minutes without Base Tower falling.",
      },
      {
        stars = 2,
        desc = "Slay at least 50 enemy demons during the defense.",
      },
      {
        stars = 3,
        desc = "Win without losing any defense structure (T1, T2, Base all alive).",
      },
    },
    Rewards = {
      blueprints = { "hel_3", "hel_4" },
      goldBounty = 300,
      unlocksMissions = { "M5_3" },
    },
  },
  ["M5_3"] = {
    Id = "M5_3",
    Chapter = 5,
    Name = "The Infernal Caldera",
    TerritoryName = "Caldera of Molten Souls",
    PinCoord = Vector2.new(1620, 1050),
    Prerequisites = { "M5_2" },
    Archetype = "mutator",
    EnemyFaction = "hellfire",
    AIDifficulty = "Hard",
    StartingGold = 80,
    EnemyDeck = { "hel_1", "hel_3", "hel_4", "hel_5", "hel_6", "hel_6" },
    Briefing = "The searing heat boils the blood of all combatants. ALL units gain +35% Attack Speed and +25% Move Speed, but suffer 1.0% Max HP burn per second!",
    Mutator = {
      id = "blood_frenzy",
      name = "Blood Frenzy & Magma Heat",
      desc = "All units gain +35% ATK speed and +25% move speed, but take 1.0% Max HP burn damage per second.",
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Shatter the Caldera Base Tower.",
      },
      {
        stars = 2,
        desc = "Upgrade player income to Level 6 before completing the match.",
      },
      {
        stars = 3,
        desc = "Destroy the enemy Base in under 2:15 minutes (135.0s).",
      },
    },
    Rewards = {
      blueprints = { "hel_5", "hel_6" },
      goldBounty = 330,
      unlocksMissions = { "M5_4_BOSS" },
    },
  },
  ["M5_4_BOSS"] = {
    Id = "M5_4_BOSS",
    Chapter = 5,
    Name = "Throne of the Archfiend: Lord Malgok",
    TerritoryName = "The Obsidian Skull Citadel",
    PinCoord = Vector2.new(1860, 1000),
    Prerequisites = { "M5_3" },
    Archetype = "titan_slayer",
    EnemyFaction = "hellfire",
    AIDifficulty = "Hard",
    StartingGold = 90,
    EnemyDeck = { "hel_4", "hel_6", "hel_7", "hel_8", "hel_9", "hel_9" },
    Briefing = "Lord Malgok towers over the obsidian throne. His flaming sword heals him for 50 HP on every swing and his Meteor Rain burns entire armies to ash!",
    BossProps = {
      name = "Arch-Demon Lord Malgok",
      baseUnitId = "hel_9",
      scale = 2.2,
      maxHp = 4200,
      dmg = 65,
      rng = 7.5,
      cd = 0.9,
      spd = 10.5,
      passives = { "Greater Vampirism: +50 HP Lifesteal on strike", "-20% Ranged DMG Taken" },
      telegraphedAbility = {
        name = "Cataclysmic Meteor Rain",
        cooldown = 14,
        castTime = 1.8,
        aoeDmg = 115,
        radius = 120,
        meteorCount = 3,
        color = "#f43f5e",
      },
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Slay Arch-Demon Lord Malgok and raze the Skull Citadel.",
      },
      {
        stars = 2,
        desc = "Player Base Tower sustains at least 1,200 HP at match conclusion.",
      },
      {
        stars = 3,
        desc = "Slay Lord Malgok in under 3:30 minutes (210.0s).",
      },
    },
    Rewards = {
      blueprints = { "hel_7", "hel_8", "hel_9" },
      trophy = "horn_of_malgok",
      goldBounty = 360,
      unlocksMissions = { "M6_1" },
    },
  },
  ["M6_1"] = {
    Id = "M6_1",
    Chapter = 6,
    Name = "Automated Perimeter Breaker",
    TerritoryName = "Cyber-Citadel Trench 01",
    PinCoord = Vector2.new(1150, 240),
    Prerequisites = { "M4_4_BOSS", "M5_4_BOSS" },
    Archetype = "siege",
    EnemyFaction = "mechanica",
    AIDifficulty = "Hard",
    StartingGold = 60,
    EnemyDeck = { "mec_1", "mec_2", "mec_3", "mec_1", "mec_2", "mec_3" },
    Briefing = "We have reached Sector Zero, the central brain of Geo-Terra. The Mechanica Corps builds war machines that feel no pain or fear. Shatter their sentries!",
    StarCriteria = {
      {
        stars = 1,
        desc = "Destroy the Cyber-Citadel Base Tower.",
      },
      {
        stars = 2,
        desc = "Suffer fewer than 10 total friendly unit casualties during the entire battle.",
      },
      {
        stars = 3,
        desc = "Finish the match in under 2:45 minutes (165.0s).",
      },
    },
    Rewards = {
      blueprints = { "mec_1", "mec_2" },
      goldBounty = 350,
      unlocksMissions = { "M6_2" },
    },
  },
  ["M6_2"] = {
    Id = "M6_2",
    Chapter = 6,
    Name = "Siege Engine Blitzkrieg",
    TerritoryName = "Railgun Assembly Foundry",
    PinCoord = Vector2.new(1380, 180),
    Prerequisites = { "M6_1" },
    Archetype = "blitzkrieg",
    EnemyFaction = "mechanica",
    AIDifficulty = "Hard",
    StartingGold = 90,
    EnemyDeck = { "mec_3", "mec_4", "mec_5", "mec_3", "mec_4", "mec_5" },
    Briefing = "At 02:20, the foundry superlasers complete charging and vaporize our army. Push forward with maximum tempo and destroy their power couplings!",
    TimeLimitSeconds = 140,
    StarCriteria = {
      {
        stars = 1,
        desc = "Destroy the Assembly Foundry Base before 02:20.",
      },
      {
        stars = 2,
        desc = "Cast Commander Spell [Q] at least twice during the siege.",
      },
      {
        stars = 3,
        desc = "Destroy the enemy Base in under 1:45 minutes (105.0s).",
      },
    },
    Rewards = {
      blueprints = { "mec_4", "mec_5" },
      goldBounty = 380,
      unlocksMissions = { "M6_3" },
    },
  },
  ["M6_3"] = {
    Id = "M6_3",
    Chapter = 6,
    Name = "Overclocked Factory Core",
    TerritoryName = "Nanite Fusion Reactor",
    PinCoord = Vector2.new(1620, 270),
    Prerequisites = { "M6_2" },
    Archetype = "mutator",
    EnemyFaction = "mechanica",
    AIDifficulty = "Extreme",
    StartingGold = 100,
    EnemyDeck = { "mec_1", "mec_3", "mec_4", "mec_6", "mec_8", "mec_8" },
    Briefing = "Microscopic repair nanites infuse the atmosphere. All mechanical units (Square & Hexagon) regenerate 5% Max HP every 3 seconds and gain +20% move speed!",
    Mutator = {
      id = "nanite_regeneration",
      name = "Autonomous Nanite Matrix",
      desc = "All Square & Hexagon units regenerate 5% Max HP every 3s and gain +20% move speed.",
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Destroy the Nanite Fusion Reactor Base.",
      },
      {
        stars = 2,
        desc = "Destroy all three enemy defense structures (T1, T2, and Base).",
      },
      {
        stars = 3,
        desc = "Upgrade player income to Level 7 before match completion.",
      },
    },
    Rewards = {
      blueprints = { "mec_3", "mec_6" },
      goldBounty = 420,
      unlocksMissions = { "M6_4_FINAL_BOSS" },
    },
  },
  ["M6_4_FINAL_BOSS"] = {
    Id = "M6_4_FINAL_BOSS",
    Chapter = 6,
    Name = "The Core Prime: Cybernetic Colossus",
    TerritoryName = "Central Processing Throne: Geo-Terra Core",
    PinCoord = Vector2.new(1860, 210),
    Prerequisites = { "M6_3" },
    Archetype = "titan_slayer",
    EnemyFaction = "mechanica",
    AIDifficulty = "Extreme",
    StartingGold = 110,
    EnemyDeck = { "mec_4", "mec_6", "mec_7", "mec_8", "mec_9", "mec_9" },
    Briefing = "Colossus Prime, the autonomous superintelligence of Geo-Terra, activates. Dual autocannons, orbital ion strikes, and an emergency nanite barrier stand between us and planetary liberation!",
    BossProps = {
      name = "Colossus Prime",
      baseUnitId = "mec_7",
      scale = 2.4,
      maxHp = 5500,
      dmg = 75,
      rng = 8.5,
      cd = 0.95,
      spd = 9,
      passives = { "Cyber Speed Aura: +20% ATK Spd (28 studs)", "Enrage at 25% HP: +35% ATK Spd & +25% Speed" },
      hasPhase2Shield = true,
      phase2ShieldAmount = 1200,
      telegraphedAbility = {
        name = "Orbital Ion Cannon Death-Ray",
        cooldown = 18,
        castTime = 2,
        aoeDmg = 160,
        radius = 150,
        color = "#38bdf8",
      },
    },
    StarCriteria = {
      {
        stars = 1,
        desc = "Slay Colossus Prime and liberate all of Geo-Terra!",
      },
      {
        stars = 2,
        desc = "Player Outer Turret 1 (T1) survives the final boss encounter.",
      },
      {
        stars = 3,
        desc = "Defeat Colossus Prime in under 4:00 minutes (240.0s).",
      },
    },
    Rewards = {
      blueprints = { "mec_7", "mec_8", "mec_9" },
      trophy = "crown_of_geo_terra",
      goldBounty = 1000,
      unlocksMissions = {},
    },
  },
}


-- Quick Access Helpers
function ConquestConfig.GetMission(missionId: string)
  return ConquestConfig.Missions[missionId]
end

function ConquestConfig.GetRelic(relicId: string)
  return ConquestConfig.Relics[relicId]
end

function ConquestConfig.GetMissionsForChapter(chapterNum: number)
  local list = {}
  for _, m in pairs(ConquestConfig.Missions) do
    if m.Chapter == chapterNum then
      table.insert(list, m)
    end
  end
  table.sort(list, function(a, b) return a.Id < b.Id end)
  return list
end

function ConquestConfig.IsMissionUnlocked(missionId: string, completedMissions: { [string]: any })
  local m = ConquestConfig.Missions[missionId]
  if not m then return false end
  if not m.Prerequisites or #m.Prerequisites == 0 then return true end
  for _, reqId in ipairs(m.Prerequisites) do
    local record = completedMissions[reqId]
    if not record or (record.Stars or 0) <= 0 then
      return false
    end
  end
  return true
end

return ConquestConfig
