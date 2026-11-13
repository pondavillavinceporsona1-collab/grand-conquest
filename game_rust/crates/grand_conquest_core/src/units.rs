use crate::model::{FactionDef, FactionId, PassiveKey, SpellFxType, UnitDef, UnitShape, UnitTier};

pub const FACTIONS: [FactionDef; 6] = [
    FactionDef {
        id: FactionId::Solaris,
        q_spell_name: "Excalibur Sky-Strike",
        q_spell_desc: "Giant golden blade deals 115 AoE DMG (16-stud)",
        q_fx_type: SpellFxType::Excalibur,
        e_spell_name: "Royal Aegis",
        e_spell_desc: "Heals allies +75 HP & +45% SPD for 4s",
        affinity_3pc_desc: "+15% Healing Received (6-pc: x2 & -10s Q/E CD)",
    },
    FactionDef {
        id: FactionId::Aegis,
        q_spell_name: "Orbital Airstrike",
        q_spell_desc: "Tactical missiles deal 115 AoE DMG (16-stud)",
        q_fx_type: SpellFxType::Airstrike,
        e_spell_name: "Tactical Overclock",
        e_spell_desc: "Nano-stim heals +75 HP & +45% SPD for 4s",
        affinity_3pc_desc: "+10% Ranged ATK Speed (6-pc: x2 & -10s Q/E CD)",
    },
    FactionDef {
        id: FactionId::Hive,
        q_spell_name: "Mothership Plasma Beam",
        q_spell_desc: "Bio-laser sweeps for 115 AoE DMG",
        q_fx_type: SpellFxType::Plasma,
        e_spell_name: "Hive Frenzy",
        e_spell_desc: "Heals Aliens +75 HP & surges +45% SPD",
        affinity_3pc_desc: "+1.5 Studs/s Move Speed (6-pc: x2 & -10s Q/E CD)",
    },
    FactionDef {
        id: FactionId::Hellfire,
        q_spell_name: "Hellfire Cataclysm",
        q_spell_desc: "Molten meteor deals 115 AoE DMG",
        q_fx_type: SpellFxType::Hellfire,
        e_spell_name: "Bloodlust Pact",
        e_spell_desc: "Heals +75 HP & ignites +45% furious SPD",
        affinity_3pc_desc: "+5 HP Restored on Kill (6-pc: x2 & -10s Q/E CD)",
    },
    FactionDef {
        id: FactionId::Void,
        q_spell_name: "Event Horizon",
        q_spell_desc: "Black hole implodes for 115 AoE DMG",
        q_fx_type: SpellFxType::Void,
        e_spell_name: "Phase Veil",
        e_spell_desc: "Restores +75 HP & phases +45% SPD",
        affinity_3pc_desc: "-10% Enemy DEF Aura 16u (6-pc: x2 & -10s Q/E CD)",
    },
    FactionDef {
        id: FactionId::Mechanica,
        q_spell_name: "Ion Cannon Barrage",
        q_spell_desc: "Satellite beam deals 115 AoE DMG",
        q_fx_type: SpellFxType::Ion,
        e_spell_name: "Nanobot Overdrive",
        e_spell_desc: "Repairs +75 HP & overclocks +45% SPD",
        affinity_3pc_desc: "+8% Max HP to All Units (6-pc: x2 & -10s Q/E CD)",
    },
];

