use std::env;
use std::fs;
use std::time::Instant;

use grand_conquest_core::ai_trainer::{train_weights_evolutionary, train_weights_evolutionary_seeded};
use grand_conquest_core::ai_weights::TrainedBrainWeights;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut generations = 12;
    let mut population = 16;
    let mut matches_per_eval = 12;
    let mut seed = 2026;
    let mut out_file = "trained_weights.json".to_string();
    let mut init_weights: Option<TrainedBrainWeights> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--generations" | "-g" => {
                if let Some(v) = args.get(i + 1) {
                    generations = v.parse().unwrap_or(generations);
                    i += 1;
                }
            }
            "--population" | "-p" => {
                if let Some(v) = args.get(i + 1) {
                    population = v.parse().unwrap_or(population);
                    i += 1;
                }
            }
            "--matches" | "-m" => {
                if let Some(v) = args.get(i + 1) {
                    matches_per_eval = v.parse().unwrap_or(matches_per_eval);
                    i += 1;
                }
            }
            "--seed" | "-s" => {
                if let Some(v) = args.get(i + 1) {
                    seed = v.parse().unwrap_or(seed);
                    i += 1;
                }
            }
            "--out" | "-o" => {
                if let Some(v) = args.get(i + 1) {
                    out_file = v.clone();
                    i += 1;
                }
            }
            "--init" | "-i" => {
                if let Some(path) = args.get(i + 1) {
                    if let Ok(data) = fs::read_to_string(path) {
                        if let Ok(w) = serde_json::from_str::<TrainedBrainWeights>(&data) {
                            println!("Loaded initial baseline seed weights from: {}", path);
                            init_weights = Some(w);
                        }
                    }
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    println!("============================================================");
    println!("     GRAND CONQUEST - HEADLESS AI SELF-PLAY TRAINER        ");
    println!("============================================================");
    println!("Generations:      {}", generations);
    println!("Population Size:  {}", population);
    println!("Matches / Eval:   {}", matches_per_eval);
    println!("Est. Matches:     {}", generations * population * matches_per_eval);
    println!("Parallel Threads: {}", rayon::current_num_threads());
    println!("Random Seed:      {}", seed);
    println!("Seeded Baseline:  {}", if init_weights.is_some() { "YES (Warm Start)" } else { "NO (Cold Start)" });
    println!("Output File:      {}", out_file);
    println!("------------------------------------------------------------");

    let t0 = Instant::now();
    let summary = if let Some(init_w) = init_weights {
        train_weights_evolutionary_seeded(generations, population, matches_per_eval, seed, Some(init_w))
    } else {
        train_weights_evolutionary(generations, population, matches_per_eval, seed)
    };
    let elapsed = t0.elapsed();

    println!("\n=== TRAINING COMPLETED in {:.2?} ===", elapsed);
    println!("Matches Simulated:  {}", summary.matches_played);
    println!(
        "Simulation Speed:   {:.0} matches/second",
        (summary.matches_played as f64) / elapsed.as_secs_f64().max(0.001)
    );
    println!("Baseline Fitness:   {:.2}", summary.baseline_fitness);
    println!("Best Fitness:       {:.2}", summary.best_fitness);
    println!("Win Rate:           {:.1}%", summary.best_win_rate * 100.0);
    println!("Avg Gold Efficiency:{:.2}x", summary.avg_gold_efficiency);
    println!("\n=== EVOLVED WEIGHTS STRUCT ===");
    println!("{:#?}", summary.best_weights);

    if let Ok(json) = serde_json::to_string_pretty(&summary.best_weights) {
        let _ = fs::write(&out_file, &json);
        println!("\nSaved weights JSON to: {}", out_file);
    }
}
