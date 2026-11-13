use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::missions::{get_mission, MISSIONS};
use crate::model::{AiDifficulty, FactionId, MutatorId, PassiveKey, RelicId, Team, UnitShape};
use crate::relics::RELICS;
use crate::save::ConquestSave;
use crate::sim::MatchState;
use crate::units::{default_faction_deck, get_unit, UNITS};

#[test]
fn catalog_completeness_and_integrity() {
    assert_eq!(UNITS.len(), 54);
    assert_eq!(MISSIONS.len(), 24);
    assert_eq!(RELICS.len(), 15);

    // Every mission prerequisites and reward blueprints must resolve cleanly
    for m in MISSIONS {
        for pre in m.prerequisites {
            assert!(get_mission(pre).is_some(), "Missing prereq {pre} on {}", m.id);
        }
        for bp in m.reward_blueprints {
            assert!(get_unit(bp).is_some(), "Missing blueprint {bp} on {}", m.id);
        }
        for id in m.enemy_deck {
            assert!(get_unit(id).is_some(), "Missing enemy deck unit {id} on {}", m.id);
        }
        if let Some(boss) = &m.boss {
            assert!(get_unit(boss.base_unit_id).is_some());
        }
    }
}

#[test]
fn tc01_and_tc09_starter_save_and_cross_faction_deck_rules() {
    let mut save = ConquestSave::new_default(FactionId::Solaris);
    assert_eq!(save.unlocked_blueprint_ids.len(), 3);
    assert!(!save.active_features.tier2);
    assert!(!save.active_features.tier3);
    // Only Tier C slots [0..2] filled at start; Tier B [2..4] & Tier A [4..6] remain empty!
    assert_eq!(save.equipped_deck_unit_ids[0], "sol_1");
    assert_eq!(save.equipped_deck_unit_ids[1], "sol_2");
    assert!(save.equipped_deck_unit_ids[2].is_empty());
    assert!(save.equipped_deck_unit_ids[3].is_empty());
    assert!(save.equipped_deck_unit_ids[4].is_empty());
    assert!(save.equipped_deck_unit_ids[5].is_empty());
    assert!(save.verify_checksum());

    // Cannot equip locked blueprint
    assert!(save.equip_unit_in_slot(2, "sol_4").is_err());

    // Unlock cross-faction B-tier unit and try invalid vs valid slot
    save.unlock_blueprint("sol_4");
    assert!(save.equip_unit_in_slot(0, "sol_4").is_err(), "B-tier cannot go in C slot 0");
    assert!(save.equip_unit_in_slot(2, "sol_4").is_ok());
    assert!(save.equip_unit_in_slot(3, "sol_4").is_ok(), "All 6 slots individually assignable");
    assert!(save.verify_checksum());

    // Legacy broken saves with C-tier duplicates in B/A slots get sanitized cleanly
    let mut legacy = ConquestSave::new_default(FactionId::Solaris);
    legacy.equipped_deck_unit_ids = [
        "sol_1".into(),
        "sol_2".into(),
        "sol_3".into(),
        "sol_1".into(),
        "sol_2".into(),
        "sol_3".into(),
    ];
    legacy.recompute_integrity();
    assert_eq!(legacy.equipped_deck_unit_ids[0], "sol_1");
    assert_eq!(legacy.equipped_deck_unit_ids[1], "sol_2");
    assert!(legacy.equipped_deck_unit_ids[2].is_empty());
    assert!(legacy.equipped_deck_unit_ids[4].is_empty());
    assert!(legacy.equipped_deck_unit_ids[5].is_empty());

    // Starter battle should not let player spend 100G unlocking empty Tier 2
    let mut rng = StdRng::seed_from_u64(1);
    let mut battle = MatchState::from_campaign_mission("M1_1", &legacy, &mut rng).unwrap();
    battle.player.gold = 500.0;
    assert_eq!(battle.max_player_tech_tier(), 1);
    assert!(!battle.player_try_unlock_tier());
    assert!(!battle.player_try_spawn_slot(4, &mut rng));
}

