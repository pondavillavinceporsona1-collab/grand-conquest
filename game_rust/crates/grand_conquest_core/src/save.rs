use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::missions::{chapter_missions, get_mission};
use crate::model::{FactionId, MAX_RELIC_POWER};
use crate::relics::{get_relic_by_str, RELICS};
use crate::units::{default_faction_deck, get_unit, starter_unit_ids};

const CHECKSUM_SALT: &str = "GEO_TERRA_SALT_v2";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MissionRecord {
    pub stars: u8,
    pub best_time_seconds: f32,
    pub times_attempted: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveFeatures {
    pub tier2: bool,
    pub tier3: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConquestSave {
    pub schema_version: u32,
    pub player_commander_faction: FactionId,
    pub account_gold_vault: u32,
    pub total_conquest_stars: u32,
    pub skirmish_wins: u32,
    pub missions: BTreeMap<String, MissionRecord>,
    pub unlocked_blueprint_ids: Vec<String>,
    pub equipped_deck_unit_ids: [String; 6],
    pub equipped_relic_ids: Vec<String>,
    pub unlocked_relic_ids: Vec<String>,
    pub unlocked_trophies: Vec<String>,
    pub active_features: ActiveFeatures,
    #[serde(default)]
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize)]
struct SaveHashView<'a> {
    schema_version: u32,
    player_commander_faction: FactionId,
    account_gold_vault: u32,
    total_conquest_stars: u32,
    skirmish_wins: u32,
    missions: &'a BTreeMap<String, MissionRecord>,
    unlocked_blueprint_ids: &'a Vec<String>,
    equipped_deck_unit_ids: &'a [String; 6],
    equipped_relic_ids: &'a Vec<String>,
    unlocked_relic_ids: &'a Vec<String>,
    unlocked_trophies: &'a Vec<String>,
    active_features: &'a ActiveFeatures,
}

impl ConquestSave {
    pub fn slot_req_tier(slot_idx: usize) -> u8 {
        match slot_idx {
            0 | 1 => 1,
            2 | 3 => 2,
            _ => 3,
        }
    }

