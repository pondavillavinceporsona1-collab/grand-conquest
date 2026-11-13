use rand::Rng;

use crate::ai::{build_smart_ai_deck, update_ai_commander};
use crate::ai_weights::AiBrainTelemetry;
use crate::missions::{get_mission, BossDef, MissionDef, WaveDef};
use crate::model::{
    AiDifficulty, AiPosture, ArmyOrder, FactionDef, FactionId, MissionArchetype, MutatorId,
    PassiveKey, RelicId, SpellFxType, StructureKind, Team, UnitDef, UnitShape, UnitTier,
    AURA_RADIUS_STUDS, LANE_BOTTOM, LANE_TOP, STUD_TO_PX, WALK_SPEED_SCALE, WORLD_H, WORLD_W,
};
use crate::save::ConquestSave;
use crate::units::{default_faction_deck, evaluate_deck_affinities, get_faction, get_unit, DeckAffinityBonus};

#[derive(Debug, Clone)]
pub struct StructureEntity {
    pub id: &'static str,
    pub team: Team,
    pub kind: StructureKind,
    pub order: u8,
    pub name: &'static str,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub dmg: f32,
    pub rng: f32,
    pub cd: f32,
    pub atk_timer: f32,
    pub is_invulnerable_override: bool,
    pub took_damage: bool,
}

#[derive(Debug, Clone)]
pub struct UnitEntity {
    pub uid: u64,
    pub is_boss: bool,
    pub boss_def: Option<&'static BossDef>,
    pub team: Team,
    pub def: &'static UnitDef,
    pub name: String,
    pub tier: UnitTier,
    pub shape: UnitShape,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub aim_angle: f32,
    pub radius: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub dmg: f32,
    pub base_rng_px: f32,
    pub cd: f32,
    pub spd: f32,
    pub passive: PassiveKey,
    pub atk_timer: f32,
    pub shots_fired: u32,
    pub support_timer: f32,
    pub brood_timer: f32,
    pub poison_timer: f32,
    pub poison_tick_accum: f32,
    pub poison_dps: f32,
    pub slow_timer: f32,
    pub slow_mult: f32,
    pub shield_hp: f32,
    pub shield_timer: Option<f32>,
    pub nanite_used: bool,
    pub death_immune_timer: f32,
    pub death_immune_used: bool,
    pub aura_def_bonus: f32,
    pub aura_atk_spd_bonus: f32,
    pub enemy_void_shred: f32,
    pub boss_phase: u8,
    pub boss_ability_timer: f32,
    pub boss_cast_remaining: Option<f32>,
    pub boss_cast_target_x: f32,
    pub boss_cast_target_y: f32,
    pub boss_aura_heal_accum: f32,
    pub phase_shift_accum: f32,
    pub ranged_immune_timer: f32,
    pub lunge_offset: f32,
    pub hit_flash: f32,
    pub walk_phase: f32,
    pub idle_phase: f32,
    pub atk_anim_timer: f32,
    pub atk_anim_duration: f32,
    pub is_moving: bool,
}

#[derive(Debug, Clone)]
pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub tx: f32,
    pub ty: f32,
    pub target_uid: Option<u64>,
    pub target_struct_id: Option<&'static str>,
    pub attacker_uid: Option<u64>,
    pub team: Team,
    pub dmg: f32,
    pub speed: f32,
    pub color_rgb: [u8; 3],
    pub radius: f32,
    pub apply_poison: bool,
}

#[derive(Debug, Clone)]
pub enum VisualEffectKind {
    SpawnRing,
    DeathBurst,
    HealWave,
    SpellQ(SpellFxType),
    SpellE,
    BossTelegraph {
        name: String,
        radius: f32,
        line_length: f32,
        aim_angle: f32,
    },
    BossImpact {
        radius: f32,
        line_length: f32,
        aim_angle: f32,
    },
    PlasmaCrater {
        radius: f32,
        dps: f32,
        tick_accum: f32,
    },
}

#[derive(Debug, Clone)]
pub struct VisualEffect {
    pub kind: VisualEffectKind,
    pub team: Team,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color_rgb: [u8; 3],
    pub life: f32,
    pub max_life: f32,
}

#[derive(Debug, Clone)]
pub struct FloatingText {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub color_rgb: [u8; 3],
    pub size: f32,
    pub life: f32,
}

#[derive(Debug, Clone)]
pub struct CommanderState {
    pub team: Team,
    pub faction: &'static FactionDef,
    pub deck: [Option<&'static UnitDef>; 6],
    pub gold: f32,
    pub income_lvl: u32,
    pub tier: u8,
    pub q_cd: f32,
    pub e_cd: f32,
    pub q_cd_max: f32,
    pub e_cd_max: f32,
    pub surge_timer: f32,
    pub surge_speed_mult: f32,
    pub decision_timer: f32,
    pub order: ArmyOrder,
}

impl CommanderState {
    pub fn has_tier_units(&self, tech_tier: u8) -> bool {
        self.deck
            .iter()
            .flatten()
            .any(|u| u.tier.required_tech_tier() == tech_tier)
    }
}

#[derive(Debug, Clone)]
pub struct WaveRuntime {
    pub def: &'static WaveDef,
    pub fired: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MatchStats {
    pub units_spawned: u32,
    pub gold_earned: f32,
    pub q_casts: u32,
    pub e_casts: u32,
    pub enemy_kills: u32,
    pub blue_casualties: u32,
    pub blue_a_tier_deaths: u32,
    pub triangles_spawned: u32,
    pub blue_hit_by_enemy_q: bool,
    pub max_simultaneous_poisoned_blue: usize,
    pub blue_death_immunity_triggers: u32,
}

#[derive(Debug, Clone)]
pub struct StarAuditDetail {
    pub star: u8,
    pub earned: bool,
    pub desc: String,
}

#[derive(Debug, Clone)]
pub struct MatchOutcomeSummary {
    pub is_campaign: bool,
    pub mission_id: Option<String>,
    pub mission_name: String,
    pub is_victory: bool,
    pub elapsed_secs: f32,
    pub stars_earned: u8,
    pub star_details: Vec<StarAuditDetail>,
    pub gold_bounty: u32,
    pub unlocked_blueprints: Vec<String>,
    pub unlocked_trophy: Option<String>,
    pub newly_unlocked_relics: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEvent {
    Deploy,
    SwordHit,
    SpellQ,
    SpellE,
    HealWave,
    BossWarning,
    Victory,
    Defeat,
}

#[derive(Debug, Clone)]
pub struct MatchState {
    pub is_campaign: bool,
    pub mission: Option<&'static MissionDef>,
    pub archetype: MissionArchetype,
    pub ai_difficulty: AiDifficulty,
    pub ai_posture: AiPosture,
    pub ai_bank_goal: f32,
    pub ai_brain: AiBrainTelemetry,
    pub active_mutator: Option<MutatorId>,
    pub active_relics: Vec<RelicId>,
    pub affinity: DeckAffinityBonus,
    pub time: f32,
    pub income_timer: f32,
    pub mutator_tick_accum: f32,
    pub defense_timer: Option<f32>,
    pub blitz_timer: Option<f32>,
    pub wave_queue: Vec<WaveRuntime>,
    pub over: bool,
    pub winner: Option<Team>,
    pub banner_text: String,
    pub stats: MatchStats,
    pub player: CommanderState,
    pub ai: CommanderState,
    pub structures: Vec<StructureEntity>,
    pub units: Vec<UnitEntity>,
    pub projectiles: Vec<Projectile>,
    pub effects: Vec<VisualEffect>,
    pub floating_texts: Vec<FloatingText>,
    pub sound_queue: Vec<SoundEvent>,
    next_uid: u64,
    sim_scratch: SimScratch,
}

const GRID_COLS: usize = 32;
const GRID_ROWS: usize = 8;
const GRID_CELLS: usize = GRID_COLS * GRID_ROWS;
const GRID_CELL_W: f32 = WORLD_W / GRID_COLS as f32;
const GRID_CELL_H: f32 = WORLD_H / GRID_ROWS as f32;

#[derive(Debug, Clone, Copy)]
struct GridUnit {
    uid: u64,
    x: f32,
    y: f32,
    radius: f32,
}

/// Compact CSR uniform spatial grid partitioned along lane width & height so
/// targeting, separation steering, and aura queries avoid O(N^2) scans at 500+ units.
#[derive(Debug, Clone)]
pub(crate) struct TeamSpatialGrid {
    entries: Vec<GridUnit>,
    scratch: Vec<(usize, GridUnit)>,
    counts: Vec<usize>,
    offsets: Vec<usize>,
    cursors: Vec<usize>,
    max_radius: f32,
}

impl Default for TeamSpatialGrid {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            scratch: Vec::new(),
            counts: vec![0; GRID_CELLS],
            offsets: vec![0; GRID_CELLS + 1],
            cursors: vec![0; GRID_CELLS],
            max_radius: 0.0,
        }
    }
}

impl TeamSpatialGrid {
    #[inline]
    fn coords(x: f32, y: f32) -> (usize, usize) {
        let col = ((x / GRID_CELL_W).floor() as isize).clamp(0, (GRID_COLS - 1) as isize) as usize;
        let row = ((y / GRID_CELL_H).floor() as isize).clamp(0, (GRID_ROWS - 1) as isize) as usize;
        (col, row)
    }

    pub(crate) fn build_for_team(&mut self, units: &[UnitEntity], team: Team) {
        self.counts.fill(0);
        self.scratch.clear();
        self.max_radius = 0.0;
        for u in units {
            if u.hp <= 0.0 || u.team != team {
                continue;
            }
            let (col, row) = Self::coords(u.x, u.y);
            let cell = col * GRID_ROWS + row;
            self.counts[cell] += 1;
            self.max_radius = self.max_radius.max(u.radius);
            self.scratch.push((
                cell,
                GridUnit {
                    uid: u.uid,
                    x: u.x,
                    y: u.y,
                    radius: u.radius,
                },
            ));
        }

        let total = self.scratch.len();
        self.entries.resize(
            total,
            GridUnit {
                uid: 0,
                x: 0.0,
                y: 0.0,
                radius: 0.0,
            },
        );
        let mut acc = 0usize;
        for cell in 0..GRID_CELLS {
            self.offsets[cell] = acc;
            self.cursors[cell] = acc;
            acc += self.counts[cell];
        }
        self.offsets[GRID_CELLS] = acc;

        for &(cell, gu) in &self.scratch {
            let dst = self.cursors[cell];
            self.entries[dst] = gu;
            self.cursors[cell] = dst + 1;
        }
    }

    #[inline]
    pub(crate) fn nearest_to(&self, ux: f32, uy: f32) -> Option<(u64, f32, f32, f32)> {
        if self.entries.is_empty() {
            return None;
        }
        let c0 = ((ux / GRID_CELL_W).floor() as isize).clamp(0, (GRID_COLS - 1) as isize) as usize;
        let mut best_edge_dist = f32::INFINITY;
        let mut best: Option<(u64, f32, f32, f32)> = None;

        let check_col =
            |col: usize, best_edge_dist: &mut f32, best: &mut Option<(u64, f32, f32, f32)>| {
                let start = self.offsets[col * GRID_ROWS];
                let end = self.offsets[(col + 1) * GRID_ROWS];
                for s in &self.entries[start..end] {
                    let dx = s.x - ux;
                    let dy = s.y - uy;
                    let d_sq = dx * dx + dy * dy;
                    if best_edge_dist.is_finite() {
                        let max_center_d = *best_edge_dist + s.radius;
                        if max_center_d <= 0.0 || d_sq >= max_center_d * max_center_d {
                            continue;
                        }
                    }
                    let edge_d = d_sq.sqrt() - s.radius;
                    if edge_d < *best_edge_dist {
                        *best_edge_dist = edge_d;
                        *best = Some((s.uid, s.x, s.y, edge_d));
                    }
                }
            };

        check_col(c0, &mut best_edge_dist, &mut best);
        let mut step = 1usize;
        let mut left_done = c0 == 0;
        let mut right_done = c0 + 1 >= GRID_COLS;
        while !left_done || !right_done {
            if !left_done {
                if let Some(col) = c0.checked_sub(step) {
                    let col_right = (col + 1) as f32 * GRID_CELL_W;
                    let min_dx = (ux - col_right).max(0.0);
                    if min_dx - self.max_radius >= best_edge_dist {
                        left_done = true;
                    } else {
                        check_col(col, &mut best_edge_dist, &mut best);
                        if col == 0 {
                            left_done = true;
                        }
                    }
                } else {
                    left_done = true;
                }
            }
            if !right_done {
                let col = c0 + step;
                if col < GRID_COLS {
                    let col_left = col as f32 * GRID_CELL_W;
                    let min_dx = (col_left - ux).max(0.0);
                    if min_dx - self.max_radius >= best_edge_dist {
                        right_done = true;
                    } else {
                        check_col(col, &mut best_edge_dist, &mut best);
                        if col + 1 >= GRID_COLS {
                            right_done = true;
                        }
                    }
                } else {
                    right_done = true;
                }
            }
            step += 1;
        }
        best
    }