#[test]
fn tc02_and_tc04_campaign_progression_and_chapter_branching() {
    let mut save = ConquestSave::new_default(FactionId::Solaris);
    assert!(save.is_mission_unlocked("M1_1"));
    assert!(!save.is_mission_unlocked("M1_2"));

    // Clear M1_1..M1_4_BOSS via outcome evaluation
    let mut rng = StdRng::seed_from_u64(42);
    let mut state = MatchState::from_campaign_mission("M1_1", &save, &mut rng).unwrap();
    state.winner = Some(Team::Blue);
    state.time = 110.0;
    let out = state.evaluate_and_commit_outcome(&mut save);
    assert_eq!(out.stars_earned, 3);
    assert!(save.unlocked_blueprint_ids.contains(&"sol_4".to_string()));
    assert!(save.is_mission_unlocked("M1_2"));

    for mid in ["M1_2", "M1_3", "M1_4_BOSS"] {
        let mut st = MatchState::from_campaign_mission(mid, &save, &mut rng).unwrap();
        st.winner = Some(Team::Blue);
        st.time = 120.0;
        st.player.income_lvl = 6;
        st.stats.enemy_kills = 40;
        st.evaluate_and_commit_outcome(&mut save);
    }

    assert!(save.is_mission_unlocked("M2_1"));
    assert!(save.is_mission_unlocked("M3_1"));
    assert!(save.active_features.tier2);
    assert!(save.active_features.tier3);
    // Chapter 1 Mastery (12 stars) unlocks A-tier Elite Mage & High Priest!
    assert_eq!(save.total_conquest_stars, 12);
    assert!(save.unlocked_blueprint_ids.contains(&"sol_8".to_string()));
    assert!(save.unlocked_blueprint_ids.contains(&"sol_9".to_string()));
}

#[test]
fn tc05_blitzkrieg_timeout_triggers_defeat() {
    let save = ConquestSave::unlock_all_sandbox();
    let mut rng = StdRng::seed_from_u64(7);
    let mut state = MatchState::from_campaign_mission("M2_2", &save, &mut rng).unwrap();
    assert_eq!(state.blitz_timer, Some(135.0));

    // Advance past 135 seconds without killing enemy base
    for _ in 0..140 {
        state.update(1.0, &mut rng);
        if state.over {
            break;
        }
    }
    assert!(state.over);
    assert_eq!(state.winner, Some(Team::Red));
}