pub const UNITS: [UnitDef; 54] = [
    // ======================== SOLARIS EMPIRE ========================
    UnitDef { id: "sol_1", num: 1, faction: FactionId::Solaris, tier: UnitTier::C, name: "Footman", cost: 6, hp: 90.0, dmg: 15.0, rng_studs: 5.5, cd: 1.0, spd_studs: 14.0, shape: UnitShape::Circle, passive: PassiveKey::None, passive_desc: "Standard melee strike", sprite_key: "solaris_footman", unlock_hint: "Starter / Chapter 1" },
    UnitDef { id: "sol_2", num: 2, faction: FactionId::Solaris, tier: UnitTier::C, name: "Archer", cost: 9, hp: 50.0, dmg: 12.0, rng_studs: 24.0, cd: 1.05, spd_studs: 13.0, shape: UnitShape::Triangle, passive: PassiveKey::None, passive_desc: "Ranged arrow volley", sprite_key: "solaris_archer", unlock_hint: "Starter / Chapter 1" },
    UnitDef { id: "sol_3", num: 3, faction: FactionId::Solaris, tier: UnitTier::C, name: "Shield Man", cost: 10, hp: 130.0, dmg: 8.0, rng_studs: 5.5, cd: 1.1, spd_studs: 12.0, shape: UnitShape::Square, passive: PassiveKey::None, passive_desc: "High-HP early blocker", sprite_key: "solaris_shield_man", unlock_hint: "Starter / Chapter 1" },
    UnitDef { id: "sol_4", num: 4, faction: FactionId::Solaris, tier: UnitTier::B, name: "Knight", cost: 16, hp: 160.0, dmg: 22.0, rng_studs: 6.0, cd: 1.0, spd_studs: 14.5, shape: UnitShape::Square, passive: PassiveKey::DmgRed25, passive_desc: "-25% DMG Taken", sprite_key: "solaris_knight", unlock_hint: "Clear M1.1" },
    UnitDef { id: "sol_5", num: 5, faction: FactionId::Solaris, tier: UnitTier::B, name: "Mage", cost: 20, hp: 85.0, dmg: 30.0, rng_studs: 25.0, cd: 1.3, spd_studs: 12.0, shape: UnitShape::Diamond, passive: PassiveKey::FourthHit45, passive_desc: "4th Hit: 45 DMG (1.5x)", sprite_key: "solaris_mage", unlock_hint: "Clear M1.3" },
    UnitDef { id: "sol_6", num: 6, faction: FactionId::Solaris, tier: UnitTier::B, name: "Priest", cost: 32, hp: 80.0, dmg: 8.0, rng_studs: 28.0, cd: 1.5, spd_studs: 12.0, shape: UnitShape::Cross, passive: PassiveKey::Heal20_3s5u, passive_desc: "Heals 20 HP/3s (5 allies)", sprite_key: "solaris_priest", unlock_hint: "Clear M1.2" },
    UnitDef { id: "sol_7", num: 7, faction: FactionId::Solaris, tier: UnitTier::A, name: "Elite Knight", cost: 35, hp: 280.0, dmg: 35.0, rng_studs: 6.5, cd: 1.0, spd_studs: 13.5, shape: UnitShape::Hexagon, passive: PassiveKey::EliteKnight, passive_desc: "-25% DMG; +10% ATK Spd Aura", sprite_key: "solaris_elite_knight", unlock_hint: "Clear M1.4 Boss" },
    UnitDef { id: "sol_8", num: 8, faction: FactionId::Solaris, tier: UnitTier::A, name: "Elite Mage", cost: 45, hp: 175.0, dmg: 50.0, rng_studs: 28.0, cd: 1.4, spd_studs: 11.5, shape: UnitShape::Diamond, passive: PassiveKey::None, passive_desc: "Heavy arcane artillery", sprite_key: "solaris_elite_mage", unlock_hint: "Chapter 1 Mastery (12 Stars)" },
    UnitDef { id: "sol_9", num: 9, faction: FactionId::Solaris, tier: UnitTier::A, name: "High Priest", cost: 55, hp: 160.0, dmg: 12.0, rng_studs: 30.0, cd: 1.5, spd_studs: 11.5, shape: UnitShape::Cross, passive: PassiveKey::Heal30_4s99u, passive_desc: "Heals 30 HP/4s (99 allies)", sprite_key: "solaris_high_priest", unlock_hint: "Chapter 1 Mastery (12 Stars)" },

    // ======================== AEGIS COALITION ========================
    UnitDef { id: "aeg_1", num: 1, faction: FactionId::Aegis, tier: UnitTier::C, name: "Riot Trooper", cost: 7, hp: 120.0, dmg: 12.0, rng_studs: 5.5, cd: 1.0, spd_studs: 13.5, shape: UnitShape::Square, passive: PassiveKey::None, passive_desc: "Frontline baton trooper", sprite_key: "aegis_riot_trooper", unlock_hint: "Clear M2.1" },
    UnitDef { id: "aeg_2", num: 2, faction: FactionId::Aegis, tier: UnitTier::C, name: "Police", cost: 9, hp: 50.0, dmg: 12.0, rng_studs: 23.0, cd: 1.0, spd_studs: 13.5, shape: UnitShape::Triangle, passive: PassiveKey::None, passive_desc: "Sidearm ranged support", sprite_key: "aegis_police", unlock_hint: "Clear M2.1" },
    UnitDef { id: "aeg_3", num: 3, faction: FactionId::Aegis, tier: UnitTier::C, name: "Medic Bot", cost: 12, hp: 60.0, dmg: 5.0, rng_studs: 26.0, cd: 1.4, spd_studs: 13.0, shape: UnitShape::Cross, passive: PassiveKey::Heal10_3s3u, passive_desc: "Heals 10 HP/3s (3 allies)", sprite_key: "aegis_medic_bot", unlock_hint: "Clear M2.3" },
    UnitDef { id: "aeg_4", num: 4, faction: FactionId::Aegis, tier: UnitTier::B, name: "Rifleman", cost: 18, hp: 85.0, dmg: 20.0, rng_studs: 24.0, cd: 0.95, spd_studs: 13.0, shape: UnitShape::Triangle, passive: PassiveKey::Sniper36, passive_desc: "Sniper: 36 RNG (1st 3 shots)", sprite_key: "aegis_rifleman", unlock_hint: "Clear M2.2" },
    UnitDef { id: "aeg_5", num: 5, faction: FactionId::Aegis, tier: UnitTier::B, name: "Riot Shield", cost: 20, hp: 200.0, dmg: 12.0, rng_studs: 5.8, cd: 1.05, spd_studs: 11.5, shape: UnitShape::Square, passive: PassiveKey::DmgRed25, passive_desc: "-25% DMG Taken", sprite_key: "aegis_riot_shield", unlock_hint: "Clear M2.2" },
    UnitDef { id: "aeg_6", num: 6, faction: FactionId::Aegis, tier: UnitTier::B, name: "Grenade Thrower", cost: 25, hp: 95.0, dmg: 35.0, rng_studs: 25.0, cd: 1.4, spd_studs: 12.0, shape: UnitShape::Diamond, passive: PassiveKey::FirstHit50, passive_desc: "1st Hit: 50 DMG burst", sprite_key: "aegis_grenade_thrower", unlock_hint: "Clear M2.3" },
    UnitDef { id: "aeg_7", num: 7, faction: FactionId::Aegis, tier: UnitTier::A, name: "Elite Riot", cost: 35, hp: 300.0, dmg: 20.0, rng_studs: 6.2, cd: 1.0, spd_studs: 12.0, shape: UnitShape::Hexagon, passive: PassiveKey::EliteRiot, passive_desc: "-15% DMG; -15% DEF Aura", sprite_key: "aegis_elite_riot", unlock_hint: "Clear M2.4 Boss" },
    UnitDef { id: "aeg_8", num: 8, faction: FactionId::Aegis, tier: UnitTier::A, name: "Shotgunner", cost: 40, hp: 160.0, dmg: 35.0, rng_studs: 17.0, cd: 1.05, spd_studs: 13.5, shape: UnitShape::Triangle, passive: PassiveKey::ShotgunFalloff, passive_desc: "35 DMG close / 15 long", sprite_key: "aegis_shotgunner", unlock_hint: "Clear M2.4 Boss" },
    UnitDef { id: "aeg_9", num: 9, faction: FactionId::Aegis, tier: UnitTier::A, name: "Heavy Gunner", cost: 45, hp: 220.0, dmg: 17.0, rng_studs: 26.0, cd: 0.4, spd_studs: 8.5, shape: UnitShape::Triangle, passive: PassiveKey::Gatling, passive_desc: "Rapid Fire (0.4s CD)", sprite_key: "aegis_heavy_gunner", unlock_hint: "Chapter 2 Mastery (12 Stars)" },

    // ======================== HIVE MIND ========================
    UnitDef { id: "hiv_1", num: 1, faction: FactionId::Hive, tier: UnitTier::C, name: "Alien Grunt", cost: 6, hp: 65.0, dmg: 18.0, rng_studs: 5.5, cd: 0.95, spd_studs: 15.5, shape: UnitShape::Circle, passive: PassiveKey::None, passive_desc: "Fast bio-swarmling", sprite_key: "hive_alien_grunt", unlock_hint: "Clear M3.1" },
    UnitDef { id: "hiv_2", num: 2, faction: FactionId::Hive, tier: UnitTier::C, name: "Plasma Shooter", cost: 10, hp: 45.0, dmg: 15.0, rng_studs: 23.0, cd: 1.0, spd_studs: 13.5, shape: UnitShape::Triangle, passive: PassiveKey::None, passive_desc: "Ranged plasma needles", sprite_key: "hive_plasma_shooter", unlock_hint: "Clear M3.1" },
    UnitDef { id: "hiv_3", num: 3, faction: FactionId::Hive, tier: UnitTier::C, name: "Alien Guard", cost: 9, hp: 110.0, dmg: 8.0, rng_studs: 5.5, cd: 1.1, spd_studs: 12.0, shape: UnitShape::Square, passive: PassiveKey::None, passive_desc: "Carapace blocker", sprite_key: "hive_alien_guard", unlock_hint: "Clear M3.3" },
    UnitDef { id: "hiv_4", num: 4, faction: FactionId::Hive, tier: UnitTier::B, name: "Mutated Warrior", cost: 15, hp: 140.0, dmg: 25.0, rng_studs: 6.0, cd: 1.0, spd_studs: 14.0, shape: UnitShape::Circle, passive: PassiveKey::None, passive_desc: "Aggressive bio-bruiser", sprite_key: "hive_mutated_warrior", unlock_hint: "Clear M3.2" },
    UnitDef { id: "hiv_5", num: 5, faction: FactionId::Hive, tier: UnitTier::B, name: "Plasma Specialist", cost: 22, hp: 80.0, dmg: 35.0, rng_studs: 25.0, cd: 1.25, spd_studs: 12.5, shape: UnitShape::Diamond, passive: PassiveKey::None, passive_desc: "High-output bio-caster", sprite_key: "hive_plasma_specialist", unlock_hint: "Clear M3.3" },
    UnitDef { id: "hiv_6", num: 6, faction: FactionId::Hive, tier: UnitTier::B, name: "Acid Spitter", cost: 24, hp: 90.0, dmg: 20.0, rng_studs: 23.0, cd: 1.2, spd_studs: 12.5, shape: UnitShape::Diamond, passive: PassiveKey::AcidPoison, passive_desc: "5 Poison DPS for 4.0s", sprite_key: "hive_acid_spitter", unlock_hint: "Clear M3.2" },
    UnitDef { id: "hiv_7", num: 7, faction: FactionId::Hive, tier: UnitTier::A, name: "Alien Juggernaut", cost: 35, hp: 300.0, dmg: 40.0, rng_studs: 6.5, cd: 1.05, spd_studs: 12.0, shape: UnitShape::Hexagon, passive: PassiveKey::None, passive_desc: "Massive apex titan", sprite_key: "hive_alien_juggernaut", unlock_hint: "Clear M3.4 Boss" },
    UnitDef { id: "hiv_8", num: 8, faction: FactionId::Hive, tier: UnitTier::A, name: "Plasma Overlord", cost: 45, hp: 190.0, dmg: 55.0, rng_studs: 27.0, cd: 1.45, spd_studs: 11.5, shape: UnitShape::Diamond, passive: PassiveKey::None, passive_desc: "Long-range bio-artillery", sprite_key: "hive_plasma_overlord", unlock_hint: "Chapter 3 Mastery (12 Stars)" },
    UnitDef { id: "hiv_9", num: 9, faction: FactionId::Hive, tier: UnitTier::A, name: "Hive Commander", cost: 45, hp: 240.0, dmg: 28.0, rng_studs: 24.0, cd: 1.15, spd_studs: 12.0, shape: UnitShape::Hexagon, passive: PassiveKey::Broodmother, passive_desc: "Summons 2 Grunts / 11s", sprite_key: "hive_hive_commander", unlock_hint: "Clear M3.4 Boss" },

    // ======================== HELLFIRE LEGION ========================
    UnitDef { id: "hel_1", num: 1, faction: FactionId::Hellfire, tier: UnitTier::C, name: "Imp", cost: 6, hp: 45.0, dmg: 20.0, rng_studs: 5.5, cd: 0.9, spd_studs: 16.0, shape: UnitShape::Circle, passive: PassiveKey::None, passive_desc: "High-speed glass cannon", sprite_key: "hellfire_imp", unlock_hint: "Clear M5.1" },
    UnitDef { id: "hel_2", num: 2, faction: FactionId::Hellfire, tier: UnitTier::C, name: "Hell Hound", cost: 8, hp: 70.0, dmg: 18.0, rng_studs: 5.5, cd: 0.9, spd_studs: 16.5, shape: UnitShape::Circle, passive: PassiveKey::None, passive_desc: "Fastest unit in game (16.5)", sprite_key: "hellfire_hell_hound", unlock_hint: "Clear M5.1" },
    UnitDef { id: "hel_3", num: 3, faction: FactionId::Hellfire, tier: UnitTier::C, name: "Demon Soldier", cost: 9, hp: 100.0, dmg: 15.0, rng_studs: 5.8, cd: 1.0, spd_studs: 13.0, shape: UnitShape::Square, passive: PassiveKey::None, passive_desc: "Sturdy legionnaire", sprite_key: "hellfire_demon_soldier", unlock_hint: "Clear M5.2" },
    UnitDef { id: "hel_4", num: 4, faction: FactionId::Hellfire, tier: UnitTier::B, name: "Blood Knight", cost: 18, hp: 200.0, dmg: 20.0, rng_studs: 6.0, cd: 1.0, spd_studs: 14.0, shape: UnitShape::Square, passive: PassiveKey::Lifesteal30, passive_desc: "+30 HP Lifesteal per hit", sprite_key: "hellfire_blood_knight", unlock_hint: "Clear M5.2" },
    UnitDef { id: "hel_5", num: 5, faction: FactionId::Hellfire, tier: UnitTier::B, name: "Flame Caster", cost: 22, hp: 80.0, dmg: 35.0, rng_studs: 24.0, cd: 1.3, spd_studs: 12.0, shape: UnitShape::Diamond, passive: PassiveKey::None, passive_desc: "Hurls brimstone fireballs", sprite_key: "hellfire_flame_caster", unlock_hint: "Clear M5.3" },
    UnitDef { id: "hel_6", num: 6, faction: FactionId::Hellfire, tier: UnitTier::B, name: "Soul Reaper", cost: 24, hp: 110.0, dmg: 25.0, rng_studs: 6.2, cd: 1.0, spd_studs: 14.0, shape: UnitShape::Circle, passive: PassiveKey::SoulHarvest, passive_desc: "+10 HP restored on Kill", sprite_key: "hellfire_soul_reaper", unlock_hint: "Clear M5.3" },
    UnitDef { id: "hel_7", num: 7, faction: FactionId::Hellfire, tier: UnitTier::A, name: "Demon Brute", cost: 38, hp: 350.0, dmg: 40.0, rng_studs: 6.5, cd: 1.1, spd_studs: 12.0, shape: UnitShape::Hexagon, passive: PassiveKey::None, passive_desc: "350 HP Hulking Behemoth", sprite_key: "hellfire_demon_brute", unlock_hint: "Clear M5.4 Boss" },
    UnitDef { id: "hel_8", num: 8, faction: FactionId::Hellfire, tier: UnitTier::A, name: "Infernal Archmage", cost: 48, hp: 195.0, dmg: 60.0, rng_studs: 27.0, cd: 1.45, spd_studs: 11.5, shape: UnitShape::Diamond, passive: PassiveKey::None, passive_desc: "60 DMG Hellfire Nuker", sprite_key: "hellfire_infernal_archmage", unlock_hint: "Clear M5.4 Boss" },
    UnitDef { id: "hel_9", num: 9, faction: FactionId::Hellfire, tier: UnitTier::A, name: "Demon Lord", cost: 50, hp: 280.0, dmg: 40.0, rng_studs: 6.5, cd: 1.0, spd_studs: 13.0, shape: UnitShape::Hexagon, passive: PassiveKey::Lifesteal50, passive_desc: "+50 HP Lifesteal per hit", sprite_key: "hellfire_demon_lord", unlock_hint: "Clear M5.4 Boss" },

    // ======================== ABYSSAL VOID ========================
    UnitDef { id: "voi_1", num: 1, faction: FactionId::Void, tier: UnitTier::C, name: "Shadowling", cost: 6, hp: 50.0, dmg: 18.0, rng_studs: 5.5, cd: 0.95, spd_studs: 15.5, shape: UnitShape::Circle, passive: PassiveKey::None, passive_desc: "Swift shadow striker", sprite_key: "abyssal_shadowling", unlock_hint: "Clear M4.1" },
    UnitDef { id: "voi_2", num: 2, faction: FactionId::Void, tier: UnitTier::C, name: "Void Archer", cost: 10, hp: 40.0, dmg: 14.0, rng_studs: 24.0, cd: 1.0, spd_studs: 13.0, shape: UnitShape::Triangle, passive: PassiveKey::None, passive_desc: "Dark-matter arrows", sprite_key: "abyssal_void_archer", unlock_hint: "Clear M4.1" },
    UnitDef { id: "voi_3", num: 3, faction: FactionId::Void, tier: UnitTier::C, name: "Dark Sentinel", cost: 9, hp: 100.0, dmg: 8.0, rng_studs: 5.5, cd: 1.1, spd_studs: 12.0, shape: UnitShape::Square, passive: PassiveKey::None, passive_desc: "Obsidian frontline guard", sprite_key: "abyssal_dark_sentinel", unlock_hint: "Clear M4.3" },
    UnitDef { id: "voi_4", num: 4, faction: FactionId::Void, tier: UnitTier::B, name: "Void Stalker", cost: 17, hp: 120.0, dmg: 30.0, rng_studs: 6.0, cd: 1.0, spd_studs: 14.5, shape: UnitShape::Circle, passive: PassiveKey::None, passive_desc: "High-DPS melee assassin", sprite_key: "abyssal_void_stalker", unlock_hint: "Clear M4.2" },
    UnitDef { id: "voi_5", num: 5, faction: FactionId::Void, tier: UnitTier::B, name: "Abyss Mage", cost: 23, hp: 75.0, dmg: 40.0, rng_studs: 25.0, cd: 1.35, spd_studs: 12.0, shape: UnitShape::Diamond, passive: PassiveKey::None, passive_desc: "40 DMG dark caster", sprite_key: "abyssal_abyss_mage", unlock_hint: "Clear M4.2" },
    UnitDef { id: "voi_6", num: 6, faction: FactionId::Void, tier: UnitTier::B, name: "Soul Drainer", cost: 24, hp: 100.0, dmg: 15.0, rng_studs: 28.0, cd: 0.5, spd_studs: 12.5, shape: UnitShape::Diamond, passive: PassiveKey::DeathImmunity, passive_desc: "1s Death Immunity at 1 HP", sprite_key: "abyssal_soul_drainer", unlock_hint: "Clear M4.3" },
    UnitDef { id: "voi_7", num: 7, faction: FactionId::Void, tier: UnitTier::A, name: "Void Reaper", cost: 38, hp: 260.0, dmg: 45.0, rng_studs: 6.5, cd: 1.0, spd_studs: 13.5, shape: UnitShape::Hexagon, passive: PassiveKey::None, passive_desc: "Mythic scythe executioner", sprite_key: "abyssal_void_reaper", unlock_hint: "Clear M4.4 Boss" },
    UnitDef { id: "voi_8", num: 8, faction: FactionId::Void, tier: UnitTier::A, name: "Abyssal Sorcerer", cost: 48, hp: 190.0, dmg: 65.0, rng_studs: 28.0, cd: 1.5, spd_studs: 11.5, shape: UnitShape::Diamond, passive: PassiveKey::None, passive_desc: "65 DMG Supreme Nuker", sprite_key: "abyssal_abyssal_sorcerer", unlock_hint: "Clear M4.4 Boss" },
    UnitDef { id: "voi_9", num: 9, faction: FactionId::Void, tier: UnitTier::A, name: "Void Warden", cost: 50, hp: 270.0, dmg: 35.0, rng_studs: 6.5, cd: 1.05, spd_studs: 12.0, shape: UnitShape::Hexagon, passive: PassiveKey::VoidWarden, passive_desc: "-20% DEF Aura (24 studs)", sprite_key: "abyssal_void_warden", unlock_hint: "Clear M4.4 Boss" },

    // ======================== MECHANICA CORPS ========================
    UnitDef { id: "mec_1", num: 1, faction: FactionId::Mechanica, tier: UnitTier::C, name: "Combat Bot", cost: 8, hp: 100.0, dmg: 15.0, rng_studs: 5.5, cd: 1.0, spd_studs: 13.5, shape: UnitShape::Circle, passive: PassiveKey::None, passive_desc: "Balanced chassis infantry", sprite_key: "mechanica_combat_bot", unlock_hint: "Clear M6.1" },
    UnitDef { id: "mec_2", num: 2, faction: FactionId::Mechanica, tier: UnitTier::C, name: "Assault Drone", cost: 10, hp: 50.0, dmg: 12.0, rng_studs: 24.0, cd: 0.95, spd_studs: 14.5, shape: UnitShape::Triangle, passive: PassiveKey::None, passive_desc: "Rapid laser drone", sprite_key: "mechanica_assault_drone", unlock_hint: "Clear M6.1" },
    UnitDef { id: "mec_3", num: 3, faction: FactionId::Mechanica, tier: UnitTier::C, name: "Shield Robot", cost: 12, hp: 150.0, dmg: 8.0, rng_studs: 5.5, cd: 1.1, spd_studs: 12.0, shape: UnitShape::Square, passive: PassiveKey::None, passive_desc: "150 HP Titanium wall", sprite_key: "mechanica_shield_robot", unlock_hint: "Clear M6.3" },
    UnitDef { id: "mec_4", num: 4, faction: FactionId::Mechanica, tier: UnitTier::B, name: "Warbot", cost: 18, hp: 200.0, dmg: 25.0, rng_studs: 6.2, cd: 1.0, spd_studs: 13.0, shape: UnitShape::Square, passive: PassiveKey::None, passive_desc: "Heavy combat enforcer", sprite_key: "mechanica_warbot", unlock_hint: "Clear M6.2" },
    UnitDef { id: "mec_5", num: 5, faction: FactionId::Mechanica, tier: UnitTier::B, name: "Plasma Robot", cost: 22, hp: 120.0, dmg: 35.0, rng_studs: 25.0, cd: 1.25, spd_studs: 12.5, shape: UnitShape::Diamond, passive: PassiveKey::None, passive_desc: "Armored ranged cannon", sprite_key: "mechanica_plasma_robot", unlock_hint: "Clear M6.2" },
    UnitDef { id: "mec_6", num: 6, faction: FactionId::Mechanica, tier: UnitTier::B, name: "Repair Bot", cost: 25, hp: 100.0, dmg: 8.0, rng_studs: 28.0, cd: 1.4, spd_studs: 12.5, shape: UnitShape::Cross, passive: PassiveKey::Heal25_6s5u, passive_desc: "Heals 25 HP/6s (5 allies)", sprite_key: "mechanica_repair_bot", unlock_hint: "Clear M6.3" },
    UnitDef { id: "mec_7", num: 7, faction: FactionId::Mechanica, tier: UnitTier::A, name: "Destroyer Mech", cost: 40, hp: 400.0, dmg: 45.0, rng_studs: 6.8, cd: 1.05, spd_studs: 11.5, shape: UnitShape::Hexagon, passive: PassiveKey::None, passive_desc: "400 HP Colossal Mech", sprite_key: "mechanica_destroyer_mech", unlock_hint: "Clear M6.4 Final Boss" },
    UnitDef { id: "mec_8", num: 8, faction: FactionId::Mechanica, tier: UnitTier::A, name: "Heavy Artillery Bot", cost: 50, hp: 250.0, dmg: 65.0, rng_studs: 29.0, cd: 1.5, spd_studs: 11.0, shape: UnitShape::Diamond, passive: PassiveKey::None, passive_desc: "29 RNG Siege Cannon", sprite_key: "mechanica_heavy_artillery_bot", unlock_hint: "Clear M6.4 Final Boss" },
    UnitDef { id: "mec_9", num: 9, faction: FactionId::Mechanica, tier: UnitTier::A, name: "Cybernetic Commander", cost: 55, hp: 300.0, dmg: 35.0, rng_studs: 26.0, cd: 1.0, spd_studs: 12.5, shape: UnitShape::Hexagon, passive: PassiveKey::CyberCmdr, passive_desc: "+20% ATK Spd Aura (24s)", sprite_key: "mechanica_cybernetic_commander", unlock_hint: "Clear M6.4 Final Boss" },
];