    #[inline]
    fn accumulate_separation(&self, u_uid: u64, ux: f32, uy: f32, sep_rad: f32) -> (f32, f32) {
        if self.entries.is_empty() || sep_rad <= 0.0 {
            return (0.0, 0.0);
        }
        let sep_rad_sq = sep_rad * sep_rad;
        let (min_c, min_r) = Self::coords(ux - sep_rad, uy - sep_rad);
        let (max_c, max_r) = Self::coords(ux + sep_rad, uy + sep_rad);
        let mut sep_x = 0.0_f32;
        let mut sep_y = 0.0_f32;
        for col in min_c..=max_c {
            let start = self.offsets[col * GRID_ROWS + min_r];
            let end = self.offsets[col * GRID_ROWS + max_r + 1];
            for a in &self.entries[start..end] {
                if a.uid == u_uid {
                    continue;
                }
                let adx = ux - a.x;
                let ady = uy - a.y;
                let adist_sq = adx * adx + ady * ady;
                if adist_sq > 0.0 && adist_sq < sep_rad_sq {
                    let adist = adist_sq.sqrt();
                    let push = (sep_rad - adist) / sep_rad;
                    sep_x += (adx / adist) * push;
                    sep_y += (ady / adist) * push;
                }
            }
        }
        (sep_x, sep_y)
    }

