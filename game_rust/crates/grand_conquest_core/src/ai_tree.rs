use rand::Rng;

use crate::ai_weights::{
    unit_weight_vector, ArmyWeightVector, DecisionNodeRecord, PlayerStyleTracker,
    TrainedBrainWeights,
};
use crate::neural_brain::{BattlefieldStateVector, DeepNeuralBrain, NN_ACTIONS};
use crate::model::{
    AiDifficulty, ArmyOrder, Team, UnitDef, UnitShape, UnitTier, LANE_BOTTOM, LANE_TOP,
    STUD_TO_PX, WORLD_W,
};
use crate::sim::MatchState;

/// Candidate tactical decision plan generated in the AI Commander's search tree.
#[derive(Debug, Clone)]
pub enum CandidatePlan {
    SoloSpawn {
        unit: &'static UnitDef,
        custom_y: f32,
        batch: usize,
    },
    CounterWaveCombo {
        tank: &'static UnitDef,
        support: &'static UnitDef,
    },
    BaitUnderTurret {
        hold_order: ArmyOrder,
        counter_unit: &'static UnitDef,
    },
    StrategicRetreat {
        order: ArmyOrder,
        reason: &'static str,
    },
    CastSpellQ {
        cluster_count: usize,
        cluster_val: f32,
    },
    CastSpellE {
        injured_count: usize,
    },
    MacroIncome {
        cost: u32,
    },
    MacroTech {
        cost: u32,
    },
    PoolForMacroEco {
        target_gold: f32,
        next_lvl: u32,
    },
    PoolForMacroTech {
        target_gold: f32,
        next_tier: u8,
    },
    PoolForCounterWave {
        target_gold: f32,
        goal_unit: &'static UnitDef,
    },
}

#[derive(Debug, Clone)]
pub struct ScoredNode {
    pub plan: CandidatePlan,
    pub order: ArmyOrder,
    pub gold_cost: f32,
    pub label: String,
    pub weight_gain: f32,
    pub efficiency_per_gold: f32,
    pub lookahead_score: f32,
    pub utility_score: f32,
}

/// Computes Blue & Red army weight vectors from current live units on the battlefield.
pub fn evaluate_battlefield_weights(
    state: &MatchState,
    w: &TrainedBrainWeights,
) -> (ArmyWeightVector, ArmyWeightVector, f32, f32) {
    let mut blue_v = ArmyWeightVector::default();
    let mut red_v = ArmyWeightVector::default();

    for u in state.units.iter().filter(|u| u.hp > 0.0) {
        let uv = unit_weight_vector(u.def);
        let hp_frac = (u.hp / u.max_hp.max(1.0)).clamp(0.15, 1.0);
        let boss_mult = if u.is_boss { 2.4 } else { 1.0 };
        let scaled = uv.scaled(hp_frac * boss_mult);

        if u.team == Team::Blue {
            // Units pushed past Red's middle lane exert heavier offensive weight
            let forward = ((u.x / WORLD_W).max(0.1)).powf(1.6);
            let pos_mult = 0.70 + 0.60 * forward;
            blue_v.add(scaled.scaled(pos_mult));
        } else {
            // Friendly units defending under our turret gain defensive posture weight
            let under_turret = if u.x > 840.0 { 1.20 } else { 1.0 };
            red_v.add(scaled.scaled(under_turret));
        }
    }

    let blue_eff = blue_v.effective_total(w.synergy_mult * 0.85);
    let red_eff = red_v.effective_total(w.synergy_mult);
    (blue_v, red_v, blue_eff, red_eff)
}