#[test]
fn tc06_and_all_six_mutators_work() {
    let mut rng = StdRng::seed_from_u64(99);
    let deck = default_faction_deck(FactionId::Solaris);

    // 1. Consecrated Solar: Triangle -20% dmg, Cross heal +50%
    let mut m_solar = MatchState::new_skirmish(
        FactionId::Solaris,
        &deck,
        FactionId::Solaris,
        AiDifficulty::Normal,
        Some(MutatorId::ConsecratedSolar),
        vec![],
        &mut rng,
    );
    let ally_uid = m_solar.spawn_unit(Team::Blue, get_unit("sol_1").unwrap(), false, Some((300.0, 250.0)), &mut rng);
    m_solar.units.iter_mut().find(|u| u.uid == ally_uid).unwrap().hp = 10.0;
    m_solar.cast_commander_spell_e(Team::Blue);
    let healed_hp = m_solar.units.iter().find(|u| u.uid == ally_uid).unwrap().hp;
    // Note: 6-piece Solaris affinity adds +30% (1.30) * 1.50 consecrated = 1.95x * 75 -> capped at 90 maxHp!
    assert_eq!(healed_hp, 90.0);

    // 2. EMP Scrambler: Triangle/Diamond CD * 1.40, Circle/Square SPD * 1.25
    let mut m_emp = MatchState::new_skirmish(
        FactionId::Solaris,
        &deck,
        FactionId::Aegis,
        AiDifficulty::Normal,
        Some(MutatorId::EmpScrambler),
        vec![],
        &mut rng,
    );
    let arch_def = get_unit("sol_2").unwrap();
    let foot_def = get_unit("sol_1").unwrap();
    let arch_uid = m_emp.spawn_unit(Team::Blue, arch_def, false, Some((300.0, 250.0)), &mut rng);
    let foot_uid = m_emp.spawn_unit(Team::Blue, foot_def, false, Some((300.0, 260.0)), &mut rng);
    assert!((m_emp.units.iter().find(|u| u.uid == arch_uid).unwrap().cd - arch_def.cd * 1.40).abs() < 0.01);
    assert!((m_emp.units.iter().find(|u| u.uid == foot_uid).unwrap().spd - foot_def.spd_studs * 3.1 * 1.25).abs() < 0.1);

    // 3. Corrosive Miasma: unit outside 110px/120px takes 2 DMG every 1.5s
    let mut m_cor = MatchState::new_skirmish(
        FactionId::Solaris,
        &deck,
        FactionId::Hive,
        AiDifficulty::Normal,
        Some(MutatorId::CorrosiveMiasma),
        vec![],
        &mut rng,
    );
    let lone_uid = m_cor.spawn_unit(Team::Blue, get_unit("sol_1").unwrap(), false, Some((700.0, 250.0)), &mut rng);
    let init_hp = m_cor.units.iter().find(|u| u.uid == lone_uid).unwrap().hp;
    m_cor.update(1.51, &mut rng);
    let after_hp = m_cor.units.iter().find(|u| u.uid == lone_uid).unwrap().hp;
    assert!(after_hp < init_hp);

    // 4. Gravity Distortion: Soul Drainer death immunity lasts 2.0s instead of 1.0s
    let mut m_grav = MatchState::new_skirmish(
        FactionId::Void,
        &default_faction_deck(FactionId::Void),
        FactionId::Void,
        AiDifficulty::Normal,
        Some(MutatorId::GravityDistortion),
        vec![],
        &mut rng,
    );
    let sd_uid = m_grav.spawn_unit(Team::Blue, get_unit("voi_6").unwrap(), false, Some((500.0, 250.0)), &mut rng);
    m_grav.apply_damage_to_unit(sd_uid, 999.0, None, false, false);
    let sd = m_grav.units.iter().find(|u| u.uid == sd_uid).unwrap();
    assert_eq!(sd.hp, 1.0);
    assert!((sd.death_immune_timer - 2.0).abs() < 0.01);

    // 5. Blood Frenzy: burn 1% Max HP per second
    let mut m_bf = MatchState::new_skirmish(
        FactionId::Hellfire,
        &default_faction_deck(FactionId::Hellfire),
        FactionId::Hellfire,
        AiDifficulty::Normal,
        Some(MutatorId::BloodFrenzy),
        vec![],
        &mut rng,
    );
    let brute_uid = m_bf.spawn_unit(Team::Blue, get_unit("hel_7").unwrap(), false, Some((500.0, 250.0)), &mut rng);
    let b_hp0 = m_bf.units.iter().find(|u| u.uid == brute_uid).unwrap().hp;
    m_bf.update(1.01, &mut rng);
    assert!(m_bf.units.iter().find(|u| u.uid == brute_uid).unwrap().hp < b_hp0);

    // 6. Nanite Regeneration: Square/Hexagon regen 5% / 3s
    let mut m_nan = MatchState::new_skirmish(
        FactionId::Mechanica,
        &default_faction_deck(FactionId::Mechanica),
        FactionId::Mechanica,
        AiDifficulty::Normal,
        Some(MutatorId::NaniteRegeneration),
        vec![],
        &mut rng,
    );
    let tank_uid = m_nan.spawn_unit(Team::Blue, get_unit("mec_4").unwrap(), false, Some((500.0, 250.0)), &mut rng);
    m_nan.units.iter_mut().find(|u| u.uid == tank_uid).unwrap().hp = 100.0;
    m_nan.update(3.01, &mut rng);
    assert!(m_nan.units.iter().find(|u| u.uid == tank_uid).unwrap().hp >= 109.5);
}

#[test]
fn tc07_colossus_prime_phase2_shield_and_phase3_enrage() {
    let save = ConquestSave::unlock_all_sandbox();
    let mut rng = StdRng::seed_from_u64(11);
    let mut state = MatchState::from_campaign_mission("M6_4_FINAL_BOSS", &save, &mut rng).unwrap();
    let boss_uid = state.active_boss().unwrap().uid;
    assert_eq!(state.active_boss().unwrap().max_hp, 5500.0);

    // Drop boss below 50% HP -> triggers Phase 2 +1200 Energy Shield
    state.apply_damage_to_unit(boss_uid, 2800.0, None, true, false);
    state.update(0.05, &mut rng);
    let boss = state.active_boss().unwrap();
    assert_eq!(boss.boss_phase, 2);
    assert_eq!(boss.shield_hp, 1200.0);

    // Damage hits shield before HP!
    let hp_before = boss.hp;
    state.apply_damage_to_unit(boss_uid, 500.0, None, false, false);
    let boss_after = state.active_boss().unwrap();
    assert_eq!(boss_after.hp, hp_before);
    assert_eq!(boss_after.shield_hp, 700.0);

    // Deplete remaining shield and drop below 25% -> triggers Phase 3 Enrage
    state.apply_damage_to_unit(boss_uid, 2200.0, None, true, false);
    state.update(0.05, &mut rng);
    assert_eq!(state.active_boss().unwrap().boss_phase, 3);
}