pub fn get_faction(id: FactionId) -> &'static FactionDef {
    FACTIONS
        .iter()
        .find(|f| f.id == id)
        .expect("all FactionId variants are present")
}

pub fn get_unit(id: &str) -> Option<&'static UnitDef> {
    UNITS.iter().find(|u| u.id == id)
}

pub fn faction_units(faction: FactionId) -> impl Iterator<Item = &'static UnitDef> {
    UNITS.iter().filter(move |u| u.faction == faction)
}

pub fn starter_unit_ids(faction: FactionId) -> [&'static str; 3] {
    match faction {
        FactionId::Solaris => ["sol_1", "sol_2", "sol_3"],
        FactionId::Aegis => ["aeg_1", "aeg_2", "aeg_3"],
        FactionId::Hive => ["hiv_1", "hiv_2", "hiv_3"],
        FactionId::Hellfire => ["hel_1", "hel_2", "hel_3"],
        FactionId::Void => ["voi_1", "voi_2", "voi_3"],
        FactionId::Mechanica => ["mec_1", "mec_2", "mec_3"],
    }
}

pub fn default_faction_deck(faction: FactionId) -> [String; 6] {
    let c: Vec<&UnitDef> = faction_units(faction).filter(|u| u.tier == UnitTier::C).collect();
    let b: Vec<&UnitDef> = faction_units(faction).filter(|u| u.tier == UnitTier::B).collect();
    let a: Vec<&UnitDef> = faction_units(faction).filter(|u| u.tier == UnitTier::A).collect();
    [
        c[0].id.to_string(),
        c[1].id.to_string(),
        b[0].id.to_string(),
        b[1].id.to_string(),
        a[0].id.to_string(),
        a[1].id.to_string(),
    ]
}