    pub fn new_default(starter: FactionId) -> Self {
        let starters = starter_unit_ids(starter);
        // Only Tier C starters ([1] & [2]) are filled initially. Tier B ([3]-[4])
        // and Tier A ([5]-[6]) remain locked/empty until blueprints of those tiers unlock!
        let deck = [
            starters[0].to_string(),
            starters[1].to_string(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ];
        let mut save = Self {
            schema_version: 2,
            player_commander_faction: starter,
            account_gold_vault: 0,
            total_conquest_stars: 0,
            skirmish_wins: 0,
            missions: BTreeMap::new(),
            unlocked_blueprint_ids: starters.iter().map(|s| (*s).to_string()).collect(),
            equipped_deck_unit_ids: deck,
            equipped_relic_ids: Vec::new(),
            unlocked_relic_ids: Vec::new(),
            unlocked_trophies: Vec::new(),
            active_features: ActiveFeatures {
                tier2: false,
                tier3: false,
            },
            checksum: String::new(),
        };
        save.recompute_integrity();
        save
    }

    pub fn unlock_all_sandbox() -> Self {
        let mut save = Self::new_default(FactionId::Solaris);
        save.unlocked_blueprint_ids = crate::units::UNITS.iter().map(|u| u.id.to_string()).collect();
        save.equipped_deck_unit_ids = default_faction_deck(FactionId::Solaris);
        save.unlocked_relic_ids = RELICS.iter().map(|r| r.id.as_str().to_string()).collect();
        save.active_features = ActiveFeatures {
            tier2: true,
            tier3: true,
        };
        save.recompute_integrity();
        save
    }

    pub fn compute_checksum(&self) -> String {
        let view = SaveHashView {
            schema_version: self.schema_version,
            player_commander_faction: self.player_commander_faction,
            account_gold_vault: self.account_gold_vault,
            total_conquest_stars: self.total_conquest_stars,
            skirmish_wins: self.skirmish_wins,
            missions: &self.missions,
            unlocked_blueprint_ids: &self.unlocked_blueprint_ids,
            equipped_deck_unit_ids: &self.equipped_deck_unit_ids,
            equipped_relic_ids: &self.equipped_relic_ids,
            unlocked_relic_ids: &self.unlocked_relic_ids,
            unlocked_trophies: &self.unlocked_trophies,
            active_features: &self.active_features,
        };
        let mut s = serde_json::to_string(&view).unwrap_or_default();
        s.push_str(CHECKSUM_SALT);
        let mut hash: u32 = 2166136261;
        for b in s.as_bytes() {
            hash ^= *b as u32;
            hash = hash.wrapping_mul(16777619);
        }
        format!("{hash:08x}")
    }

    pub fn verify_checksum(&self) -> bool {
        !self.checksum.is_empty() && self.checksum == self.compute_checksum()
    }

    pub fn recompute_integrity(&mut self) {
        self.total_conquest_stars = self.missions.values().map(|r| r.stars as u32).sum();

        for relic in RELICS {
            if self.total_conquest_stars >= relic.stars_req {
                let rid = relic.id.as_str().to_string();
                if !self.unlocked_relic_ids.contains(&rid) {
                    self.unlocked_relic_ids.push(rid);
                }
            }
        }

        // Chapter Mastery 12-star blueprint rewards (sol_8, sol_9, aeg_9, hiv_8)
        let ch1_stars = self.chapter_stars(1);
        let ch2_stars = self.chapter_stars(2);
        let ch3_stars = self.chapter_stars(3);
        if ch1_stars >= 12 {
            self.unlock_blueprint("sol_8");
            self.unlock_blueprint("sol_9");
        }
        if ch2_stars >= 12 {
            self.unlock_blueprint("aeg_9");
        }
        if ch3_stars >= 12 {
            self.unlock_blueprint("hiv_8");
        }

        // Sync tier unlock flags with unlocked unit blueprints
        for id in &self.unlocked_blueprint_ids {
            if let Some(u) = get_unit(id) {
                match u.tier {
                    crate::model::UnitTier::B => self.active_features.tier2 = true,
                    crate::model::UnitTier::A => self.active_features.tier3 = true,
                    crate::model::UnitTier::C => {}
                }
            }
        }

        self.sanitize_and_autofill_deck();
        self.checksum = self.compute_checksum();
    }

    /// Removes mismatched/lower-tier duplicates from higher-tier slots (e.g. legacy C-tier
    /// filler in B/A slots) and auto-equips newly unlocked B/A blueprints into empty matching slots.
    pub fn sanitize_and_autofill_deck(&mut self) {
        // 1. Clear any slot holding a locked blueprint or wrong-tier unit
        for slot_idx in 0..6 {
            let cur = self.equipped_deck_unit_ids[slot_idx].as_str();
            if cur.is_empty() {
                continue;
            }
            let req_tier = Self::slot_req_tier(slot_idx);
            let is_unlocked = self.unlocked_blueprint_ids.iter().any(|x| x == cur);
            let tier_ok = get_unit(cur)
                .map(|u| u.tier.required_tech_tier() == req_tier)
                .unwrap_or(false);
            if !is_unlocked || !tier_ok {
                self.equipped_deck_unit_ids[slot_idx].clear();
            }
        }

        // 2. Auto-fill empty slots cleanly with distinct unlocked blueprints of that exact tier
        // (preferring the player's chosen commander faction first).
        for slot_idx in 0..6 {
            if !self.equipped_deck_unit_ids[slot_idx].is_empty() {
                continue;
            }
            let req_tier = Self::slot_req_tier(slot_idx);
            let candidate = self
                .unlocked_blueprint_ids
                .iter()
                .filter_map(|id| get_unit(id))
                .filter(|u| u.tier.required_tech_tier() == req_tier)
                .filter(|u| !self.equipped_deck_unit_ids.iter().any(|eq| eq == u.id))
                .min_by_key(|u| (u.faction != self.player_commander_faction, u.cost));

            if let Some(unit) = candidate {
                self.equipped_deck_unit_ids[slot_idx] = unit.id.to_string();
            }
        }
    }

    pub fn has_unlocked_tier(&self, tech_tier: u8) -> bool {
        self.unlocked_blueprint_ids
            .iter()
            .filter_map(|id| get_unit(id))
            .any(|u| u.tier.required_tech_tier() == tech_tier)
    }

    pub fn chapter_stars(&self, ch: u8) -> u32 {
        chapter_missions(ch)
            .filter_map(|m| self.missions.get(m.id))
            .map(|r| r.stars as u32)
            .sum()
    }

    pub fn unlock_blueprint(&mut self, unit_id: &str) {
        if get_unit(unit_id).is_some() && !self.unlocked_blueprint_ids.iter().any(|x| x == unit_id) {
            self.unlocked_blueprint_ids.push(unit_id.to_string());
        }
    }

    pub fn is_mission_unlocked(&self, mission_id: &str) -> bool {
        let Some(m) = get_mission(mission_id) else {
            return false;
        };
        if m.prerequisites.is_empty() {
            return true;
        }
        m.prerequisites.iter().all(|pre| {
            self.missions
                .get(*pre)
                .map(|rec| rec.stars > 0)
                .unwrap_or(false)
        })
    }

    pub fn mission_stars(&self, mission_id: &str) -> u8 {
        self.missions.get(mission_id).map(|r| r.stars).unwrap_or(0)
    }

    pub fn used_relic_power(&self) -> u8 {
        self.equipped_relic_ids
            .iter()
            .filter_map(|id| get_relic_by_str(id))
            .map(|r| r.rp_cost)
            .sum()
    }

    pub fn try_toggle_relic(&mut self, relic_id: &str) -> Result<bool, &'static str> {
        if let Some(pos) = self.equipped_relic_ids.iter().position(|id| id == relic_id) {
            self.equipped_relic_ids.remove(pos);
            self.recompute_integrity();
            return Ok(false);
        }
        let Some(def) = get_relic_by_str(relic_id) else {
            return Err("Unknown relic identifier.");
        };
        let unlocked = self.unlocked_relic_ids.iter().any(|x| x == relic_id)
            || self.total_conquest_stars >= def.stars_req;
        if !unlocked {
            return Err("Relic requires additional Conquest Stars.");
        }
        if self.used_relic_power() + def.rp_cost > MAX_RELIC_POWER {
            return Err("Insufficient Relic Power capacity (max 4 RP).");
        }
        self.equipped_relic_ids.push(relic_id.to_string());
        self.recompute_integrity();
        Ok(true)
    }