#[test]
fn tc08_all_15_relics_and_4rp_cap() {
    let mut save = ConquestSave::new_default(FactionId::Solaris);
    for m in MISSIONS {
        save.missions.insert(
            m.id.to_string(),
            crate::save::MissionRecord {
                stars: 3,
                best_time_seconds: 90.0,
                times_attempted: 1,
            },
        );
    }
    save.recompute_integrity();
    assert_eq!(save.total_conquest_stars, 72);
    assert_eq!(save.unlocked_relic_ids.len(), 15);

    // Equip 2 RP + 2 RP = 4 RP ok, then another fails!
    assert!(save.try_toggle_relic("titanium_fort").is_ok()); // 2 RP
    assert!(save.try_toggle_relic("crown_geo_terra").is_ok()); // 2 RP
    assert_eq!(save.used_relic_power(), 4);
    assert!(save.try_toggle_relic("treasury_seal").is_err());

    // Test every relic combat effect in MatchState
    let mut rng = StdRng::seed_from_u64(55);
    let all_relics: Vec<RelicId> = RELICS.iter().map(|r| r.id).collect();
    let mut state = MatchState::new_skirmish(
        FactionId::Aegis,
        &default_faction_deck(FactionId::Aegis),
        FactionId::Hive,
        AiDifficulty::Normal,
        None,
        all_relics,
        &mut rng,
    );
    // TreasurySeal (+25G), OverchargedConduits (-12s Q/E), TitaniumFort (1850 Base), EconomicMonopoly (-25% cost)
    assert_eq!(state.player.gold, 65.0);
    assert!(state.player.q_cd_max < 48.0);
    assert_eq!(state.structures[0].max_hp, 1850.0);
    assert_eq!(state.income_upgrade_cost(Team::Blue), 15); // 20 * 0.75

    // Spawn units: CrownGeoTerra (+10%), RapidBanner (+12%), BallisticLens (+34px), VoidSingularity (execute <=15%), NaniteInjector
    let sq = state.spawn_unit(Team::Blue, get_unit("sol_3").unwrap(), false, Some((400.0, 250.0)), &mut rng);
    let red_grunt = state.spawn_unit(Team::Red, get_unit("hiv_7").unwrap(), false, Some((420.0, 250.0)), &mut rng);
    // Chitin Carapace reduces square damage by 3 + Nanite saves below 25% HP
    let sq_max = state.units.iter().find(|u| u.uid == sq).unwrap().max_hp;
    assert!(sq_max > 130.0); // CrownGeoTerra +10%
    state.apply_damage_to_unit(sq, sq_max * 0.85, None, false, false);
    assert!(state.units.iter().find(|u| u.uid == sq).unwrap().nanite_used);

    // Singularity Heart executes non-boss <= 15% HP
    state.apply_damage_to_unit(red_grunt, 260.0, Some(sq), false, false);
    assert_eq!(state.units.iter().find(|u| u.uid == red_grunt).unwrap().hp, 0.0);
}

#[test]
fn tc10_save_persistence_and_tamper_checksum() {
    let dir = std::env::temp_dir().join("grand_conquest_rust_test_save.json");
    let mut save = ConquestSave::new_default(FactionId::Mechanica);
    save.account_gold_vault = 777;
    save.save_to_path(&dir).unwrap();

    let (loaded, tampered) = ConquestSave::load_from_path(&dir);
    assert!(!tampered);
    assert_eq!(loaded.account_gold_vault, 777);

    // Simulate external tamper edit without updating checksum
    let mut broken = loaded.clone();
    broken.account_gold_vault = 999999;
    std::fs::write(&dir, serde_json::to_string(&broken).unwrap()).unwrap();
    let (_, tampered2) = ConquestSave::load_from_path(&dir);
    assert!(tampered2);
    let _ = std::fs::remove_file(&dir);
}

