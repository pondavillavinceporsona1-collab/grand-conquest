use crate::model::RelicId;

#[derive(Debug, Clone, Copy)]
pub struct RelicDef {
    pub id: RelicId,
    pub name: &'static str,
    pub stars_req: u32,
    pub rp_cost: u8,
    pub badge: &'static str,
    pub desc: &'static str,
}

pub const RELICS: [RelicDef; 15] = [
    RelicDef {
        id: RelicId::TreasurySeal,
        name: "Treasury Seal",
        stars_req: 5,
        rp_cost: 1,
        badge: "ECON",
        desc: "Starting gold increased from 40G to 65G (+25G initial vault).",
    },
    RelicDef {
        id: RelicId::RapidBanner,
        name: "Rapid Deployment Banner",
        stars_req: 10,
        rp_cost: 1,
        badge: "TEMPO",
        desc: "Unit movement speed across all tiers increased by +12%.",
    },
    RelicDef {
        id: RelicId::OverchargedConduits,
        name: "Overcharged Conduits",
        stars_req: 15,
        rp_cost: 2,
        badge: "CDR",
        desc: "Commander Spells [Q] and [E] cooldowns reduced from 60s to 48s (-20%).",
    },
    RelicDef {
        id: RelicId::TitaniumFort,
        name: "Titanium Fortification",
        stars_req: 20,
        rp_cost: 2,
        badge: "FORT",
        desc: "Base Tower HP increased from 1500 to 1850; Turrets gain +15% firing range.",
    },
    RelicDef {
        id: RelicId::VampiricStone,
        name: "Vampiric Bloodstone",
        stars_req: 25,
        rp_cost: 2,
        badge: "LEECH",
        desc: "All friendly melee units restore +10 HP on every hit.",
    },
    RelicDef {
        id: RelicId::ArcaneMagnifier,
        name: "Arcane Magnifier",
        stars_req: 30,
        rp_cost: 1,
        badge: "MAGE",
        desc: "All Mages & Artillery deal +15% damage.",
    },
    RelicDef {
        id: RelicId::BallisticLens,
        name: "Ballistic Compensator",
        stars_req: 35,
        rp_cost: 1,
        badge: "RANGE",
        desc: "All Marksmen gain +4.0 (+34px) firing range.",
    },
    RelicDef {
        id: RelicId::SacredPhylactery,
        name: "Sacred Phylactery",
        stars_req: 40,
        rp_cost: 2,
        badge: "WARD",
        desc: "All Healer waves also grant a +25 HP barrier for 4s.",
    },
    RelicDef {
        id: RelicId::ChitinCarapace,
        name: "Chitin-Plated Carapace",
        stars_req: 45,
        rp_cost: 2,
        badge: "ARMOR",
        desc: "All Tanks & Blockers take 3 less damage from every hit.",
    },
    RelicDef {
        id: RelicId::VoidSingularity,
        name: "Singularity Heart",
        stars_req: 50,
        rp_cost: 2,
        badge: "EXEC",
        desc: "Enemies below 15% Max HP are instantly executed (excluding Titans).",
    },
    RelicDef {
        id: RelicId::NaniteInjector,
        name: "Nanite Emergency Injector",
        stars_req: 55,
        rp_cost: 2,
        badge: "SAVE",
        desc: "Friendly units dropping below 25% HP instantly restore 30% Max HP (once per life).",
    },
    RelicDef {
        id: RelicId::EconomicMonopoly,
        name: "Economic Monopoly",
        stars_req: 60,
        rp_cost: 1,
        badge: "DISC",
        desc: "Permanent 25% discount on [U] Income Upgrade costs.",
    },
    RelicDef {
        id: RelicId::OrbitalLens,
        name: "Targeting Satellite Matrix",
        stars_req: 64,
        rp_cost: 1,
        badge: "AOE",
        desc: "Commander Spell [Q] AoE radius increased by +25% (170px radius blast).",
    },
    RelicDef {
        id: RelicId::SurgeAmplifier,
        name: "Tactical Surge Amplifier",
        stars_req: 68,
        rp_cost: 2,
        badge: "SURGE",
        desc: "Commander Spell [E] heals +100 HP (up from 75) and surges +65% speed (up from +45%).",
    },
    RelicDef {
        id: RelicId::CrownGeoTerra,
        name: "Crown of Geo-Terra",
        stars_req: 72,
        rp_cost: 2,
        badge: "APEX",
        desc: "Ultimate 100% Mastery: All friendly units gain +10% HP, +10% DMG, and +10% Speed!",
    },
];

pub fn get_relic(id: RelicId) -> &'static RelicDef {
    RELICS
        .iter()
        .find(|r| r.id == id)
        .expect("all RelicId variants present")
}

pub fn get_relic_by_str(s: &str) -> Option<&'static RelicDef> {
    RelicId::from_str_id(s).map(get_relic)
}