/// Generates candidate decision nodes across tactical combat, army stances, spells,
/// and macro investments, evaluating each with trained weight vectors and mini combat lookahead.
pub fn search_decision_tree<R: Rng>(
    state: &MatchState,
    weights: &TrainedBrainWeights,
    tracker: &PlayerStyleTracker,
    blue_w: ArmyWeightVector,
    _red_w: ArmyWeightVector,
    blue_eff: f32,
    red_eff: f32,
    threat_y: f32,
    urgent_defense: bool,
    best_q_cluster: (usize, f32),
    rng: &mut R,
) -> (ScoredNode, Vec<DecisionNodeRecord>, usize, f32, [f32; NN_ACTIONS]) {
    let mut nodes: Vec<ScoredNode> = Vec::with_capacity(40);
    let unlocked: Vec<&'static UnitDef> = state
        .ai
        .deck
        .iter()
        .flatten()
        .copied()
        .filter(|u| u.tier.required_tech_tier() <= state.ai.tier)
        .collect();

    if unlocked.is_empty() {
        let fallback = ScoredNode {
            plan: CandidatePlan::StrategicRetreat {
                order: state.ai.order,
                reason: "No Unlocked Units",
            },
            order: state.ai.order,
            gold_cost: 0.0,
            label: "Hold (No Unlocked Units)".to_string(),
            weight_gain: 0.0,
            efficiency_per_gold: 1.0,
            lookahead_score: 0.0,
            utility_score: 0.0,
        };
        return (fallback, vec![], 0, 0.0, [0.16; NN_ACTIONS]);
    }

    let gold = state.ai.gold;
    let income_rate = state.get_income_rate(Team::Red);
    let front_enemy_x = state
        .units
        .iter()
        .filter(|u| u.team == Team::Blue && u.hp > 0.0)
        .map(|u| u.x)
        .fold(100.0_f32, f32::max);
    let front_friendly_x = state
        .units
        .iter()
        .filter(|u| u.team == Team::Red && u.hp > 0.0)
        .map(|u| u.x)
        .fold(WORLD_W - 100.0_f32, f32::min);

    let enemy_swarming = tracker.swarm_threat_bias >= 0.40 || blue_w.swarm >= 1.4;
    let enemy_approaching_turret = front_enemy_x >= 820.0;

    // -------------------------------------------------------------
    // NODE CATEGORY 1: Tactical Army Stance Decisions (Hold / Retreat / Attack)
    // -------------------------------------------------------------
    if state.ai_difficulty != AiDifficulty::Easy {
        // 1a. Hold to Bait Swarm into Turret Range
        if enemy_approaching_turret && front_enemy_x < 980.0 && red_eff < blue_eff * 1.10 {
            if let Some(counter) = unlocked
                .iter()
                .copied()
                .max_by(|a, b| unit_counter_score(a, &blue_w, tracker, weights).partial_cmp(&unit_counter_score(b, &blue_w, tracker, weights)).unwrap())
            {
                let bait_bonus = weights.turret_bait_bonus * 28.0;
                let util = 45.0 + bait_bonus + (blue_w.swarm * 12.0);
                nodes.push(ScoredNode {
                    plan: CandidatePlan::BaitUnderTurret {
                        hold_order: ArmyOrder::Hold,
                        counter_unit: counter,
                    },
                    order: ArmyOrder::Hold,
                    gold_cost: 0.0,
                    label: format!("Bait Swarm to Turret -> Hold & Ambush ({})", counter.name),
                    weight_gain: bait_bonus * 0.1,
                    efficiency_per_gold: 3.5,
                    lookahead_score: 18.0,
                    utility_score: util,
                });
            }
        }

        // 1b. Tactical Retreat when cut off deep
        let red_alive = state.units.iter().filter(|u| u.team == Team::Red && u.hp > 0.0).count();
        if red_alive > 0 && front_friendly_x < 680.0 && blue_eff > red_eff * 1.35 + 1.2 {
            let preserve_score = weights.retreat_preserve_mult * 32.0;
            nodes.push(ScoredNode {
                plan: CandidatePlan::StrategicRetreat {
                    order: ArmyOrder::Retreat,
                    reason: "Deep & Overnumbered",
                },
                order: ArmyOrder::Retreat,
                gold_cost: 0.0,
                label: "Retreat: Fall back to Bastion & Regroup (+30% SPD)".to_string(),
                weight_gain: red_eff * 0.4,
                efficiency_per_gold: 3.0,
                lookahead_score: 14.0,
                utility_score: 40.0 + preserve_score,
            });
        }
    }

    // -------------------------------------------------------------
    // NODE CATEGORY 2: Unit Deployment & Coordinated Counter-Waves
    // -------------------------------------------------------------
    let tanks: Vec<&'static UnitDef> = unlocked
        .iter()
        .copied()
        .filter(|u| matches!(u.shape, UnitShape::Square | UnitShape::Hexagon))
        .collect();
    let supports: Vec<&'static UnitDef> = unlocked
        .iter()
        .copied()
        .filter(|u| matches!(u.shape, UnitShape::Triangle | UnitShape::Diamond | UnitShape::Cross))
        .collect();

    // 2a. Coordinated Combined-Arms Counter-Wave: Tank + Ranged/AoE behind
    if state.ai_difficulty != AiDifficulty::Easy && !tanks.is_empty() && !supports.is_empty() {
        for &t in &tanks {
            for &s in &supports {
                let combo_cost = (t.cost + s.cost) as f32;
                let mut combo_w = unit_weight_vector(t);
                combo_w.add(unit_weight_vector(s));
                let adv = combo_w.counter_advantage_against(blue_w, weights);
                let syn = combo_w.effective_total(weights.synergy_mult);

                if gold >= combo_cost {
                    let eff = (adv + syn) / combo_cost.max(1.0);
                    let mut util = 52.0 + adv * 14.0 + syn * 12.0;
                    if enemy_swarming && matches!(s.shape, UnitShape::Diamond) {
                        util += 25.0 * tracker.swarm_threat_bias;
                    }
                    nodes.push(ScoredNode {
                        plan: CandidatePlan::CounterWaveCombo { tank: t, support: s },
                        order: ArmyOrder::Attack,
                        gold_cost: combo_cost,
                        label: format!("Counter-Wave: {} (Tank) + {} ({})", t.name, s.name, s.shape.short_role()),
                        weight_gain: syn,
                        efficiency_per_gold: eff,
                        lookahead_score: 12.0 + adv * 4.0,
                        utility_score: util,
                    });
                } else if !urgent_defense && gold >= combo_cost * weights.wave_pool_min_ratio {
                    // Pool gold briefly for this superior wave rather than leaking weak single units
                    let wait = (combo_cost - gold) / income_rate.max(1.0);
                    if wait <= 4.2 {
                        let pool_util = 48.0 + adv * 10.0 + weights.hold_to_pool_bonus * 18.0;
                        nodes.push(ScoredNode {
                            plan: CandidatePlan::PoolForCounterWave {
                                target_gold: combo_cost,
                                goal_unit: t,
                            },
                            order: ArmyOrder::Hold,
                            gold_cost: 0.0,
                            label: format!("Pool Gold ({:.0}/{combo_cost}G) for {} combo", gold, t.name),
                            weight_gain: syn * 0.8,
                            efficiency_per_gold: 2.8,
                            lookahead_score: 8.0,
                            utility_score: pool_util,
                        });
                    }
                }
            }
        }
    }

    // 2b. Solo Unit Deployments (Single or Tactical Batch)
    for &u in &unlocked {
        let u_cost = u.cost as f32;
        let uv = unit_weight_vector(u);
        let counter_adv = uv.counter_advantage_against(blue_w, weights);
        let eff = (counter_adv + uv.raw_sum()) / u_cost.max(1.0);

        // Compute adaptive bonus from live player habits
        let mut habit_bonus = 0.0_f32;
        if enemy_swarming {
            if matches!(u.shape, UnitShape::Diamond | UnitShape::Square) || u.passive_desc.contains("AoE") {
                habit_bonus += 28.0 * tracker.swarm_threat_bias;
            }
        }
        if tracker.armor_threat_bias >= 0.40 && (matches!(u.shape, UnitShape::Diamond) || u.passive_desc.contains("Poison")) {
            habit_bonus += 24.0 * tracker.armor_threat_bias;
        }
        if tracker.ranged_threat_bias >= 0.40 && matches!(u.shape, UnitShape::Circle | UnitShape::Square) {
            habit_bonus += 22.0 * tracker.ranged_threat_bias;
        }

        let custom_y = (threat_y + rng.gen_range(-14.0..14.0)).clamp(LANE_TOP + 28.0, LANE_BOTTOM - 28.0);
        let batch_size = if state.ai_difficulty == AiDifficulty::Extreme
            && gold >= u_cost * 3.0
            && (enemy_swarming || urgent_defense)
            && u.tier == UnitTier::C
        {
            2
        } else {
            1
        };
        let total_c = u_cost * batch_size as f32;

        if gold >= total_c {
            let util = 38.0 + counter_adv * 16.0 + habit_bonus + (uv.raw_sum() * 8.0);
            nodes.push(ScoredNode {
                plan: CandidatePlan::SoloSpawn {
                    unit: u,
                    custom_y,
                    batch: batch_size,
                },
                order: ArmyOrder::Attack,
                gold_cost: total_c,
                label: if batch_size == 1 {
                    format!("Deploy {} ({} • {:.0}G)", u.name, u.shape.short_role(), u_cost)
                } else {
                    format!("Deploy x{} {} (Wave • {:.0}G)", batch_size, u.name, total_c)
                },
                weight_gain: uv.raw_sum() * batch_size as f32,
                efficiency_per_gold: eff,
                lookahead_score: 6.0 + counter_adv * 2.0,
                utility_score: util,
            });
        }
    }

    // -------------------------------------------------------------
    // NODE CATEGORY 3: Commander Spells ([Q] Orbital Strike / [E] Surge)
    // -------------------------------------------------------------
    if state.ai.q_cd <= 0.0 && best_q_cluster.0 > 0 {
        let (c_count, c_val) = best_q_cluster;
        let q_min_count = match state.ai_difficulty {
            AiDifficulty::Easy => 5,
            AiDifficulty::Normal => 3,
            AiDifficulty::Hard | AiDifficulty::Extreme => 2,
        };
        if c_count >= q_min_count {
            let util = 45.0 + (c_count as f32) * 8.0 + (c_val * 0.35) * weights.spell_q_efficiency;
            nodes.push(ScoredNode {
                plan: CandidatePlan::CastSpellQ {
                    cluster_count: c_count,
                    cluster_val: c_val,
                },
                order: state.ai.order,
                gold_cost: 0.0,
                label: format!("Cast [Q] Orbital Blast ({} targets • {:.0}G)", c_count, c_val),
                weight_gain: (c_count as f32) * 0.35,
                efficiency_per_gold: 5.0,
                lookahead_score: 22.0,
                utility_score: util,
            });
        }
    }

    if state.ai.e_cd <= 0.0 {
        let injured = state
            .units
            .iter()
            .filter(|u| u.team == Team::Red && u.hp > 0.0 && u.hp < u.max_hp * 0.75)
            .count();
        if injured >= 2 {
            let util = 42.0 + (injured as f32) * 9.0 * weights.spell_e_efficiency;
            nodes.push(ScoredNode {
                plan: CandidatePlan::CastSpellE { injured_count: injured },
                order: state.ai.order,
                gold_cost: 0.0,
                label: format!("Cast [E] Surge & Heal ({} injured units)", injured),
                weight_gain: (injured as f32) * 0.25,
                efficiency_per_gold: 4.5,
                lookahead_score: 16.0,
                utility_score: util,
            });
        }
    }

    // -------------------------------------------------------------
    // NODE CATEGORY 4: Macro Economy & Tech Tier Upgrades (+ Eco/Tech Banking)
    // -------------------------------------------------------------
    if !urgent_defense {
        let eco_cost = state.income_upgrade_cost(Team::Red);
        let safe_for_macro = front_enemy_x < 780.0 && red_eff + 0.55 >= blue_eff;
        let max_eco = match state.ai_difficulty {
            AiDifficulty::Easy => 2,
            AiDifficulty::Normal => 5,
            AiDifficulty::Hard => 7,
            AiDifficulty::Extreme => 8,
        };

        let eco_util = 66.0 + (9 - state.ai.income_lvl as i32).max(1) as f32 * 5.2 * weights.eco_roi_weight;
        if state.ai.income_lvl < max_eco && safe_for_macro {
            if gold >= eco_cost as f32 {
                nodes.push(ScoredNode {
                    plan: CandidatePlan::MacroIncome { cost: eco_cost },
                    order: state.ai.order,
                    gold_cost: eco_cost as f32,
                    label: format!("Invest Economy -> Lvl {} (+2G/s, {}G)", state.ai.income_lvl + 1, eco_cost),
                    weight_gain: 0.9,
                    efficiency_per_gold: 2.5,
                    lookahead_score: 16.0,
                    utility_score: eco_util,
                });
            } else {
                let wait_s = (eco_cost as f32 - gold) / income_rate.max(1.0);
                let ahead_on_board = red_eff >= blue_eff + 0.15 && front_enemy_x < 680.0;
                if (ahead_on_board || gold >= (eco_cost as f32) * 0.42) && wait_s <= 10.0 {
                    nodes.push(ScoredNode {
                        plan: CandidatePlan::PoolForMacroEco {
                            target_gold: eco_cost as f32,
                            next_lvl: state.ai.income_lvl + 1,
                        },
                        order: ArmyOrder::Hold,
                        gold_cost: 0.0,
                        label: format!("Bank Gold ({:.0}/{}G) for Eco Lvl {}", gold, eco_cost, state.ai.income_lvl + 1),
                        weight_gain: 0.7,
                        efficiency_per_gold: 2.4,
                        lookahead_score: 12.0,
                        utility_score: eco_util - wait_s * 1.1,
                    });
                }
            }
        }

        let max_tier = state.max_ai_tech_tier();
        let req_eco_for_tier = if state.ai.tier == 1 { 2 } else { 3 };
        if state.ai.tier < max_tier && state.ai.income_lvl >= req_eco_for_tier && safe_for_macro {
            if let Some(t_cost) = MatchState::next_tier_cost(state.ai.tier) {
                let tech_util = 108.0 + (state.ai.tier as f32) * 14.0 * weights.tech_rush_weight;
                if gold >= t_cost as f32 {
                    nodes.push(ScoredNode {
                        plan: CandidatePlan::MacroTech { cost: t_cost },
                        order: state.ai.order,
                        gold_cost: t_cost as f32,
                        label: format!("Unlock Tech Tier {} ({}G)", state.ai.tier + 1, t_cost),
                        weight_gain: 1.6,
                        efficiency_per_gold: 2.7,
                        lookahead_score: 18.0,
                        utility_score: tech_util,
                    });
                } else {
                    let wait_s = (t_cost as f32 - gold) / income_rate.max(1.0);
                    let ahead_on_board = red_eff >= blue_eff + 0.15 && front_enemy_x < 680.0;
                    if (ahead_on_board || gold >= (t_cost as f32) * 0.35) && wait_s <= 16.0 {
                        nodes.push(ScoredNode {
                            plan: CandidatePlan::PoolForMacroTech {
                                target_gold: t_cost as f32,
                                next_tier: state.ai.tier + 1,
                            },
                            order: ArmyOrder::Hold,
                            gold_cost: 0.0,
                            label: format!("Bank Gold ({:.0}/{}G) for Tech T{}", gold, t_cost, state.ai.tier + 1),
                            weight_gain: 1.2,
                            efficiency_per_gold: 2.5,
                            lookahead_score: 14.0,
                            utility_score: tech_util - wait_s * 0.85,
                        });
                    }
                }
            }
        }
    }

    let total_evaled = nodes.len();

    // Run real 2.5-second mini combat simulation lookahead on top candidates
    if weights.lookahead_weight > 0.05 {
        for n in &mut nodes {
            let look = mini_combat_lookahead(state, n, threat_y);
            n.lookahead_score = look;
            n.utility_score += look * weights.lookahead_weight;
        }
    }

    // Deep Neural Network Actor-Critic evaluation (Intuition Guidance)
    let nn_vec = BattlefieldStateVector::extract_from_match(state);
    let (nn_val, nn_pol) = DeepNeuralBrain::load_or_default().forward_eval(&nn_vec);

    for n in &mut nodes {
        let policy_boost = match &n.plan {
            CandidatePlan::SoloSpawn { .. }
            | CandidatePlan::CounterWaveCombo { .. }
            | CandidatePlan::PoolForCounterWave { .. } => nn_pol[0],
            CandidatePlan::BaitUnderTurret { .. } => nn_pol[1],
            CandidatePlan::StrategicRetreat { .. } => nn_pol[2],
            CandidatePlan::CastSpellQ { .. } => nn_pol[3],
            CandidatePlan::CastSpellE { .. } => nn_pol[4],
            CandidatePlan::MacroIncome { .. }
            | CandidatePlan::MacroTech { .. }
            | CandidatePlan::PoolForMacroEco { .. }
            | CandidatePlan::PoolForMacroTech { .. } => nn_pol[5],
        };
        // Blend Neural Policy Intuition (up to +30% score influence)
        n.utility_score *= 0.85 + 0.30 * policy_boost;
    }

    // Sort nodes by final utility score descending
    nodes.sort_by(|a, b| b.utility_score.partial_cmp(&a.utility_score).unwrap_or(std::cmp::Ordering::Equal));

    let top_records: Vec<DecisionNodeRecord> = nodes
        .iter()
        .take(5)
        .enumerate()
        .map(|(idx, n)| DecisionNodeRecord {
            label: n.label.clone(),
            order: n.order,
            gold_cost: n.gold_cost,
            weight_gain: n.weight_gain,
            efficiency_per_gold: n.efficiency_per_gold,
            lookahead_delta: n.lookahead_score,
            utility_score: n.utility_score,
            chosen: idx == 0,
        })
        .collect();

    let best = nodes
        .first()
        .cloned()
        .unwrap_or_else(|| ScoredNode {
            plan: CandidatePlan::BaitUnderTurret {
                hold_order: ArmyOrder::Attack,
                counter_unit: unlocked[0],
            },
            order: ArmyOrder::Attack,
            gold_cost: 0.0,
            label: "Maintain Pressure".to_string(),
            weight_gain: 0.0,
            efficiency_per_gold: 1.0,
            lookahead_score: 0.0,
            utility_score: 10.0,
        });

    (best, top_records, total_evaled, nn_val, nn_pol)
}