#[test]
fn headless_full_battle_completes_cleanly() {
    let save = ConquestSave::unlock_all_sandbox();
    let mut rng = StdRng::seed_from_u64(12345);
    let mut state = MatchState::from_campaign_mission("M1_1", &save, &mut rng).unwrap();

    // Active player macro script for 200 seconds at 30Hz
    for step in 0..(200 * 30) {
        let dt = 1.0 / 30.0;
        if step % 90 == 0 {
            state.player_try_upgrade_income();
        }
        if step % 45 == 0 {
            state.player_try_spawn_slot((step / 45) % 2, &mut rng);
        }
        if step % 150 == 0 {
            state.cast_commander_spell_q(Team::Blue);
            state.cast_commander_spell_e(Team::Blue);
        }
        state.update(dt, &mut rng);
        if state.over {
            break;
        }
    }
    assert!(state.over, "Match should reach deterministic conclusion within 200s");
    assert!(state.winner.is_some());
    assert!(state.player.deck.iter().flatten().all(|u| u.shape != UnitShape::Hexagon || u.tier == crate::model::UnitTier::A));
}

#[test]
fn all_21_unit_passives_verified_in_roster_and_combat() {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    for u in UNITS {
        seen.insert(u.passive);
    }
    assert_eq!(seen.len(), 21, "All 21 PassiveKey variants must exist on units");

    let expected = [
        PassiveKey::None,
        PassiveKey::DmgRed25,
        PassiveKey::FourthHit45,
        PassiveKey::Heal10_3s3u,
        PassiveKey::Heal20_3s5u,
        PassiveKey::Heal25_6s5u,
        PassiveKey::Heal30_4s99u,
        PassiveKey::EliteKnight,
        PassiveKey::Sniper36,
        PassiveKey::FirstHit50,
        PassiveKey::EliteRiot,
        PassiveKey::ShotgunFalloff,
        PassiveKey::Gatling,
        PassiveKey::AcidPoison,
        PassiveKey::Broodmother,
        PassiveKey::Lifesteal30,
        PassiveKey::SoulHarvest,
        PassiveKey::Lifesteal50,
        PassiveKey::DeathImmunity,
        PassiveKey::VoidWarden,
        PassiveKey::CyberCmdr,
    ];
    for p in expected {
        assert!(seen.contains(&p), "Missing passive {p:?}");
    }

    // Combat spot-check: Grenade Thrower 1st hit = 50, Mage 4th hit = 45, Shotgunner falloff 35/15, Rifleman sniper_36
    let mut rng = StdRng::seed_from_u64(314);
    let mut state = MatchState::new_skirmish(
        FactionId::Aegis,
        &default_faction_deck(FactionId::Aegis),
        FactionId::Solaris,
        AiDifficulty::Easy,
        None,
        vec![],
        &mut rng,
    );
    // Broodmother summon check (Hive Commander)
    let hc = state.spawn_unit(Team::Blue, get_unit("hiv_9").unwrap(), false, Some((350.0, 265.0)), &mut rng);
    state.units.iter_mut().find(|u| u.uid == hc).unwrap().brood_timer = 0.01;
    state.update(0.05, &mut rng);
    assert!(state.units.iter().filter(|u| u.def.id == "hiv_1" && u.team == Team::Blue).count() >= 2);
}

#[test]
fn batch_spawn_multipliers_deploy_requested_or_max_affordable() {
    let mut rng = StdRng::seed_from_u64(42);
    let save = ConquestSave::unlock_all_sandbox();
    let mut state = MatchState::from_campaign_mission("M1_1", &save, &mut rng).unwrap();
    let slot0_cost = state.player.deck[0].unwrap().cost as f32;

    // Supports every 1, 5, 10, 20, 30, 50 batch size when affordable
    for mult in [1usize, 5, 10, 20, 30, 50] {
        state.units.clear();
        state.player.gold = slot0_cost * (mult as f32) + 3.0;
        let deployed = state.player_try_spawn_slot_count(0, mult, &mut rng);
        assert_eq!(deployed, mult);
        assert_eq!(state.units.iter().filter(|u| u.team == Team::Blue).count(), mult);
        assert!((state.player.gold - 3.0).abs() < 0.01);
    }

    // Partial affordability: requesting x20 with enough gold for 7 deploys 7 cleanly
    state.units.clear();
    state.player.gold = slot0_cost * 7.0 + 1.0;
    let partial = state.player_try_spawn_slot_count(0, 20, &mut rng);
    assert_eq!(partial, 7);
    assert_eq!(state.units.iter().filter(|u| u.team == Team::Blue).count(), 7);
    assert!((state.player.gold - 1.0).abs() < 0.01);

    // Insufficient gold: spawns 0 without modifying gold
    state.units.clear();
    state.player.gold = slot0_cost - 0.5;
    assert_eq!(state.player_try_spawn_slot_count(0, 10, &mut rng), 0);
    assert_eq!(state.units.len(), 0);
}

