use serde::{Deserialize, Serialize};

use crate::model::{AiDifficulty, ArmyOrder, PassiveKey, UnitDef, UnitShape, UnitTier};

/// Multi-dimensional battlefield weight of a unit or whole army.
///
/// Each axis represents how much tactical pressure a unit contributes in that role
/// (roughly scaled so a basic 6G C-tier melee soldier contributes ~0.12 total weight,
/// 20 of them sum to ~2.4, and a high-tier Vanguard + Ranged battery weighs in to counter).
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct ArmyWeightVector {
    /// Fragile/cheap melee mass that threatens by numbers, vulnerable to AoE & cleave.
    pub swarm: f32,
    /// Frontline durability that absorbs shots and protects fragile backliners.
    pub tank: f32,
    /// Sustained physical/pierce ranged firepower behind the frontline.
    pub ranged_dps: f32,
    /// Burst, poison, or AoE/splash damage that melts swarms and high-armor targets.
    pub magic_aoe: f32,
    /// Healing & buff aura multipliers that extend army longevity.
    pub support: f32,
    /// High-speed melee divers that punish exposed archers/mages.
    pub assassin: f32,
}

impl ArmyWeightVector {
    pub fn add(&mut self, other: ArmyWeightVector) {
        self.swarm += other.swarm;
        self.tank += other.tank;
        self.ranged_dps += other.ranged_dps;
        self.magic_aoe += other.magic_aoe;
        self.support += other.support;
        self.assassin += other.assassin;
    }

    pub fn scaled(self, factor: f32) -> Self {
        Self {
            swarm: self.swarm * factor,
            tank: self.tank * factor,
            ranged_dps: self.ranged_dps * factor,
            magic_aoe: self.magic_aoe * factor,
            support: self.support * factor,
            assassin: self.assassin * factor,
        }
    }

    /// Raw sum across all 6 weight dimensions (without combined-arms synergy).
    pub fn raw_sum(self) -> f32 {
        self.swarm + self.tank + self.ranged_dps + self.magic_aoe + self.support + self.assassin
    }

    /// Combined-arms synergy multiplier: a shielded frontline protects ranged/magic/healers,
    /// multiplying real combat weight beyond the raw sum of isolated units.
    pub fn effective_total(self, synergy_mult: f32) -> f32 {
        let base = self.raw_sum();
        let backline = self.ranged_dps + self.magic_aoe + self.support * 0.7;
        let combined_pairs = self.tank.min(backline);
        let heal_amp = self.support.min(base * 0.45) * 0.40;
        base + combined_pairs * 0.55 * synergy_mult + heal_amp * synergy_mult
    }

    /// Computes how strongly `self` (friendly composition) counters `enemy` composition,
    /// using rock-paper-scissors weight matrix interactions.
    pub fn counter_advantage_against(self, enemy: ArmyWeightVector, w: &TrainedBrainWeights) -> f32 {
        let vs_swarm = (self.magic_aoe * w.aoe_vs_swarm + self.tank * 0.65)
            * enemy.swarm.min(6.0);
        let vs_tank = (self.magic_aoe * w.magic_vs_tank + self.ranged_dps * 0.55)
            * enemy.tank.min(6.0);
        let vs_ranged = (self.assassin * w.assassin_vs_ranged + self.tank * 0.50)
            * enemy.ranged_dps.min(6.0);
        let vs_assassin = (self.tank * 0.85 + self.swarm * 0.40) * enemy.assassin.min(4.0);
        let vs_support = (self.magic_aoe * 0.80 + self.assassin * 0.70) * enemy.support.min(3.5);

        vs_swarm + vs_tank + vs_ranged + vs_assassin + vs_support
    }

    pub fn dominant_label(self) -> (&'static str, f32) {
        let axes = [
            ("Swarm", self.swarm),
            ("Armor", self.tank),
            ("Ranged", self.ranged_dps),
            ("Magic/AoE", self.magic_aoe),
            ("Support", self.support),
            ("Striker", self.assassin),
        ];
        axes.into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(("Balanced", 0.0))
    }
}

