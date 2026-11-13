use rand::Rng;

use crate::ai_tree::{evaluate_battlefield_weights, search_decision_tree, CandidatePlan};
use crate::ai_weights::TrainedBrainWeights;
use crate::model::{
    AiDifficulty, AiPosture, ArmyOrder, FactionId, MissionArchetype, MutatorId, PassiveKey, Team,
    UnitDef, UnitShape, UnitTier, LANE_BOTTOM, LANE_TOP, STUD_TO_PX, WORLD_W,
};
use crate::sim::MatchState;
use crate::units::faction_units;

/// Parameterized personality settings controlling AI reaction speed, macro, and tactics.
#[derive(Debug, Clone, Copy)]
struct DifficultyProfile {
    min_interval: f32,
    max_interval: f32,
    eco_cap: u32,
    hold_line_eco_cap: u32,
    tech_cap: u8,
    min_eco_for_t2: u32,
    min_eco_for_t3: u32,
    /// Minimum fraction of next tier cost already banked before entering `TechUp` wait mode.
    tech_bank_ratio: f32,
    counter_weight: f32,
    mutator_weight: f32,
    /// Higher values prevent trickling bare backliners without a tank and allow short saves for better units.
    banking_discipline: f32,
    q_min_cluster: usize,
    q_min_value: f32,
    e_min_injured: usize,
    lane_precision: f32,
    lane_jitter: f32,
}

impl DifficultyProfile {
    fn for_difficulty(diff: AiDifficulty) -> Self {
        match diff {
            AiDifficulty::Easy => Self {
                min_interval: 1.25,
                max_interval: 1.70,
                eco_cap: 2,
                hold_line_eco_cap: 1,
                tech_cap: 2,
                min_eco_for_t2: 1,
                min_eco_for_t3: 99,
                tech_bank_ratio: 0.90,
                counter_weight: 0.15,
                mutator_weight: 0.15,
                banking_discipline: 0.0,
                q_min_cluster: 5,
                q_min_value: 85.0,
                e_min_injured: 4,
                lane_precision: 0.20,
                lane_jitter: 18.0,
            },
            AiDifficulty::Normal => Self {
                min_interval: 0.85,
                max_interval: 1.15,
                eco_cap: 5,
                hold_line_eco_cap: 2,
                tech_cap: 3,
                min_eco_for_t2: 2,
                min_eco_for_t3: 3,
                tech_bank_ratio: 0.58,
                counter_weight: 0.55,
                mutator_weight: 0.65,
                banking_discipline: 0.45,
                q_min_cluster: 3,
                q_min_value: 52.0,
                e_min_injured: 3,
                lane_precision: 0.65,
                lane_jitter: 9.0,
            },
            AiDifficulty::Hard => Self {
                min_interval: 0.62,
                max_interval: 0.85,
                eco_cap: 7,
                hold_line_eco_cap: 3,
                tech_cap: 3,
                min_eco_for_t2: 2,
                min_eco_for_t3: 4,
                tech_bank_ratio: 0.45,
                counter_weight: 1.0,
                mutator_weight: 1.05,
                banking_discipline: 0.80,
                q_min_cluster: 3,
                q_min_value: 44.0,
                e_min_injured: 2,
                lane_precision: 0.90,
                lane_jitter: 4.0,
            },
            AiDifficulty::Extreme => Self {
                min_interval: 0.48,
                max_interval: 0.66,
                eco_cap: 8,
                hold_line_eco_cap: 3,
                tech_cap: 3,
                min_eco_for_t2: 2,
                min_eco_for_t3: 4,
                tech_bank_ratio: 0.40,
                counter_weight: 1.35,
                mutator_weight: 1.30,
                banking_discipline: 1.0,
                q_min_cluster: 2,
                q_min_value: 42.0,
                e_min_injured: 2,
                lane_precision: 1.0,
                lane_jitter: 2.0,
            },
        }
    }
}

#[derive(Debug, Default)]
struct ShapeCounts {
    tanks: usize,     // Square + Hexagon
    strikers: usize,  // Circle
    marksmen: usize,  // Triangle
    mages: usize,     // Diamond
    healers: usize,   // Cross
}

impl ShapeCounts {
    fn record(&mut self, shape: UnitShape) {
        match shape {
            UnitShape::Square | UnitShape::Hexagon => self.tanks += 1,
            UnitShape::Circle => self.strikers += 1,
            UnitShape::Triangle => self.marksmen += 1,
            UnitShape::Diamond => self.mages += 1,
            UnitShape::Cross => self.healers += 1,
        }
    }

    fn ranged_total(&self) -> usize {
        self.marksmen + self.mages
    }
}

/// Snapshot of live field pressure, role composition, structures, and cluster spell value.
struct BattleAssessment {
    blue_count: usize,
    red_count: usize,
    blue_power: f32,
    red_power: f32,
    blue_front_x: f32,
    red_front_x: f32,
    threat_y: f32,
    red_vanguard_pos: Option<(f32, f32)>,
    red_boss_pos: Option<(f32, f32)>,
    blue_shapes: ShapeCounts,
    red_shapes: ShapeCounts,
    red_injured_count: usize,
    red_injured_value: f32,
    red_engaged_count: usize,
    red_t1_ratio: f32,
    red_t2_ratio: f32,
    red_base_ratio: f32,
    blue_t2_alive: bool,
    blue_base_ratio: f32,
    urgent_threat: bool,
    best_q_cluster_count: usize,
    best_q_cluster_value: f32,
    best_q_center_x: f32,
}

