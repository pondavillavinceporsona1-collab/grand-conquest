use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

use crate::ai::update_ai_commander_with_weights;
use crate::ai_weights::TrainedBrainWeights;
use crate::model::{AiDifficulty, FactionId, MutatorId, Team};
use crate::sim::MatchState;
use crate::units::default_faction_deck;

#[derive(Debug, Clone)]
pub struct TrainingSummary {
    pub generations: usize,
    pub matches_played: usize,
    pub baseline_fitness: f32,
    pub best_fitness: f32,
    pub best_win_rate: f32,
    pub avg_gold_efficiency: f32,
    pub best_weights: TrainedBrainWeights,
}

/// Evaluates a candidate `TrainedBrainWeights` across multiple headless matches against
/// distinct player strategies (Swarm Bombing, Heavy Tank+Mage Push, Ranged Siege)
/// and computes a scalar Reward/Punishment fitness score.
pub fn evaluate_weights_fitness(
    candidate: &TrainedBrainWeights,
    seed: u64,
    matches_per_candidate: usize,
) -> (f32, f32, f32) {
    let mut rng = StdRng::seed_from_u64(seed);
    let matchups = [
        (FactionId::Solaris, FactionId::Hellfire, None),
        (FactionId::Hive, FactionId::Mechanica, Some(MutatorId::NaniteRegeneration)),
        (FactionId::Aegis, FactionId::Void, Some(MutatorId::EmpScrambler)),
        (FactionId::Hellfire, FactionId::Solaris, Some(MutatorId::ConsecratedSolar)),
    ];

    let mut total_reward = 0.0_f32;
    let mut wins = 0usize;
    let mut eff_sum = 0.0_f32;

    for m_idx in 0..matches_per_candidate {
        let (p_fac, e_fac, mut_id) = matchups[m_idx % matchups.len()];
        let mut state = MatchState::new_skirmish(
            p_fac,
            &default_faction_deck(p_fac),
            e_fac,
            AiDifficulty::Extreme,
            mut_id,
            vec![],
            &mut rng,
        );
        // Fair starting state for training benchmark (equal starting gold, no handicap)
        state.player.gold = 60.0;
        state.ai.gold = 60.0;

        let strategy_mode = m_idx % 3;
        let max_steps = 1100; // up to 110s per match at dt=0.1
        for step in 0..max_steps {
            // Opponent simulator: tests whether candidate AI beats different human habits
            match strategy_mode {
                0 => {
                    // Swarm Bomber: upgrades eco then bursts x5..x10 C-tier waves
                    if step % 50 == 10 && state.player.income_lvl < 4 {
                        let _ = state.player_try_upgrade_income();
                    }
                    if step % 65 == 40 {
                        let _ = state.player_try_spawn_slot_count(0, 8, &mut rng);
                    }
                }
                1 => {
                    // Heavy Tank + Mage Techer
                    if step % 40 == 10 && state.player.income_lvl < 4 {
                        let _ = state.player_try_upgrade_income();
                    }
                    if state.player.tier < 2 && state.player.gold >= 95.0 {
                        let _ = state.player_try_unlock_tier();
                    }
                    if step % 35 == 20 {
                        let slot = if state.player.tier >= 2 { 2 + (step / 35) % 2 } else { 0 };
                        let _ = state.player_try_spawn_slot_count(slot, 1, &mut rng);
                    }
                }
                _ => {
                    // Balanced Pressure + Spells
                    if step % 55 == 5 && state.player.income_lvl < 3 {
                        let _ = state.player_try_upgrade_income();
                    }
                    if step % 30 == 12 {
                        let _ = state.player_try_spawn_slot_count((step / 30) % 2, 2, &mut rng);
                    }
                    if step % 150 == 90 {
                        state.cast_commander_spell_q(Team::Blue);
                    }
                }
            }

            // Step world physics & override AI update tick with candidate weights
            step_with_custom_weights(&mut state, 0.1, candidate, &mut rng);
            if state.over {
                break;
            }
        }

        // Compute Reward & Punishment score
        let mut reward = 0.0_f32;
        if state.winner == Some(Team::Red) {
            wins += 1;
            reward += 1000.0 + (120.0 - state.time).max(0.0) * 3.0;
        } else if state.winner == Some(Team::Blue) {
            reward -= 750.0;
        } else {
            // Timeout tiebreaker by structure HP advantage
            let red_hp: f32 = state.structures.iter().filter(|s| s.team == Team::Red).map(|s| s.hp).sum();
            let blue_hp: f32 = state.structures.iter().filter(|s| s.team == Team::Blue).map(|s| s.hp).sum();
            reward += (red_hp - blue_hp) * 0.25;
        }

        // Turret preservation bonus
        let red_turret_hp: f32 = state
            .structures
            .iter()
            .filter(|s| {
                s.team == Team::Red
                    && matches!(
                        s.kind,
                        crate::model::StructureKind::Turret1 | crate::model::StructureKind::Turret2
                    )
            })
            .map(|s| s.hp)
            .sum();
        reward += red_turret_hp * 0.22;

        // Gold efficiency reward: Enemy gold killed / Red gold spent
        let red_spent = state.ai_brain.player_tracker.ai_gold_spent.max(20.0);
        let blue_killed_est = (state.stats.enemy_kills as f32) * 8.0 + state.ai_brain.player_tracker.total_gold_spent * 0.6;
        let efficiency = (blue_killed_est / red_spent).clamp(0.1, 4.0);
        eff_sum += efficiency;
        reward += efficiency * 180.0;

        total_reward += reward;
    }

    let n = matches_per_candidate.max(1) as f32;
    (total_reward / n, (wins as f32) / n, eff_sum / n)
}