#[derive(Debug, Clone, Default)]
pub struct DeckAffinityBonus {
    pub solaris_heal_mult: f32,      // 1.0, 1.15, or 1.30
    pub aegis_ranged_spd_bonus: f32, // 0.0, 0.10, or 0.20
    pub hive_move_studs_bonus: f32,  // 0.0, 1.5, or 3.0
    pub hellfire_kill_heal: f32,     // 0.0, 5.0, or 10.0
    pub void_shred_aura: f32,        // 0.0, 0.10, or 0.20 (extra dmg taken by enemies within 16 studs)
    pub mechanica_max_hp_mult: f32,  // 1.0, 1.08, or 1.16
    pub pure_6_cdr_secs: f32,        // 0.0 or 10.0
    pub active_summary: Vec<(FactionId, usize, bool)>, // (faction, count, is_6_pc)
}

pub fn evaluate_deck_affinities(deck_unit_ids: &[String; 6]) -> DeckAffinityBonus {
    let mut counts = [0usize; 6];
    for id in deck_unit_ids {
        if let Some(u) = get_unit(id) {
            let idx = match u.faction {
                FactionId::Solaris => 0,
                FactionId::Aegis => 1,
                FactionId::Hive => 2,
                FactionId::Hellfire => 3,
                FactionId::Void => 4,
                FactionId::Mechanica => 5,
            };
            counts[idx] += 1;
        }
    }

    let mut bonus = DeckAffinityBonus {
        solaris_heal_mult: 1.0,
        aegis_ranged_spd_bonus: 0.0,
        hive_move_studs_bonus: 0.0,
        hellfire_kill_heal: 0.0,
        void_shred_aura: 0.0,
        mechanica_max_hp_mult: 1.0,
        pure_6_cdr_secs: 0.0,
        active_summary: Vec::new(),
    };

    for (i, fac) in FactionId::ALL.iter().enumerate() {
        let count = counts[i];
        if count >= 3 {
            let mult = if count >= 6 { 2.0 } else { 1.0 };
            let is_6 = count >= 6;
            if is_6 {
                bonus.pure_6_cdr_secs = 10.0;
            }
            match fac {
                FactionId::Solaris => bonus.solaris_heal_mult += 0.15 * mult,
                FactionId::Aegis => bonus.aegis_ranged_spd_bonus += 0.10 * mult,
                FactionId::Hive => bonus.hive_move_studs_bonus += 1.5 * mult,
                FactionId::Hellfire => bonus.hellfire_kill_heal += 5.0 * mult,
                FactionId::Void => bonus.void_shred_aura += 0.10 * mult,
                FactionId::Mechanica => bonus.mechanica_max_hp_mult += 0.08 * mult,
            }
            bonus.active_summary.push((*fac, count, is_6));
        }
    }
    bonus
}