/// Builds a 6-card Skirmish deck from the faction's full 9-unit roster tailored to
/// difficulty, enemy shapes, and active arena mutator.
pub fn build_smart_ai_deck(
    enemy_faction: FactionId,
    player_deck: &[Option<&'static UnitDef>; 6],
    ai_difficulty: AiDifficulty,
    mutator: Option<MutatorId>,
) -> [String; 6] {
    let profile = DifficultyProfile::for_difficulty(ai_difficulty);
    let mut c_units: Vec<&UnitDef> = faction_units(enemy_faction)
        .filter(|u| u.tier == UnitTier::C)
        .collect();
    let mut b_units: Vec<&UnitDef> = faction_units(enemy_faction)
        .filter(|u| u.tier == UnitTier::B)
        .collect();
    let mut a_units: Vec<&UnitDef> = faction_units(enemy_faction)
        .filter(|u| u.tier == UnitTier::A)
        .collect();

    if ai_difficulty == AiDifficulty::Easy {
        return [
            c_units[0].id.to_string(),
            c_units[1].id.to_string(),
            b_units[0].id.to_string(),
            b_units[1].id.to_string(),
            a_units[0].id.to_string(),
            a_units[1].id.to_string(),
        ];
    }

    let mut player_shapes = ShapeCounts::default();
    for u in player_deck.iter().flatten() {
        player_shapes.record(u.shape);
    }

    let pick_two = |pool: &mut Vec<&'static UnitDef>, want_tank: bool, want_healer: bool| -> [&'static UnitDef; 2] {
        pool.sort_by(|a, b| {
            let sa = score_deck_candidate(a, &player_shapes, mutator, profile, want_tank, want_healer);
            let sb = score_deck_candidate(b, &player_shapes, mutator, profile, want_tank, want_healer);
            sb.partial_cmp(&sa).unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut chosen = [pool[0], pool[1]];
        if chosen[0].cost > chosen[1].cost {
            chosen.swap(0, 1);
        }
        chosen
    };

    let c_pair = pick_two(&mut c_units, true, false);
    let has_c_tank = c_pair.iter().any(|u| matches!(u.shape, UnitShape::Square));
    let b_pair = pick_two(&mut b_units, !has_c_tank, true);
    let a_pair = pick_two(&mut a_units, false, false);

    [
        c_pair[0].id.to_string(),
        c_pair[1].id.to_string(),
        b_pair[0].id.to_string(),
        b_pair[1].id.to_string(),
        a_pair[0].id.to_string(),
        a_pair[1].id.to_string(),
    ]
}

fn score_deck_candidate(
    u: &UnitDef,
    player_shapes: &ShapeCounts,
    mutator: Option<MutatorId>,
    profile: DifficultyProfile,
    want_tank: bool,
    want_healer: bool,
) -> f32 {
    let mut score = 50.0 + u.cost as f32 * 0.15;
    if want_tank && matches!(u.shape, UnitShape::Square | UnitShape::Hexagon) {
        score += 24.0;
    }
    if want_healer && u.shape == UnitShape::Cross {
        score += 22.0;
    }
    // Counter player deck composition
    let cw = profile.counter_weight;
    if player_shapes.tanks >= 2 && matches!(u.shape, UnitShape::Diamond | UnitShape::Square) {
        score += 18.0 * cw;
    }
    if player_shapes.strikers >= 2 && (u.shape == UnitShape::Square || u.passive_desc.contains("AoE")) {
        score += 18.0 * cw;
    }
    if player_shapes.ranged_total() >= 2 && matches!(u.shape, UnitShape::Circle | UnitShape::Square) {
        score += 16.0 * cw;
    }
    score += mutator_unit_bonus(u, mutator) * profile.mutator_weight;
    score
}

/// Evaluates how favorably the arena mutator interacts with a given unit archetype.
fn mutator_unit_bonus(u: &UnitDef, mutator: Option<MutatorId>) -> f32 {
    let Some(m) = mutator else {
        return 0.0;
    };
    match m {
        MutatorId::ConsecratedSolar => match u.shape {
            UnitShape::Cross => 34.0,
            UnitShape::Square | UnitShape::Circle => 8.0,
            UnitShape::Triangle => -18.0,
            _ => 0.0,
        },
        MutatorId::EmpScrambler => match u.shape {
            UnitShape::Circle | UnitShape::Square => 26.0,
            UnitShape::Triangle | UnitShape::Diamond => -32.0,
            _ => 6.0,
        },
        MutatorId::CorrosiveMiasma => match u.shape {
            UnitShape::Cross => 30.0,
            UnitShape::Square | UnitShape::Hexagon => 14.0,
            _ => -4.0,
        },
        MutatorId::GravityDistortion => {
            let mut b = if matches!(u.shape, UnitShape::Square | UnitShape::Hexagon) {
                12.0
            } else {
                0.0
            };
            if u.passive == PassiveKey::DeathImmunity {
                b += 26.0;
            }
            b
        }
        MutatorId::BloodFrenzy => match u.shape {
            UnitShape::Circle | UnitShape::Diamond => 16.0,
            UnitShape::Cross => 20.0,
            _ => 6.0,
        },
        MutatorId::NaniteRegeneration => match u.shape {
            UnitShape::Square | UnitShape::Hexagon => 28.0,
            _ => 0.0,
        },
    }
}

/// Counts Red AI deck slots by tier (C, B, A).
pub fn count_deck_by_tier(state: &MatchState) -> (usize, usize, usize) {
    let (mut c, mut b, mut a) = (0, 0, 0);
    for u in state.ai.deck.iter().flatten() {
        match u.tier {
            UnitTier::C => c += 1,
            UnitTier::B => b += 1,
            UnitTier::A => a += 1,
        }
    }
    (c, b, a)
}

/// Strategic AI Commander decision tick: updates posture, evaluates the multi-node
/// decision tree with trained troop weights & mini combat lookahead, casts spells,
/// invests in economy/tech tiers, and deploys coordinated counter-waves.
pub fn update_ai_commander<R: Rng>(state: &mut MatchState, dt: f32, rng: &mut R) {
    let weights = TrainedBrainWeights::for_difficulty(state.ai_difficulty);
    update_ai_commander_with_weights(state, dt, &weights, rng);
}

/// Evaluates & steps the AI Commander using an explicit `TrainedBrainWeights` profile
/// (used both in live battles and by the headless Rust self-play trainer).
pub fn update_ai_commander_with_weights<R: Rng>(
    state: &mut MatchState,
    dt: f32,
    weights: &TrainedBrainWeights,
    rng: &mut R,
) {
    if state.over {
        return;
    }

    state.ai.decision_timer -= dt;
    if state.ai.decision_timer > 0.0 {
        return;
    }

    let profile = DifficultyProfile::for_difficulty(state.ai_difficulty);
    state.ai.decision_timer = rng.gen_range(profile.min_interval..profile.max_interval);

    let intel = assess_battlefield(state);
    let (blue_w, red_w, blue_eff, red_eff) = evaluate_battlefield_weights(state, weights);
    state.ai_brain.blue_weight = blue_w;
    state.ai_brain.red_weight = red_w;
    state.ai_brain.blue_effective_weight = blue_eff;
    state.ai_brain.red_effective_weight = red_eff;

    let posture = decide_posture(state, &intel, profile);
    state.ai_posture = posture;

    let baseline_order = choose_army_order(state, &intel, posture);

    // Easy keeps classic lightweight behavior so M1.1 & campaign entry remain welcoming.
    if state.ai_difficulty == AiDifficulty::Easy {
        state.set_army_order(Team::Red, baseline_order);
        state.ai_brain.active_plan_summary = "Easy Heuristic Patrol".to_string();
        state.ai_brain.total_nodes_evaluated = 1;
        state.ai_brain.bait_mode_active = false;
        state.ai_brain.pooled_wave_active = false;
        try_cast_strategic_spells(state, &intel, posture, profile);
        if try_economy_and_tech(state, &intel, posture, profile) {
            return;
        }
        if let Some((unit_def, custom_y)) = pick_unit_and_lane(state, &intel, posture, profile, rng) {
            if state.ai.gold >= unit_def.cost as f32 {
                state.ai.gold -= unit_def.cost as f32;
                state.ai_brain.player_tracker.ai_gold_spent += unit_def.cost as f32;
                state.ai_bank_goal = 0.0;
                let spawn_x = WORLD_W - 140.0 - rng.gen_range(0.0..24.0);
                state.spawn_unit(Team::Red, unit_def, false, Some((spawn_x, custom_y)), rng);
            }
        }
        return;
    }

    // Normal / Hard / Extreme: Hybrid Decision Tree + Trained Weight Vectors + 2.5s Combat Lookahead
    let tracker_snapshot = state.ai_brain.player_tracker.clone();
    let (best_node, top_records, total_nodes, nn_val, nn_pol) = search_decision_tree(
        state,
        weights,
        &tracker_snapshot,
        blue_w,
        red_w,
        blue_eff,
        red_eff,
        intel.threat_y,
        intel.urgent_threat,
        (intel.best_q_cluster_count, intel.best_q_cluster_value),
        rng,
    );

    state.ai_brain.top_nodes = top_records;
    state.ai_brain.total_nodes_evaluated = total_nodes;
    state.ai_brain.active_plan_summary = best_node.label.clone();
    state.ai_brain.lookahead_score = best_node.lookahead_score;
    state.ai_brain.efficiency_ratio = best_node.efficiency_per_gold;
    state.ai_brain.neural_win_prob = nn_val;
    state.ai_brain.neural_policy = nn_pol;
    state.ai_brain.bait_mode_active = matches!(best_node.plan, CandidatePlan::BaitUnderTurret { .. });
    state.ai_brain.pooled_wave_active = matches!(
        best_node.plan,
        CandidatePlan::CounterWaveCombo { .. } | CandidatePlan::PoolForCounterWave { .. }
    );

    // Tactical spells fire immediately alongside deployment/macro so urgent clusters aren't missed
    try_cast_strategic_spells(state, &intel, posture, profile);

    // Tactical army stance: honor Retreat when forward troops are cut off, otherwise apply winning node stance
    let target_order = if baseline_order == ArmyOrder::Retreat {
        ArmyOrder::Retreat
    } else if matches!(
        best_node.plan,
        CandidatePlan::BaitUnderTurret { .. } | CandidatePlan::PoolForCounterWave { .. }
    ) {
        ArmyOrder::Hold
    } else {
        baseline_order
    };
    state.set_army_order(Team::Red, target_order);

    match best_node.plan {
        CandidatePlan::CounterWaveCombo { tank, support } => {
            let combo_cost = (tank.cost + support.cost) as f32;
            if state.ai.gold >= combo_cost {
                state.ai.gold -= combo_cost;
                state.ai_brain.player_tracker.ai_gold_spent += combo_cost;
                state.ai_bank_goal = 0.0;
                let tx = WORLD_W - 152.0 - rng.gen_range(0.0..12.0);
                let sx = WORLD_W - 118.0 - rng.gen_range(0.0..12.0);
                let ty = intel.threat_y.clamp(LANE_TOP + 30.0, LANE_BOTTOM - 30.0);
                let sy = (ty + rng.gen_range(-20.0..20.0)).clamp(LANE_TOP + 30.0, LANE_BOTTOM - 30.0);
                state.spawn_unit(Team::Red, tank, false, Some((tx, ty)), rng);
                state.spawn_unit(Team::Red, support, false, Some((sx, sy)), rng);
            }
        }
        CandidatePlan::SoloSpawn {
            unit,
            custom_y,
            batch,
        } => {
            for i in 0..batch {
                if state.ai.gold >= unit.cost as f32 {
                    state.ai.gold -= unit.cost as f32;
                    state.ai_brain.player_tracker.ai_gold_spent += unit.cost as f32;
                    state.ai_bank_goal = 0.0;
                    let sx = WORLD_W - 135.0 - (i as f32) * 14.0 - rng.gen_range(0.0..12.0);
                    let sy = (custom_y + (i as f32 - 0.5) * 24.0 + rng.gen_range(-8.0..8.0))
                        .clamp(LANE_TOP + 28.0, LANE_BOTTOM - 28.0);
                    state.spawn_unit(Team::Red, unit, false, Some((sx, sy)), rng);
                }
            }
        }
        CandidatePlan::BaitUnderTurret { counter_unit, .. } => {
            // While baiting enemy into turret range, reinforce turret defense if comfortable
            if state.ai.gold >= counter_unit.cost as f32 && intel.urgent_threat {
                state.ai.gold -= counter_unit.cost as f32;
                state.ai_brain.player_tracker.ai_gold_spent += counter_unit.cost as f32;
                let sx = WORLD_W - 140.0 - rng.gen_range(0.0..16.0);
                let sy = intel.threat_y.clamp(LANE_TOP + 28.0, LANE_BOTTOM - 28.0);
                state.spawn_unit(Team::Red, counter_unit, false, Some((sx, sy)), rng);
            }
        }
        CandidatePlan::PoolForCounterWave { target_gold, .. }
        | CandidatePlan::PoolForMacroEco { target_gold, .. }
        | CandidatePlan::PoolForMacroTech { target_gold, .. } => {
            state.ai_bank_goal = target_gold;
        }
        CandidatePlan::MacroIncome { .. } => {
            let _ = state.ai_try_upgrade_income();
            state.ai_bank_goal = 0.0;
        }
        CandidatePlan::MacroTech { .. } => {
            let _ = state.ai_try_unlock_tier();
            state.ai_bank_goal = 0.0;
        }
        CandidatePlan::CastSpellQ { .. }
        | CandidatePlan::CastSpellE { .. }
        | CandidatePlan::StrategicRetreat { .. } => {
            // Spells/stance already resolved above; invest remaining gold on defender or macro if urgent
            if !try_economy_and_tech(state, &intel, posture, profile) {
                if let Some((unit_def, custom_y)) = pick_unit_and_lane(state, &intel, posture, profile, rng) {
                    if state.ai.gold >= unit_def.cost as f32 {
                        state.ai.gold -= unit_def.cost as f32;
                        state.ai_brain.player_tracker.ai_gold_spent += unit_def.cost as f32;
                        let spawn_x = WORLD_W - 140.0 - rng.gen_range(0.0..24.0);
                        state.spawn_unit(Team::Red, unit_def, false, Some((spawn_x, custom_y)), rng);
                    }
                }
            }
        }
    }
}

fn assess_battlefield(state: &MatchState) -> BattleAssessment {
    let mut intel = BattleAssessment {
        blue_count: 0,
        red_count: 0,
        blue_power: 0.0,
        red_power: 0.0,
        blue_front_x: 100.0,
        red_front_x: WORLD_W - 100.0,
        threat_y: (LANE_TOP + LANE_BOTTOM) * 0.5,
        red_vanguard_pos: None,
        red_boss_pos: None,
        blue_shapes: ShapeCounts::default(),
        red_shapes: ShapeCounts::default(),
        red_injured_count: 0,
        red_injured_value: 0.0,
        red_engaged_count: 0,
        red_t1_ratio: 1.0,
        red_t2_ratio: 1.0,
        red_base_ratio: 1.0,
        blue_t2_alive: true,
        blue_base_ratio: 1.0,
        urgent_threat: false,
        best_q_cluster_count: 0,
        best_q_cluster_value: 0.0,
        best_q_center_x: WORLD_W * 0.5,
    };

    for st in &state.structures {
        let ratio = if st.max_hp > 0.0 {
            (st.hp / st.max_hp).clamp(0.0, 1.0)
        } else {
            0.0
        };
        match st.id {
            "e_t1" => intel.red_t1_ratio = ratio,
            "e_t2" => intel.red_t2_ratio = ratio,
            "e_base" => intel.red_base_ratio = ratio,
            "p_t2" => intel.blue_t2_alive = st.hp > 0.0,
            "p_base" => intel.blue_base_ratio = ratio,
            _ => {}
        }
    }

    let mut blue_units: Vec<(f32, f32, f32)> = Vec::new();
    let mut threat_y_sum = 0.0_f32;
    let mut threat_w_sum = 0.0_f32;
    let mut front_red_x = WORLD_W;

    for u in state.units.iter().filter(|u| u.hp > 0.0) {
        let hp_frac = (u.hp / u.max_hp.max(1.0)).clamp(0.15, 1.0);
        let value = (u.def.cost as f32).max(15.0) * (0.35 + 0.65 * hp_frac) * if u.is_boss { 2.4 } else { 1.0 };

        if u.team == Team::Blue {
            intel.blue_count += 1;
            intel.blue_power += value;
            intel.blue_shapes.record(u.shape);
            if u.x > intel.blue_front_x {
                intel.blue_front_x = u.x;
            }
            let forward_w = ((u.x / WORLD_W).max(0.1)).powi(2) * (u.def.cost as f32).max(12.0);
            threat_y_sum += u.y * forward_w;
            threat_w_sum += forward_w;
            blue_units.push((u.x, u.y, (u.def.cost as f32).max(12.0) * hp_frac));
        } else {
            intel.red_count += 1;
            intel.red_power += value;
            intel.red_shapes.record(u.shape);
            if u.x < intel.red_front_x {
                intel.red_front_x = u.x;
            }
            if u.is_boss {
                intel.red_boss_pos = Some((u.x, u.y));
            }
            if u.x < front_red_x && matches!(u.shape, UnitShape::Square | UnitShape::Hexagon | UnitShape::Circle) {
                front_red_x = u.x;
                intel.red_vanguard_pos = Some((u.x, u.y));
            }
            if u.hp < u.max_hp * 0.78 {
                intel.red_injured_count += 1;
                intel.red_injured_value += (u.max_hp - u.hp).min(90.0) * if u.is_boss { 2.0 } else { 1.0 };
            }
        }
    }

    if threat_w_sum > 0.0 {
        intel.threat_y = (threat_y_sum / threat_w_sum).clamp(LANE_TOP + 28.0, LANE_BOTTOM - 28.0);
    }

    // Sort Blue units by X so engagement & [Q] cluster scans run in O(B log B) instead of O(N^2)
    blue_units.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // Count Red units actively in close combat with Blue units
    let engage_r_sq = 130.0_f32 * 130.0;
    for u in state.units.iter().filter(|u| u.team == Team::Red && u.hp > 0.0) {
        let start = blue_units.partition_point(|&(bx, _, _)| bx < u.x - 130.0);
        for &(bx, by, _) in &blue_units[start..] {
            if bx > u.x + 130.0 {
                break;
            }
            let dx = bx - u.x;
            let dy = by - u.y;
            if dx * dx + dy * dy <= engage_r_sq {
                intel.red_engaged_count += 1;
                break;
            }
        }
    }

    // Evaluate best cluster target for Red's [Q] AoE Spell via sliding X-window
    let aoe_radius = 16.0 * STUD_TO_PX;
    let aoe_radius_sq = aoe_radius * aoe_radius;
    let mut hi = 0usize;
    let mut back = 0usize;
    for lo in 0..blue_units.len() {
        let (cx, cy, _) = blue_units[lo];
        while back < lo && cx - blue_units[back].0 > aoe_radius {
            back += 1;
        }
        while hi < blue_units.len() && blue_units[hi].0 - cx <= aoe_radius {
            hi += 1;
        }
        let mut count = 0usize;
        let mut gold_val = 0.0_f32;
        for &(ox, oy, val) in &blue_units[back..hi] {
            let dx = ox - cx;
            let dy = oy - cy;
            if dx * dx + dy * dy <= aoe_radius_sq {
                count += 1;
                gold_val += val;
            }
        }
        if gold_val > intel.best_q_cluster_value {
            intel.best_q_cluster_count = count;
            intel.best_q_cluster_value = gold_val;
            intel.best_q_center_x = cx;
        }
    }

    let first_line_x = if intel.red_t1_ratio > 0.0 { 910.0 } else { 1040.0 };
    intel.urgent_threat = intel.blue_front_x >= first_line_x
        || intel.red_base_ratio < 0.85
        || intel.red_t2_ratio < 0.70
        || (intel.blue_power > intel.red_power * 1.40 + 25.0 && intel.blue_front_x >= 680.0);

    intel
}

fn decide_posture(
    state: &MatchState,
    intel: &BattleAssessment,
    profile: DifficultyProfile,
) -> AiPosture {
    if intel.urgent_threat {
        return AiPosture::Defend;
    }

    // Siege opportunity when Blue inner defenses are cracked or Red has commanding board control
    if (!intel.blue_t2_alive || intel.blue_base_ratio < 0.65) && intel.red_power >= intel.blue_power {
        return AiPosture::AllIn;
    }
    if state.archetype == MissionArchetype::TitanSlayer
        && intel.red_boss_pos.map(|(bx, _)| bx < 880.0).unwrap_or(false)
        && intel.red_power >= intel.blue_power * 0.9
    {
        return AiPosture::AllIn;
    }

    // Tech unlock timing: Hold gold briefly if we actually have higher-tier cards in deck
    let max_deck_tier = state.max_ai_tech_tier().min(profile.tech_cap);
    if state.ai.tier < max_deck_tier {
        let req_eco = if state.ai.tier == 1 {
            profile.min_eco_for_t2
        } else {
            profile.min_eco_for_t3
        };
        let tier_cost = MatchState::next_tier_cost(state.ai.tier).unwrap_or(999) as f32;
        let has_field_buffer = intel.red_power + 22.0 >= intel.blue_power && intel.blue_front_x < 860.0;
        if state.ai.income_lvl >= req_eco
            && has_field_buffer
            && state.ai.gold >= tier_cost * profile.tech_bank_ratio
        {
            return AiPosture::TechUp;
        }
    }

    // Economy expansion when lane pressure is stable
    let eco_cap = effective_eco_cap(state, profile);
    let safe_for_eco = intel.blue_front_x < 780.0 && intel.red_power + 35.0 >= intel.blue_power;
    if state.ai.income_lvl < eco_cap && safe_for_eco && state.ai_difficulty != AiDifficulty::Easy {
        return AiPosture::EcoBoom;
    }
    if state.ai_difficulty == AiDifficulty::Easy
        && state.ai.income_lvl < eco_cap
        && safe_for_eco
        && intel.red_count >= intel.blue_count
    {
        return AiPosture::EcoBoom;
    }

    AiPosture::CounterPush
}

fn choose_army_order(state: &MatchState, intel: &BattleAssessment, posture: AiPosture) -> ArmyOrder {
    if state.ai_difficulty == AiDifficulty::Easy {
        return ArmyOrder::Attack;
    }

    if posture == AiPosture::AllIn {
        return ArmyOrder::Attack;
    }

    if state.ai_difficulty == AiDifficulty::Normal {
        let should_hold = (posture == AiPosture::Defend && intel.red_power < intel.blue_power * 0.92)
            || (intel.red_count > 0 && intel.red_count <= 2 && intel.blue_count >= 2 && intel.blue_front_x < 860.0);
        return if should_hold { ArmyOrder::Hold } else { ArmyOrder::Attack };
    }

    // Hard & Extreme: full Attack / Hold / Retreat maneuvering
    if intel.red_count > 0 {
        let stranded_deep = intel.red_front_x < 680.0 && intel.blue_power > intel.red_power * 1.35 + 25.0;
        let cut_off_on_defense = posture == AiPosture::Defend && intel.red_front_x < intel.blue_front_x - 100.0;
        if (stranded_deep || cut_off_on_defense) && intel.red_front_x < 960.0 {
            return ArmyOrder::Retreat;
        }
    }

    let critical_mass = intel.red_count >= 5
        || (intel.red_power >= intel.blue_power * 1.04 && (intel.red_shapes.tanks >= 1 || intel.red_count >= 3));
    if critical_mass || intel.red_count == 0 {
        return ArmyOrder::Attack;
    }

    let hold_to_stage = (posture == AiPosture::Defend && intel.red_power < intel.blue_power * 1.05)
        || (intel.red_count <= 4
            && (intel.red_power < intel.blue_power * 0.92
                || intel.red_shapes.tanks == 0
                || posture == AiPosture::TechUp))
        || (state.archetype == MissionArchetype::Blitzkrieg && intel.blue_front_x < 880.0);

    if hold_to_stage {
        ArmyOrder::Hold
    } else {
        ArmyOrder::Attack
    }
}

fn effective_eco_cap(state: &MatchState, profile: DifficultyProfile) -> u32 {
    if state.archetype == MissionArchetype::HoldTheLine {
        profile.hold_line_eco_cap
    } else if state.archetype == MissionArchetype::Blitzkrieg {
        // Blitzkrieg ends early; cap eco slightly lower so gold converts to blockers.
        profile.eco_cap.saturating_sub(2).max(2)
    } else {
        profile.eco_cap
    }
}

fn try_cast_strategic_spells(
    state: &mut MatchState,
    intel: &BattleAssessment,
    posture: AiPosture,
    profile: DifficultyProfile,
) {
    // [Q] Precision AoE Strike
    if state.ai.q_cd <= 0.0 && intel.blue_count > 0 {
        let siege_emergency = intel.best_q_center_x >= 960.0 && intel.best_q_cluster_count >= 2;
        let cluster_worthy = intel.best_q_cluster_count >= profile.q_min_cluster
            && intel.best_q_cluster_value >= profile.q_min_value;
        if siege_emergency || cluster_worthy {
            state.cast_commander_spell_q(Team::Red);
        }
    }

    // [E] Army Surge & Mass Heal (+75 HP & +45% SPD for 4s)
    if state.ai.e_cd <= 0.0 && intel.red_count > 0 {
        let boss_damaged = state.units.iter().any(|u| {
            u.team == Team::Red && u.is_boss && u.hp > 0.0 && u.hp < u.max_hp * 0.72
        });
        let heal_worthy = intel.red_injured_count >= profile.e_min_injured && intel.red_injured_value >= 80.0;
        let clash_push = state.ai_difficulty == AiDifficulty::Extreme
            && matches!(posture, AiPosture::CounterPush | AiPosture::AllIn)
            && intel.red_engaged_count >= 3
            && intel.red_count >= 4;
        let urgent_def_heal = posture == AiPosture::Defend && intel.red_injured_count >= 2 && intel.red_count >= 3;
        if boss_damaged || heal_worthy || clash_push || urgent_def_heal {
            state.cast_commander_spell_e(Team::Red);
        }
    }
}

/// Evaluates whether the AI Commander should upgrade Economy (`income_lvl`),
/// unlock the next Troop Tier (`tier`), or bank gold briefly for that milestone.
/// Returns `true` when AI decided to wait/bank gold instead of spending on troops.
fn try_economy_and_tech(
    state: &mut MatchState,
    intel: &BattleAssessment,
    posture: AiPosture,
    profile: DifficultyProfile,
) -> bool {
    let max_deck_tier = state.max_ai_tech_tier().min(profile.tech_cap);
    let can_unlock_next = state.ai.tier < max_deck_tier;

    // 1. Unlock next Troop Tier immediately if affordable & posture isn't critical emergency
    if can_unlock_next {
        if let Some(tier_cost) = MatchState::next_tier_cost(state.ai.tier) {
            let allow_unlock = match posture {
                AiPosture::Defend => state.ai.gold >= tier_cost as f32 + 30.0 && intel.red_count >= 2,
                _ => true,
            };
            if allow_unlock && state.ai.gold >= tier_cost as f32 {
                if state.ai_try_unlock_tier() {
                    state.ai_bank_goal = 0.0;
                    return false;
                }
            }
            if posture == AiPosture::TechUp && state.ai.gold < tier_cost as f32 {
                state.ai_bank_goal = tier_cost as f32;
                return true;
            }
        }
    }

    // 2. Upgrade Gold Economy based on safe board window & payback horizon
    let eco_cap = effective_eco_cap(state, profile);
    if state.ai.income_lvl < eco_cap {
        let eco_cost = state.income_upgrade_cost(Team::Red) as f32;
        let urgent = matches!(posture, AiPosture::Defend) || intel.red_base_ratio < 0.85;
        let safe_to_invest = !urgent
            && (intel.red_power + 25.0 >= intel.blue_power || intel.blue_front_x < 700.0);

        // Don't spend eco gold right before finishing a Tier unlock we were banking for.
        let waiting_on_tech = can_unlock_next
            && posture == AiPosture::TechUp
            && MatchState::next_tier_cost(state.ai.tier)
                .map(|c| state.ai.gold >= c as f32 * 0.65)
                .unwrap_or(false);

        if safe_to_invest && !waiting_on_tech {
            if state.ai.gold >= eco_cost {
                if state.ai_try_upgrade_income() {
                    state.ai_bank_goal = 0.0;
                    return false;
                }
            } else if posture == AiPosture::EcoBoom
                && profile.banking_discipline >= 0.45
                && eco_cost - state.ai.gold <= state.get_income_rate(Team::Red) * 4.5
            {
                state.ai_bank_goal = eco_cost;
                return true;
            }
        }
    }

    false
}

fn pick_unit_and_lane<R: Rng>(
    state: &mut MatchState,
    intel: &BattleAssessment,
    posture: AiPosture,
    profile: DifficultyProfile,
    rng: &mut R,
) -> Option<(&'static UnitDef, f32)> {
    let unlocked: Vec<&'static UnitDef> = state
        .ai
        .deck
        .iter()
        .flatten()
        .copied()
        .filter(|u| u.tier.required_tech_tier() <= state.ai.tier)
        .collect();

    if unlocked.is_empty() {
        return None;
    }

    let mut best_unit: Option<&'static UnitDef> = None;
    let mut best_score = f32::NEG_INFINITY;

    for u in &unlocked {
        let mut s = score_battle_unit(state, u, intel, posture, profile);
        if state.ai_difficulty == AiDifficulty::Easy {
            s += rng.gen_range(-25.0..25.0);
        } else {
            s += rng.gen_range(-4.0..4.0);
        }
        if s > best_score {
            best_score = s;
            best_unit = Some(u);
        }
    }

    let chosen = best_unit?;
    let gold = state.ai.gold;

    // Banking Discipline: on Normal+, hold gold briefly so we deploy the strategic
    // best unit (Tank/Healer/Mythic) rather than wasting gold on cheap mismatched fodder.
    if gold < chosen.cost as f32 {
        let wait_secs = (chosen.cost as f32 - gold) / state.get_income_rate(Team::Red).max(1.0);
        let critical_emergency = intel.red_base_ratio < 0.65 || intel.blue_front_x >= 1120.0;
        let can_patiently_wait = !critical_emergency
            && profile.banking_discipline > 0.25
            && gold >= chosen.cost as f32 * (0.72 - 0.16 * profile.banking_discipline)
            && wait_secs <= 5.5 * profile.banking_discipline;

        if can_patiently_wait {
            state.ai_bank_goal = chosen.cost as f32;
            return None;
        }

        // Otherwise pick highest-scoring currently affordable unit.
        let affordable = unlocked
            .into_iter()
            .filter(|u| gold >= u.cost as f32)
            .max_by(|a, b| {
                let sa = score_battle_unit(state, a, intel, posture, profile);
                let sb = score_battle_unit(state, b, intel, posture, profile);
                sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
            })?;

        let y = pick_spawn_y(affordable, intel, posture, profile, rng);
        return Some((affordable, y));
    }

    let y = pick_spawn_y(chosen, intel, posture, profile, rng);
    Some((chosen, y))
}

fn score_battle_unit(
    state: &MatchState,
    u: &UnitDef,
    intel: &BattleAssessment,
    posture: AiPosture,
    profile: DifficultyProfile,
) -> f32 {
    let mut score = 40.0 + (u.cost as f32) * 0.38;
    let tier_mult = match u.tier {
        UnitTier::C => 0.0,
        UnitTier::B => 12.0,
        UnitTier::A => 26.0,
    };
    score += tier_mult;

    // 1. Role Composition & Combined-Arms Synergy
    let red_frontliners = intel.red_shapes.tanks + if intel.red_boss_pos.is_some() { 1 } else { 0 };
    match u.shape {
        UnitShape::Square | UnitShape::Hexagon => {
            if red_frontliners == 0 {
                score += 42.0; // Establish frontline before fielding fragile DPS
            } else if red_frontliners < intel.red_shapes.ranged_total() + 1 {
                score += 18.0;
            }
            if posture == AiPosture::Defend {
                score += 24.0;
            }
        }
        UnitShape::Circle => {
            if red_frontliners == 0 && posture != AiPosture::Defend {
                score += 16.0;
            }
            if posture == AiPosture::AllIn {
                score += 14.0;
            }
        }
        UnitShape::Triangle | UnitShape::Diamond => {
            if red_frontliners == 0 && state.ai_difficulty != AiDifficulty::Easy {
                score -= 22.0; // Avoid feeding unsupported backline
            } else {
                score += 20.0;
            }
        }
        UnitShape::Cross => {
            if intel.red_shapes.healers >= 2 {
                score -= 48.0; // Never over-stack healers
            } else if intel.red_count >= 2
                && (intel.red_injured_count >= 1 || intel.red_boss_pos.is_some() || red_frontliners >= 1)
            {
                score += 34.0;
            } else {
                score -= 35.0; // Never spawn an isolated healer
            }
        }
    }

    // 2. Counter-Composition vs Live Blue Army
    let cw = profile.counter_weight;
    if intel.blue_shapes.tanks >= 2 {
        if matches!(u.shape, UnitShape::Diamond) || u.passive == PassiveKey::AcidPoison {
            score += 26.0 * cw;
        }
    }
    if intel.blue_shapes.strikers >= 3 {
        if u.shape == UnitShape::Square || u.passive_desc.contains("AoE") || u.passive_desc.contains("chain") {
            score += 24.0 * cw;
        }
    }
    if intel.blue_shapes.ranged_total() >= 3 {
        if matches!(u.shape, UnitShape::Circle | UnitShape::Square) {
            score += 22.0 * cw;
        }
    }
    if intel.blue_shapes.healers >= 1 && matches!(u.shape, UnitShape::Diamond | UnitShape::Circle) {
        score += 16.0 * cw;
    }

    // 3. Mutator Adaptation
    score += mutator_unit_bonus(u, state.active_mutator) * profile.mutator_weight;

    // 4. Mission Archetype Adaptation
    match state.archetype {
        MissionArchetype::Blitzkrieg => {
            // Red wins if Blue runs out of time -> favor durable defensive walls & sustain
            if matches!(u.shape, UnitShape::Square | UnitShape::Hexagon | UnitShape::Cross) {
                score += 18.0 * profile.counter_weight.max(0.5);
            }
        }
        MissionArchetype::HoldTheLine => {
            // Reinforce scripted waves with sustain & ranged fire without flood-spamming cheap melee
            if matches!(u.shape, UnitShape::Cross | UnitShape::Triangle | UnitShape::Diamond) {
                score += 12.0;
            }
        }
        MissionArchetype::TitanSlayer => {
            if intel.red_boss_pos.is_some()
                && matches!(u.shape, UnitShape::Cross | UnitShape::Square | UnitShape::Triangle)
            {
                score += 18.0;
            }
        }
        _ => {}
    }

    score
}

fn pick_spawn_y<R: Rng>(
    unit: &UnitDef,
    intel: &BattleAssessment,
    posture: AiPosture,
    profile: DifficultyProfile,
    rng: &mut R,
) -> f32 {
    let mid_y = (LANE_TOP + LANE_BOTTOM) * 0.5;
    let target_y = if posture == AiPosture::Defend || intel.blue_front_x >= 800.0 {
        intel.threat_y
    } else if let Some((_, boss_y)) = intel.red_boss_pos {
        boss_y
    } else if let Some((_, van_y)) = intel.red_vanguard_pos {
        van_y
    } else {
        intel.threat_y
    };

    // Stagger ranged/healer units slightly off exact frontline center so stacks spread cleanly
    let role_offset = match unit.shape {
        UnitShape::Triangle | UnitShape::Diamond => rng.gen_range(-24.0..24.0),
        UnitShape::Cross => rng.gen_range(-10.0..10.0),
        _ => 0.0,
    };

    let blended_y = mid_y * (1.0 - profile.lane_precision) + target_y * profile.lane_precision;
    let jitter = rng.gen_range(-profile.lane_jitter..profile.lane_jitter);
    (blended_y + role_offset + jitter).clamp(LANE_TOP + 26.0, LANE_BOTTOM - 26.0)
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use super::*;
    use crate::model::{AiDifficulty, AiPosture, FactionId, MutatorId, Team, UnitShape};
    use crate::save::ConquestSave;
    use crate::sim::MatchState;
    use crate::units::{default_faction_deck, get_unit};

    #[test]
    fn ai_upgrades_economy_and_valid_tiers_without_wasting_empty_tiers() {
        // 1. Skirmish Hard: AI should upgrade both income_lvl and tier up to T2/T3
        let mut rng = StdRng::seed_from_u64(42);
        let mut skirmish = MatchState::new_skirmish(
            FactionId::Solaris,
            &default_faction_deck(FactionId::Solaris),
            FactionId::Mechanica,
            AiDifficulty::Hard,
            None,
            vec![],
            &mut rng,
        );
        for _ in 0..900 {
            skirmish.update(0.1, &mut rng);
            if skirmish.over {
                break;
            }
        }
        assert!(skirmish.ai.income_lvl >= 2, "Expected Hard AI eco >= 2, got {}", skirmish.ai.income_lvl);
        assert!(skirmish.ai.tier >= 2, "Expected Hard AI tier >= 2, got {}", skirmish.ai.tier);

        // 2. Campaign M1_1 (C-tier only deck): AI must NEVER waste gold unlocking T2
        let save = ConquestSave::new_default(FactionId::Solaris);
        let mut m1 = MatchState::from_campaign_mission("M1_1", &save, &mut rng).unwrap();
        m1.ai.gold = 500.0;
        for _ in 0..200 {
            m1.update(0.1, &mut rng);
            if m1.over {
                break;
            }
        }
        assert_eq!(m1.ai.tier, 1, "M1_1 AI has only C units and must remain Tier 1");

        // 3. Campaign M1_3 (C + B units, no A units): AI unlocks T2, never wastes 250G on T3
        let mut m3 = MatchState::from_campaign_mission("M1_3", &save, &mut rng).unwrap();
        m3.ai.gold = 600.0;
        for _ in 0..200 {
            m3.update(0.1, &mut rng);
            if m3.over {
                break;
            }
        }
        assert!(m3.ai.tier <= 2, "M1_3 AI has no A-tier units so tier must cap at 2, got {}", m3.ai.tier);
    }

    #[test]
    fn ai_defends_threatened_lane_and_casts_q_on_cluster() {
        let mut rng = StdRng::seed_from_u64(77);
        let mut state = MatchState::new_skirmish(
            FactionId::Solaris,
            &default_faction_deck(FactionId::Solaris),
            FactionId::Aegis,
            AiDifficulty::Hard,
            None,
            vec![],
            &mut rng,
        );
        state.ai.q_cd = 0.0;
        state.ai.gold = 120.0;
        state.ai.decision_timer = 0.01;

        // Place a clustered Blue assault on the upper lane near Red's outer turret
        let attacker = get_unit("sol_1").unwrap();
        for i in 0..4 {
            state.spawn_unit(
                Team::Blue,
                attacker,
                false,
                Some((980.0 - i as f32 * 12.0, 145.0)),
                &mut rng,
            );
        }

        state.update(0.05, &mut rng);

        assert_eq!(state.ai_posture, AiPosture::Defend);
        assert!(state.ai.q_cd > 10.0, "Hard AI should fire [Q] on clustered Blue siege");
        let spawned_red: Vec<_> = state
            .units
            .iter()
            .filter(|u| u.team == Team::Red && u.y < 230.0)
            .collect();
        assert!(!spawned_red.is_empty(), "Defender should spawn aligned toward upper threatened lane");
    }

    #[test]
    fn smart_skirmish_deck_adapts_to_mutators_and_enemy_composition() {
        let p_tank_ids = [
            "sol_3".to_string(),
            "sol_3".to_string(),
            "sol_4".to_string(),
            "sol_4".to_string(),
            "sol_7".to_string(),
            "sol_7".to_string(),
        ];
        let p_tank_deck: [Option<&UnitDef>; 6] = p_tank_ids.map(|id| get_unit(&id));

        // ConsecratedSolar boosts Cross healers (+50%) -> Solaris AI should include sol_6 Priest
        let solar_deck = build_smart_ai_deck(
            FactionId::Solaris,
            &p_tank_deck,
            AiDifficulty::Hard,
            Some(MutatorId::ConsecratedSolar),
        );
        assert!(
            solar_deck.iter().any(|id| id == "sol_6"),
            "Expected Hard Solaris AI on ConsecratedSolar to draft Priest (sol_6), got {solar_deck:?}"
        );

        // EmpScrambler penalizes Triangle/Diamond and buffs Circle/Square -> Aegis B/A shouldn't double-draft weak ranged
        let emp_deck = build_smart_ai_deck(
            FactionId::Aegis,
            &p_tank_deck,
            AiDifficulty::Extreme,
            Some(MutatorId::EmpScrambler),
        );
        let ranged_cnt = emp_deck
            .iter()
            .filter_map(|id| get_unit(id))
            .filter(|u| matches!(u.shape, UnitShape::Triangle | UnitShape::Diamond))
            .count();
        assert!(
            ranged_cnt <= 2,
            "Expected Extreme AI on EmpScrambler to limit penalized ranged units, got {emp_deck:?}"
        );
    }

    #[test]
    fn hard_and_extreme_outscale_easy_over_match_timeline() {
        let simulate = |diff: AiDifficulty| -> (u32, u8, usize) {
            let mut rng = StdRng::seed_from_u64(1234);
            let mut state = MatchState::new_skirmish(
                FactionId::Solaris,
                &default_faction_deck(FactionId::Solaris),
                FactionId::Solaris,
                diff,
                None,
                vec![],
                &mut rng,
            );
            // Simulate steady Blue player spawning Footmen + Knight pressure
            for step in 0..800 {
                if step % 60 == 15 {
                    let _ = state.player_try_upgrade_income();
                }
                if step % 35 == 10 {
                    let _ = state.player_try_spawn_slot(0, &mut rng);
                }
                state.update(0.1, &mut rng);
                if state.over {
                    break;
                }
            }
            let red_alive = state.units.iter().filter(|u| u.team == Team::Red && u.hp > 0.0).count();
            (state.ai.income_lvl, state.ai.tier, red_alive)
        };

        let (easy_eco, easy_tier, _) = simulate(AiDifficulty::Easy);
        let (hard_eco, hard_tier, _) = simulate(AiDifficulty::Hard);
        let (ext_eco, ext_tier, _) = simulate(AiDifficulty::Extreme);

        assert!(hard_eco > easy_eco, "Hard eco ({hard_eco}) should exceed Easy eco ({easy_eco})");
        assert!(ext_eco > easy_eco, "Extreme eco ({ext_eco}) should exceed Easy ({easy_eco})");
        assert!(hard_tier >= easy_tier, "Hard tier ({hard_tier}) should be >= Easy ({easy_tier})");
        assert!(ext_tier >= hard_tier, "Extreme tier ({ext_tier}) should be >= Hard ({hard_tier})");
    }

    #[test]
    fn army_orders_control_player_and_ai_troop_movement() {
        let mut rng = StdRng::seed_from_u64(99);
        let mut state = MatchState::new_skirmish(
            FactionId::Solaris,
            &default_faction_deck(FactionId::Solaris),
            FactionId::Hellfire,
            AiDifficulty::Hard,
            None,
            vec![],
            &mut rng,
        );

        // 1. Player HOLD order keeps troops stationary in place
        let knight = get_unit("sol_4").unwrap();
        let uid = state.spawn_unit(Team::Blue, knight, false, Some((700.0, 265.0)), &mut rng);
        state.set_army_order(Team::Blue, ArmyOrder::Hold);
        for _ in 0..20 {
            state.update(0.1, &mut rng);
        }
        let x_after_hold = state.units.iter().find(|u| u.uid == uid).unwrap().x;
        assert!((x_after_hold - 700.0).abs() < 1.0, "Blue Hold should keep troops stationary in place, x={x_after_hold}");

        // 2. Player ATTACK order marches troops forward
        state.set_army_order(Team::Blue, ArmyOrder::Attack);
        for _ in 0..20 {
            state.update(0.1, &mut rng);
        }
        let x_after_attack = state.units.iter().find(|u| u.uid == uid).unwrap().x;
        assert!(x_after_attack > 700.0, "Blue Attack should advance forward, x={x_after_attack}");

        // 3. Player RETREAT order sprints troops back toward Blue Bastion (moving backwards)
        state.set_army_order(Team::Blue, ArmyOrder::Retreat);
        for _ in 0..35 {
            state.update(0.1, &mut rng);
        }
        let x_after_retreat = state.units.iter().find(|u| u.uid == uid).unwrap().x;
        assert!(x_after_retreat < x_after_attack, "Blue Retreat should fall back toward Bastion, x={x_after_retreat}");

        // 3. Hard AI issues Retreat when outnumbered forward troops are deep in Blue territory
        let imp = get_unit("hel_1").unwrap();
        let elite = get_unit("sol_7").unwrap();
        state.spawn_unit(Team::Red, imp, false, Some((480.0, 265.0)), &mut rng);
        for i in 0..4 {
            state.spawn_unit(Team::Blue, elite, false, Some((620.0 + i as f32 * 10.0, 265.0)), &mut rng);
        }
        state.ai.decision_timer = 0.01;
        state.update(0.05, &mut rng);
        assert_eq!(state.ai.order, ArmyOrder::Retreat);
    }
}