/// Helper step that ticks physics & structures while running `update_ai_commander_with_weights`.
fn step_with_custom_weights<R: Rng>(
    state: &mut MatchState,
    dt: f32,
    weights: &TrainedBrainWeights,
    rng: &mut R,
) {
    // Pause built-in AI tick inside `state.update` by shifting its timer temporarily,
    // then run our custom candidate weights tick right after.
    let saved_timer = state.ai.decision_timer;
    state.ai.decision_timer = 999.0;
    state.update(dt, rng);
    state.ai.decision_timer = saved_timer;
    if !state.over {
        update_ai_commander_with_weights(state, dt, weights, rng);
    }
}

/// Runs an evolutionary self-play training loop (mutation + elitist selection using
/// Reward & Punishment fitness) and returns the best calibrated `TrainedBrainWeights`.
pub fn train_weights_evolutionary(
    generations: usize,
    population_size: usize,
    matches_per_eval: usize,
    seed: u64,
) -> TrainingSummary {
    train_weights_evolutionary_seeded(generations, population_size, matches_per_eval, seed, None)
}

/// Runs evolutionary training starting from an optional seed baseline weights profile.
pub fn train_weights_evolutionary_seeded(
    generations: usize,
    population_size: usize,
    matches_per_eval: usize,
    seed: u64,
    seed_weights: Option<TrainedBrainWeights>,
) -> TrainingSummary {
    let mut rng = StdRng::seed_from_u64(seed);
    let baseline = seed_weights.unwrap_or_else(TrainedBrainWeights::hard_trained);
    let (base_fit, _, _) = evaluate_weights_fitness(&baseline, seed, matches_per_eval);

    let mut population = Vec::with_capacity(population_size);
    population.push(baseline);
    if seed_weights.is_none() {
        population.push(TrainedBrainWeights::extreme_master());
    }
    while population.len() < population_size {
        population.push(mutate_weights(&baseline, &mut rng, 0.18));
    }

    let mut best_w = population[0];
    let mut best_fit = f32::NEG_INFINITY;
    let mut best_wr = 0.0_f32;
    let mut best_eff = 0.0_f32;
    let mut total_matches = matches_per_eval; // include baseline eval

    for gen_idx in 0..generations {
        let mut scored: Vec<(f32, f32, f32, TrainedBrainWeights)> = population
            .par_iter()
            .enumerate()
            .map(|(i, w)| {
                let eval_seed = seed.wrapping_add((gen_idx as u64) * 1009 + (i as u64) * 37);
                let (fit, wr, eff) = evaluate_weights_fitness(w, eval_seed, matches_per_eval);
                (fit, wr, eff, *w)
            })
            .collect();
        total_matches += population.len() * matches_per_eval;

        for (fit, wr, eff, w) in &scored {
            if *fit > best_fit {
                best_fit = *fit;
                best_w = *w;
                best_wr = *wr;
                best_eff = *eff;
            }
        }

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        let elite_a = scored[0].3;
        let elite_b = scored.get(1).map(|x| x.3).unwrap_or(elite_a);

        population.clear();
        population.push(elite_a);
        population.push(elite_b);
        while population.len() < population_size {
            let parent = if rng.gen_bool(0.65) { elite_a } else { elite_b };
            let sigma = (0.16 - (gen_idx as f32) * 0.02).max(0.05);
            population.push(mutate_weights(&parent, &mut rng, sigma));
        }
    }

    TrainingSummary {
        generations,
        matches_played: total_matches,
        baseline_fitness: base_fit,
        best_fitness: best_fit,
        best_win_rate: best_wr,
        avg_gold_efficiency: best_eff,
        best_weights: best_w,
    }
}