    #[inline]
    fn any_within_radius(&self, qx: f32, qy: f32, radius: f32) -> bool {
        if self.entries.is_empty() {
            return false;
        }
        let r_sq = radius * radius;
        let (min_c, min_r) = Self::coords(qx - radius, qy - radius);
        let (max_c, max_r) = Self::coords(qx + radius, qy + radius);
        for col in min_c..=max_c {
            let start = self.offsets[col * GRID_ROWS + min_r];
            let end = self.offsets[col * GRID_ROWS + max_r + 1];
            for u in &self.entries[start..end] {
                let dx = u.x - qx;
                let dy = u.y - qy;
                if dx * dx + dy * dy <= r_sq {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, Default)]
struct SimScratch {
    blue_grid: TeamSpatialGrid,
    red_grid: TeamSpatialGrid,
}

impl MatchState {
    pub fn has_relic(&self, id: RelicId) -> bool {
        self.active_relics.contains(&id)
    }

    pub fn from_campaign_mission<R: Rng>(
        mission_id: &str,
        save: &ConquestSave,
        rng: &mut R,
    ) -> Result<Self, &'static str> {
        let mission = get_mission(mission_id).ok_or("Mission ID not found")?;
        let p_deck = resolve_player_deck(&save.equipped_deck_unit_ids);
        let e_deck_ids = [
            mission.enemy_deck[0].to_string(),
            mission.enemy_deck[1].to_string(),
            mission.enemy_deck[2].to_string(),
            mission.enemy_deck[3].to_string(),
            mission.enemy_deck[4].to_string(),
            mission.enemy_deck[5].to_string(),
        ];
        let e_deck = resolve_ai_deck(&e_deck_ids, mission.enemy_faction);
        let relics: Vec<RelicId> = save
            .equipped_relic_ids
            .iter()
            .filter_map(|s| RelicId::from_str_id(s))
            .collect();
        let affinity = evaluate_deck_affinities(&save.equipped_deck_unit_ids);

        let mut state = Self::build_core(
            true,
            Some(mission),
            mission.archetype,
            mission.ai_difficulty,
            mission.mutator,
            mission.starting_gold,
            save.player_commander_faction,
            mission.enemy_faction,
            p_deck,
            e_deck,
            relics,
            affinity,
            &format!("Campaign Mission: {}", mission.name),
        );

        match mission.archetype {
            MissionArchetype::HoldTheLine => {
                state.defense_timer = mission.time_limit_seconds;
                state.wave_queue = mission
                    .waves
                    .iter()
                    .map(|w| WaveRuntime { def: w, fired: false })
                    .collect();
                if let Some(eb) = state.structures.iter_mut().find(|s| s.id == "e_base") {
                    eb.is_invulnerable_override = true;
                }
            }
            MissionArchetype::Blitzkrieg => {
                state.blitz_timer = mission.time_limit_seconds;
            }
            MissionArchetype::TitanSlayer => {
                if let Some(boss_def) = mission.boss.as_ref() {
                    // Store static reference by looking back up in mission.boss
                    let static_boss = mission.boss.as_ref().unwrap();
                    let _ = boss_def;
                    state.spawn_boss(static_boss);
                }
            }
            MissionArchetype::Siege | MissionArchetype::Mutator => {}
        }

        state.spawn_difficulty_bonus_units(rng);
        Ok(state)
    }

    pub fn new_skirmish<R: Rng>(
        player_faction: FactionId,
        player_deck_ids: &[String; 6],
        enemy_faction: FactionId,
        ai_difficulty: AiDifficulty,
        mutator: Option<MutatorId>,
        equipped_relics: Vec<RelicId>,
        rng: &mut R,
    ) -> Self {
        let p_deck = resolve_ai_deck(player_deck_ids, player_faction);
        let e_deck_ids = build_smart_ai_deck(enemy_faction, &p_deck, ai_difficulty, mutator);
        let e_deck = resolve_ai_deck(&e_deck_ids, enemy_faction);
        let affinity = evaluate_deck_affinities(player_deck_ids);

        let mut state = Self::build_core(
            false,
            None,
            if mutator.is_some() {
                MissionArchetype::Mutator
            } else {
                MissionArchetype::Siege
            },
            ai_difficulty,
            mutator,
            40.0,
            player_faction,
            enemy_faction,
            p_deck,
            e_deck,
            equipped_relics,
            affinity,
            &format!(
                "Skirmish: {} vs {} ({})",
                player_faction.display_name(),
                enemy_faction.display_name(),
                ai_difficulty.name()
            ),
        );
        state.spawn_difficulty_bonus_units(rng);
        state
    }

    #[allow(clippy::too_many_arguments)]
    fn build_core(
        is_campaign: bool,
        mission: Option<&'static MissionDef>,
        archetype: MissionArchetype,
        ai_difficulty: AiDifficulty,
        active_mutator: Option<MutatorId>,
        starting_gold: f32,
        player_faction: FactionId,
        enemy_faction: FactionId,
        p_deck: [Option<&'static UnitDef>; 6],
        e_deck: [Option<&'static UnitDef>; 6],
        active_relics: Vec<RelicId>,
        affinity: DeckAffinityBonus,
        banner: &str,
    ) -> Self {
        let base_cd = 60.0 - affinity.pure_6_cdr_secs;
        let q_cd_max = if active_relics.contains(&RelicId::OverchargedConduits) {
            (48.0_f32 - affinity.pure_6_cdr_secs).max(24.0)
        } else {
            base_cd
        };

        let mut state = Self {
            is_campaign,
            mission,
            archetype,
            ai_difficulty,
            ai_posture: AiPosture::EcoBoom,
            ai_bank_goal: 0.0,
            ai_brain: AiBrainTelemetry::default(),
            active_mutator,
            active_relics,
            affinity,
            time: 0.0,
            income_timer: 0.0,
            mutator_tick_accum: 0.0,
            defense_timer: None,
            blitz_timer: None,
            wave_queue: Vec::new(),
            over: false,
            winner: None,
            banner_text: banner.to_string(),
            stats: MatchStats {
                gold_earned: starting_gold,
                ..Default::default()
            },
            player: CommanderState {
                team: Team::Blue,
                faction: get_faction(player_faction),
                deck: p_deck,
                gold: starting_gold,
                income_lvl: 0,
                tier: 1,
                q_cd: 0.0,
                e_cd: 0.0,
                q_cd_max,
                e_cd_max: q_cd_max,
                surge_timer: 0.0,
                surge_speed_mult: 1.45,
                decision_timer: 0.0,
                order: ArmyOrder::Attack,
            },
            ai: CommanderState {
                team: Team::Red,
                faction: get_faction(enemy_faction),
                deck: e_deck,
                gold: starting_gold,
                income_lvl: 0,
                tier: 1,
                q_cd: 12.0,
                e_cd: 18.0,
                q_cd_max: 60.0,
                e_cd_max: 60.0,
                surge_timer: 0.0,
                surge_speed_mult: 1.45,
                decision_timer: 1.1,
                order: ArmyOrder::Attack,
            },
            structures: vec![
                StructureEntity { id: "p_base", team: Team::Blue, kind: StructureKind::Base, order: 3, name: "Blue Base Tower", x: 95.0, y: 265.0, radius: 38.0, hp: 1500.0, max_hp: 1500.0, dmg: 28.0, rng: 220.0, cd: 1.1, atk_timer: 0.0, is_invulnerable_override: false, took_damage: false },
                StructureEntity { id: "p_t2", team: Team::Blue, kind: StructureKind::Turret2, order: 2, name: "Blue Inner Turret 2", x: 245.0, y: 265.0, radius: 25.0, hp: 600.0, max_hp: 600.0, dmg: 20.0, rng: 195.0, cd: 1.0, atk_timer: 0.0, is_invulnerable_override: false, took_damage: false },
                StructureEntity { id: "p_t1", team: Team::Blue, kind: StructureKind::Turret1, order: 1, name: "Blue Outer Turret 1", x: 410.0, y: 265.0, radius: 22.0, hp: 450.0, max_hp: 450.0, dmg: 16.0, rng: 180.0, cd: 0.95, atk_timer: 0.0, is_invulnerable_override: false, took_damage: false },
                StructureEntity { id: "e_t1", team: Team::Red, kind: StructureKind::Turret1, order: 1, name: "Red Outer Turret 1", x: WORLD_W - 410.0, y: 265.0, radius: 22.0, hp: 450.0, max_hp: 450.0, dmg: 16.0, rng: 180.0, cd: 0.95, atk_timer: 0.0, is_invulnerable_override: false, took_damage: false },
                StructureEntity { id: "e_t2", team: Team::Red, kind: StructureKind::Turret2, order: 2, name: "Red Inner Turret 2", x: WORLD_W - 245.0, y: 265.0, radius: 25.0, hp: 600.0, max_hp: 600.0, dmg: 20.0, rng: 195.0, cd: 1.0, atk_timer: 0.0, is_invulnerable_override: false, took_damage: false },
                StructureEntity { id: "e_base", team: Team::Red, kind: StructureKind::Base, order: 3, name: "Red Base Tower", x: WORLD_W - 95.0, y: 265.0, radius: 38.0, hp: 1500.0, max_hp: 1500.0, dmg: 28.0, rng: 220.0, cd: 1.1, atk_timer: 0.0, is_invulnerable_override: false, took_damage: false },
            ],
            units: Vec::new(),
            projectiles: Vec::new(),
            effects: Vec::new(),
            floating_texts: Vec::new(),
            sound_queue: Vec::new(),
            next_uid: 1,
            sim_scratch: SimScratch::default(),
        };

        // Relic on_match_init hooks
        if state.has_relic(RelicId::TreasurySeal) {
            state.player.gold += 25.0;
            state.stats.gold_earned += 25.0;
        }
        if state.has_relic(RelicId::TitaniumFort) {
            for s in &mut state.structures {
                if s.team == Team::Blue {
                    if s.kind == StructureKind::Base {
                        s.max_hp = 1850.0;
                        s.hp = 1850.0;
                    } else {
                        s.rng *= 1.15;
                    }
                }
            }
        }

        state
    }

    fn spawn_difficulty_bonus_units<R: Rng>(&mut self, rng: &mut R) {
        if self.archetype == MissionArchetype::HoldTheLine {
            return;
        }
        let tiers = self.ai_difficulty.bonus_tiers();
        for (idx, t) in tiers.iter().enumerate() {
            let pool: Vec<_> = self.ai.deck.iter().flatten().copied().filter(|u| u.tier == *t).collect();
            if !pool.is_empty() {
                let def = pool[idx % pool.len()];
                self.spawn_unit(Team::Red, def, true, None, rng);
            }
        }
    }

    fn spawn_boss(&mut self, boss_def: &'static BossDef) {
        let base_def = get_unit(boss_def.base_unit_id).unwrap_or_else(|| get_unit("sol_7").unwrap());
        let uid = self.next_uid;
        self.next_uid += 1;

        self.units.push(UnitEntity {
            uid,
            is_boss: true,
            boss_def: Some(boss_def),
            team: Team::Red,
            def: base_def,
            name: boss_def.name.to_string(),
            tier: UnitTier::A,
            shape: UnitShape::Hexagon,
            x: WORLD_W - 150.0,
            y: 265.0,
            vx: -1.0,
            vy: 0.0,
            aim_angle: std::f32::consts::PI,
            radius: 20.0 * boss_def.scale,
            hp: boss_def.max_hp,
            max_hp: boss_def.max_hp,
            dmg: boss_def.dmg,
            base_rng_px: boss_def.rng_studs * STUD_TO_PX,
            cd: boss_def.cd,
            spd: boss_def.spd_studs * WALK_SPEED_SCALE,
            passive: PassiveKey::None,
            atk_timer: 0.0,
            shots_fired: 0,
            support_timer: 3.0,
            brood_timer: boss_def.brood_spawn_interval.unwrap_or(999.0),
            poison_timer: 0.0,
            poison_tick_accum: 0.0,
            poison_dps: 0.0,
            slow_timer: 0.0,
            slow_mult: 1.0,
            shield_hp: 0.0,
            shield_timer: None,
            nanite_used: false,
            death_immune_timer: 0.0,
            death_immune_used: false,
            aura_def_bonus: 0.0,
            aura_atk_spd_bonus: 0.0,
            enemy_void_shred: 0.0,
            boss_phase: 1,
            boss_ability_timer: boss_def.ability.cooldown * 0.65,
            boss_cast_remaining: None,
            boss_cast_target_x: WORLD_W * 0.5,
            boss_cast_target_y: 265.0,
            boss_aura_heal_accum: 0.0,
            phase_shift_accum: 0.0,
            ranged_immune_timer: 0.0,
            lunge_offset: 0.0,
            hit_flash: 0.0,
            walk_phase: 0.0,
            idle_phase: 0.0,
            atk_anim_timer: 0.0,
            atk_anim_duration: 0.45,
            is_moving: true,
        });
    }

    pub fn active_boss(&self) -> Option<&UnitEntity> {
        self.units.iter().find(|u| u.is_boss && u.hp > 0.0)
    }

    pub fn is_structure_vulnerable(&self, st: &StructureEntity) -> bool {
        if st.hp <= 0.0 || st.is_invulnerable_override {
            return false;
        }
        !self
            .structures
            .iter()
            .any(|other| other.team == st.team && other.order < st.order && other.hp > 0.0)
    }

    pub fn get_income_rate(&self, team: Team) -> f32 {
        match team {
            Team::Blue => {
                let bonus = self
                    .mission
                    .map(|m| m.player_income_bonus_rate)
                    .unwrap_or(0.0);
                3.0 + self.player.income_lvl as f32 * 2.0 + bonus
            }
            Team::Red => (3.0 + self.ai.income_lvl as f32 * 2.0) * self.ai_difficulty.income_mult(),
        }
    }

    pub fn income_upgrade_cost(&self, team: Team) -> u32 {
        let lvl = match team {
            Team::Blue => self.player.income_lvl,
            Team::Red => self.ai.income_lvl,
        };
        let base = 20 + lvl * 15;
        if team == Team::Blue && self.has_relic(RelicId::EconomicMonopoly) {
            ((base as f32) * 0.75).round() as u32
        } else {
            base
        }
    }

    pub fn next_tier_cost(tier: u8) -> Option<u32> {
        match tier {
            1 => Some(100),
            2 => Some(250),
            _ => None,
        }
    }

    pub fn player_try_upgrade_income(&mut self) -> bool {
        if self.over {
            return false;
        }
        let cost = self.income_upgrade_cost(Team::Blue);
        if self.player.gold < cost as f32 {
            self.set_banner(format!(
                "Need {cost}G to upgrade Income to Lvl {}!",
                self.player.income_lvl + 1
            ));
            return false;
        }
        self.player.gold -= cost as f32;
        self.player.income_lvl += 1;
        let rate = self.get_income_rate(Team::Blue);
        self.set_banner(format!(
            "[U] Income Upgraded to Lvl {} (+{rate:.0}G/s)!",
            self.player.income_lvl
        ));
        self.add_floating_text(
            140.0,
            120.0,
            format!("INCOME LVL {} (+{rate:.0}G/s)", self.player.income_lvl),
            [52, 211, 153],
            14.0,
        );
        self.sound_queue.push(SoundEvent::Deploy);
        true
    }

    pub fn max_player_tech_tier(&self) -> u8 {
        if self.player.has_tier_units(3) {
            3
        } else if self.player.has_tier_units(2) {
            2
        } else {
            1
        }
    }

    pub fn player_try_unlock_tier(&mut self) -> bool {
        if self.over {
            return false;
        }
        let max_deck_tier = self.max_player_tech_tier();
        if self.player.tier >= max_deck_tier {
            self.set_banner(
                "No higher-tier units in deck yet — unlock B/A champions in Campaign!".to_string(),
            );
            return false;
        }
        let Some(cost) = Self::next_tier_cost(self.player.tier) else {
            return false;
        };
        if self.player.gold < cost as f32 {
            self.set_banner(format!(
                "Need {cost}G to unlock Tier {} Champions!",
                self.player.tier + 1
            ));
            return false;
        }
        self.player.gold -= cost as f32;
        self.player.tier += 1;
        let unlocks = if self.player.tier == 2 {
            "Slots [3] & [4] (B-Tier Veterans)"
        } else {
            "Slots [5] & [6] (A-Tier Mythics)"
        };
        self.set_banner(format!("[T] Unlocked Tier {}! {unlocks} ready!", self.player.tier));
        self.add_floating_text(
            160.0,
            150.0,
            format!("TIER {} UNLOCKED!", self.player.tier),
            [192, 132, 252],
            15.0,
        );
        self.sound_queue.push(SoundEvent::Deploy);
        true
    }

    pub fn max_ai_tech_tier(&self) -> u8 {
        if self.ai.has_tier_units(3) {
            3
        } else if self.ai.has_tier_units(2) {
            2
        } else {
            1
        }
    }

    pub fn ai_try_upgrade_income(&mut self) -> bool {
        if self.over {
            return false;
        }
        let cost = self.income_upgrade_cost(Team::Red);
        if self.ai.gold < cost as f32 {
            return false;
        }
        self.ai.gold -= cost as f32;
        self.ai.income_lvl += 1;
        let rate = self.get_income_rate(Team::Red);
        self.add_floating_text(
            WORLD_W - 140.0,
            120.0,
            format!("ENEMY ECO LVL {} (+{rate:.0}G/s)", self.ai.income_lvl),
            [251, 146, 60],
            13.0,
        );
        true
    }

    pub fn set_army_order(&mut self, team: Team, order: ArmyOrder) {
        if self.over {
            return;
        }
        let cmd = if team == Team::Blue {
            &mut self.player
        } else {
            &mut self.ai
        };
        if cmd.order == order {
            return;
        }
        cmd.order = order;
        if team == Team::Blue {
            self.set_banner(format!("Army Orders: {} — {}", order.label(), order.desc()));
            self.sound_queue.push(SoundEvent::Deploy);
        } else {
            let tag_col = match order {
                ArmyOrder::Attack => [248, 113, 113],
                ArmyOrder::Hold => [251, 191, 36],
                ArmyOrder::Retreat => [192, 132, 252],
            };
            self.add_floating_text(
                WORLD_W - 150.0,
                95.0,
                format!("ENEMY ORDER: {}", order.label()),
                tag_col,
                13.5,
            );
        }
    }

    pub fn hold_line_x(&self, team: Team) -> f32 {
        let mut best_front = None::<f32>;
        for st in &self.structures {
            if st.team == team && st.hp > 0.0 {
                best_front = Some(match best_front {
                    None => st.x,
                    Some(cur) => {
                        if team == Team::Blue {
                            cur.max(st.x)
                        } else {
                            cur.min(st.x)
                        }
                    }
                });
            }
        }
        let anchor = best_front.unwrap_or(if team == Team::Blue { 95.0 } else { WORLD_W - 95.0 });
        if team == Team::Blue {
            anchor + 65.0
        } else {
            anchor - 65.0
        }
    }

    pub fn retreat_anchor_x(&self, team: Team) -> f32 {
        if team == Team::Blue {
            160.0
        } else {
            WORLD_W - 160.0
        }
    }

    pub fn ai_try_unlock_tier(&mut self) -> bool {
        if self.over || self.ai.tier >= self.max_ai_tech_tier() {
            return false;
        }
        let Some(cost) = Self::next_tier_cost(self.ai.tier) else {
            return false;
        };
        if self.ai.gold < cost as f32 {
            return false;
        }
        self.ai.gold -= cost as f32;
        self.ai.tier += 1;
        self.add_floating_text(
            WORLD_W - 160.0,
            150.0,
            format!("ENEMY TIER {}!", self.ai.tier),
            [248, 113, 113],
            14.5,
        );
        self.sound_queue.push(SoundEvent::Deploy);
        true
    }

    pub fn player_try_spawn_slot<R: Rng>(&mut self, slot_idx: usize, rng: &mut R) -> bool {
        self.player_try_spawn_slot_count(slot_idx, 1, rng) > 0
    }

    pub fn player_try_spawn_slot_count<R: Rng>(
        &mut self,
        slot_idx: usize,
        requested: usize,
        rng: &mut R,
    ) -> usize {
        if self.over || slot_idx >= 6 || requested == 0 {
            return 0;
        }
        let Some(unit_def) = self.player.deck[slot_idx] else {
            let need = match slot_idx {
                2 | 3 => "B-Tier (unlock by winning early Campaign battles)",
                4 | 5 => "A-Tier (unlock by defeating Chapter 1 Boss)",
                _ => "a unit in Armory",
            };
            self.set_banner(format!("Slot [{}] is empty — equip {need}.", slot_idx + 1));
            return 0;
        };
        let req_tier = unit_def.tier.required_tech_tier();
        if self.player.tier < req_tier {
            self.set_banner(format!(
                "Slot [{}] ({}) requires Tier {req_tier}! Upgrade [T] first.",
                slot_idx + 1,
                unit_def.name
            ));
            return 0;
        }
        let unit_cost = unit_def.cost as f32;
        if self.player.gold < unit_cost {
            self.set_banner(format!(
                "Need {}G to deploy {}!",
                unit_def.cost, unit_def.name
            ));
            return 0;
        }

        let max_afford = if unit_cost > 0.0 {
            (self.player.gold / unit_cost).floor() as usize
        } else {
            requested
        };
        let spawn_n = max_afford.min(requested);
        if spawn_n == 0 {
            return 0;
        }

        self.player.gold = (self.player.gold - unit_cost * spawn_n as f32).max(0.0);
        self.stats.units_spawned += spawn_n as u32;
        if unit_def.shape == UnitShape::Triangle {
            self.stats.triangles_spawned += spawn_n as u32;
        }
        self.ai_brain.player_tracker.observe_player_deploy(unit_def, spawn_n);

        let usable_h = LANE_BOTTOM - LANE_TOP - 90.0;
        for i in 0..spawn_n {
            let pos = if spawn_n == 1 {
                None
            } else {
                // Spread batch deployments into staggered ranks across the lane instead of stacking on one pixel
                let rows = 10usize.min(spawn_n);
                let col = i / rows;
                let row = i % rows;
                let sx = (126.0 + (col as f32) * 14.0 + rng.gen_range(-3.0..3.0))
                    .clamp(112.0, 215.0);
                let frac = (row as f32 + 0.5) / (rows as f32);
                let sy = (LANE_TOP + 45.0 + frac * usable_h + rng.gen_range(-8.0..8.0))
                    .clamp(LANE_TOP + 36.0, LANE_BOTTOM - 36.0);
                Some((sx, sy))
            };
            self.spawn_unit(Team::Blue, unit_def, false, pos, rng);
        }

        if spawn_n == 1 {
            self.set_banner(format!(
                "Deployed [{}] {} ({})!",
                slot_idx + 1,
                unit_def.name,
                unit_def.shape.short_role()
            ));
        } else {
            self.set_banner(format!(
                "Deployed x{spawn_n} [{}] {} ({}G)!",
                slot_idx + 1,
                unit_def.name,
                (unit_def.cost as usize) * spawn_n
            ));
        }
        self.sound_queue.push(SoundEvent::Deploy);
        spawn_n
    }

    pub fn spawn_unit<R: Rng>(
        &mut self,
        team: Team,
        unit_def: &'static UnitDef,
        _is_free_summon: bool,
        custom_pos: Option<(f32, f32)>,
        rng: &mut R,
    ) -> u64 {
        let is_blue = team == Team::Blue;
        let (spawn_x, spawn_y) = if let Some(p) = custom_pos {
            p
        } else {
            let sx = if is_blue {
                140.0 + rng.gen_range(0.0..25.0)
            } else {
                WORLD_W - 140.0 - rng.gen_range(0.0..25.0)
            };
            let sy = LANE_TOP + 45.0 + rng.gen_range(0.0..(LANE_BOTTOM - LANE_TOP - 90.0));
            (sx, sy)
        };

        let base_radius = match unit_def.tier {
            UnitTier::A => 16.0,
            UnitTier::B => 12.5,
            UnitTier::C => 10.0,
        };

        let mut max_hp = unit_def.hp;
        let mut dmg = unit_def.dmg;
        let mut base_rng_px = unit_def.rng_studs * STUD_TO_PX;
        let mut cd = unit_def.cd;
        let mut spd = unit_def.spd_studs * WALK_SPEED_SCALE;

        // Player Relic Hooks (on_spawn_unit)
        if is_blue {
            if self.has_relic(RelicId::RapidBanner) {
                spd *= 1.12;
            }
            if self.has_relic(RelicId::BallisticLens) && unit_def.shape == UnitShape::Triangle {
                base_rng_px += 4.0 * STUD_TO_PX;
            }
            if self.has_relic(RelicId::CrownGeoTerra) {
                max_hp = (max_hp * 1.10).round();
                dmg = (dmg * 1.10).round();
                spd *= 1.10;
            }

            // Deck Faction Affinity bonuses (3-pc & 6-pc)
            max_hp = (max_hp * self.affinity.mechanica_max_hp_mult).round();
            spd += self.affinity.hive_move_studs_bonus * WALK_SPEED_SCALE;
            if self.affinity.aegis_ranged_spd_bonus > 0.0
                && matches!(unit_def.shape, UnitShape::Triangle | UnitShape::Diamond)
            {
                cd /= 1.0 + self.affinity.aegis_ranged_spd_bonus;
            }
        }

        // Environmental Mutator Spawn Modifiers
        if let Some(mut_id) = self.active_mutator {
            match mut_id {
                MutatorId::EmpScrambler => {
                    if matches!(unit_def.shape, UnitShape::Triangle | UnitShape::Diamond) {
                        cd *= 1.40;
                    } else if matches!(unit_def.shape, UnitShape::Circle | UnitShape::Square) {
                        spd *= 1.25;
                    }
                }
                MutatorId::BloodFrenzy => {
                    cd /= 1.35;
                    spd *= 1.25;
                }
                MutatorId::NaniteRegeneration => {
                    if matches!(unit_def.shape, UnitShape::Square | UnitShape::Hexagon) {
                        spd *= 1.20;
                    }
                }
                _ => {}
            }
        }

        let uid = self.next_uid;
        self.next_uid += 1;

        self.units.push(UnitEntity {
            uid,
            is_boss: false,
            boss_def: None,
            team,
            def: unit_def,
            name: unit_def.name.to_string(),
            tier: unit_def.tier,
            shape: unit_def.shape,
            x: spawn_x,
            y: spawn_y,
            vx: if is_blue { 1.0 } else { -1.0 },
            vy: 0.0,
            aim_angle: if is_blue { 0.0 } else { std::f32::consts::PI },
            radius: base_radius,
            hp: max_hp,
            max_hp,
            dmg,
            base_rng_px,
            cd,
            spd,
            passive: unit_def.passive,
            atk_timer: rng.gen_range(0.0..0.25),
            shots_fired: 0,
            support_timer: 1.5,
            brood_timer: 6.0,
            poison_timer: 0.0,
            poison_tick_accum: 0.0,
            poison_dps: 5.0,
            slow_timer: 0.0,
            slow_mult: 1.0,
            shield_hp: 0.0,
            shield_timer: None,
            nanite_used: false,
            death_immune_timer: 0.0,
            death_immune_used: false,
            aura_def_bonus: 0.0,
            aura_atk_spd_bonus: 0.0,
            enemy_void_shred: 0.0,
            boss_phase: 1,
            boss_ability_timer: 0.0,
            boss_cast_remaining: None,
            boss_cast_target_x: 0.0,
            boss_cast_target_y: 0.0,
            boss_aura_heal_accum: 0.0,
            phase_shift_accum: 0.0,
            ranged_immune_timer: 0.0,
            lunge_offset: 0.0,
            hit_flash: 0.0,
            walk_phase: rng.gen_range(0.0..(std::f32::consts::PI * 2.0)),
            idle_phase: rng.gen_range(0.0..(std::f32::consts::PI * 2.0)),
            atk_anim_timer: 0.0,
            atk_anim_duration: if unit_def.passive == PassiveKey::Gatling {
                0.18
            } else {
                0.38
            },
            is_moving: true,
        });

        self.push_minor_effect(VisualEffect {
            kind: VisualEffectKind::SpawnRing,
            team,
            x: spawn_x,
            y: spawn_y,
            radius: base_radius * 2.2,
            color_rgb: if is_blue { [96, 165, 250] } else { [248, 113, 113] },
            life: 0.35,
            max_life: 0.35,
        });

        uid
    }

    pub fn cast_commander_spell_q(&mut self, team: Team) -> bool {
        if self.over {
            return false;
        }
        let is_blue = team == Team::Blue;
        let cmd = if is_blue { &mut self.player } else { &mut self.ai };
        if cmd.q_cd > 0.0 {
            return false;
        }
        cmd.q_cd = cmd.q_cd_max;
        let fx_type = cmd.faction.q_fx_type;
        let spell_name = cmd.faction.q_spell_name;

        if is_blue {
            self.stats.q_casts += 1;
        }

        let enemy_team = team.enemy();
        let mut enemies: Vec<(u64, f32, f32)> = self
            .units
            .iter()
            .filter(|u| u.team == enemy_team && u.hp > 0.0)
            .map(|u| (u.uid, u.x, u.y))
            .collect();

        let mut target_x = if is_blue { WORLD_W * 0.65 } else { WORLD_W * 0.35 };
        let mut target_y = (LANE_TOP + LANE_BOTTOM) * 0.5;

        if !enemies.is_empty() {
            enemies.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let mut best_count = 0usize;
            let mut back = 0usize;
            let mut hi = 0usize;
            let r_sq = 135.0_f32 * 135.0;
            for lo in 0..enemies.len() {
                let (_, cx, cy) = enemies[lo];
                while back < lo && cx - enemies[back].1 > 135.0 {
                    back += 1;
                }
                while hi < enemies.len() && enemies[hi].1 - cx <= 135.0 {
                    hi += 1;
                }
                let count = enemies[back..hi]
                    .iter()
                    .filter(|&&(_, ox, oy)| {
                        let dx = ox - cx;
                        let dy = oy - cy;
                        dx * dx + dy * dy <= r_sq
                    })
                    .count();
                if count > best_count {
                    best_count = count;
                    target_x = cx;
                    target_y = cy;
                }
            }
        } else if let Some(st) = self
            .structures
            .iter()
            .find(|s| s.team == enemy_team && self.is_structure_vulnerable(s))
        {
            target_x = st.x;
            target_y = st.y;
        }

        let mut aoe_radius = 16.0 * STUD_TO_PX; // 136px
        if is_blue && self.has_relic(RelicId::OrbitalLens) {
            aoe_radius *= 1.25; // 170px
        }

        let mut hit_uids = Vec::new();
        for &(uid, ex, ey) in &enemies {
            if (ex - target_x).hypot(ey - target_y) <= aoe_radius {
                hit_uids.push(uid);
            }
        }
        if !is_blue && !hit_uids.is_empty() {
            self.stats.blue_hit_by_enemy_q = true;
        }
        for uid in hit_uids {
            self.apply_damage_to_unit(uid, 115.0, None, true, false);
        }

        // Damage vulnerable enemy structures within AoE
        let vuln_ids: Vec<&'static str> = self
            .structures
            .iter()
            .filter(|st| {
                st.team == enemy_team
                    && self.is_structure_vulnerable(st)
                    && (st.x - target_x).hypot(st.y - target_y) <= aoe_radius + st.radius
            })
            .map(|st| st.id)
            .collect();

        for id in vuln_ids {
            if let Some(st) = self.structures.iter_mut().find(|s| s.id == id) {
                st.hp = (st.hp - 115.0).max(0.0);
                st.took_damage = true;
                let (sx, sy) = (st.x, st.y);
                self.add_floating_text(sx, sy - 30.0, "-115 [Q]".into(), [251, 113, 133], 15.0);
            }
        }

        self.effects.push(VisualEffect {
            kind: VisualEffectKind::SpellQ(fx_type),
            team,
            x: target_x,
            y: target_y,
            radius: aoe_radius,
            color_rgb: if is_blue { [96, 165, 250] } else { [248, 113, 113] },
            life: 0.9,
            max_life: 0.9,
        });

        self.add_floating_text(
            target_x,
            target_y - 45.0,
            format!("{}! (-115 AoE)", spell_name.to_uppercase()),
            if is_blue { [96, 165, 250] } else { [248, 113, 113] },
            15.0,
        );
        self.set_banner(format!(
            "{} cast [Q] {spell_name} (115 AoE DMG)!",
            if is_blue { "COMMANDER" } else { "ENEMY" }
        ));
        self.sound_queue.push(SoundEvent::SpellQ);
        true
    }

    pub fn cast_commander_spell_e(&mut self, team: Team) -> bool {
        if self.over {
            return false;
        }
        let is_blue = team == Team::Blue;
        let (spell_name, mut heal_amt, mut surge_speed) = {
            let cmd = if is_blue { &mut self.player } else { &mut self.ai };
            if cmd.e_cd > 0.0 {
                return false;
            }
            cmd.e_cd = cmd.e_cd_max;
            cmd.surge_timer = 4.0;
            (cmd.faction.e_spell_name, 75.0_f32, 1.45_f32)
        };

        if is_blue {
            self.stats.e_casts += 1;
            if self.has_relic(RelicId::SurgeAmplifier) {
                heal_amt = 100.0;
                surge_speed = 1.65;
            }
            self.player.surge_speed_mult = surge_speed;
            heal_amt *= self.affinity.solaris_heal_mult;
        } else {
            self.ai.surge_speed_mult = surge_speed;
        }

        if self.active_mutator == Some(MutatorId::ConsecratedSolar) {
            heal_amt *= 1.50;
        }
        heal_amt = heal_amt.round();

        let mut healed_coords = Vec::new();
        for u in &mut self.units {
            if u.team == team && u.hp > 0.0 {
                u.hp = (u.hp + heal_amt).min(u.max_hp);
                healed_coords.push((u.x, u.y - u.radius - 8.0));
            }
        }
        for (hx, hy) in healed_coords {
            self.add_floating_text(
                hx,
                hy,
                format!("+{heal_amt:.0} HP & SURGE!"),
                [52, 211, 153],
                11.0,
            );
        }

        self.effects.push(VisualEffect {
            kind: VisualEffectKind::SpellE,
            team,
            x: 0.0,
            y: 0.0,
            radius: 0.0,
            color_rgb: if is_blue { [56, 189, 248] } else { [251, 113, 133] },
            life: 0.75,
            max_life: 0.75,
        });

        let pct = ((surge_speed - 1.0) * 100.0).round() as i32;
        self.set_banner(format!(
            "{} cast [E] {spell_name} (+{heal_amt:.0} HP & +{pct}% SPD)!",
            if is_blue { "COMMANDER" } else { "ENEMY" }
        ));
        self.sound_queue.push(SoundEvent::SpellE);
        true
    }

    pub fn update<R: Rng>(&mut self, dt: f32, rng: &mut R) {
        if self.over || dt <= 0.0 {
            return;
        }
        self.time += dt;

        // 1. Passive Income Tick (Every 1.0s)
        self.income_timer += dt;
        if self.income_timer >= 1.0 {
            self.income_timer -= 1.0;
            let p_gain = self.get_income_rate(Team::Blue);
            let ai_gain = self.get_income_rate(Team::Red);
            self.player.gold += p_gain;
            self.stats.gold_earned += p_gain;
            self.ai.gold += ai_gain;
        }

        // 2. Cooldowns & Surge Buff Timers
        for cmd in [&mut self.player, &mut self.ai] {
            if cmd.q_cd > 0.0 {
                cmd.q_cd = (cmd.q_cd - dt).max(0.0);
            }
            if cmd.e_cd > 0.0 {
                cmd.e_cd = (cmd.e_cd - dt).max(0.0);
            }
            if cmd.surge_timer > 0.0 {
                cmd.surge_timer = (cmd.surge_timer - dt).max(0.0);
            }
        }

        // 3. Campaign Hold-the-Line / Blitzkrieg Timers & Wave Queues
        if self.archetype == MissionArchetype::HoldTheLine {
            if let Some(t) = self.defense_timer.as_mut() {
                *t = (*t - dt).max(0.0);
            }
            let mut spawned_waves = Vec::new();
            for (idx, w) in self.wave_queue.iter_mut().enumerate() {
                if self.time >= w.def.trigger_sec && !w.fired {
                    w.fired = true;
                    spawned_waves.push((idx, w.def));
                }
            }
            for (_, wdef) in spawned_waves {
                self.set_banner(format!("WARNING: {}!", wdef.alert_name));
                self.sound_queue.push(SoundEvent::BossWarning);
                for sp in wdef.spawns {
                    if let Some(udef) = get_unit(sp.unit_id) {
                        for i in 0..sp.count {
                            let y = (sp.y_offset + rng.gen_range(-12.0..12.0))
                                .clamp(LANE_TOP + 20.0, LANE_BOTTOM - 20.0);
                            self.spawn_unit(
                                Team::Red,
                                udef,
                                true,
                                Some((WORLD_W - 120.0 - (i as f32) * 18.0, y)),
                                rng,
                            );
                        }
                    }
                }
            }
            let p_base_alive = self
                .structures
                .iter()
                .find(|s| s.id == "p_base")
                .map(|s| s.hp > 0.0)
                .unwrap_or(false);
            if self.defense_timer.unwrap_or(1.0) <= 0.0 && p_base_alive {
                self.over = true;
                self.winner = Some(Team::Blue);
                self.sound_queue.push(SoundEvent::Victory);
                return;
            }
        }

        if self.archetype == MissionArchetype::Blitzkrieg {
            if let Some(t) = self.blitz_timer.as_mut() {
                *t = (*t - dt).max(0.0);
                if *t <= 0.0 {
                    self.over = true;
                    self.winner = Some(Team::Red);
                    self.set_banner(
                        "BLITZKRIEG TIME EXPIRED! Orbital strike obliterated our vanguard!".into(),
                    );
                    self.sound_queue.push(SoundEvent::Defeat);
                    return;
                }
            }
        }

        // 4. AI Commander Decision Loop
        update_ai_commander(self, dt, rng);

        // 5. Evaluate Aura Fields & Void Synergy
        self.evaluate_auras(dt);

        // 6. Boss Phase & Telegraphed Ability Controller
        self.update_boss_mechanics(dt, rng);

        // 7. Mutator Periodic Effects + Plasma Craters
        self.update_mutators_and_craters(dt);

        // 8. Update Units: Status, Healers, Broodmother, Steering, Combat
        self.update_units(dt, rng);

        // 9. Track simultaneous poisoned blue units (for M3_2 3-star criterion)
        let poisoned_blue = self
            .units
            .iter()
            .filter(|u| u.team == Team::Blue && u.hp > 0.0 && u.poison_timer > 0.0)
            .count();
        self.stats.max_simultaneous_poisoned_blue =
            self.stats.max_simultaneous_poisoned_blue.max(poisoned_blue);

        // 10. Defensive Structures Firing
        self.update_structures(dt);

        // 11. Projectiles Update (including Gravity Distortion curving)
        self.update_projectiles(dt);

        // 12. Prune Dead Units & Expire Visual Effects
        self.units.retain(|u| u.hp > 0.0);
        for fx in &mut self.effects {
            fx.life -= dt;
        }
        self.effects.retain(|fx| fx.life > 0.0);

        for ft in &mut self.floating_texts {
            ft.y -= 22.0 * dt;
            ft.life -= dt;
        }
        self.floating_texts.retain(|ft| ft.life > 0.0);

        // 13. Victory / Defeat Check
        let p_base_dead = self
            .structures
            .iter()
            .find(|s| s.id == "p_base")
            .map(|s| s.hp <= 0.0)
            .unwrap_or(false);
        let e_base_dead = self
            .structures
            .iter()
            .find(|s| s.id == "e_base")
            .map(|s| s.hp <= 0.0)
            .unwrap_or(false);

        if p_base_dead {
            self.over = true;
            self.winner = Some(Team::Red);
            self.sound_queue.push(SoundEvent::Defeat);
        } else if e_base_dead {
            if self.archetype == MissionArchetype::TitanSlayer && self.active_boss().is_some() {
                // Boss must also be defeated
                return;
            }
            self.over = true;
            self.winner = Some(Team::Blue);
            self.sound_queue.push(SoundEvent::Victory);
        }
    }

    fn evaluate_auras(&mut self, dt: f32) {
        let aura_px = AURA_RADIUS_STUDS * STUD_TO_PX;
        let void_shred_px = 16.0 * STUD_TO_PX;

        for u in &mut self.units {
            u.aura_def_bonus = 0.0;
            u.aura_atk_spd_bonus = 0.0;
            u.enemy_void_shred = 0.0;
        }

        // Snapshot aura sources
        let mut sources: Vec<(Team, f32, f32, f32, f32, f32)> = Vec::new();
        let mut boss_heal_pulses: Vec<(f32, f32, f32, f32)> = Vec::new();

        for u in &mut self.units {
            if u.hp <= 0.0 {
                continue;
            }
            let mut def_aura = 0.0_f32;
            let mut spd_aura = 0.0_f32;
            let mut range_px = aura_px;

            match u.passive {
                PassiveKey::EliteRiot => def_aura = 0.15,
                PassiveKey::VoidWarden => def_aura = 0.20,
                PassiveKey::EliteKnight => spd_aura = 0.10,
                PassiveKey::CyberCmdr => spd_aura = 0.20,
                _ => {}
            }

            if let Some(bdef) = u.boss_def {
                def_aura = def_aura.max(bdef.aura_def);
                spd_aura = spd_aura.max(bdef.aura_atk_spd);
                range_px = bdef.aura_range_studs * STUD_TO_PX;

                if bdef.aura_heal_per_6s > 0.0 {
                    u.boss_aura_heal_accum += dt;
                    if u.boss_aura_heal_accum >= 6.0 {
                        u.boss_aura_heal_accum -= 6.0;
                        boss_heal_pulses.push((u.x, u.y, range_px, bdef.aura_heal_per_6s));
                    }
                }
            }

            if def_aura > 0.0 || spd_aura > 0.0 {
                sources.push((u.team, u.x, u.y, range_px, def_aura, spd_aura));
            }
        }

        // Apply Void deck affinity armor shred aura around alive Blue units via spatial grid
        let blue_void_shred = self.affinity.void_shred_aura;
        if blue_void_shred > 0.0 {
            self.sim_scratch.blue_grid.build_for_team(&self.units, Team::Blue);
        }

        let blue_grid = &self.sim_scratch.blue_grid;
        for u in &mut self.units {
            if u.hp <= 0.0 {
                continue;
            }
            for &(s_team, sx, sy, s_rng, def_a, spd_a) in &sources {
                let dx = u.x - sx;
                let dy = u.y - sy;
                if u.team == s_team && dx * dx + dy * dy <= s_rng * s_rng {
                    u.aura_def_bonus = u.aura_def_bonus.max(def_a);
                    u.aura_atk_spd_bonus = u.aura_atk_spd_bonus.max(spd_a);
                }
            }
            if u.team == Team::Red && blue_void_shred > 0.0 && blue_grid.any_within_radius(u.x, u.y, void_shred_px) {
                u.enemy_void_shred = blue_void_shred;
            }
        }

        // Apply Boss Aura Heal Pulses (Grand Paladin Aurelius)
        let heal_mult = if self.active_mutator == Some(MutatorId::ConsecratedSolar) {
            1.5
        } else {
            1.0
        };
        let mut float_heals = Vec::new();
        for (bx, by, brng, h_amt) in boss_heal_pulses {
            let brng_sq = brng * brng;
            for u in &mut self.units {
                let dx = u.x - bx;
                let dy = u.y - by;
                if u.team == Team::Red && u.hp > 0.0 && dx * dx + dy * dy <= brng_sq {
                    let final_h = (h_amt * heal_mult).round();
                    u.hp = (u.hp + final_h).min(u.max_hp);
                    float_heals.push((u.x, u.y - u.radius - 6.0, final_h));
                }
            }
        }
        for (hx, hy, amt) in float_heals {
            self.add_floating_text(hx, hy, format!("+{amt:.0} AURA"), [251, 191, 36], 10.0);
        }
    }

    fn update_boss_mechanics<R: Rng>(&mut self, dt: f32, rng: &mut R) {
        let mut boss_uid_and_state = None;
        for u in &mut self.units {
            if u.is_boss && u.hp > 0.0 {
                if let Some(bdef) = u.boss_def {
                    // Phase Shift (Sovereign Malakor: immune to ranged hits 1.5s every 10s)
                    if let Some(interval) = bdef.phase_shift_interval {
                        u.phase_shift_accum += dt;
                        if u.phase_shift_accum >= interval {
                            u.phase_shift_accum -= interval;
                            u.ranged_immune_timer = 1.5;
                        }
                    }
                    if u.ranged_immune_timer > 0.0 {
                        u.ranged_immune_timer = (u.ranged_immune_timer - dt).max(0.0);
                    }

                    // Boss Phase transitions
                    let hp_pct = u.hp / u.max_hp;
                    if hp_pct <= 0.50 && u.boss_phase == 1 {
                        u.boss_phase = 2;
                        if bdef.has_phase2_shield {
                            u.shield_hp += bdef.phase2_shield_amount;
                            u.shield_timer = None;
                            boss_uid_and_state = Some((u.uid, 2u8, u.name.clone()));
                        }
                    } else if hp_pct <= 0.25 && u.boss_phase == 2 {
                        u.boss_phase = 3;
                        u.cd *= 0.70;
                        u.spd *= 1.25;
                        boss_uid_and_state = Some((u.uid, 3u8, u.name.clone()));
                    }
                }
            }
        }

        if let Some((_, phase, bname)) = boss_uid_and_state {
            if phase == 2 {
                self.set_banner(format!("{bname} ACTIVATED EMERGENCY SHIELD!"));
            } else {
                self.set_banner(format!("{bname} IS ENRAGED (+35% ATK SPD, +25% MOVE)!"));
            }
            self.sound_queue.push(SoundEvent::BossWarning);
        }

        // Boss Brood Spawner (Gorgon Leviathan: 2 Grunts / 8s)
        let mut brood_spawns: Vec<(f32, f32)> = Vec::new();
        for u in &mut self.units {
            if u.is_boss && u.hp > 0.0 {
                if let Some(bdef) = u.boss_def {
                    if let Some(interval) = bdef.brood_spawn_interval {
                        u.brood_timer -= dt;
                        if u.brood_timer <= 0.0 {
                            u.brood_timer = interval;
                            brood_spawns.push((u.x, u.y));
                        }
                    }
                }
            }
        }
        if let Some(grunt) = get_unit("hiv_1") {
            for (bx, by) in brood_spawns {
                self.spawn_unit(
                    Team::Red,
                    grunt,
                    true,
                    Some((bx - 24.0, (by - 18.0).max(LANE_TOP + 25.0))),
                    rng,
                );
                self.spawn_unit(
                    Team::Red,
                    grunt,
                    true,
                    Some((bx - 24.0, (by + 18.0).min(LANE_BOTTOM - 25.0))),
                    rng,
                );
            }
        }

        // Boss Telegraphed Abilities
        // Step 1: Decrement active telegraph cast timer or ability cooldown
        let mut started_cast: Option<(String, f32, f32, f32, f32, f32, f32, [u8; 3])> = None;
        let mut execute_cast: Option<(u64, &'static BossDef, f32, f32, f32)> = None;

        // Precompute highest-HP Blue unit and nearest Blue unit for targeting
        let highest_hp_blue = self
            .units
            .iter()
            .filter(|u| u.team == Team::Blue && u.hp > 0.0)
            .max_by(|a, b| a.hp.partial_cmp(&b.hp).unwrap_or(std::cmp::Ordering::Equal))
            .map(|u| (u.x, u.y));

        for u in &mut self.units {
            if !u.is_boss || u.hp <= 0.0 {
                continue;
            }
            let Some(bdef) = u.boss_def else { continue };
            let ab = &bdef.ability;

            if let Some(rem) = u.boss_cast_remaining.as_mut() {
                *rem -= dt;
                if *rem <= 0.0 {
                    u.boss_cast_remaining = None;
                    execute_cast = Some((
                        u.uid,
                        bdef,
                        u.boss_cast_target_x,
                        u.boss_cast_target_y,
                        u.aim_angle,
                    ));
                }
            } else {
                u.boss_ability_timer -= dt;
                if u.boss_ability_timer <= 0.0 {
                    u.boss_ability_timer = ab.cooldown;
                    let (tx, ty) = if ab.line_length > 0.0 {
                        (u.x, u.y)
                    } else if ab.name.contains("Ion") {
                        highest_hp_blue.unwrap_or((u.x - 90.0, u.y))
                    } else {
                        highest_hp_blue
                            .map(|(bx, by)| if (bx - u.x).hypot(by - u.y) <= ab.radius + 90.0 { (bx, by) } else { (u.x, u.y) })
                            .unwrap_or((u.x, u.y))
                    };
                    u.boss_cast_remaining = Some(ab.cast_time);
                    u.boss_cast_target_x = tx;
                    u.boss_cast_target_y = ty;
                    started_cast = Some((
                        format!("{} CHARGING {}!", u.name, ab.name.to_uppercase()),
                        tx,
                        ty,
                        ab.cast_time,
                        ab.radius,
                        ab.line_length,
                        u.aim_angle,
                        ab.color_rgb,
                    ));
                }
            }
        }

        if let Some((banner, tx, ty, cast_t, rad, line_len, ang, col)) = started_cast {
            self.set_banner(banner.clone());
            self.sound_queue.push(SoundEvent::BossWarning);
            self.effects.push(VisualEffect {
                kind: VisualEffectKind::BossTelegraph {
                    name: banner,
                    radius: rad,
                    line_length: line_len,
                    aim_angle: ang,
                },
                team: Team::Red,
                x: tx,
                y: ty,
                radius: rad,
                color_rgb: col,
                life: cast_t,
                max_life: cast_t,
            });
        }

        if let Some((boss_uid, bdef, tx, ty, aim_ang)) = execute_cast {
            let ab = &bdef.ability;
            self.set_banner(format!("{} CAST {}!", bdef.name, ab.name.to_uppercase()));

            let meteor_count = ab.meteor_count.max(1);
            let mut target_centers = Vec::with_capacity(meteor_count);
            if meteor_count == 1 {
                target_centers.push((tx, ty));
            } else {
                for m in 0..meteor_count {
                    let spread = (m as f32 - 1.0) * 95.0;
                    target_centers.push((
                        (tx + spread).clamp(110.0, WORLD_W - 110.0),
                        (ty + ((m as f32) - 1.0) * 28.0).clamp(LANE_TOP + 40.0, LANE_BOTTOM - 40.0),
                    ));
                }
            }

            for (cx, cy) in target_centers {
                let mut hit_blue = Vec::new();
                for u in &self.units {
                    if u.team == Team::Blue && u.hp > 0.0 {
                        let hit = if ab.line_length > 0.0 {
                            // Directional forward corridor (Caustic Acid Vomit)
                            let dx = u.x - cx;
                            let dy = u.y - cy;
                            let fwd = dx * aim_ang.cos() + dy * aim_ang.sin();
                            let perp = (-dx * aim_ang.sin() + dy * aim_ang.cos()).abs();
                            fwd >= -10.0 && fwd <= ab.line_length && perp <= 55.0
                        } else {
                            (u.x - cx).hypot(u.y - cy) <= ab.radius
                        };
                        if hit {
                            hit_blue.push(u.uid);
                        }
                    }
                }

                for uid in hit_blue {
                    self.apply_damage_to_unit(uid, ab.aoe_dmg, Some(boss_uid), true, false);
                    if let Some(idx) = self.find_alive_unit_idx(uid) {
                        let target = &mut self.units[idx];
                        if ab.slow_percent > 0.0 {
                            target.slow_timer = ab.slow_duration;
                            target.slow_mult = 1.0 - ab.slow_percent;
                        }
                        if ab.poison_dps > 0.0 {
                            target.poison_timer = ab.poison_duration;
                            target.poison_dps = ab.poison_dps;
                        }
                        if ab.name.contains("Black Hole") {
                            let dx = cx - target.x;
                            let dy = cy - target.y;
                            let dist = dx.hypot(dy).max(1.0);
                            target.x += (dx / dist) * 55.0;
                            target.y += (dy / dist) * 55.0;
                        }
                    }
                }

                self.effects.push(VisualEffect {
                    kind: VisualEffectKind::BossImpact {
                        radius: ab.radius,
                        line_length: ab.line_length,
                        aim_angle: aim_ang,
                    },
                    team: Team::Red,
                    x: cx,
                    y: cy,
                    radius: ab.radius,
                    color_rgb: ab.color_rgb,
                    life: 0.75,
                    max_life: 0.75,
                });

                if ab.name.contains("Ion") {
                    self.effects.push(VisualEffect {
                        kind: VisualEffectKind::PlasmaCrater {
                            radius: 75.0,
                            dps: 18.0,
                            tick_accum: 0.0,
                        },
                        team: Team::Red,
                        x: cx,
                        y: cy,
                        radius: 75.0,
                        color_rgb: ab.color_rgb,
                        life: 5.0,
                        max_life: 5.0,
                    });
                }
            }
        }
    }

    fn update_mutators_and_craters(&mut self, dt: f32) {
        if let Some(mut_id) = self.active_mutator {
            self.mutator_tick_accum += dt;
            match mut_id {
                MutatorId::CorrosiveMiasma if self.mutator_tick_accum >= 1.5 => {
                    self.mutator_tick_accum -= 1.5;
                    let safe_healers: Vec<(Team, f32, f32)> = self
                        .units
                        .iter()
                        .filter(|u| u.hp > 0.0 && u.shape == UnitShape::Cross)
                        .map(|u| (u.team, u.x, u.y))
                        .collect();
                    let safe_structs: Vec<(Team, f32, f32)> = self
                        .structures
                        .iter()
                        .filter(|s| s.hp > 0.0)
                        .map(|s| (s.team, s.x, s.y))
                        .collect();
                    let mut hurt_uids = Vec::new();
                    for u in &self.units {
                        if u.hp <= 0.0 {
                            continue;
                        }
                        let near_struct = safe_structs.iter().any(|&(t, sx, sy)| {
                            let dx = sx - u.x;
                            let dy = sy - u.y;
                            t == u.team && dx * dx + dy * dy <= 120.0 * 120.0
                        });
                        let near_healer = !near_struct
                            && safe_healers.iter().any(|&(t, hx, hy)| {
                                let dx = hx - u.x;
                                let dy = hy - u.y;
                                t == u.team && dx * dx + dy * dy <= 110.0 * 110.0
                            });
                        if !near_struct && !near_healer {
                            hurt_uids.push(u.uid);
                        }
                    }
                    for uid in hurt_uids {
                        self.apply_damage_to_unit(uid, 2.0, None, false, false);
                    }
                }
                MutatorId::BloodFrenzy if self.mutator_tick_accum >= 1.0 => {
                    self.mutator_tick_accum -= 1.0;
                    let targets: Vec<(u64, f32)> = self
                        .units
                        .iter()
                        .filter(|u| u.hp > 0.0)
                        .map(|u| (u.uid, (u.max_hp * 0.01).max(1.0)))
                        .collect();
                    for (uid, dmg) in targets {
                        self.apply_damage_to_unit(uid, dmg, None, false, false);
                    }
                }
                MutatorId::NaniteRegeneration if self.mutator_tick_accum >= 3.0 => {
                    self.mutator_tick_accum -= 3.0;
                    for u in &mut self.units {
                        if u.hp > 0.0 && matches!(u.shape, UnitShape::Square | UnitShape::Hexagon) {
                            u.hp = (u.hp + u.max_hp * 0.05).min(u.max_hp);
                        }
                    }
                }
                _ => {}
            }
        }

        // Persistent Plasma Crater burn (Colossus Prime)
        let mut crater_hits: Vec<(u64, f32)> = Vec::new();
        for fx in &mut self.effects {
            if let VisualEffectKind::PlasmaCrater { radius, dps, tick_accum } = &mut fx.kind {
                *tick_accum += dt;
                if *tick_accum >= 0.5 {
                    *tick_accum -= 0.5;
                    let r_sq = (*radius) * (*radius);
                    for u in &self.units {
                        let dx = u.x - fx.x;
                        let dy = u.y - fx.y;
                        if u.team == Team::Blue && u.hp > 0.0 && dx * dx + dy * dy <= r_sq {
                            crater_hits.push((u.uid, *dps * 0.5));
                        }
                    }
                }
            }
        }
        for (uid, dmg) in crater_hits {
            self.apply_damage_to_unit(uid, dmg, None, true, false);
        }
    }

    fn update_units<R: Rng>(&mut self, dt: f32, rng: &mut R) {
        // Phase 1: timers, poison ticks, broodmother spawns, healer waves
        let mut poison_hits: Vec<(u64, f32)> = Vec::new();
        let mut brood_spawns: Vec<(Team, f32, f32)> = Vec::new();
        let mut heal_actions: Vec<(Team, f32, f32, f32, f32, usize)> = Vec::new(); // team, hx, hy, rng, amt, max_targets

        for u in &mut self.units {
            if u.hp <= 0.0 {
                continue;
            }
            if u.hit_flash > 0.0 {
                u.hit_flash = (u.hit_flash - dt).max(0.0);
            }
            if u.lunge_offset > 0.0 {
                u.lunge_offset = (u.lunge_offset - dt * 28.0).max(0.0);
            }
            if u.death_immune_timer > 0.0 {
                u.death_immune_timer = (u.death_immune_timer - dt).max(0.0);
            }
            if u.slow_timer > 0.0 {
                u.slow_timer = (u.slow_timer - dt).max(0.0);
                if u.slow_timer == 0.0 {
                    u.slow_mult = 1.0;
                }
            }
            if let Some(st) = u.shield_timer.as_mut() {
                *st -= dt;
                if *st <= 0.0 {
                    u.shield_timer = None;
                    u.shield_hp = 0.0;
                }
            }
            if u.atk_anim_timer > 0.0 {
                u.atk_anim_timer = (u.atk_anim_timer - dt).max(0.0);
            }
            u.idle_phase += dt * 3.4;

            if u.poison_timer > 0.0 {
                u.poison_timer = (u.poison_timer - dt).max(0.0);
                u.poison_tick_accum += dt;
                if u.poison_tick_accum >= 0.5 {
                    u.poison_tick_accum -= 0.5;
                    poison_hits.push((u.uid, u.poison_dps * 0.5));
                }
            }

            if u.passive == PassiveKey::Broodmother {
                u.brood_timer -= dt;
                if u.brood_timer <= 0.0 {
                    u.brood_timer = 11.0;
                    brood_spawns.push((u.team, u.x, u.y));
                }
            }

            if let Some((amt, interval, max_targets)) = u.passive.heal_params() {
                u.support_timer -= dt;
                if u.support_timer <= 0.0 {
                    u.support_timer = interval;
                    heal_actions.push((u.team, u.x, u.y, u.base_rng_px + 35.0, amt, max_targets));
                }
            }
        }

        for (uid, dmg) in poison_hits {
            self.apply_damage_to_unit(uid, dmg, None, false, false);
        }

        if let Some(grunt) = get_unit("hiv_1") {
            for (team, bx, by) in brood_spawns {
                let dir = if team == Team::Blue { -1.0 } else { 1.0 };
                self.spawn_unit(
                    team,
                    grunt,
                    true,
                    Some((bx + dir * 24.0, (by - 18.0).max(LANE_TOP + 25.0))),
                    rng,
                );
                self.spawn_unit(
                    team,
                    grunt,
                    true,
                    Some((bx + dir * 24.0, (by + 18.0).min(LANE_BOTTOM - 25.0))),
                    rng,
                );
                self.add_floating_text(bx, by - 22.0, "SPAWNED 2 GRUNTS!".into(), [52, 211, 153], 10.0);
            }
        }

        // Process Healer Waves (with Consecrated Solar mutator, Solaris Affinity, & Sacred Phylactery Relic)
        let has_phylactery = self.has_relic(RelicId::SacredPhylactery);
        for (h_team, hx, hy, h_rng, base_amt, max_targets) in heal_actions {
            let mut heal_amt = base_amt;
            if h_team == Team::Blue {
                heal_amt *= self.affinity.solaris_heal_mult;
            }
            if self.active_mutator == Some(MutatorId::ConsecratedSolar) {
                heal_amt *= 1.50;
            }
            heal_amt = heal_amt.round();

            let h_rng_sq = h_rng * h_rng;
            let mut injured: Vec<(usize, f32)> = self
                .units
                .iter()
                .enumerate()
                .filter(|(_, a)| {
                    if a.team != h_team || a.hp <= 0.0 || a.hp >= a.max_hp {
                        return false;
                    }
                    let dx = a.x - hx;
                    let dy = a.y - hy;
                    dx * dx + dy * dy <= h_rng_sq
                })
                .map(|(i, a)| (i, a.hp / a.max_hp))
                .collect();

            injured.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            injured.truncate(max_targets);

            if !injured.is_empty() {
                let mut texts = Vec::new();
                for (idx, _) in injured {
                    let ally = &mut self.units[idx];
                    ally.hp = (ally.hp + heal_amt).min(ally.max_hp);
                    if has_phylactery && h_team == Team::Blue {
                        ally.shield_hp += 25.0;
                        ally.shield_timer = Some(4.0);
                    }
                    texts.push((ally.x, ally.y - ally.radius - 6.0));
                }
                for (tx, ty) in texts {
                    self.add_floating_text(tx, ty, format!("+{heal_amt:.0} HP"), [74, 222, 128], 10.0);
                }
                self.effects.push(VisualEffect {
                    kind: VisualEffectKind::HealWave,
                    team: h_team,
                    x: hx,
                    y: hy,
                    radius: (h_rng * 0.6).min(130.0),
                    color_rgb: [74, 222, 128],
                    life: 0.45,
                    max_life: 0.45,
                });
                self.sound_queue.push(SoundEvent::HealWave);
            }
        }

        // Phase 2: Spatial-grid target acquisition, separation steering, melee/ranged execution
        #[derive(Clone, Copy)]
        struct StructSnap {
            id: &'static str,
            team: Team,
            x: f32,
            y: f32,
            radius: f32,
        }
        let vuln_structs: Vec<StructSnap> = self
            .structures
            .iter()
            .filter(|s| self.is_structure_vulnerable(s))
            .map(|s| StructSnap {
                id: s.id,
                team: s.team,
                x: s.x,
                y: s.y,
                radius: s.radius,
            })
            .collect();

        let blue_surge = self.player.surge_timer > 0.0;
        let blue_surge_mult = if blue_surge { self.player.surge_speed_mult } else { 1.0 };
        let red_surge = self.ai.surge_timer > 0.0;
        let red_surge_mult = if red_surge { self.ai.surge_speed_mult } else { 1.0 };
        let blue_order = self.player.order;
        let red_order = self.ai.order;
        let _blue_hold_x = self.hold_line_x(Team::Blue);
        let _red_hold_x = self.hold_line_x(Team::Red);
        let blue_retreat_x = self.retreat_anchor_x(Team::Blue);
        let red_retreat_x = self.retreat_anchor_x(Team::Red);
        let has_arcane_mag = self.has_relic(RelicId::ArcaneMagnifier);
        let has_vamp_stone = self.has_relic(RelicId::VampiricStone);
        let consecrated_solar = self.active_mutator == Some(MutatorId::ConsecratedSolar);

        self.sim_scratch.blue_grid.build_for_team(&self.units, Team::Blue);
        self.sim_scratch.red_grid.build_for_team(&self.units, Team::Red);

        enum PendingAttack {
            MeleeUnit { attacker_uid: u64, target_uid: u64, dmg: f32 },
            MeleeStruct { struct_id: &'static str, dmg: f32 },
            SpawnProjectile(Projectile),
        }
        let mut pending_attacks = Vec::new();
        let mut melee_heals: Vec<(f32, f32, f32, &'static str)> = Vec::new();

        let blue_grid = &self.sim_scratch.blue_grid;
        let red_grid = &self.sim_scratch.red_grid;
        let units = &mut self.units;

        for i in 0..units.len() {
            if units[i].hp <= 0.0 {
                continue;
            }
            let u_uid = units[i].uid;
            let u_team = units[i].team;
            let ux = units[i].x;
            let uy = units[i].y;
            let u_rad = units[i].radius;
            let enemy_team = u_team.enemy();
            let (ally_grid, enemy_grid) = if u_team == Team::Blue {
                (blue_grid, red_grid)
            } else {
                (red_grid, blue_grid)
            };

            let eff_rng_px = if units[i].passive == PassiveKey::Sniper36 && units[i].shots_fired < 3 {
                36.0 * STUD_TO_PX
            } else {
                units[i].base_rng_px
            };

            let mut best_dist = f32::INFINITY;
            let mut best_pos = None;
            let mut best_unit_uid = None;
            let mut best_struct_id = None;

            if let Some((euid, ex, ey, edist)) = enemy_grid.nearest_to(ux, uy) {
                best_dist = edist;
                best_pos = Some((ex, ey));
                best_unit_uid = Some(euid);
            }

            for st in &vuln_structs {
                if st.team == enemy_team {
                    let dx = st.x - ux;
                    let dy = st.y - uy;
                    let d_sq = dx * dx + dy * dy;
                    if best_dist.is_finite() {
                        let max_cd = best_dist + st.radius;
                        if max_cd <= 0.0 || d_sq >= max_cd * max_cd {
                            continue;
                        }
                    }
                    let d = d_sq.sqrt() - st.radius;
                    if d < best_dist {
                        best_dist = d;
                        best_pos = Some((st.x, st.y));
                        best_unit_uid = None;
                        best_struct_id = Some(st.id);
                    }
                }
            }

            let Some((tx, ty)) = best_pos else {
                continue;
            };

            let dx = tx - ux;
            let dy = ty - uy;
            let aim_angle = dy.atan2(dx);
            units[i].aim_angle = aim_angle;

            let eff_cd = units[i].cd / (1.0 + units[i].aura_atk_spd_bonus);
            units[i].atk_timer = (units[i].atk_timer - dt).max(0.0);

            // Evaluate Army Order stance (Attack / Hold / Retreat; bosses always Attack)
            let unit_order = if units[i].is_boss {
                ArmyOrder::Attack
            } else if u_team == Team::Blue {
                blue_order
            } else {
                red_order
            };
            let home_dir = if u_team == Team::Blue { 1.0_f32 } else { -1.0_f32 };
            let backline_shift = match units[i].shape {
                UnitShape::Triangle | UnitShape::Diamond | UnitShape::Cross => 34.0,
                UnitShape::Circle => 8.0,
                _ => 0.0,
            };

            let (should_attack, move_target): (bool, Option<(f32, f32, f32)>) = match unit_order {
                ArmyOrder::Attack => {
                    if best_dist <= eff_rng_px {
                        (true, None)
                    } else {
                        (false, Some((tx, ty, 1.0)))
                    }
                }
                ArmyOrder::Retreat => {
                    let anchor_x = (if u_team == Team::Blue { blue_retreat_x } else { red_retreat_x })
                        - home_dir * (backline_shift * 0.5);
                    let far_from_base = if u_team == Team::Blue {
                        ux > anchor_x + 28.0
                    } else {
                        ux < anchor_x - 28.0
                    };
                    if far_from_base {
                        let safe_y = uy.clamp(LANE_TOP + 36.0, LANE_BOTTOM - 36.0);
                        (false, Some((anchor_x, safe_y, 1.30)))
                    } else if best_dist <= eff_rng_px {
                        (true, None)
                    } else {
                        (false, None)
                    }
                }
                ArmyOrder::Hold => {
                    // Hold: Troops do NOT move. They hold their ground in place.
                    // If an enemy enters weapon range, attack! Otherwise stay put.
                    if best_dist <= eff_rng_px {
                        (true, None)
                    } else {
                        (false, None)
                    }
                }
            };

            if should_attack {
                units[i].is_moving = false;
                let wp = units[i].walk_phase;
                units[i].walk_phase += (0.0 - wp.sin()) * (dt * 8.0).min(1.0);

                if units[i].atk_timer <= 0.0 {
                    units[i].atk_timer = eff_cd;
                    let anim_dur = (eff_cd * 0.78).clamp(0.18, 0.42);
                    units[i].atk_anim_duration = anim_dur;
                    units[i].atk_anim_timer = anim_dur;

                    let mut dmg = units[i].dmg;
                    let hit_num = units[i].shots_fired + 1;
                    match units[i].passive {
                        PassiveKey::FirstHit50 if hit_num == 1 => dmg = 50.0,
                        PassiveKey::FourthHit45 if hit_num % 4 == 0 => dmg = 45.0,
                        PassiveKey::ShotgunFalloff => {
                            let close = 8.5 * STUD_TO_PX;
                            dmg = if best_dist <= close { 35.0 } else { 15.0 };
                        }
                        _ => {}
                    }

                    // Relic on_compute_damage (Arcane Magnifier)
                    if u_team == Team::Blue && has_arcane_mag && units[i].shape == UnitShape::Diamond {
                        dmg = (dmg * 1.15).round();
                    }
                    // Mutator Consecrated Solar (-20% Triangle ranged DMG)
                    if consecrated_solar && units[i].shape == UnitShape::Triangle {
                        dmg = (dmg * 0.80).round().max(1.0);
                    }

                    units[i].shots_fired += 1;

                    let is_melee = units[i].def.rng_studs <= 7.5 || units[i].is_boss;
                    if is_melee {
                        units[i].lunge_offset = 7.0;
                        if let Some(t_uid) = best_unit_uid {
                            pending_attacks.push(PendingAttack::MeleeUnit {
                                attacker_uid: u_uid,
                                target_uid: t_uid,
                                dmg,
                            });
                        } else if let Some(sid) = best_struct_id {
                            pending_attacks.push(PendingAttack::MeleeStruct {
                                struct_id: sid,
                                dmg,
                            });
                        }

                        // Lifesteal Passive + Boss Lifesteal + Vampiric Bloodstone Relic
                        let mut leech = match units[i].passive {
                            PassiveKey::Lifesteal50 => 50.0,
                            PassiveKey::Lifesteal30 => 30.0,
                            _ => 0.0,
                        };
                        if let Some(bdef) = units[i].boss_def {
                            leech += bdef.lifesteal_on_hit;
                        }
                        let vamp_relic = if u_team == Team::Blue && has_vamp_stone { 10.0 } else { 0.0 };
                        let total_leech = leech + vamp_relic;
                        if total_leech > 0.0 {
                            units[i].hp = (units[i].hp + total_leech).min(units[i].max_hp);
                            melee_heals.push((ux, uy - u_rad - 7.0, total_leech, if leech > 0.0 { "LEECH" } else { "VAMP" }));
                        }
                        self.sound_queue.push(SoundEvent::SwordHit);
                    } else {
                        let color_rgb = if units[i].passive == PassiveKey::AcidPoison {
                            [74, 222, 128]
                        } else if units[i].shape == UnitShape::Diamond {
                            [192, 132, 252]
                        } else if u_team == Team::Blue {
                            [96, 165, 250]
                        } else {
                            [248, 113, 113]
                        };
                        pending_attacks.push(PendingAttack::SpawnProjectile(Projectile {
                            x: ux + aim_angle.cos() * (u_rad + 4.0),
                            y: uy + aim_angle.sin() * (u_rad + 4.0),
                            tx,
                            ty,
                            target_uid: best_unit_uid,
                            target_struct_id: best_struct_id,
                            attacker_uid: Some(u_uid),
                            team: u_team,
                            dmg,
                            speed: 440.0,
                            color_rgb,
                            radius: if units[i].shape == UnitShape::Diamond { 4.5 } else { 3.0 },
                            apply_poison: units[i].passive == PassiveKey::AcidPoison,
                        }));
                    }
                }
            } else if let Some((mx, my, order_spd_mult)) = move_target {
                // Separation steering toward tactical target via local spatial grid cells
                let mdx = mx - ux;
                let mdy = my - uy;
                let dist_c = mdx.hypot(mdy).max(1.0);
                let seek_x = mdx / dist_c;
                let seek_y = mdy / dist_c;

                let sep_rad = u_rad * 2.45;
                let (sep_x, sep_y) = ally_grid.accumulate_separation(u_uid, ux, uy, sep_rad);

                let comb_x = seek_x + sep_x * 0.85;
                let comb_y = seek_y + sep_y * 0.85;
                let comb_len = comb_x.hypot(comb_y).max(1.0);
                let dir_x = comb_x / comb_len;
                let dir_y = comb_y / comb_len;

                let surge_mult = if u_team == Team::Blue { blue_surge_mult } else { red_surge_mult };
                let slow_mult = units[i].slow_mult;
                let spd = units[i].spd * surge_mult * slow_mult * order_spd_mult;

                units[i].aim_angle = dir_y.atan2(dir_x);
                units[i].is_moving = true;
                units[i].walk_phase += dt * (units[i].spd * 0.22) * surge_mult * slow_mult * order_spd_mult;
                units[i].vx = dir_x;
                units[i].vy = dir_y;
                units[i].x = (ux + dir_x * spd * dt).clamp(70.0, WORLD_W - 70.0);
                units[i].y = (uy + dir_y * spd * dt).clamp(LANE_TOP + u_rad, LANE_BOTTOM - u_rad);
            } else {
                // Holding ground cleanly in formation
                units[i].is_moving = false;
                let wp = units[i].walk_phase;
                units[i].walk_phase += (0.0 - wp.sin()) * (dt * 8.0).min(1.0);
            }
        }

        for (hx, hy, amt, tag) in melee_heals {
            self.add_floating_text(hx, hy, format!("+{amt:.0} {tag}"), [251, 113, 133], 10.0);
        }

        for atk in pending_attacks {
            match atk {
                PendingAttack::MeleeUnit { attacker_uid, target_uid, dmg } => {
                    self.apply_damage_to_unit(target_uid, dmg, Some(attacker_uid), false, false);
                }
                PendingAttack::MeleeStruct { struct_id, dmg } => {
                    if let Some(st) = self.structures.iter_mut().find(|s| s.id == struct_id) {
                        if !st.is_invulnerable_override {
                            st.hp = (st.hp - dmg).max(0.0);
                            st.took_damage = true;
                        }
                    }
                }
                PendingAttack::SpawnProjectile(p) => {
                    self.projectiles.push(p);
                }
            }
        }
    }

    fn update_structures(&mut self, dt: f32) {
        for st in &mut self.structures {
            if st.hp <= 0.0 {
                continue;
            }
            st.atk_timer = (st.atk_timer - dt).max(0.0);
            if st.atk_timer <= 0.0 {
                let enemy_team = st.team.enemy();
                let mut nearest = None;
                let mut nearest_d_sq = st.rng * st.rng;
                for u in &self.units {
                    if u.hp > 0.0 && u.team == enemy_team {
                        let dx = u.x - st.x;
                        let dy = u.y - st.y;
                        let d_sq = dx * dx + dy * dy;
                        if d_sq <= nearest_d_sq {
                            nearest_d_sq = d_sq;
                            nearest = Some((u.uid, u.x, u.y));
                        }
                    }
                }
                if let Some((uid, tx, ty)) = nearest {
                    st.atk_timer = st.cd;
                    self.projectiles.push(Projectile {
                        x: st.x,
                        y: st.y - 10.0,
                        tx,
                        ty,
                        target_uid: Some(uid),
                        target_struct_id: None,
                        attacker_uid: None,
                        team: st.team,
                        dmg: st.dmg,
                        speed: 480.0,
                        color_rgb: if st.team == Team::Blue { [56, 189, 248] } else { [251, 113, 133] },
                        radius: 4.5,
                        apply_poison: false,
                    });
                }
            }
        }
    }

    fn update_projectiles(&mut self, dt: f32) {
        let grav_warp = self.active_mutator == Some(MutatorId::GravityDistortion);
        let lane_center_y = (LANE_TOP + LANE_BOTTOM) * 0.5;

        let mut i = self.projectiles.len();
        while i > 0 {
            i -= 1;
            if let Some(tuid) = self.projectiles[i].target_uid {
                if let Some(tidx) = self.find_alive_unit_idx(tuid) {
                    let u = &self.units[tidx];
                    self.projectiles[i].tx = u.x;
                    self.projectiles[i].ty = u.y;
                }
            }

            let dx = self.projectiles[i].tx - self.projectiles[i].x;
            let dy = self.projectiles[i].ty - self.projectiles[i].y;
            let dist_sq = dx * dx + dy * dy;
            let step = self.projectiles[i].speed * dt;
            let hit_thresh = step + 8.0;

            if dist_sq <= hit_thresh * hit_thresh {
                let p = self.projectiles.swap_remove(i);
                if let Some(sid) = p.target_struct_id {
                    let vuln = self
                        .structures
                        .iter()
                        .find(|s| s.id == sid)
                        .map(|s| self.is_structure_vulnerable(s))
                        .unwrap_or(false);
                    if vuln {
                        if let Some(st) = self.structures.iter_mut().find(|s| s.id == sid) {
                            st.hp = (st.hp - p.dmg).max(0.0);
                            st.took_damage = true;
                        }
                    }
                } else if let Some(tuid) = p.target_uid {
                    self.apply_damage_to_unit(tuid, p.dmg, p.attacker_uid, false, true);
                    if p.apply_poison {
                        if let Some(idx) = self.find_alive_unit_idx(tuid) {
                            let target = &mut self.units[idx];
                            target.poison_timer = 4.0;
                            target.poison_dps = 5.0;
                        }
                    }
                }
            } else {
                let dist = dist_sq.sqrt().max(0.0001);
                let p = &mut self.projectiles[i];
                p.x += (dx / dist) * step;
                let grav_curve = if grav_warp { (lane_center_y - p.y) * 1.4 * dt } else { 0.0 };
                p.y += (dy / dist) * step + grav_curve;
            }
        }
    }

    #[inline]
    fn find_alive_unit_idx(&self, uid: u64) -> Option<usize> {
        if let Ok(idx) = self.units.binary_search_by_key(&uid, |u| u.uid) {
            if self.units[idx].hp > 0.0 {
                return Some(idx);
            }
        }
        None
    }

    #[inline]
    fn push_minor_effect(&mut self, fx: VisualEffect) {
        if self.effects.len() < 160 {
            self.effects.push(fx);
        }
    }

    pub fn apply_damage_to_unit(
        &mut self,
        target_uid: u64,
        raw_amount: f32,
        attacker_uid: Option<u64>,
        is_spell: bool,
        is_ranged: bool,
    ) {
        let Some(t_idx) = self.find_alive_unit_idx(target_uid) else {
            return;
        };

        let target_team = self.units[t_idx].team;
        let target_shape = self.units[t_idx].shape;
        let target_x = self.units[t_idx].x;
        let target_y = self.units[t_idx].y;
        let target_r = self.units[t_idx].radius;

        if self.units[t_idx].death_immune_timer > 0.0 {
            self.add_floating_text(target_x, target_y - target_r - 6.0, "IMMUNE".into(), [192, 132, 252], 10.0);
            return;
        }
        if is_ranged && self.units[t_idx].ranged_immune_timer > 0.0 {
            self.add_floating_text(target_x, target_y - target_r - 6.0, "PHASE!".into(), [192, 132, 252], 10.0);
            return;
        }

        let mut final_dmg = raw_amount;

        // Flat Damage Block (Chitin Carapace Relic for Blue Squares + Gorgon Leviathan Carapace)
        if target_team == Team::Blue && target_shape == UnitShape::Square && self.has_relic(RelicId::ChitinCarapace) {
            final_dmg = (final_dmg - 3.0).max(1.0);
        }
        if let Some(bdef) = self.units[t_idx].boss_def {
            if bdef.flat_dmg_block > 0.0 {
                final_dmg = (final_dmg - bdef.flat_dmg_block).max(1.0);
            }
            if !is_ranged && !is_spell {
                final_dmg *= bdef.melee_dmg_taken_mult;
            } else if is_ranged {
                final_dmg *= bdef.ranged_dmg_taken_mult;
            }
            final_dmg *= bdef.all_dmg_taken_mult;
        }

        // Innate Damage Reduction Passives
        match self.units[t_idx].passive {
            PassiveKey::DmgRed25 | PassiveKey::EliteKnight => final_dmg *= 0.75,
            PassiveKey::EliteRiot => final_dmg *= 0.85,
            _ => {}
        }

        // Defensive Aura
        if self.units[t_idx].aura_def_bonus > 0.0 {
            final_dmg *= 1.0 - self.units[t_idx].aura_def_bonus;
        }
        // Void Deck Affinity Shred
        if self.units[t_idx].enemy_void_shred > 0.0 {
            final_dmg *= 1.0 + self.units[t_idx].enemy_void_shred;
        }

        final_dmg = final_dmg.round().max(1.0);

        // Nanite Emergency Injector Relic (once per life when Blue unit would drop <= 25% HP)
        if target_team == Team::Blue && self.has_relic(RelicId::NaniteInjector) && !self.units[t_idx].nanite_used {
            let max_hp = self.units[t_idx].max_hp;
            let rem = self.units[t_idx].hp - final_dmg;
            if rem <= max_hp * 0.25 {
                self.units[t_idx].nanite_used = true;
                let heal = (max_hp * 0.30).round();
                self.units[t_idx].hp = (self.units[t_idx].hp + heal).min(max_hp);
                self.add_floating_text(
                    target_x,
                    target_y - target_r - 12.0,
                    format!("+{heal:.0} NANITE!"),
                    [52, 211, 153],
                    11.0,
                );
                return;
            }
        }

        // Absorb Shield Barrier first (Sacred Phylactery + Colossus Prime Phase 2 Shield)
        if self.units[t_idx].shield_hp > 0.0 {
            let absorbed = final_dmg.min(self.units[t_idx].shield_hp);
            self.units[t_idx].shield_hp -= absorbed;
            final_dmg -= absorbed;
            if self.units[t_idx].shield_hp <= 0.0 {
                self.units[t_idx].shield_hp = 0.0;
                self.units[t_idx].shield_timer = None;
            }
        }

        if final_dmg > 0.0 {
            self.units[t_idx].hp -= final_dmg;
        }
        self.units[t_idx].hit_flash = 0.14;

        // Soul Drainer Death Immunity check (1.0s normal, 2.0s in Gravity Distortion mutator)
        if self.units[t_idx].hp <= 0.0
            && self.units[t_idx].passive == PassiveKey::DeathImmunity
            && !self.units[t_idx].death_immune_used
        {
            let dur = if self.active_mutator == Some(MutatorId::GravityDistortion) {
                2.0
            } else {
                1.0
            };
            self.units[t_idx].hp = 1.0;
            self.units[t_idx].death_immune_used = true;
            self.units[t_idx].death_immune_timer = dur;
            if target_team == Team::Blue {
                self.stats.blue_death_immunity_triggers += 1;
            }
            self.add_floating_text(
                target_x,
                target_y - target_r - 12.0,
                format!("DEFY DEATH ({dur:.1}s)!"),
                [216, 180, 254],
                12.0,
            );
            return;
        }

        // Singularity Heart Relic: Execute non-boss Red units <= 15% Max HP
        if target_team == Team::Red
            && !self.units[t_idx].is_boss
            && self.has_relic(RelicId::VoidSingularity)
            && self.units[t_idx].hp > 0.0
            && self.units[t_idx].hp / self.units[t_idx].max_hp <= 0.15
        {
            self.units[t_idx].hp = 0.0;
            self.add_floating_text(target_x, target_y - target_r - 12.0, "EXECUTE!".into(), [192, 132, 252], 12.0);
        }

        if raw_amount >= 30.0 || is_spell {
            self.add_floating_text(
                target_x,
                target_y - target_r - 6.0,
                format!("-{final_dmg:.0}"),
                if target_team == Team::Blue { [252, 165, 165] } else { [147, 197, 253] },
                if final_dmg >= 45.0 { 13.0 } else { 10.0 },
            );
        }

        // Death handling
        if self.units[t_idx].hp <= 0.0 {
            self.units[t_idx].hp = 0.0;
            let died_tier = self.units[t_idx].tier;
            if target_team == Team::Red {
                self.stats.enemy_kills += 1;
            } else {
                self.stats.blue_casualties += 1;
                if died_tier == UnitTier::A {
                    self.stats.blue_a_tier_deaths += 1;
                }
            }

            self.push_minor_effect(VisualEffect {
                kind: VisualEffectKind::DeathBurst,
                team: target_team,
                x: target_x,
                y: target_y,
                radius: target_r,
                color_rgb: if target_team == Team::Blue { [59, 130, 246] } else { [239, 68, 68] },
                life: 0.3,
                max_life: 0.3,
            });

            if let Some(a_uid) = attacker_uid {
                let mut bonus_heal = 0.0_f32;
                let mut float_pos = None;
                if let Some(a_idx) = self.find_alive_unit_idx(a_uid) {
                    let att = &mut self.units[a_idx];
                    if att.passive == PassiveKey::SoulHarvest {
                        bonus_heal += 10.0;
                    }
                    if att.team == Team::Blue && self.affinity.hellfire_kill_heal > 0.0 {
                        bonus_heal += self.affinity.hellfire_kill_heal;
                    }
                    if bonus_heal > 0.0 {
                        att.hp = (att.hp + bonus_heal).min(att.max_hp);
                        float_pos = Some((att.x, att.y - att.radius - 8.0));
                    }
                }
                if let Some((ax, ay)) = float_pos {
                    self.add_floating_text(ax, ay, format!("+{bonus_heal:.0} SOUL"), [244, 63, 94], 11.0);
                }
            }
        }
    }

    pub fn evaluate_and_commit_outcome(&self, save: &mut ConquestSave) -> MatchOutcomeSummary {
        let is_victory = self.winner == Some(Team::Blue);
        let p_t1 = self.structures.iter().find(|s| s.id == "p_t1").unwrap();
        let p_t2 = self.structures.iter().find(|s| s.id == "p_t2").unwrap();
        let p_base = self.structures.iter().find(|s| s.id == "p_base").unwrap();
        let e_t1 = self.structures.iter().find(|s| s.id == "e_t1").unwrap();
        let e_t2 = self.structures.iter().find(|s| s.id == "e_t2").unwrap();
        let e_base = self.structures.iter().find(|s| s.id == "e_base").unwrap();

        if let Some(mission) = self.mission {
            let mut details = Vec::new();
            let star1 = is_victory;
            let (star2, star3) = if !is_victory {
                (false, false)
            } else {
                let s2 = match mission.id {
                    "M1_1" => p_t1.hp >= p_t1.max_hp * 0.50,
                    "M1_2" | "M3_2" | "M6_4_FINAL_BOSS" => p_t1.hp > 0.0,
                    "M1_3" => self.player.income_lvl >= 5,
                    "M1_4_BOSS" => !p_t2.took_damage,
                    "M2_1" => !self.stats.blue_hit_by_enemy_q,
                    "M2_2" | "M6_2" => self.stats.q_casts >= 2,
                    "M2_3" => {
                        self.units
                            .iter()
                            .filter(|u| u.team == Team::Blue && u.hp > 0.0 && matches!(u.shape, UnitShape::Circle | UnitShape::Square))
                            .count()
                            >= 5
                    }
                    "M2_4_BOSS" | "M3_4_BOSS" => !p_base.took_damage,
                    "M3_1" => self.stats.enemy_kills >= 40,
                    "M3_3" => p_t1.hp > 0.0 && p_t2.hp > 0.0 && p_base.hp > 0.0,
                    "M4_1" => p_t1.hp >= p_t1.max_hp * 0.60,
                    "M4_2" => self.stats.blue_a_tier_deaths == 0,
                    "M4_3" => self.stats.blue_death_immunity_triggers >= 3,
                    "M4_4_BOSS" => p_t2.hp >= p_t2.max_hp * 0.50,
                    "M5_1" => p_t1.hp >= p_t1.max_hp * 0.75,
                    "M5_2" => self.stats.enemy_kills >= 50,
                    "M5_3" => self.player.income_lvl >= 6,
                    "M5_4_BOSS" => p_base.hp >= 1200.0,
                    "M6_1" => self.stats.blue_casualties < 10,
                    "M6_3" => e_t1.hp <= 0.0 && e_t2.hp <= 0.0 && e_base.hp <= 0.0,
                    _ => p_t1.hp > 0.0,
                };

                let s3 = match mission.id {
                    "M1_2" => self.stats.enemy_kills >= 35,
                    "M1_3" => self.stats.triangles_spawned == 0,
                    "M2_3" => self.player.income_lvl >= 6,
                    "M3_2" => self.stats.max_simultaneous_poisoned_blue <= 2,
                    "M3_3" => self.stats.e_casts >= 3,
                    "M4_3" => p_t1.hp > 0.0 && p_t2.hp > 0.0,
                    "M5_2" => p_t1.hp > 0.0 && p_t2.hp > 0.0 && p_base.hp > 0.0,
                    "M6_3" => self.player.income_lvl >= 7,
                    _ => self.time <= mission.mastery_time_limit,
                };
                (s2, s3)
            };

            details.push(StarAuditDetail { star: 1, earned: star1, desc: mission.star_criteria[0].desc.to_string() });
            details.push(StarAuditDetail { star: 2, earned: star2, desc: mission.star_criteria[1].desc.to_string() });
            details.push(StarAuditDetail { star: 3, earned: star3, desc: mission.star_criteria[2].desc.to_string() });

            let stars_earned = (star1 as u8) + (star2 as u8) + (star3 as u8);
            let mut unlocked_blueprints = Vec::new();
            let mut unlocked_trophy = None;
            let mut gold_bounty = 0;

            let before_bps = save.unlocked_blueprint_ids.clone();
            let before_relics = save.unlocked_relic_ids.clone();

            let entry = save
                .missions
                .entry(mission.id.to_string())
                .or_insert(crate::save::MissionRecord {
                    stars: 0,
                    best_time_seconds: self.time,
                    times_attempted: 0,
                });
            entry.times_attempted += 1;

            if is_victory {
                if stars_earned > entry.stars {
                    entry.stars = stars_earned;
                    entry.best_time_seconds = self.time;
                } else if stars_earned == entry.stars && self.time < entry.best_time_seconds {
                    entry.best_time_seconds = self.time;
                }

                for &b in mission.reward_blueprints {
                    save.unlock_blueprint(b);
                }
                if mission.unlocks_tier2 {
                    save.active_features.tier2 = true;
                }
                if mission.unlocks_tier3 {
                    save.active_features.tier3 = true;
                }
                if let Some(tr) = mission.trophy {
                    let tr_s = tr.to_string();
                    if !save.unlocked_trophies.contains(&tr_s) {
                        save.unlocked_trophies.push(tr_s.clone());
                        unlocked_trophy = Some(tr_s);
                    }
                }
                gold_bounty = mission.gold_bounty;
                save.account_gold_vault += gold_bounty;
            }

            save.recompute_integrity();

            for bp in &save.unlocked_blueprint_ids {
                if !before_bps.contains(bp) {
                    unlocked_blueprints.push(bp.clone());
                }
            }
            let newly_unlocked_relics: Vec<String> = save
                .unlocked_relic_ids
                .iter()
                .filter(|r| !before_relics.contains(*r))
                .cloned()
                .collect();

            MatchOutcomeSummary {
                is_campaign: true,
                mission_id: Some(mission.id.to_string()),
                mission_name: mission.name.to_string(),
                is_victory,
                elapsed_secs: self.time,
                stars_earned,
                star_details: details,
                gold_bounty,
                unlocked_blueprints,
                unlocked_trophy,
                newly_unlocked_relics,
            }
        } else {
            if is_victory {
                save.skirmish_wins += 1;
                save.account_gold_vault += 75;
                save.recompute_integrity();
            }
            MatchOutcomeSummary {
                is_campaign: false,
                mission_id: None,
                mission_name: format!(
                    "Skirmish vs {} ({})",
                    self.ai.faction.id.display_name(),
                    self.ai_difficulty.name()
                ),
                is_victory,
                elapsed_secs: self.time,
                stars_earned: if is_victory { 3 } else { 0 },
                star_details: Vec::new(),
                gold_bounty: if is_victory { 75 } else { 0 },
                unlocked_blueprints: Vec::new(),
                unlocked_trophy: None,
                newly_unlocked_relics: Vec::new(),
            }
        }
    }

    pub fn set_banner(&mut self, msg: String) {
        self.banner_text = msg;
    }

    pub fn add_floating_text(&mut self, x: f32, y: f32, text: String, color_rgb: [u8; 3], size: f32) {
        let cap = if size >= 12.0 { 96 } else { 64 };
        if self.floating_texts.len() >= cap {
            return;
        }
        self.floating_texts.push(FloatingText {
            x,
            y,
            text,
            color_rgb,
            size,
            life: 0.85,
        });
    }
}

fn resolve_player_deck(ids: &[String; 6]) -> [Option<&'static UnitDef>; 6] {
    [
        get_unit(&ids[0]),
        get_unit(&ids[1]),
        get_unit(&ids[2]),
        get_unit(&ids[3]),
        get_unit(&ids[4]),
        get_unit(&ids[5]),
    ]
}

fn resolve_ai_deck(ids: &[String; 6], fallback_faction: FactionId) -> [Option<&'static UnitDef>; 6] {
    let fallback_ids = default_faction_deck(fallback_faction);
    [
        Some(get_unit(&ids[0]).unwrap_or_else(|| get_unit(&fallback_ids[0]).unwrap())),
        Some(get_unit(&ids[1]).unwrap_or_else(|| get_unit(&fallback_ids[1]).unwrap())),
        Some(get_unit(&ids[2]).unwrap_or_else(|| get_unit(&fallback_ids[2]).unwrap())),
        Some(get_unit(&ids[3]).unwrap_or_else(|| get_unit(&fallback_ids[3]).unwrap())),
        Some(get_unit(&ids[4]).unwrap_or_else(|| get_unit(&fallback_ids[4]).unwrap())),
        Some(get_unit(&ids[5]).unwrap_or_else(|| get_unit(&fallback_ids[5]).unwrap())),
    ]
}
