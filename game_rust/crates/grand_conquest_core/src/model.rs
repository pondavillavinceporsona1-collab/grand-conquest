use serde::{Deserialize, Serialize};

pub const WORLD_W: f32 = 1400.0;
pub const WORLD_H: f32 = 500.0;
pub const LANE_TOP: f32 = 95.0;
pub const LANE_BOTTOM: f32 = 435.0;
pub const STUD_TO_PX: f32 = 8.5;
pub const WALK_SPEED_SCALE: f32 = 3.1;
pub const AURA_RADIUS_STUDS: f32 = 24.0;
pub const MAX_RELIC_POWER: u8 = 4;
pub const TOTAL_MAX_STARS: u32 = 72;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactionId {
    Solaris,
    Aegis,
    Hive,
    Hellfire,
    Void,
    Mechanica,
}

impl FactionId {
    pub const ALL: [FactionId; 6] = [
        FactionId::Solaris,
        FactionId::Aegis,
        FactionId::Hive,
        FactionId::Hellfire,
        FactionId::Void,
        FactionId::Mechanica,
    ];

    pub fn id_str(self) -> &'static str {
        match self {
            FactionId::Solaris => "solaris",
            FactionId::Aegis => "aegis",
            FactionId::Hive => "hive",
            FactionId::Hellfire => "hellfire",
            FactionId::Void => "void",
            FactionId::Mechanica => "mechanica",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            FactionId::Solaris => "Solaris Empire",
            FactionId::Aegis => "Aegis Coalition",
            FactionId::Hive => "Hive Mind",
            FactionId::Hellfire => "Hellfire Legion",
            FactionId::Void => "Abyssal Void",
            FactionId::Mechanica => "Mechanica Corps",
        }
    }

    pub fn code_name(self) -> &'static str {
        match self {
            FactionId::Solaris => "Kingdom",
            FactionId::Aegis => "Republic",
            FactionId::Hive => "Alien",
            FactionId::Hellfire => "Demon",
            FactionId::Void => "Void",
            FactionId::Mechanica => "Mechanica",
        }
    }

    pub fn subtitle(self) -> &'static str {
        match self {
            FactionId::Solaris => "Knights, Mages & Holy Priests",
            FactionId::Aegis => "Modern Tactical Strike Force",
            FactionId::Hive => "Bio-Swarm, Acid & Hive Overlords",
            FactionId::Hellfire => "Hellfire, Lifesteal & Archmages",
            FactionId::Void => "Shadows, Soul Drain & Void Auras",
            FactionId::Mechanica => "Autonomous Warbots, Mechs & Artillery",
        }
    }

    pub fn accent_rgb(self) -> [u8; 3] {
        match self {
            FactionId::Solaris => [232, 185, 68],
            FactionId::Aegis => [72, 168, 240],
            FactionId::Hive => [146, 208, 66],
            FactionId::Hellfire => [236, 78, 86],
            FactionId::Void => [172, 112, 244],
            FactionId::Mechanica => [56, 206, 188],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitTier {
    C,
    B,
    A,
}

impl UnitTier {
    pub fn required_tech_tier(self) -> u8 {
        match self {
            UnitTier::C => 1,
            UnitTier::B => 2,
            UnitTier::A => 3,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            UnitTier::C => "C",
            UnitTier::B => "B",
            UnitTier::A => "A",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitShape {
    Circle,
    Square,
    Triangle,
    Diamond,
    Cross,
    Hexagon,
}

impl UnitShape {
    pub fn label(self) -> &'static str {
        match self {
            UnitShape::Circle => "CIRCLE",
            UnitShape::Square => "SQUARE",
            UnitShape::Triangle => "TRIANGLE",
            UnitShape::Diamond => "DIAMOND",
            UnitShape::Cross => "CROSS",
            UnitShape::Hexagon => "HEXAGON",
        }
    }

    pub fn role_hint(self) -> &'static str {
        match self {
            UnitShape::Circle => "Fast Striker",
            UnitShape::Square => "Armored Tank",
            UnitShape::Triangle => "Ranged Marksman",
            UnitShape::Diamond => "Arcane Artillery",
            UnitShape::Cross => "Field Medic",
            UnitShape::Hexagon => "Mythic Titan",
        }
    }

    pub fn short_role(self) -> &'static str {
        match self {
            UnitShape::Circle => "Striker",
            UnitShape::Square => "Tank",
            UnitShape::Triangle => "Marksman",
            UnitShape::Diamond => "Mage",
            UnitShape::Cross => "Healer",
            UnitShape::Hexagon => "Titan",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassiveKey {
    None,
    DmgRed25,
    FourthHit45,
    Heal10_3s3u,
    Heal20_3s5u,
    Heal25_6s5u,
    Heal30_4s99u,
    EliteKnight,
    Sniper36,
    FirstHit50,
    EliteRiot,
    ShotgunFalloff,
    Gatling,
    AcidPoison,
    Broodmother,
    Lifesteal30,
    SoulHarvest,
    Lifesteal50,
    DeathImmunity,
    VoidWarden,
    CyberCmdr,
}

impl PassiveKey {
    pub fn is_healer_wave(self) -> bool {
        matches!(
            self,
            PassiveKey::Heal10_3s3u
                | PassiveKey::Heal20_3s5u
                | PassiveKey::Heal25_6s5u
                | PassiveKey::Heal30_4s99u
        )
    }

    pub fn heal_params(self) -> Option<(f32, f32, usize)> {
        match self {
            PassiveKey::Heal10_3s3u => Some((10.0, 3.0, 3)),
            PassiveKey::Heal20_3s5u => Some((20.0, 3.0, 5)),
            PassiveKey::Heal25_6s5u => Some((25.0, 6.0, 5)),
            PassiveKey::Heal30_4s99u => Some((30.0, 4.0, 99)),
            _ => Option::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpellFxType {
    Excalibur,
    Airstrike,
    Plasma,
    Hellfire,
    Void,
    Ion,
}

#[derive(Debug, Clone, Copy)]
pub struct FactionDef {
    pub id: FactionId,
    pub q_spell_name: &'static str,
    pub q_spell_desc: &'static str,
    pub q_fx_type: SpellFxType,
    pub e_spell_name: &'static str,
    pub e_spell_desc: &'static str,
    pub affinity_3pc_desc: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct UnitDef {
    pub id: &'static str,
    pub num: u8,
    pub faction: FactionId,
    pub tier: UnitTier,
    pub name: &'static str,
    pub cost: i32,
    pub hp: f32,
    pub dmg: f32,
    pub rng_studs: f32,
    pub cd: f32,
    pub spd_studs: f32,
    pub shape: UnitShape,
    pub passive: PassiveKey,
    pub passive_desc: &'static str,
    pub sprite_key: &'static str,
    pub unlock_hint: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Team {
    Blue,
    Red,
}

impl Team {
    pub fn enemy(self) -> Team {
        match self {
            Team::Blue => Team::Red,
            Team::Red => Team::Blue,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiDifficulty {
    Easy,
    Normal,
    Hard,
    Extreme,
}

impl AiDifficulty {
    pub const ALL: [AiDifficulty; 4] = [
        AiDifficulty::Easy,
        AiDifficulty::Normal,
        AiDifficulty::Hard,
        AiDifficulty::Extreme,
    ];

    pub fn name(self) -> &'static str {
        match self {
            AiDifficulty::Easy => "Easy",
            AiDifficulty::Normal => "Normal",
            AiDifficulty::Hard => "Hard",
            AiDifficulty::Extreme => "Extreme",
        }
    }

    pub fn income_mult(self) -> f32 {
        match self {
            AiDifficulty::Easy => 0.85,
            AiDifficulty::Normal => 1.0,
            AiDifficulty::Hard => 1.20,
            AiDifficulty::Extreme => 1.45,
        }
    }

    pub fn desc(self) -> &'static str {
        match self {
            AiDifficulty::Easy => "-15% Gold Income • Passive eco, low tech cap, simple spawns",
            AiDifficulty::Normal => "1.0x Balanced Income • Role-balanced squads, eco & tier scaling",
            AiDifficulty::Hard => "+20% Income, +2 Vanguards • Counter-comp, wave banking & defense",
            AiDifficulty::Extreme => "+45% Income, +Vanguard & Veteran • Full macro curve, combo waves & precision spells",
        }
    }

    pub fn bonus_tiers(self) -> &'static [UnitTier] {
        match self {
            AiDifficulty::Easy | AiDifficulty::Normal => &[],
            AiDifficulty::Hard => &[UnitTier::C, UnitTier::C],
            AiDifficulty::Extreme => &[UnitTier::C, UnitTier::B],
        }
    }
}

/// High-level strategic posture currently executed by the Enemy AI Commander.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AiPosture {
    #[default]
    EcoBoom,
    TechUp,
    Defend,
    CounterPush,
    AllIn,
}

impl AiPosture {
    pub fn short_tag(self) -> &'static str {
        match self {
            AiPosture::EcoBoom => "ECO",
            AiPosture::TechUp => "TECH",
            AiPosture::Defend => "DEFEND",
            AiPosture::CounterPush => "PUSH",
            AiPosture::AllIn => "ALL-IN",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            AiPosture::EcoBoom => "Expanding Economy",
            AiPosture::TechUp => "Unlocking Higher Tier",
            AiPosture::Defend => "Fortifying Defense",
            AiPosture::CounterPush => "Staging Counter-Push",
            AiPosture::AllIn => "Siege All-In",
        }
    }
}

/// High-level tactical stance command issued by a Commander to deployed troops.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArmyOrder {
    #[default]
    Attack,
    Hold,
    Retreat,
}

impl ArmyOrder {
    pub const ALL: [ArmyOrder; 3] = [ArmyOrder::Attack, ArmyOrder::Hold, ArmyOrder::Retreat];

    pub fn label(self) -> &'static str {
        match self {
            ArmyOrder::Attack => "ATTACK",
            ArmyOrder::Hold => "HOLD",
            ArmyOrder::Retreat => "RETREAT",
        }
    }

    pub fn desc(self) -> &'static str {
        match self {
            ArmyOrder::Attack => "Advance forward across the lane and assault enemy units & towers.",
            ArmyOrder::Hold => "Hold current position. Troops stand ground & fire only on enemies in range.",
            ArmyOrder::Retreat => "Move back towards home base at +30% speed to regroup.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionArchetype {
    Siege,
    HoldTheLine,
    Blitzkrieg,
    TitanSlayer,
    Mutator,
}

impl MissionArchetype {
    pub fn label(self) -> &'static str {
        match self {
            MissionArchetype::Siege => "SIEGE",
            MissionArchetype::HoldTheLine => "HOLD THE LINE",
            MissionArchetype::Blitzkrieg => "BLITZKRIEG",
            MissionArchetype::TitanSlayer => "TITAN SLAYER",
            MissionArchetype::Mutator => "MUTATOR",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutatorId {
    ConsecratedSolar,
    EmpScrambler,
    CorrosiveMiasma,
    GravityDistortion,
    BloodFrenzy,
    NaniteRegeneration,
}

impl MutatorId {
    pub const ALL: [MutatorId; 6] = [
        MutatorId::ConsecratedSolar,
        MutatorId::EmpScrambler,
        MutatorId::CorrosiveMiasma,
        MutatorId::GravityDistortion,
        MutatorId::BloodFrenzy,
        MutatorId::NaniteRegeneration,
    ];

    pub fn name(self) -> &'static str {
        match self {
            MutatorId::ConsecratedSolar => "Consecrated Solar Conduits",
            MutatorId::EmpScrambler => "Electromagnetic Interference",
            MutatorId::CorrosiveMiasma => "Corrosive Miasma",
            MutatorId::GravityDistortion => "Gravity Inversion & Chrono Stasis",
            MutatorId::BloodFrenzy => "Blood Frenzy & Magma Heat",
            MutatorId::NaniteRegeneration => "Autonomous Nanite Matrix",
        }
    }

    pub fn desc(self) -> &'static str {
        match self {
            MutatorId::ConsecratedSolar => {
                "All Cross & Spell heals restore +50% HP. All Triangle ranged damage reduced by -20%."
            }
            MutatorId::EmpScrambler => {
                "Ranged Triangle & Diamond attack cooldowns +40%. Melee Circle & Square speed +25%."
            }
            MutatorId::CorrosiveMiasma => {
                "All units suffer 2 DMG every 1.5s unless within 110px of an allied structure or Cross healer."
            }
            MutatorId::GravityDistortion => {
                "Death Immunity duration doubled to 2.0s. Projectiles curve inward toward lane center."
            }
            MutatorId::BloodFrenzy => {
                "All units gain +35% ATK speed and +25% move speed, but burn 1.0% Max HP every 1.0s."
            }
            MutatorId::NaniteRegeneration => {
                "All Square & Hexagon units regenerate 5% Max HP every 3.0s and gain +20% move speed."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructureKind {
    Turret1,
    Turret2,
    Base,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelicId {
    TreasurySeal,
    RapidBanner,
    OverchargedConduits,
    TitaniumFort,
    VampiricStone,
    ArcaneMagnifier,
    BallisticLens,
    SacredPhylactery,
    ChitinCarapace,
    VoidSingularity,
    NaniteInjector,
    EconomicMonopoly,
    OrbitalLens,
    SurgeAmplifier,
    CrownGeoTerra,
}

impl RelicId {
    pub fn as_str(self) -> &'static str {
        match self {
            RelicId::TreasurySeal => "treasury_seal",
            RelicId::RapidBanner => "rapid_banner",
            RelicId::OverchargedConduits => "overcharged_conduits",
            RelicId::TitaniumFort => "titanium_fort",
            RelicId::VampiricStone => "vampiric_stone",
            RelicId::ArcaneMagnifier => "arcane_magnifier",
            RelicId::BallisticLens => "ballistic_lens",
            RelicId::SacredPhylactery => "sacred_phylactery",
            RelicId::ChitinCarapace => "chitin_carapace",
            RelicId::VoidSingularity => "void_singularity",
            RelicId::NaniteInjector => "nanite_injector",
            RelicId::EconomicMonopoly => "economic_monopoly",
            RelicId::OrbitalLens => "orbital_lens",
            RelicId::SurgeAmplifier => "surge_amplifier",
            RelicId::CrownGeoTerra => "crown_geo_terra",
        }
    }

    pub fn from_str_id(s: &str) -> Option<Self> {
        match s {
            "treasury_seal" => Some(RelicId::TreasurySeal),
            "rapid_banner" => Some(RelicId::RapidBanner),
            "overcharged_conduits" => Some(RelicId::OverchargedConduits),
            "titanium_fort" => Some(RelicId::TitaniumFort),
            "vampiric_stone" => Some(RelicId::VampiricStone),
            "arcane_magnifier" => Some(RelicId::ArcaneMagnifier),
            "ballistic_lens" => Some(RelicId::BallisticLens),
            "sacred_phylactery" => Some(RelicId::SacredPhylactery),
            "chitin_carapace" => Some(RelicId::ChitinCarapace),
            "void_singularity" => Some(RelicId::VoidSingularity),
            "nanite_injector" => Some(RelicId::NaniteInjector),
            "economic_monopoly" => Some(RelicId::EconomicMonopoly),
            "orbital_lens" => Some(RelicId::OrbitalLens),
            "surge_amplifier" => Some(RelicId::SurgeAmplifier),
            "crown_geo_terra" => Some(RelicId::CrownGeoTerra),
            _ => None,
        }
    }
}