fn unit_counter_score(
    u: &UnitDef,
    blue_w: &ArmyWeightVector,
    tracker: &PlayerStyleTracker,
    weights: &TrainedBrainWeights,
) -> f32 {
    let uv = unit_weight_vector(u);
    let mut score = uv.counter_advantage_against(*blue_w, weights);
    if tracker.swarm_threat_bias >= 0.40 && (matches!(u.shape, UnitShape::Diamond | UnitShape::Square) || u.passive_desc.contains("AoE")) {
        score += 1.2;
    }
    score
}

#[derive(Clone)]
struct MiniUnit {
    team: Team,
    x: f32,
    hp: f32,
    max_hp: f32,
    dps: f32,
    rng_px: f32,
    spd_px: f32,
    gold_val: f32,
    is_aoe: bool,
}

/// Fast micro-simulation lookahead: actually fast-forwards simplified battlefield units + Red front turret
/// by 2.5 seconds (10 × 0.25s steps) under this candidate node to project real combat trade efficiency.
fn mini_combat_lookahead(state: &MatchState, node: &ScoredNode, threat_y: f32) -> f32 {
    let _ = threat_y;
    let mut sim_units: Vec<MiniUnit> = state
        .units
        .iter()
        .filter(|u| u.hp > 0.0)
        .take(80)
        .map(|u| MiniUnit {
            team: u.team,
            x: u.x,
            hp: u.hp,
            max_hp: u.max_hp,
            dps: u.dmg / u.cd.max(0.25),
            rng_px: u.base_rng_px,
            spd_px: u.spd * 22.0,
            gold_val: u.def.cost as f32,
            is_aoe: matches!(u.shape, UnitShape::Diamond),
        })
        .collect();

    // Active frontmost standing Red turret assists if Hold/Bait brings fight into turret range
    let mut red_turret_x: Option<f32> = None;
    let mut red_turret_hp = 0.0_f32;
    for s in &state.structures {
        if s.team == Team::Red && s.hp > 0.0 {
            if red_turret_x.map(|rx| s.x < rx).unwrap_or(true) {
                red_turret_x = Some(s.x);
                red_turret_hp = s.hp;
            }
        }
    }

    let push_red = |list: &mut Vec<MiniUnit>, u: &UnitDef| {
        list.push(MiniUnit {
            team: Team::Red,
            x: WORLD_W - 145.0,
            hp: u.hp,
            max_hp: u.hp,
            dps: u.dmg / u.cd.max(0.25),
            rng_px: u.rng_studs * STUD_TO_PX,
            spd_px: u.spd_studs * 3.1 * 22.0,
            gold_val: u.cost as f32,
            is_aoe: matches!(u.shape, UnitShape::Diamond),
        });
    };

    let init_blue_val: f32 = sim_units
        .iter()
        .filter(|u| u.team == Team::Blue)
        .map(|u| u.gold_val * (u.hp / u.max_hp.max(1.0)))
        .sum();
    let mut init_red_val: f32 = sim_units
        .iter()
        .filter(|u| u.team == Team::Red)
        .map(|u| u.gold_val * (u.hp / u.max_hp.max(1.0)))
        .sum();

    let red_order = node.order;
    let _hold_x = red_turret_x.unwrap_or(WORLD_W - 180.0) - 55.0;

    // Apply candidate node's immediate deployment / spell onto the mini world
    match &node.plan {
        CandidatePlan::SoloSpawn { unit, batch, .. } => {
            for _ in 0..*batch {
                push_red(&mut sim_units, unit);
                init_red_val += unit.cost as f32;
            }
        }
        CandidatePlan::CounterWaveCombo { tank, support } => {
            push_red(&mut sim_units, tank);
            push_red(&mut sim_units, support);
            init_red_val += (tank.cost + support.cost) as f32;
        }
        CandidatePlan::CastSpellQ { .. } => {
            // [Q] deals 115 AoE to clustered forward Blue troops
            if let Some(cx) = sim_units.iter().filter(|u| u.team == Team::Blue).map(|u| u.x).max_by(|a, b| a.partial_cmp(b).unwrap()) {
                for u in &mut sim_units {
                    if u.team == Team::Blue && (u.x - cx).abs() <= 16.0 * STUD_TO_PX {
                        u.hp -= 115.0;
                    }
                }
            }
        }
        CandidatePlan::CastSpellE { .. } => {
            for u in &mut sim_units {
                if u.team == Team::Red && u.hp > 0.0 {
                    u.hp = (u.hp + 75.0).min(u.max_hp);
                    u.spd_px *= 1.45;
                }
            }
        }
        _ => {}
    }

    // Run 10 x 0.25s = 2.5s fast-forward simulation steps
    let dt = 0.25_f32;
    let init_turret_hp = red_turret_hp;
    for _step in 0..10 {
        let snap = sim_units.clone();
        for u in &mut sim_units {
            if u.hp <= 0.0 {
                continue;
            }
            let enemy_team = u.team.enemy();
            let nearest = snap
                .iter()
                .enumerate()
                .filter(|(_, e)| e.team == enemy_team && e.hp > 0.0)
                .min_by(|(_, a), (_, b)| (a.x - u.x).abs().partial_cmp(&(b.x - u.x).abs()).unwrap());

            if let Some((_, tgt)) = nearest {
                let dist = (tgt.x - u.x).abs();
                if dist <= u.rng_px.max(28.0) {
                    continue; // Attacking handled below
                }
            }

            if u.team == Team::Blue {
                let stop_x = red_turret_x.unwrap_or(WORLD_W - 95.0) - 20.0;
                if u.x < stop_x {
                    u.x += u.spd_px * dt;
                }
            } else {
                match red_order {
                    ArmyOrder::Attack => u.x -= u.spd_px * dt,
                    ArmyOrder::Hold => {
                        // Troops hold current position without moving
                    }
                    ArmyOrder::Retreat => {
                        u.x = (u.x + u.spd_px * 1.3 * dt).min(WORLD_W - 140.0);
                    }
                }
            }
        }

        // Resolve DPS & Turret defense
        let mut dmg_to_idx = vec![0.0_f32; sim_units.len()];
        for (_idx, u) in sim_units.iter().enumerate() {
            if u.hp <= 0.0 {
                continue;
            }
            let enemy_team = u.team.enemy();
            let mut best_i = None;
            let mut best_d = f32::INFINITY;
            for (ei, e) in sim_units.iter().enumerate() {
                if e.team == enemy_team && e.hp > 0.0 {
                    let d = (e.x - u.x).abs();
                    if d < best_d {
                        best_d = d;
                        best_i = Some(ei);
                    }
                }
            }
            if let Some(ti) = best_i {
                if best_d <= u.rng_px.max(28.0) {
                    dmg_to_idx[ti] += u.dps * dt * if u.is_aoe { 1.15 } else { 1.0 };
                }
            } else if u.team == Team::Blue {
                if let Some(rx) = red_turret_x {
                    if (rx - u.x).abs() <= u.rng_px.max(35.0) {
                        red_turret_hp = (red_turret_hp - u.dps * dt).max(0.0);
                    }
                }
            }
        }
        // Front Red turret shoots nearest Blue inside 260px
        if red_turret_hp > 0.0 {
            if let Some(rx) = red_turret_x {
                if let Some((bi, _)) = sim_units
                    .iter()
                    .enumerate()
                    .filter(|(_, u)| u.team == Team::Blue && u.hp > 0.0 && (rx - u.x).abs() <= 265.0)
                    .max_by(|(_, a), (_, b)| a.x.partial_cmp(&b.x).unwrap())
                {
                    dmg_to_idx[bi] += 36.0 * dt;
                }
            }
        }
        for (i, d) in dmg_to_idx.into_iter().enumerate() {
            sim_units[i].hp -= d;
        }
    }

    let end_blue_val: f32 = sim_units
        .iter()
        .filter(|u| u.team == Team::Blue && u.hp > 0.0)
        .map(|u| u.gold_val * (u.hp / u.max_hp.max(1.0)))
        .sum();
    let end_red_val: f32 = sim_units
        .iter()
        .filter(|u| u.team == Team::Red && u.hp > 0.0)
        .map(|u| u.gold_val * (u.hp / u.max_hp.max(1.0)))
        .sum();

    let blue_killed_val = (init_blue_val - end_blue_val).max(0.0);
    let red_lost_val = (init_red_val - end_red_val).max(0.0);
    let turret_dmg_taken = (init_turret_hp - red_turret_hp).max(0.0);

    // Reward Economy / Tech investments in the mini lookahead when board is stable (prevents starving macro)
    if matches!(
        node.plan,
        CandidatePlan::MacroIncome { .. }
            | CandidatePlan::MacroTech { .. }
            | CandidatePlan::PoolForMacroEco { .. }
            | CandidatePlan::PoolForMacroTech { .. }
    ) && end_red_val + 18.0 >= end_blue_val
        && turret_dmg_taken < 5.0
    {
        return 16.0;
    }

    // Reward = Enemy Gold Destroyed - Our Gold Lost - Turret Damage Penalty
    let net_trade = (blue_killed_val * 0.45) - (red_lost_val * 0.35) - (turret_dmg_taken * 0.12);
    net_trade.clamp(-25.0, 35.0)
}