/// Derives a calibrated 6-axis battlefield weight vector directly from a unit's stats & passive.
///
/// Example calibration:
/// - `sol_1` Footman (6G, 90 HP, 15 DPS, Circle): ~0.12 total (`0.09` swarm + `0.03` assassin),
///   so 20 Footmen = `~2.40` total swarm weight.
/// - `sol_4` Knight (16G, 160 HP -25% DMG, Square) + 3× `sol_2`/`sol_5`: combines into ~2.7+
///   effective weight with high combined-arms synergy + counter advantage.
pub fn unit_weight_vector(u: &UnitDef) -> ArmyWeightVector {
    let dps = u.dmg / u.cd.max(0.25);
    let eff_hp = match u.passive {
        PassiveKey::DmgRed25 | PassiveKey::EliteKnight => u.hp * 1.33,
        PassiveKey::EliteRiot => u.hp * 1.18,
        PassiveKey::Lifesteal30 => u.hp + 95.0,
        PassiveKey::Lifesteal50 => u.hp + 150.0,
        PassiveKey::DeathImmunity => u.hp * 1.25,
        _ => u.hp,
    };

    let durability_w = eff_hp / 920.0;
    let firepower_w = dps / 145.0;
    let tier_bonus = match u.tier {
        UnitTier::C => 1.00,
        UnitTier::B => 1.15,
        UnitTier::A => 1.32,
    };

    let mut v = ArmyWeightVector::default();
    match u.shape {
        UnitShape::Circle => {
            if u.tier == UnitTier::C {
                v.swarm = (durability_w * 0.55 + firepower_w * 0.55).max(0.08);
                v.assassin = (u.spd_studs / 16.0) * 0.04;
            } else {
                v.assassin = (firepower_w * 0.85 + durability_w * 0.45) * tier_bonus;
                v.swarm = firepower_w * 0.25;
            }
        }
        UnitShape::Square | UnitShape::Hexagon => {
            v.tank = (durability_w * 1.15 + firepower_w * 0.30) * tier_bonus;
            if matches!(
                u.passive,
                PassiveKey::EliteKnight | PassiveKey::EliteRiot | PassiveKey::VoidWarden | PassiveKey::CyberCmdr
            ) {
                v.support = 0.22 * tier_bonus;
            }
            if u.passive == PassiveKey::Broodmother {
                v.swarm = 0.28;
            }
        }
        UnitShape::Triangle => {
            v.ranged_dps = (firepower_w * 1.05 + (u.rng_studs / 28.0) * 0.10) * tier_bonus;
            v.tank = durability_w * 0.20;
        }
        UnitShape::Diamond => {
            v.magic_aoe = (firepower_w * 1.15 + (u.rng_studs / 28.0) * 0.12) * tier_bonus;
            if u.passive == PassiveKey::AcidPoison {
                v.magic_aoe += 0.18;
            }
            v.tank = durability_w * 0.18;
        }
        UnitShape::Cross => {
            let heal_power = u
                .passive
                .heal_params()
                .map(|(amt, cd, max_t)| (amt / cd.max(1.0)) * (max_t.min(8) as f32) * 0.015)
                .unwrap_or(0.25);
            v.support = (heal_power + durability_w * 0.25) * tier_bonus;
        }
    }
    v
}

/// Reward/punishment calibrated weight vector (trainable via `ai_trainer` self-play).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TrainedBrainWeights {
    pub synergy_mult: f32,
    pub aoe_vs_swarm: f32,
    pub magic_vs_tank: f32,
    pub assassin_vs_ranged: f32,
    pub gold_efficiency_gain: f32,
    pub turret_bait_bonus: f32,
    pub hold_to_pool_bonus: f32,
    pub eco_roi_weight: f32,
    pub tech_rush_weight: f32,
    pub spell_q_efficiency: f32,
    pub spell_e_efficiency: f32,
    pub lookahead_weight: f32,
    pub wave_pool_min_ratio: f32,
    pub retreat_preserve_mult: f32,
}

impl TrainedBrainWeights {
    /// Returns self-play calibrated weights tuned for each AI difficulty tier.
    pub fn for_difficulty(diff: AiDifficulty) -> Self {
        match diff {
            AiDifficulty::Easy => Self::easy_baseline(),
            AiDifficulty::Normal => Self::normal_calibrated(),
            AiDifficulty::Hard => Self::hard_trained(),
            AiDifficulty::Extreme => Self::extreme_master(),
        }
    }