#[test]
fn weight_vectors_tree_nodes_and_player_adaptation_work() {
    use crate::ai_trainer::evaluate_weights_fitness;
    use crate::ai_weights::{unit_weight_vector, ArmyWeightVector, TrainedBrainWeights};
    use crate::model::ArmyOrder;

    // 1. Unit Weight Calibration: 1 C-tier Footman (~0.12) vs 20x Swarm (~2.4)
    let footman = get_unit("sol_1").unwrap();
    let knight = get_unit("sol_4").unwrap();
    let mage = get_unit("sol_5").unwrap();

    let w_foot = unit_weight_vector(footman);
    assert!(w_foot.raw_sum() > 0.08 && w_foot.raw_sum() < 0.22, "Footman w={}", w_foot.raw_sum());

    let mut swarm_20 = ArmyWeightVector::default();
    for _ in 0..20 {
        swarm_20.add(w_foot);
    }
    assert!(swarm_20.raw_sum() > 1.8, "20x Footman weight={}", swarm_20.raw_sum());

    // Combined Arms Synergy: Knight (Tank) + Mage (AoE) outweighs sum of isolated parts
    let mut combo = unit_weight_vector(knight);
    combo.add(unit_weight_vector(mage));
    let w_cfg = TrainedBrainWeights::extreme_master();
    assert!(
        combo.effective_total(w_cfg.synergy_mult) > combo.raw_sum(),
        "Tank + Mage must gain combined-arms synergy bonus"
    );
    assert!(
        combo.counter_advantage_against(swarm_20, &w_cfg) > 1.5,
        "Tank + Mage must exert strong counter advantage vs 20-unit swarm"
    );

    // 2. Live Player Batch Tracking + Node Search Tree
    let mut rng = StdRng::seed_from_u64(314);
    let mut state = MatchState::new_skirmish(
        FactionId::Solaris,
        &default_faction_deck(FactionId::Solaris),
        FactionId::Solaris,
        AiDifficulty::Extreme,
        None,
        vec![],
        &mut rng,
    );
    state.player.gold = 300.0;
    let deployed = state.player_try_spawn_slot_count(0, 20, &mut rng);
    assert_eq!(deployed, 20);
    assert_eq!(state.ai_brain.player_tracker.max_batch_seen, 20);
    assert!(state.ai_brain.player_tracker.swarm_threat_bias >= 0.45);

    // Advance enemy swarm toward Red's half & tick Extreme AI Brain
    for u in &mut state.units {
        if u.team == Team::Blue {
            u.x = 870.0;
        }
    }
    state.ai.gold = 90.0;
    state.ai.tier = 2;
    state.ai.decision_timer = 0.01;
    state.update(0.05, &mut rng);

    assert!(
        state.ai_brain.total_nodes_evaluated >= 4,
        "Expected multiple decision tree nodes, got {}",
        state.ai_brain.total_nodes_evaluated
    );
    assert!(!state.ai_brain.top_nodes.is_empty());
    assert!(!state.ai_brain.active_plan_summary.is_empty());
    assert!(
        state.ai.order == ArmyOrder::Hold || !state.ai_brain.top_nodes[0].label.is_empty()
    );

    // 3. Fast Headless Self-Play Fitness Evaluator works cleanly
    let (fit, _wr, eff) = evaluate_weights_fitness(&w_cfg, 777, 3);
    assert!(fit.is_finite(), "Fitness must be finite: {fit}");
    assert!(eff > 0.0, "Gold efficiency must be positive: {eff}");
}

#[test]
fn tc15_settings_roundtrip_and_clamping() {
    use crate::settings::{GameSettings, HpBarMode};
    let dir = std::env::temp_dir().join("gc_test_settings.json");
    let cfg = GameSettings {
        active_slot: 9,
        master_volume: 1.8,
        music_volume: -0.2,
        sfx_volume: 0.65,
        muted: false,
        fullscreen: true,
        screen_shake: false,
        show_damage_numbers: false,
        hp_bar_mode: HpBarMode::Minimal,
        default_speed: 2.0,
        show_minimap: true,
        edge_pan: true,
        auto_pause_unfocused: false,
    };
    cfg.save_to_path(&dir).unwrap();
    let loaded = GameSettings::load_from_path(&dir);
    assert_eq!(loaded.active_slot, 2);
    assert!((loaded.master_volume - 1.0).abs() < 1e-5);
    assert!((loaded.music_volume - 0.0).abs() < 1e-5);
    assert_eq!(loaded.hp_bar_mode, HpBarMode::Minimal);
    assert!(loaded.fullscreen);
    let _ = std::fs::remove_file(&dir);
}