fn mutate_weights<R: Rng>(base: &TrainedBrainWeights, rng: &mut R, sigma: f32) -> TrainedBrainWeights {
    let mut j = |val: f32, lo: f32, hi: f32| -> f32 {
        (val + rng.gen_range(-sigma..sigma) * val.max(0.4)).clamp(lo, hi)
    };
    TrainedBrainWeights {
        synergy_mult: j(base.synergy_mult, 0.5, 2.2),
        aoe_vs_swarm: j(base.aoe_vs_swarm, 0.8, 2.8),
        magic_vs_tank: j(base.magic_vs_tank, 0.8, 2.5),
        assassin_vs_ranged: j(base.assassin_vs_ranged, 0.8, 2.5),
        gold_efficiency_gain: j(base.gold_efficiency_gain, 0.8, 2.5),
        turret_bait_bonus: j(base.turret_bait_bonus, 0.5, 2.4),
        hold_to_pool_bonus: j(base.hold_to_pool_bonus, 0.4, 2.2),
        eco_roi_weight: j(base.eco_roi_weight, 0.5, 2.0),
        tech_rush_weight: j(base.tech_rush_weight, 0.5, 2.0),
        spell_q_efficiency: j(base.spell_q_efficiency, 0.8, 2.5),
        spell_e_efficiency: j(base.spell_e_efficiency, 0.8, 2.2),
        lookahead_weight: j(base.lookahead_weight, 0.2, 2.0),
        wave_pool_min_ratio: j(base.wave_pool_min_ratio, 0.45, 0.90),
        retreat_preserve_mult: j(base.retreat_preserve_mult, 0.5, 2.2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn run_self_play_training_arena() {
        let summary = train_weights_evolutionary(6, 10, 8, 2026);
        println!("=== SELF-PLAY TRAINING SUMMARY ===");
        println!(
            "Generations: {}, Total Matches: {}",
            summary.generations, summary.matches_played
        );
        println!(
            "Baseline Fitness: {:.2} -> Best Evolved Fitness: {:.2} (Win Rate: {:.1}%, Avg Gold Eff: {:.2}x)",
            summary.baseline_fitness,
            summary.best_fitness,
            summary.best_win_rate * 100.0,
            summary.avg_gold_efficiency
        );
        println!("Best Weights: {:#?}", summary.best_weights);
        assert!(summary.best_fitness >= summary.baseline_fitness);
    }
}