    pub fn easy_baseline() -> Self {
        Self {
            synergy_mult: 0.25,
            aoe_vs_swarm: 0.45,
            magic_vs_tank: 0.45,
            assassin_vs_ranged: 0.40,
            gold_efficiency_gain: 0.35,
            turret_bait_bonus: 0.10,
            hold_to_pool_bonus: 0.05,
            eco_roi_weight: 0.50,
            tech_rush_weight: 0.40,
            spell_q_efficiency: 0.55,
            spell_e_efficiency: 0.50,
            lookahead_weight: 0.0,
            wave_pool_min_ratio: 1.0,
            retreat_preserve_mult: 0.20,
        }
    }

    pub fn normal_calibrated() -> Self {
        Self {
            synergy_mult: 0.72,
            aoe_vs_swarm: 1.05,
            magic_vs_tank: 1.00,
            assassin_vs_ranged: 0.92,
            gold_efficiency_gain: 0.85,
            turret_bait_bonus: 0.65,
            hold_to_pool_bonus: 0.45,
            eco_roi_weight: 0.90,
            tech_rush_weight: 0.88,
            spell_q_efficiency: 0.95,
            spell_e_efficiency: 0.88,
            lookahead_weight: 0.35,
            wave_pool_min_ratio: 0.78,
            retreat_preserve_mult: 0.65,
        }
    }

    pub fn hard_trained() -> Self {
        Self {
            synergy_mult: 1.12,
            aoe_vs_swarm: 1.48,
            magic_vs_tank: 1.38,
            assassin_vs_ranged: 1.30,
            gold_efficiency_gain: 1.25,
            turret_bait_bonus: 1.18,
            hold_to_pool_bonus: 0.95,
            eco_roi_weight: 1.12,
            tech_rush_weight: 1.18,
            spell_q_efficiency: 1.28,
            spell_e_efficiency: 1.18,
            lookahead_weight: 0.85,
            wave_pool_min_ratio: 0.68,
            retreat_preserve_mult: 1.05,
        }
    }

    /// Evolved master weights from Colab Pro parallel self-play arena.
    /// Emphasizes lethal anti-swarm AoE (1.79x), turret baiting (1.50x),
    /// economy scaling (1.40x), and assassin flanking (1.31x).
    pub fn extreme_master() -> Self {
        for path in &["assets/best_weights.json", "../assets/best_weights.json", "best_weights.json"] {
            if let Ok(data) = std::fs::read_to_string(path) {
                if let Ok(w) = serde_json::from_str::<Self>(&data) {
                    return w;
                }
            }
        }
        if let Ok(w) = serde_json::from_str::<Self>(include_str!("../../../assets/best_weights.json")) {
            return w;
        }

        Self {
            synergy_mult: 0.91441107,
            aoe_vs_swarm: 1.7918048,
            magic_vs_tank: 1.1027606,
            assassin_vs_ranged: 1.3148284,
            gold_efficiency_gain: 1.1016542,
            turret_bait_bonus: 1.5005647,
            hold_to_pool_bonus: 1.0831605,
            eco_roi_weight: 1.3979268,
            tech_rush_weight: 0.6834743,
            spell_q_efficiency: 1.4082308,
            spell_e_efficiency: 0.9113931,
            lookahead_weight: 0.818555,
            wave_pool_min_ratio: 0.9,
            retreat_preserve_mult: 1.127867,
        }
    }
}

/// Online reinforcement tracker that observes the player's deployment habits mid-match
/// (including x5/x10/x20/x30/x50 batch waves) and shifts counter weights in real time.
#[derive(Debug, Clone, Default)]
pub struct PlayerStyleTracker {
    pub total_units_seen: u32,
    pub total_gold_spent: f32,
    pub ai_gold_spent: f32,
    pub swarm_deploys: u32,
    pub tank_deploys: u32,
    pub ranged_deploys: u32,
    pub healer_deploys: u32,
    pub max_batch_seen: usize,
    pub avg_batch_size: f32,
    pub deploy_events: u32,
    /// Dynamic adaptive counter multipliers (0.0 .. 1.0)
    pub swarm_threat_bias: f32,
    pub armor_threat_bias: f32,
    pub ranged_threat_bias: f32,
}

