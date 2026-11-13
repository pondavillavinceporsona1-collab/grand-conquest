use crate::model::{AiDifficulty, FactionId, UnitDef, UnitTier};
use crate::save::ConquestSave;
use crate::sim::MatchStats;
use crate::units::{get_unit, UNITS};
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub struct TrophyMeta {
    pub id: &'static str,
    pub name: &'static str,
    pub subtitle: &'static str,
    pub faction: FactionId,
    pub source_mission: &'static str,
    pub lore: &'static str,
}

pub const TROPHIES: [TrophyMeta; 6] = [
    TrophyMeta {
        id: "solaris_suncrest",
        name: "Suncrest of Aurelius",
        subtitle: "Chapter I War Trophy",
        faction: FactionId::Solaris,
        source_mission: "M1.4 — Sanctum of the Sun",
        lore: "Wrested from High Paladin Aurelius atop the Sun-Gilded Citadel. Its golden prism still radiates sovereign warmth.",
    },
    TrophyMeta {
        id: "aegis_star",
        name: "Aegis Iron Star",
        subtitle: "Chapter II War Trophy",
        faction: FactionId::Aegis,
        source_mission: "M2.4 — Iron Bastion",
        lore: "Cold-forged command insignia of Marshal Ironclad, recovered after breaking the Aegis siege battery.",
    },
    TrophyMeta {
        id: "chitin_carapace",
        name: "Leviathan Chitin Plate",
        subtitle: "Chapter III War Trophy",
        faction: FactionId::Hive,
        source_mission: "M3.4 — Core of the Hive",
        lore: "Iridescent armor shard torn from the Gorgon Leviathan—nearly impervious to kinetic shrapnel.",
    },
    TrophyMeta {
        id: "void_singularity_heart",
        name: "Void Singularity Heart",
        subtitle: "Chapter IV War Trophy",
        faction: FactionId::Void,
        source_mission: "M4.4 — Event Horizon",
        lore: "Suspended dark-matter core ripped from Null-Reaver Xylar, humming in a permanent gravitational stasis.",
    },
    TrophyMeta {
        id: "horn_of_malgok",
        name: "Warhorn of Malgok",
        subtitle: "Chapter V War Trophy",
        faction: FactionId::Hellfire,
        source_mission: "M5.4 — Brimstone Throne",
        lore: "Brimstone warhorn quenched in magma. A single blast echoes across the Scorched Crags.",
    },
    TrophyMeta {
        id: "crown_of_geo_terra",
        name: "Crown of Geo-Terra",
        subtitle: "Grand Finale Sovereign Trophy",
        faction: FactionId::Mechanica,
        source_mission: "M6.4 — Judgment of Iron",
        lore: "Recovered from the fallen core of Colossus Prime, proclaiming undisputed dominion across all six provinces.",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AchievementCategory {
    Campaign,
    Combat,
    Arsenal,
    Mastery,
}

impl AchievementCategory {
    pub fn label(self) -> &'static str {
        match self {
            Self::Campaign => "Campaign",
            Self::Combat => "Combat",
            Self::Arsenal => "Arsenal",
            Self::Mastery => "Mastery",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AchievementDef {
    pub id: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
    pub category: AchievementCategory,
    pub reward_gold: u32,
}

pub const ACHIEVEMENTS: [AchievementDef; 16] = [
    AchievementDef {
        id: "ach_first_victory",
        title: "Baptism by Fire",
        desc: "Claim victory in any Campaign mission or Skirmish battle.",
        category: AchievementCategory::Campaign,
        reward_gold: 75,
    },
    AchievementDef {
        id: "ach_solaris_victor",
        title: "Sun-Gilded Conqueror",
        desc: "Defeat High Paladin Aurelius in Chapter I.",
        category: AchievementCategory::Campaign,
        reward_gold: 100,
    },
    AchievementDef {
        id: "ach_aegis_victor",
        title: "Ironwall Breacher",
        desc: "Defeat Marshal Ironclad in Chapter II.",
        category: AchievementCategory::Campaign,
        reward_gold: 125,
    },
    AchievementDef {
        id: "ach_hive_victor",
        title: "Chitin Breaker",
        desc: "Defeat the Gorgon Leviathan in Chapter III.",
        category: AchievementCategory::Campaign,
        reward_gold: 150,
    },
    AchievementDef {
        id: "ach_void_victor",
        title: "Light Beyond the Horizon",
        desc: "Defeat Null-Reaver Xylar in Chapter IV.",
        category: AchievementCategory::Campaign,
        reward_gold: 175,
    },
    AchievementDef {
        id: "ach_hellfire_victor",
        title: "Master of the Crucible",
        desc: "Defeat Warlord Malgok in Chapter V.",
        category: AchievementCategory::Campaign,
        reward_gold: 200,
    },
    AchievementDef {
        id: "ach_emperor",
        title: "Sovereign of Geo-Terra",
        desc: "Destroy Colossus Prime and unite all six provinces.",
        category: AchievementCategory::Campaign,
        reward_gold: 500,
    },
    AchievementDef {
        id: "ach_12_stars",
        title: "Rising Tactician",
        desc: "Accumulate 12 Campaign Conquest Stars.",
        category: AchievementCategory::Mastery,
        reward_gold: 100,
    },
    AchievementDef {
        id: "ach_36_stars",
        title: "Grand Marshal",
        desc: "Accumulate 36 Campaign Conquest Stars.",
        category: AchievementCategory::Mastery,
        reward_gold: 250,
    },
    AchievementDef {
        id: "ach_72_stars",
        title: "Immortal Perfection",
        desc: "Earn all 72 Campaign Conquest Stars across every territory.",
        category: AchievementCategory::Mastery,
        reward_gold: 1000,
    },
    AchievementDef {
        id: "ach_skirmisher",
        title: "Crucible Veteran",
        desc: "Win 3 Skirmish Lab battles.",
        category: AchievementCategory::Combat,
        reward_gold: 150,
    },
    AchievementDef {
        id: "ach_grandmaster_ai",
        title: "Outsmarted the Warmind",
        desc: "Defeat an Extreme Neural AI opponent in Skirmish Mode.",
        category: AchievementCategory::Mastery,
        reward_gold: 300,
    },
    AchievementDef {
        id: "ach_armory_quartermaster",
        title: "Master Quartermaster",
        desc: "Unlock 24 unique troop blueprints.",
        category: AchievementCategory::Arsenal,
        reward_gold: 150,
    },
    AchievementDef {
        id: "ach_full_arsenal",
        title: "Lord of Six Legions",
        desc: "Unlock every single one of the 54 unit blueprints.",
        category: AchievementCategory::Arsenal,
        reward_gold: 600,
    },
    AchievementDef {
        id: "ach_slayer_500",
        title: "Grim Harvest",
        desc: "Defeat 500 enemy troops across your career.",
        category: AchievementCategory::Combat,
        reward_gold: 150,
    },
    AchievementDef {
        id: "ach_veteran_1000",
        title: "Steel Tide",
        desc: "Deploy 1,000 troops across Campaign and Skirmish battles.",
        category: AchievementCategory::Combat,
        reward_gold: 150,
    },
];

#[derive(Debug, Clone, Copy)]
pub struct TitleDef {
    pub id: &'static str,
    pub label: &'static str,
    pub requirement_desc: &'static str,
    pub treasury_cost: Option<u32>,
}

pub const TITLES: [TitleDef; 7] = [
    TitleDef {
        id: "title_initiate",
        label: "Initiate Commander",
        requirement_desc: "Default rank upon swearing your campaign oath.",
        treasury_cost: None,
    },
    TitleDef {
        id: "title_vanguard",
        label: "Vanguard Captain",
        requirement_desc: "Acquire via Command Treasury Commission.",
        treasury_cost: Some(150),
    },
    TitleDef {
        id: "title_siegebreaker",
        label: "Iron Siegebreaker",
        requirement_desc: "Acquire via Command Treasury Commission.",
        treasury_cost: Some(300),
    },
    TitleDef {
        id: "title_strategist",
        label: "Grand Strategist",
        requirement_desc: "Acquire via Command Treasury Commission.",
        treasury_cost: Some(500),
    },
    TitleDef {
        id: "title_marshal",
        label: "Marshal of the Realm",
        requirement_desc: "Unlocked automatically at 24+ Campaign Stars.",
        treasury_cost: None,
    },
    TitleDef {
        id: "title_warmind_bane",
        label: "Warmind Conqueror",
        requirement_desc: "Unlocked automatically by beating Extreme AI in Skirmish.",
        treasury_cost: None,
    },
    TitleDef {
        id: "title_sovereign",
        label: "Sovereign of Geo-Terra",
        requirement_desc: "Unlocked automatically by conquering Chapter VI.",
        treasury_cost: None,
    },
];

pub fn get_title(id: &str) -> &'static TitleDef {
    TITLES.iter().find(|t| t.id == id).unwrap_or(&TITLES[0])
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CareerProfile {
    pub commander_name: String,
    pub active_title_id: String,
    pub owned_title_ids: Vec<String>,
    pub unlocked_achievements: Vec<String>,
    pub total_kills: u64,
    pub total_units_deployed: u64,
    pub total_battles: u64,
    pub total_wins: u64,
    pub total_gold_earned: u64,
    pub extreme_skirmish_wins: u32,
    pub treasury_gold_spent: u64,
}

impl Default for CareerProfile {
    fn default() -> Self {
        Self {
            commander_name: "Commander".to_string(),
            active_title_id: "title_initiate".to_string(),
            owned_title_ids: vec!["title_initiate".to_string()],
            unlocked_achievements: Vec::new(),
            total_kills: 0,
            total_units_deployed: 0,
            total_battles: 0,
            total_wins: 0,
            total_gold_earned: 0,
            extreme_skirmish_wins: 0,
            treasury_gold_spent: 0,
        }
    }
}

pub const WAR_CHEST_COST: u32 = 120;

pub fn blueprint_treasury_price(unit: &UnitDef, save: &ConquestSave) -> u32 {
    match unit.tier {
        UnitTier::C => 90,
        UnitTier::B => {
            if save.has_unlocked_tier(2) {
                160
            } else {
                210
            }
        }
        UnitTier::A => {
            if save.has_unlocked_tier(3) {
                280
            } else {
                360
            }
        }
    }
}

impl CareerProfile {
    pub fn load_from_path(path: &Path) -> Self {
        let Ok(raw) = std::fs::read_to_string(path) else {
            return Self::default();
        };
        let mut p: Self = serde_json::from_str(&raw).unwrap_or_default();
        if !p.owned_title_ids.iter().any(|t| t == "title_initiate") {
            p.owned_title_ids.push("title_initiate".to_string());
        }
        if !p.owned_title_ids.iter().any(|t| t == &p.active_title_id) {
            p.active_title_id = "title_initiate".to_string();
        }
        if p.commander_name.trim().is_empty() {
            p.commander_name = "Commander".to_string();
        }
        p
    }

    pub fn save_to_path(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }

    pub fn active_title(&self) -> &'static TitleDef {
        get_title(&self.active_title_id)
    }

    pub fn has_achievement(&self, id: &str) -> bool {
        self.unlocked_achievements.iter().any(|a| a == id)
    }

    pub fn record_match_end(
        &mut self,
        stats: &MatchStats,
        won: bool,
        is_skirmish: bool,
        ai_difficulty: AiDifficulty,
    ) {
        self.total_battles += 1;
        self.total_kills += stats.enemy_kills as u64;
        self.total_units_deployed += stats.units_spawned as u64;
        self.total_gold_earned += stats.gold_earned.max(0.0) as u64;
        if won {
            self.total_wins += 1;
            if is_skirmish && ai_difficulty == AiDifficulty::Extreme {
                self.extreme_skirmish_wins += 1;
            }
        }
    }

    /// Evaluates achievement and milestone-title criteria against current Save + Career state.
    /// Returns newly unlocked achievements and credits their Vault Gold rewards into `save`.
    pub fn evaluate_unlocks(&mut self, save: &mut ConquestSave) -> Vec<&'static AchievementDef> {
        let mut newly = Vec::new();
        let any_clear = self.total_wins > 0
            || save.total_conquest_stars > 0
            || save.skirmish_wins > 0
            || save.missions.values().any(|m| m.stars > 0);
        let has_trophy = |tid: &str| save.unlocked_trophies.iter().any(|t| t == tid);

        if save.total_conquest_stars >= 24 && !self.owned_title_ids.iter().any(|t| t == "title_marshal") {
            self.owned_title_ids.push("title_marshal".to_string());
        }
        if self.extreme_skirmish_wins > 0
            && !self.owned_title_ids.iter().any(|t| t == "title_warmind_bane")
        {
            self.owned_title_ids.push("title_warmind_bane".to_string());
        }
        if has_trophy("crown_of_geo_terra")
            && !self.owned_title_ids.iter().any(|t| t == "title_sovereign")
        {
            self.owned_title_ids.push("title_sovereign".to_string());
        }

        let checks: [(&str, bool); 16] = [
            ("ach_first_victory", any_clear),
            (
                "ach_solaris_victor",
                has_trophy("solaris_suncrest") || save.mission_stars("M1_4_BOSS") > 0,
            ),
            (
                "ach_aegis_victor",
                has_trophy("aegis_star") || save.mission_stars("M2_4_BOSS") > 0,
            ),
            (
                "ach_hive_victor",
                has_trophy("chitin_carapace") || save.mission_stars("M3_4_BOSS") > 0,
            ),
            (
                "ach_void_victor",
                has_trophy("void_singularity_heart") || save.mission_stars("M4_4_BOSS") > 0,
            ),
            (
                "ach_hellfire_victor",
                has_trophy("horn_of_malgok") || save.mission_stars("M5_4_BOSS") > 0,
            ),
            (
                "ach_emperor",
                has_trophy("crown_of_geo_terra") || save.mission_stars("M6_4_FINAL_BOSS") > 0,
            ),
            ("ach_12_stars", save.total_conquest_stars >= 12),
            ("ach_36_stars", save.total_conquest_stars >= 36),
            ("ach_72_stars", save.total_conquest_stars >= 72),
            ("ach_skirmisher", save.skirmish_wins >= 3),
            ("ach_grandmaster_ai", self.extreme_skirmish_wins > 0),
            ("ach_armory_quartermaster", save.unlocked_blueprint_ids.len() >= 24),
            ("ach_full_arsenal", save.unlocked_blueprint_ids.len() >= UNITS.len()),
            ("ach_slayer_500", self.total_kills >= 500),
            ("ach_veteran_1000", self.total_units_deployed >= 1000),
        ];

        for (id, passed) in checks {
            if passed && !self.has_achievement(id) {
                self.unlocked_achievements.push(id.to_string());
                if let Some(def) = ACHIEVEMENTS.iter().find(|a| a.id == id) {
                    save.account_gold_vault = save.account_gold_vault.saturating_add(def.reward_gold);
                    newly.push(def);
                }
            }
        }
        if !newly.is_empty() {
            save.recompute_integrity();
        }
        newly
    }

    pub fn try_buy_blueprint(
        &mut self,
        save: &mut ConquestSave,
        unit_id: &str,
    ) -> Result<&'static UnitDef, &'static str> {
        let unit = get_unit(unit_id).ok_or("Blueprint not found")?;
        if save.unlocked_blueprint_ids.iter().any(|u| u == unit_id) {
            return Err("Blueprint is already unlocked");
        }
        let price = blueprint_treasury_price(unit, save);
        if save.account_gold_vault < price {
            return Err("Insufficient Vault Gold");
        }
        save.account_gold_vault -= price;
        self.treasury_gold_spent += price as u64;
        save.unlock_blueprint(unit_id);
        Ok(unit)
    }

    pub fn try_open_war_chest<R: Rng>(
        &mut self,
        save: &mut ConquestSave,
        rng: &mut R,
    ) -> Result<&'static UnitDef, &'static str> {
        let locked: Vec<&'static UnitDef> = UNITS
            .iter()
            .filter(|u| !save.unlocked_blueprint_ids.iter().any(|id| id == u.id))
            .collect();
        if locked.is_empty() {
            return Err("All 54 blueprints are already unlocked");
        }
        if save.account_gold_vault < WAR_CHEST_COST {
            return Err("Insufficient Vault Gold for a War Chest (120G)");
        }
        let chosen = *locked.choose(rng).ok_or("No locked blueprints left")?;
        save.account_gold_vault -= WAR_CHEST_COST;
        self.treasury_gold_spent += WAR_CHEST_COST as u64;
        save.unlock_blueprint(chosen.id);
        Ok(chosen)
    }

    pub fn try_buy_or_equip_title(
        &mut self,
        save: &mut ConquestSave,
        title_id: &str,
    ) -> Result<bool, &'static str> {
        let def = TITLES
            .iter()
            .find(|t| t.id == title_id)
            .ok_or("Title not found")?;
        if self.owned_title_ids.iter().any(|t| t == title_id) {
            self.active_title_id = title_id.to_string();
            return Ok(false);
        }
        let Some(cost) = def.treasury_cost else {
            return Err("Earned exclusively via campaign mastery");
        };
        if save.account_gold_vault < cost {
            return Err("Insufficient Vault Gold");
        }
        save.account_gold_vault -= cost;
        self.treasury_gold_spent += cost as u64;
        save.recompute_integrity();
        self.owned_title_ids.push(title_id.to_string());
        self.active_title_id = title_id.to_string();
        Ok(true)
    }
}

#[derive(Debug, Clone)]
pub struct SaveSlotSummary {
    pub slot_index: usize,
    pub exists: bool,
    pub faction: FactionId,
    pub stars: u32,
    pub vault_gold: u32,
    pub unlocked_units: usize,
    pub trophies: usize,
    pub active_title: String,
    pub commander_name: String,
}

pub fn storage_dir() -> PathBuf {
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        PathBuf::from(local).join("GrandConquest")
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".grand_conquest")
    } else {
        PathBuf::from(".")
    }
}