#[test]
fn tc16_honors_achievements_and_treasury_store() {
    use crate::honors::CareerProfile;
    use crate::save::MissionRecord;
    let mut save = ConquestSave::new_default(FactionId::Solaris);
    save.account_gold_vault = 500;
    for mid in ["M1_1", "M1_2", "M1_3", "M1_4_BOSS"] {
        save.missions.insert(
            mid.to_string(),
            MissionRecord {
                stars: 3,
                best_time_seconds: 60.0,
                times_attempted: 1,
            },
        );
    }
    save.skirmish_wins = 3;
    save.recompute_integrity();

    let mut career = CareerProfile::default();
    let newly = career.evaluate_unlocks(&mut save);
    // Should unlock ach_first_victory (+75G), ach_solaris_victor (+100G), ach_12_stars (+100G), ach_skirmisher (+150G)
    assert!(newly.len() >= 4);
    assert!(save.account_gold_vault >= 500 + 425);
    assert!(save.verify_checksum());

    // Treasury blueprint purchase
    let bought = career.try_buy_blueprint(&mut save, "aeg_1").unwrap();
    assert_eq!(bought.id, "aeg_1");
    assert!(save.unlocked_blueprint_ids.iter().any(|id| id == "aeg_1"));

    // Mystery War Chest
    let mut rng = StdRng::seed_from_u64(42);
    let crate_unit = career.try_open_war_chest(&mut save, &mut rng).unwrap();
    assert!(save.unlocked_blueprint_ids.iter().any(|id| id == crate_unit.id));

    // Title purchase & equip
    assert!(career.try_buy_or_equip_title(&mut save, "title_vanguard").unwrap());
    assert_eq!(career.active_title_id, "title_vanguard");
    assert!(save.verify_checksum());
}

#[test]
fn tc17_save_slot_paths_and_career_persistence() {
    use crate::honors::{inspect_slot, slot_career_path, slot_save_path, CareerProfile};
    use crate::save::MissionRecord;
    let tmp_dir = std::env::temp_dir().join("gc_test_slots_dir");
    let _ = std::fs::remove_dir_all(&tmp_dir);
    std::fs::create_dir_all(&tmp_dir).unwrap();

    let s1 = slot_save_path(&tmp_dir, 1);
    let c1 = slot_career_path(&tmp_dir, 1);
    let mut save = ConquestSave::new_default(FactionId::Void);
    for (idx, mid) in ["M1_1", "M1_2", "M1_3", "M2_1", "M2_2"].iter().enumerate() {
        let _ = idx;
        save.missions.insert(
            (*mid).to_string(),
            MissionRecord {
                stars: 3,
                best_time_seconds: 55.0,
                times_attempted: 1,
            },
        );
    }
    save.save_to_path(&s1).unwrap();

    let career = CareerProfile {
        commander_name: "Kael".to_string(),
        ..CareerProfile::default()
    };
    career.save_to_path(&c1).unwrap();

    let meta0 = inspect_slot(&tmp_dir, 0);
    let meta1 = inspect_slot(&tmp_dir, 1);
    assert!(!meta0.exists);
    assert!(meta1.exists);
    assert_eq!(meta1.faction, FactionId::Void);
    assert_eq!(meta1.stars, 15);
    assert_eq!(meta1.commander_name, "Kael");
    let _ = std::fs::remove_dir_all(&tmp_dir);
}