impl PlayerStyleTracker {
    pub fn observe_player_deploy(&mut self, u: &UnitDef, batch_count: usize) {
        self.deploy_events += 1;
        self.total_units_seen += batch_count as u32;
        self.total_gold_spent += (u.cost as f32) * (batch_count as f32);
        if batch_count > self.max_batch_seen {
            self.max_batch_seen = batch_count;
        }
        let n = batch_count as f32;
        self.avg_batch_size = if self.deploy_events == 1 {
            n
        } else {
            self.avg_batch_size * 0.72 + n * 0.28
        };

        match u.shape {
            UnitShape::Circle if u.tier == UnitTier::C => self.swarm_deploys += batch_count as u32,
            UnitShape::Circle => {
                if batch_count >= 5 {
                    self.swarm_deploys += batch_count as u32;
                } else {
                    self.tank_deploys += (batch_count as u32) / 2;
                }
            }
            UnitShape::Square | UnitShape::Hexagon => self.tank_deploys += batch_count as u32,
            UnitShape::Triangle | UnitShape::Diamond => self.ranged_deploys += batch_count as u32,
            UnitShape::Cross => self.healer_deploys += batch_count as u32,
        }

        let denom = (self.total_units_seen as f32).max(1.0);
        let swarm_ratio = (self.swarm_deploys as f32) / denom;
        let armor_ratio = (self.tank_deploys as f32) / denom;
        let ranged_ratio = (self.ranged_deploys as f32) / denom;

        // Dropping x5..x50 waves strongly raises anti-swarm + turret baiting adaptation
        let batch_boost = ((self.avg_batch_size - 1.5).max(0.0) / 14.0).clamp(0.0, 0.45);
        self.swarm_threat_bias = (swarm_ratio * 1.15 + batch_boost).clamp(0.0, 1.0);
        self.armor_threat_bias = (armor_ratio * 1.20).clamp(0.0, 1.0);
        self.ranged_threat_bias = (ranged_ratio * 1.15).clamp(0.0, 1.0);
    }

    pub fn adaptation_summary(&self) -> String {
        if self.total_units_seen < 3 {
            return "Gathering opening telemetry...".to_string();
        }
        if self.swarm_threat_bias >= 0.45 {
            format!(
                "Anti-Swarm Alert ({:.0}%) • Max Wave x{} -> Bait & AoE",
                self.swarm_threat_bias * 100.0,
                self.max_batch_seen.max(1)
            )
        } else if self.armor_threat_bias >= 0.45 {
            format!(
                "Anti-Armor ({:.0}%) -> Prioritizing Magic / Poison",
                self.armor_threat_bias * 100.0
            )
        } else if self.ranged_threat_bias >= 0.45 {
            format!(
                "Anti-Ranged ({:.0}%) -> Tanks + Fast Strikers",
                self.ranged_threat_bias * 100.0
            )
        } else {
            format!(
                "Balanced Comp ({} units seen) -> Combined Arms",
                self.total_units_seen
            )
        }
    }
}

/// Evaluable decision tree node generated during an AI Commander thinking cycle.
#[derive(Debug, Clone)]
pub struct DecisionNodeRecord {
    pub label: String,
    pub order: ArmyOrder,
    pub gold_cost: f32,
    pub weight_gain: f32,
    pub efficiency_per_gold: f32,
    pub lookahead_delta: f32,
    pub utility_score: f32,
    pub chosen: bool,
}

/// Live snapshot of the AI Commander's node search, weights, and counter-adaptation for inspection.
#[derive(Debug, Clone, Default)]
pub struct AiBrainTelemetry {
    pub blue_weight: ArmyWeightVector,
    pub red_weight: ArmyWeightVector,
    pub blue_effective_weight: f32,
    pub red_effective_weight: f32,
    pub player_tracker: PlayerStyleTracker,
    pub top_nodes: Vec<DecisionNodeRecord>,
    pub total_nodes_evaluated: usize,
    pub active_plan_summary: String,
    pub bait_mode_active: bool,
    pub pooled_wave_active: bool,
    pub lookahead_score: f32,
    pub efficiency_ratio: f32,
    pub neural_win_prob: f32,
    pub neural_policy: [f32; 6],
}