    pub fn equip_unit_in_slot(&mut self, slot_idx: usize, unit_id: &str) -> Result<(), &'static str> {
        if slot_idx >= 6 {
            return Err("Invalid deck slot.");
        }
        if !self.unlocked_blueprint_ids.iter().any(|id| id == unit_id) {
            return Err("Unit blueprint is locked.");
        }
        let Some(def) = get_unit(unit_id) else {
            return Err("Unknown unit definition.");
        };
        let slot_req_tier = match slot_idx {
            0 | 1 => 1,
            2 | 3 => 2,
            _ => 3,
        };
        if def.tier.required_tech_tier() != slot_req_tier {
            return Err("Unit tier does not match slot tier requirement.");
        }
        self.equipped_deck_unit_ids[slot_idx] = unit_id.to_string();
        self.recompute_integrity();
        Ok(())
    }

    pub fn load_from_path(path: &Path) -> (Self, bool) {
        match std::fs::read_to_string(path) {
            Ok(raw) => match serde_json::from_str::<ConquestSave>(&raw) {
                Ok(mut parsed) => {
                    let checksum_ok = parsed.verify_checksum();
                    parsed.recompute_integrity();
                    (parsed, !checksum_ok)
                }
                Err(_) => (Self::new_default(FactionId::Solaris), false),
            },
            Err(_) => (Self::new_default(FactionId::Solaris), false),
        }
    }

    pub fn save_to_path(&mut self, path: &Path) -> std::io::Result<()> {
        self.recompute_integrity();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(path, json)
    }
}