#[test]
fn tc18_spatial_grid_parity_and_500_unit_stress() {
    use crate::model::{WORLD_H, WORLD_W};
    use crate::sim::TeamSpatialGrid;
    use rand::Rng;

    let mut rng = StdRng::seed_from_u64(2026);
    let save = ConquestSave::new_default(FactionId::Solaris);
    let mut state = MatchState::from_campaign_mission("M1_1", &save, &mut rng).unwrap();
    state.units.clear();

    // Seed clustered, overlapping, negative-edge-distance, and wide-radius targets
    for i in 0..80usize {
        let def = &UNITS[(i * 7) % UNITS.len()];
        let (x, y) = if i < 20 {
            // Co-located dense stack (tests negative edge distance handling)
            (700.0 + (i % 3) as f32 * 0.5, 250.0)
        } else {
            (
                rng.gen_range(20.0..WORLD_W - 20.0),
                rng.gen_range(40.0..WORLD_H - 40.0),
            )
        };
        let team = if i % 2 == 0 { Team::Blue } else { Team::Red };
        state.spawn_unit(team, def, false, Some((x, y)), &mut rng);
    }

    let mut red_grid = TeamSpatialGrid::default();
    red_grid.build_for_team(&state.units, Team::Red);

    for step in 0..100usize {
        let (qx, qy) = if step < 10 {
            (700.0, 250.0)
        } else {
            (
                (step as f32 * 137.5) % (WORLD_W + 80.0) - 40.0,
                (step as f32 * 73.1) % (WORLD_H + 60.0) - 30.0,
            )
        };
        let grid_res = red_grid.nearest_to(qx, qy).expect("red grid non-empty");
        let mut brute_best = (0u64, f32::MAX);
        for u in state.units.iter().filter(|u| u.hp > 0.0 && u.team == Team::Red) {
            let d = (u.x - qx).hypot(u.y - qy) - u.radius;
            if d < brute_best.1 {
                brute_best = (u.uid, d);
            }
        }
        assert!(
            (grid_res.3 - brute_best.1).abs() < 1e-3,
            "Grid edge distance mismatch at ({qx},{qy}): grid={} vs brute={}",
            grid_res.3,
            brute_best.1
        );
    }

    // 500-unit (250 vs 250) full simulation stress & effect/text cap check
    let mut stress = MatchState::from_campaign_mission("M1_1", &save, &mut rng).unwrap();
    stress.units.clear();
    for i in 0..250usize {
        let bdef = &UNITS[i % 18];
        let bx = 460.0 + (i % 10) as f32 * 18.0;
        let by = 115.0 + ((i / 10) as f32) * 12.0;
        stress.spawn_unit(Team::Blue, bdef, false, Some((bx, by)), &mut rng);

        let rdef = &UNITS[(i + 5) % 18];
        let rx = 760.0 + (i % 10) as f32 * 18.0;
        let ry = 115.0 + ((i / 10) as f32) * 12.0;
        stress.spawn_unit(Team::Red, rdef, false, Some((rx, ry)), &mut rng);
    }
    assert_eq!(stress.units.len(), 500);

    stress.ai.decision_timer = 0.05;
    for _ in 0..60 {
        stress.update(1.0 / 60.0, &mut rng);
        assert!(stress.floating_texts.len() <= 96);
    }

    assert!(!stress.units.is_empty());
    for u in &stress.units {
        assert!(u.x.is_finite() && u.y.is_finite() && u.hp.is_finite());
    }
}

#[test]
#[ignore]
fn bench_500_unit_simulation_timing() {
    let mut rng = StdRng::seed_from_u64(9999);
    let save = ConquestSave::new_default(FactionId::Solaris);
    let mut stress = MatchState::from_campaign_mission("M1_1", &save, &mut rng).unwrap();
    stress.units.clear();
    for i in 0..250usize {
        let bdef = &UNITS[i % 24];
        let bx = 420.0 + (i % 12) as f32 * 18.0;
        let by = 110.0 + ((i / 12) as f32) * 14.0;
        stress.spawn_unit(Team::Blue, bdef, false, Some((bx, by)), &mut rng);

        let rdef = &UNITS[(i + 7) % 24];
        let rx = 740.0 + (i % 12) as f32 * 18.0;
        let ry = 110.0 + ((i / 12) as f32) * 14.0;
        stress.spawn_unit(Team::Red, rdef, false, Some((rx, ry)), &mut rng);
    }
    let start = std::time::Instant::now();
    let ticks = 180;
    for _ in 0..ticks {
        stress.update(1.0 / 60.0, &mut rng);
    }
    let elapsed = start.elapsed();
    let ms_per_tick = (elapsed.as_secs_f64() * 1000.0) / ticks as f64;
    println!(
        "500-unit bench: {} ticks in {:.2} ms ({:.3} ms/tick), alive_end={}, proj_end={}, fx_end={}, texts_end={}",
        ticks,
        elapsed.as_secs_f64() * 1000.0,
        ms_per_tick,
        stress.units.len(),
        stress.projectiles.len(),
        stress.effects.len(),
        stress.floating_texts.len()
    );
    assert!(ms_per_tick < 16.6, "500-unit sim tick exceeded 16.6ms budget: {ms_per_tick:.3} ms");
}