pub fn slot_save_path(base_dir: &Path, slot_index: usize) -> PathBuf {
    match slot_index {
        0 => base_dir.join("save_conquest.json"),
        n => base_dir.join(format!("save_slot_{}.json", n + 1)),
    }
}

pub fn slot_career_path(base_dir: &Path, slot_index: usize) -> PathBuf {
    base_dir.join(format!("career_slot_{}.json", slot_index + 1))
}

pub fn settings_path(base_dir: &Path) -> PathBuf {
    base_dir.join("settings.json")
}

pub fn inspect_slot(base_dir: &Path, slot_index: usize) -> SaveSlotSummary {
    let sp = slot_save_path(base_dir, slot_index);
    let cp = slot_career_path(base_dir, slot_index);
    let exists = sp.exists();
    if !exists {
        return SaveSlotSummary {
            slot_index,
            exists: false,
            faction: FactionId::Solaris,
            stars: 0,
            vault_gold: 0,
            unlocked_units: 0,
            trophies: 0,
            active_title: "Empty Slot".to_string(),
            commander_name: "New Campaign".to_string(),
        };
    }
    let (save, _) = ConquestSave::load_from_path(&sp);
    let career = CareerProfile::load_from_path(&cp);
    SaveSlotSummary {
        slot_index,
        exists: true,
        faction: save.player_commander_faction,
        stars: save.total_conquest_stars,
        vault_gold: save.account_gold_vault,
        unlocked_units: save.unlocked_blueprint_ids.len(),
        trophies: save.unlocked_trophies.len(),
        active_title: career.active_title().label.to_string(),
        commander_name: career.commander_name,
    }
}
