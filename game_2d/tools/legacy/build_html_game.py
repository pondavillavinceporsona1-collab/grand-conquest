import os

BASE_DIR = r"C:\Users\Vince\.gemini\antigravity\scratch\epic_battle_2d"
ORIG_HTML = r"C:\Users\Vince\.gemini\antigravity\brain\088fd393-844c-4bb8-8089-3876457ef42b\.user_uploaded\media_1794192931801_7cd39f2f.html"
BUNDLE_JS = os.path.join(BASE_DIR, "epic_battle_2d_assets_bundle.js")

with open(ORIG_HTML, "r", encoding="utf-8") as f:
    html = f.read()

# 1. Update Title & Header Branding to reflect 2D Animated Soldiers + Limb Rigging
html = html.replace(
    "<title>EPIC BATTLE! v2.5.0 — 2D Geometric Tactical Auto-Battler</title>",
    "<title>EPIC BATTLE! v2.5.0 — 2D Skeletal Limb-Animated Tactical Auto-Battler</title>"
)
html = html.replace(
    "EPIC BATTLE! 2D SHAPE WARFARE",
    "EPIC BATTLE! 2D ANIMATED WARFARE"
)
html = html.replace(
    "Blue Shapes (You) vs. Red Shapes (AI) • Destroy Outer Turret 1 &rarr; Inner Turret 2 &rarr; 1,500 HP Base Tower",
    "54 Limb-Rigged 2D Champions • Blue Army (You) vs. Red Army (AI) • Sequential Turret Rule: T1 &rarr; T2 &rarr; 1,500 HP Base"
)

# 2. Add a Render Mode toggle button (60FPS 6-Limb Rig / 8-Frame Colab Sheet / Legacy Shapes) right next to Tags toggle
target_buttons = """        <!-- Labels Toggle -->
        <button id="btnToggleLabels" class="px-2.5 py-2 rounded-xl bg-slate-900 hover:bg-slate-800 border border-slate-700 text-xs font-semibold text-slate-300 cursor-pointer" title="Toggle unit name tags above shapes">
          Tags: ON
        </button>"""

replacement_buttons = """        <!-- Render Mode Toggle: 60FPS 6-Limb Skeletal Rig vs 8-Frame Sheet vs Geometric Shapes -->
        <button id="btnRenderMode" class="px-2.5 py-2 rounded-xl bg-indigo-500/20 hover:bg-indigo-500/30 border border-indigo-400/50 text-xs font-bold text-indigo-200 cursor-pointer" title="Switch between 60FPS 6-Limb Procedural Rig, 8-Frame Sprite Sheet, and Geometric Shapes">
          Rig: 60FPS Limbs
        </button>

        <!-- Labels Toggle -->
        <button id="btnToggleLabels" class="px-2.5 py-2 rounded-xl bg-slate-900 hover:bg-slate-800 border border-slate-700 text-xs font-semibold text-slate-300 cursor-pointer" title="Toggle unit name tags above soldiers">
          Tags: ON
        </button>"""

assert target_buttons in html, "target_buttons not found"
html = html.replace(target_buttons, replacement_buttons, 1)

# 3. Inject <script src="epic_battle_2d_assets_bundle.js"></script> right before the main <script>
html = html.replace(
    "  <script>\n    // =========================================================================\n    // 1. MASTER FACTION",
    '  <script src="epic_battle_2d_assets_bundle.js"></script>\n  <script>\n    // =========================================================================\n    // 1. MASTER FACTION',
    1
)

# 4. Add UNIT_SPRITE_KEYS mapping and 2D Skeletal Rig & Sprite Sheet Engine right after FACTIONS
target_after_factions = """    const BOT_DIFFICULTIES = {"""

sprite_engine_code = """    // =========================================================================
    // 1B. 54-UNIT 2D SPRITE MAPPING & 6-LIMB SKELETAL ANIMATION ENGINE
    // =========================================================================
    const UNIT_SPRITE_KEYS = {
      // Solaris Empire (Kingdom)
      sol_1: 'solaris_footman',
      sol_2: 'solaris_archer',
      sol_3: 'solaris_shield_man',
      sol_4: 'solaris_knight',
      sol_5: 'solaris_mage',
      sol_6: 'solaris_priest',
      sol_7: 'solaris_elite_knight',
      sol_8: 'solaris_elite_mage',
      sol_9: 'solaris_high_priest',
      // Aegis Coalition (Republic)
      aeg_1: 'aegis_riot_trooper',
      aeg_2: 'aegis_police',
      aeg_3: 'aegis_medic_bot',
      aeg_4: 'aegis_rifleman',
      aeg_5: 'aegis_riot_shield',
      aeg_6: 'aegis_grenade_thrower',
      aeg_7: 'aegis_elite_riot',
      aeg_8: 'aegis_shotgunner',
      aeg_9: 'aegis_heavy_gunner',
      // Hive Mind (Alien)
      hiv_1: 'hive_alien_grunt',
      hiv_2: 'hive_plasma_shooter',
      hiv_3: 'hive_alien_guard',
      hiv_4: 'hive_mutated_warrior',
      hiv_5: 'hive_plasma_specialist',
      hiv_6: 'hive_acid_spitter',
      hiv_7: 'hive_alien_juggernaut',
      hiv_8: 'hive_plasma_overlord',
      hiv_9: 'hive_hive_commander',
      // Hellfire Legion (Demon)
      hel_1: 'hellfire_imp',
      hel_2: 'hellfire_hell_hound',
      hel_3: 'hellfire_demon_soldier',
      hel_4: 'hellfire_blood_knight',
      hel_5: 'hellfire_flame_caster',
      hel_6: 'hellfire_soul_reaper',
      hel_7: 'hellfire_demon_brute',
      hel_8: 'hellfire_infernal_archmage',
      hel_9: 'hellfire_demon_lord',
      // Abyssal Void (Void)
      voi_1: 'abyssal_shadowling',
      voi_2: 'abyssal_void_archer',
      voi_3: 'abyssal_dark_sentinel',
      voi_4: 'abyssal_void_stalker',
      voi_5: 'abyssal_abyss_mage',
      voi_6: 'abyssal_soul_drainer',
      voi_7: 'abyssal_void_reaper',
      voi_8: 'abyssal_abyssal_sorcerer',
      voi_9: 'abyssal_void_warden',
      // Mechanica Corps (Mechanica)
      mec_1: 'mechanica_combat_bot',
      mec_2: 'mechanica_assault_drone',
      mec_3: 'mechanica_shield_robot',
      mec_4: 'mechanica_warbot',
      mec_5: 'mechanica_plasma_robot',
      mec_6: 'mechanica_repair_bot',
      mec_7: 'mechanica_destroyer_mech',
      mec_8: 'mechanica_heavy_artillery_bot',
      mec_9: 'mechanica_cybernetic_commander'
    };

    // Attach spriteKey to every unit definition
    Object.values(FACTIONS).forEach((fac) => {
      fac.units.forEach((u) => {
        u.spriteKey = UNIT_SPRITE_KEYS[u.id];
      });
    });

    // Render mode: 'limbs60' (60FPS 6-part skeletal rig), 'sheet8' (8-frame Colab sprite sheet), or 'shapes'
    let unitRenderMode = 'limbs60';

    const RIG_PART_ORDER = (window.EB2D_PART_ORDER) || ['back_arm', 'left_leg', 'right_leg', 'torso', 'head', 'front_arm'];
    const RIG_PIVOTS = (window.EB2D_PIVOTS) || {};
    const PORTRAIT_URLS = (window.EB2D_PORTRAITS) || {};
    const RIG_STRIP_URLS = (window.EB2D_RIG_STRIPS) || {};

    const loadedRigStrips = {};
    const loadedPortraits = {};
    const generated8FrameSheets = {};

    // 8-frame Colab keyframe poses (OpenCV counter-clockwise degrees -> Canvas clockwise radians = -deg * PI / 180)
    const COLAB_8_KEYFRAMES = [
      // Frame 0: Walk Pass 1 (Left leg forward, Right leg back, Arms counter-swing)
      { left_leg: [18, -1, -1], right_leg: [-18, 1, -1], back_arm: [-15, 0, 0], front_arm: [15, 0, 0], torso: [-2, 0, -1], head: [-2, 0, -2] },
      // Frame 1: Walk Contact
      { left_leg: [6, 0, 0], right_leg: [-6, 0, 0], back_arm: [-5, 0, 1], front_arm: [5, 0, 1], torso: [0, 0, 1], head: [0, 0, 1] },
      // Frame 2: Walk Pass 2 (Right leg forward, Left leg back, Arms swap swing)
      { left_leg: [-18, 1, -1], right_leg: [18, -1, -1], back_arm: [15, 0, 0], front_arm: [-15, 0, 0], torso: [2, 0, -1], head: [2, 0, -2] },
      // Frame 3: Walk Contact 2
      { left_leg: [-6, 0, 0], right_leg: [6, 0, 0], back_arm: [5, 0, 1], front_arm: [-5, 0, 1], torso: [0, 0, 1], head: [0, 0, 1] },
      // Frame 4: Attack Wind-Up (Raise weapon arms high, brace legs)
      { left_leg: [-12, -2, 0], right_leg: [14, -1, 0], back_arm: [-22, -2, -2], front_arm: [26, -2, -3], torso: [6, -2, 0], head: [5, -2, 0] },
      // Frame 5: Attack Strike! (Slam weapon arm down & forward, lunge torso)
      { left_leg: [20, 3, 0], right_leg: [-18, 4, -1], back_arm: [26, 4, 2], front_arm: [-30, 5, 3], torso: [-8, 4, 1], head: [-6, 4, 1] },
      // Frame 6: Attack Follow-through
      { left_leg: [10, 2, 0], right_leg: [-10, 2, 0], back_arm: [13, 2, 1], front_arm: [-15, 3, 2], torso: [-4, 2, 1], head: [-3, 2, 1] },
      // Frame 7: Return to Combat Ready
      { left_leg: [0, 0, 0], right_leg: [0, 0, 0], back_arm: [0, 0, 0], front_arm: [0, 0, 0], torso: [0, 0, 0], head: [0, 0, 0] }
    ];

    function buildOffscreen8FrameSheet(spriteKey, stripImg) {
      const piv = RIG_PIVOTS[spriteKey] || {
        head: [64, 58], torso: [64, 72], back_arm: [46, 56],
        front_arm: [82, 56], left_leg: [54, 88], right_leg: [74, 88]
      };
      const off = document.createElement('canvas');
      off.width = 128 * 8;
      off.height = 128;
      const octx = off.getContext('2d');

      COLAB_8_KEYFRAMES.forEach((pose, fIdx) => {
        const frameOx = fIdx * 128;
        RIG_PART_ORDER.forEach((partName, pIdx) => {
          const [degCCW, dx, dy] = pose[partName];
          const [px, py] = piv[partName] || [64, 64];
          const rad = (-degCCW * Math.PI) / 180;
          octx.save();
          octx.translate(frameOx + px + dx, py + dy);
          octx.rotate(rad);
          octx.drawImage(stripImg, pIdx * 128, 0, 128, 128, -px, -py, 128, 128);
          octx.restore();
        });
      });
      generated8FrameSheets[spriteKey] = off;
    }

    function preloadAll2DSprites() {
      Object.values(UNIT_SPRITE_KEYS).forEach((key) => {
        if (PORTRAIT_URLS[key]) {
          const pImg = new Image();
          pImg.src = PORTRAIT_URLS[key];
          loadedPortraits[key] = pImg;
        }
        if (RIG_STRIP_URLS[key]) {
          const sImg = new Image();
          sImg.onload = () => {
            buildOffscreen8FrameSheet(key, sImg);
          };
          sImg.src = RIG_STRIP_URLS[key];
          loadedRigStrips[key] = sImg;
        }
      });
    }
    preloadAll2DSprites();

    function getUnitPortraitHtml(unit, sizeClass = 'w-8 h-8') {
      const url = PORTRAIT_URLS[unit.spriteKey] || `assets/static_all_54/${unit.spriteKey}.png`;
      return `<img src="${url}" alt="${unit.name}" class="${sizeClass} object-contain drop-shadow-[0_2px_4px_rgba(0,0,0,0.8)] shrink-0" />`;
    }

    const BOT_DIFFICULTIES = {"""

assert target_after_factions in html, "target_after_factions not found"
html = html.replace(target_after_factions, sprite_engine_code, 1)

# 5. Add walkPhase, idlePhase, atkAnimTimer, atkAnimDuration, isMoving to spawned unit entities
target_entity_props = """        lungeOffset: 0,
        hitFlash: 0
      };"""

replacement_entity_props = """        lungeOffset: 0,
        hitFlash: 0,
        walkPhase: Math.random() * Math.PI * 2,
        idlePhase: Math.random() * Math.PI * 2,
        atkAnimTimer: 0,
        atkAnimDuration: 0.38,
        isMoving: true
      };"""

assert target_entity_props in html, "target_entity_props not found"
html = html.replace(target_entity_props, replacement_entity_props, 1)

# 6. Update updateSimulation(dt) to advance walkPhase, idlePhase, and atkAnimTimer
target_sim_timers = """        if (u.hitFlash > 0) u.hitFlash = Math.max(0, u.hitFlash - dt);
        if (u.lungeOffset > 0) u.lungeOffset = Math.max(0, u.lungeOffset - dt * 28);
        if (u.deathImmuneTimer > 0) u.deathImmuneTimer = Math.max(0, u.deathImmuneTimer - dt);"""

replacement_sim_timers = """        if (u.hitFlash > 0) u.hitFlash = Math.max(0, u.hitFlash - dt);
        if (u.lungeOffset > 0) u.lungeOffset = Math.max(0, u.lungeOffset - dt * 28);
        if (u.deathImmuneTimer > 0) u.deathImmuneTimer = Math.max(0, u.deathImmuneTimer - dt);
        if (u.atkAnimTimer > 0) u.atkAnimTimer = Math.max(0, u.atkAnimTimer - dt);
        u.idlePhase += dt * 3.4;"""

assert target_sim_timers in html, "target_sim_timers not found"
html = html.replace(target_sim_timers, replacement_sim_timers, 1)

# Trigger atkAnimTimer when unit attacks, and advance walkPhase when moving
target_atk_trigger = """        // If within attack range, stop and fire/strike!
        if (bestDist <= effectiveRangePx) {
          if (u.atkTimer <= 0) {
            u.atkTimer = effectiveCd;
            const hitDmg = computeUnitAttackDamage(u, bestDist);
            u.shotsFired += 1;"""

replacement_atk_trigger = """        // If within attack range, stop and fire/strike!
        if (bestDist <= effectiveRangePx) {
          u.isMoving = false;
          // Smoothly relax walk legs toward stance when stopped
          u.walkPhase += (0 - Math.sin(u.walkPhase)) * Math.min(1, dt * 8);
          if (u.atkTimer <= 0) {
            u.atkTimer = effectiveCd;
            u.atkAnimDuration = Math.min(0.42, Math.max(0.22, effectiveCd * 0.78));
            u.atkAnimTimer = u.atkAnimDuration;
            const hitDmg = computeUnitAttackDamage(u, bestDist);
            u.shotsFired += 1;"""

assert target_atk_trigger in html, "target_atk_trigger not found"
html = html.replace(target_atk_trigger, replacement_atk_trigger, 1)

target_move_step = """          u.x += combX * u.spd * surgeMult * dt;
          u.y += combY * u.spd * surgeMult * dt;"""

replacement_move_step = """          u.isMoving = true;
          u.walkPhase += dt * (u.spd * 0.22) * surgeMult;
          u.x += combX * u.spd * surgeMult * dt;
          u.y += combY * u.spd * surgeMult * dt;"""

assert target_move_step in html, "target_move_step not found"
html = html.replace(target_move_step, replacement_move_step, 1)

# 7. Add drawAnimated2DSoldier(ctx, u, lx, ly, isBlue, strokeCol) and call it in renderBattlefield()
target_draw_call = """        // Render the Soldier's Geometric Role Shape
        drawGeometricUnitShape(ctx, u.shape, lx, ly, u.radius, u.aimAngle, fillCol, strokeCol, u.tier);

        // Health Bar above unit
        const barW = Math.max(22, u.radius * 2.1);
        const barH = 4;
        const barY = ly - u.radius - 10;"""

replacement_draw_call = """        // Render Soldier: 60FPS 6-Limb Skeletal Rig, 8-Frame Sprite Sheet, or Legacy Geometric Shape
        const spriteKey = u.def.spriteKey;
        const stripImg = loadedRigStrips[spriteKey];
        const canDraw2D = unitRenderMode !== 'shapes' && stripImg && stripImg.complete && stripImg.naturalWidth > 0;

        if (canDraw2D) {
          drawAnimated2DSoldier(ctx, u, lx, ly, isBlue, strokeCol, stripImg);
        } else {
          drawGeometricUnitShape(ctx, u.shape, lx, ly, u.radius, u.aimAngle, fillCol, strokeCol, u.tier);
        }

        // Health Bar above unit (positioned cleanly above 2D helmet/head)
        const barW = Math.max(26, u.radius * 2.35);
        const barH = 4.5;
        const barY = canDraw2D ? ly - u.radius * 2.15 - 8 : ly - u.radius - 10;"""

assert target_draw_call in html, "target_draw_call not found"
html = html.replace(target_draw_call, replacement_draw_call, 1)

# Now insert the drawAnimated2DSoldier function right before renderBattlefield()
target_before_render = """    function renderBattlefield() {"""

draw_soldier_fn = """    function computeProceduralLimbPose(u) {
      // Returns per-part [degCCW, dx, dy] matching the Colab coordinate convention
      if (u.atkAnimTimer > 0 && u.atkAnimDuration > 0) {
        const p = 1 - u.atkAnimTimer / u.atkAnimDuration; // 0.0 -> 1.0
        if (p < 0.35) {
          // Wind-up phase (0.0 -> 0.35): raise weapon arms high, brace legs
          const t = p / 0.35;
          return {
            left_leg: [-14 * t, -2 * t, 0],
            right_leg: [16 * t, -1 * t, 0],
            back_arm: [-24 * t, -2 * t, -2 * t],
            front_arm: [28 * t, -2 * t, -3 * t],
            torso: [7 * t, -2 * t, 0],
            head: [5 * t, -2 * t, 0]
          };
        } else if (p < 0.68) {
          // Strike phase (0.35 -> 0.68): fast overhead chop / thrust downward & forward!
          const t = (p - 0.35) / 0.33;
          const lerp = (a, b) => a + (b - a) * t;
          return {
            left_leg: [lerp(-14, 22), lerp(-2, 3), 0],
            right_leg: [lerp(16, -20), lerp(-1, 4), lerp(0, -1)],
            back_arm: [lerp(-24, 28), lerp(-2, 4), lerp(-2, 2)],
            front_arm: [lerp(28, -32), lerp(-2, 6), lerp(-3, 3)],
            torso: [lerp(7, -9), lerp(-2, 4), lerp(0, 1)],
            head: [lerp(5, -7), lerp(-2, 4), lerp(0, 1)]
          };
        } else {
          // Recovery phase (0.68 -> 1.0): return smoothly to combat ready
          const t = 1 - (p - 0.68) / 0.32;
          return {
            left_leg: [22 * t, 3 * t, 0],
            right_leg: [-20 * t, 4 * t, -1 * t],
            back_arm: [28 * t, 4 * t, 2 * t],
            front_arm: [-32 * t, 6 * t, 3 * t],
            torso: [-9 * t, 4 * t, 1 * t],
            head: [-7 * t, 4 * t, 1 * t]
          };
        }
      }

      if (u.isMoving) {
        // Scissor walk cycle: legs swing in opposite sine arcs, arms counter-swing, head & torso bob
        const s = Math.sin(u.walkPhase);
        const b = Math.abs(Math.cos(u.walkPhase));
        return {
          left_leg: [s * 20, -s * 1.5, -b * 1.5],
          right_leg: [-s * 20, s * 1.5, -b * 1.5],
          back_arm: [-s * 16, 0, b * 1.0],
          front_arm: [s * 16, 0, b * 1.0],
          torso: [-s * 2.5, 0, -b * 1.8],
          head: [-s * 2.0, 0, -b * 2.4]
        };
      }

      // Combat idle breathing stance
      const br = Math.sin(u.idlePhase);
      return {
        left_leg: [0, 0, 0],
        right_leg: [0, 0, 0],
        back_arm: [-br * 3.5, 0, br * 0.6],
        front_arm: [br * 4.5, 0, br * 0.6],
        torso: [0, 0, br * 0.8],
        head: [br * 1.5, 0, br * 1.2]
      };
    }

    function drawAnimated2DSoldier(ctx, u, lx, ly, isBlue, strokeCol, stripImg) {
      const spriteKey = u.def.spriteKey;
      // Scale sprite by Tier (C=54px, B=64px, A=78px)
      const drawSize = u.tier === 'A' ? 78 : u.tier === 'B' ? 64 : 54;
      const scale = drawSize / 128;
      const facing = Math.cos(u.aimAngle) >= -0.15 ? 1 : -1;

      // 1. Tactical Team Ground Pedestal Ring under feet
      const footY = ly + u.radius * 0.65;
      ctx.save();
      ctx.translate(lx, footY);
      ctx.scale(1, 0.46);
      ctx.fillStyle = isBlue ? 'rgba(37, 99, 235, 0.42)' : 'rgba(225, 29, 72, 0.42)';
      ctx.strokeStyle = u.tier === 'A' ? '#fde047' : isBlue ? '#60a5fa' : '#fb7185';
      ctx.lineWidth = u.tier === 'A' ? 3.2 : 2.2;
      ctx.beginPath();
      ctx.arc(0, 0, u.radius * 1.28, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();
      ctx.restore();

      // 2. Draw either 60FPS 6-Limb Procedural Rig OR 8-Frame Colab Sheet
      ctx.save();
      // Anchor point (64, 92) of the 128x128 canvas maps to (lx, ly)
      ctx.translate(lx, ly);
      ctx.scale(facing * scale, scale);

      if (u.hitFlash > 0) {
        ctx.filter = 'brightness(1.85) contrast(1.15)';
      }

      if (unitRenderMode === 'sheet8' && generated8FrameSheets[spriteKey]) {
        const sheetCanvas = generated8FrameSheets[spriteKey];
        let frameIdx = 7;
        if (u.atkAnimTimer > 0 && u.atkAnimDuration > 0) {
          const p = 1 - u.atkAnimTimer / u.atkAnimDuration;
          frameIdx = 4 + Math.min(3, Math.floor(p * 4));
        } else if (u.isMoving) {
          const normWalk = ((u.walkPhase % (Math.PI * 2)) + Math.PI * 2) % (Math.PI * 2);
          frameIdx = Math.floor((normWalk / (Math.PI * 2)) * 4) % 4;
        }
        ctx.drawImage(sheetCanvas, frameIdx * 128, 0, 128, 128, -64, -92, 128, 128);
      } else {
        // 60FPS Live 6-Limb Skeletal Rig (RigBuilder.PoseRig style!)
        const piv = RIG_PIVOTS[spriteKey] || {
          head: [64, 58], torso: [64, 72], back_arm: [46, 56],
          front_arm: [82, 56], left_leg: [54, 88], right_leg: [74, 88]
        };
        const pose = computeProceduralLimbPose(u);

        for (let pIdx = 0; pIdx < RIG_PART_ORDER.length; pIdx++) {
          const partName = RIG_PART_ORDER[pIdx];
          const [degCCW, dx, dy] = pose[partName];
          const [px, py] = piv[partName] || [64, 64];
          const rad = (-degCCW * Math.PI) / 180;

          ctx.save();
          ctx.translate(px - 64 + dx, py - 92 + dy);
          ctx.rotate(rad);
          ctx.drawImage(stripImg, pIdx * 128, 0, 128, 128, -px, -py, 128, 128);
          ctx.restore();
        }
      }

      // 3. Melee Slash Crescent on Strike Frame
      if (u.atkAnimTimer > 0 && u.def.rng <= 7.5) {
        const p = 1 - u.atkAnimTimer / u.atkAnimDuration;
        if (p >= 0.32 && p <= 0.68) {
          ctx.save();
          ctx.strokeStyle = isBlue ? '#93c5fd' : '#fda4af';
          ctx.lineWidth = 4.5;
          ctx.globalAlpha = 1 - Math.abs(p - 0.5) / 0.2;
          ctx.beginPath();
          ctx.arc(10, -18, 36, -Math.PI * 0.38, Math.PI * 0.32);
          ctx.stroke();
          ctx.restore();
        }
      }

      ctx.restore();
    }

    function renderBattlefield() {"""

assert target_before_render in html, "target_before_render not found"
html = html.replace(target_before_render, draw_soldier_fn, 1)

# 8. Update the 6-Card Bottom Bar & [M] War Grimoire Modal to show 2D Unit Portraits!
target_card_inner = """            <div class="flex items-center justify-between gap-1">
              <kbd class="px-1.5 py-0.2 rounded bg-slate-800 text-amber-300 font-mono-code text-[10px] font-black">[${idx + 1}]</kbd>
              <div class="flex items-center gap-1">
                ${getShapeSvgMarkup(unit.shape)}
                <span class="text-[9px] font-bold px-1.5 py-0.2 rounded ${tierBadgeColor}">${unit.tier}</span>
              </div>
            </div>
            <div class="text-xs font-black text-white truncate mt-1">${unit.name}</div>
            <div class="text-[9px] text-slate-400 truncate">${unit.passiveDesc}</div>"""

replacement_card_inner = """            <div class="flex items-center justify-between gap-1">
              <kbd class="px-1.5 py-0.2 rounded bg-slate-800 text-amber-300 font-mono-code text-[10px] font-black">[${idx + 1}]</kbd>
              <div class="flex items-center gap-1">
                ${getShapeSvgMarkup(unit.shape)}
                <span class="text-[9px] font-bold px-1.5 py-0.2 rounded ${tierBadgeColor}">${unit.tier}</span>
              </div>
            </div>
            <div class="flex items-center gap-1.5 mt-1">
              ${getUnitPortraitHtml(unit, 'w-9 h-9')}
              <div class="min-w-0 flex-1">
                <div class="text-xs font-black text-white truncate">${unit.name}</div>
                <div class="text-[9px] text-slate-400 truncate">${unit.passiveDesc}</div>
              </div>
            </div>"""

assert target_card_inner in html, "target_card_inner not found"
html = html.replace(target_card_inner, replacement_card_inner, 1)

target_modal_card_inner = """              <div class="flex items-center gap-2.5">
                ${getShapeSvgMarkup(u.shape)}
                <div>
                  <div class="text-xs font-black text-white">${u.name} <span class="text-amber-400 font-mono-code">(${u.cost}G)</span></div>
                  <div class="text-[10px] text-slate-300">${u.hp} HP • ${u.dmg} DMG • ${u.passiveDesc}</div>
                </div>
              </div>"""

replacement_modal_card_inner = """              <div class="flex items-center gap-2.5">
                ${getUnitPortraitHtml(u, 'w-10 h-10')}
                <div>
                  <div class="flex items-center gap-1.5">
                    ${getShapeSvgMarkup(u.shape)}
                    <span class="text-xs font-black text-white">${u.name} <span class="text-amber-400 font-mono-code">(${u.cost}G)</span></span>
                  </div>
                  <div class="text-[10px] text-slate-300">${u.hp} HP • ${u.dmg} DMG • ${u.passiveDesc}</div>
                </div>
              </div>"""

assert target_modal_card_inner in html, "target_modal_card_inner not found"
html = html.replace(target_modal_card_inner, replacement_modal_card_inner, 1)

# 9. Bind btnRenderMode button click handler
target_label_btn_binding = """    document.getElementById('btnToggleLabels').onclick = () => {
      showUnitLabels = !showUnitLabels;
      document.getElementById('btnToggleLabels').textContent = `Tags: ${showUnitLabels ? 'ON' : 'OFF'}`;
    };"""

replacement_label_btn_binding = """    document.getElementById('btnRenderMode').onclick = () => {
      if (unitRenderMode === 'limbs60') {
        unitRenderMode = 'sheet8';
        document.getElementById('btnRenderMode').textContent = 'Rig: 8-Frame Sheet';
        setBanner('Switched to 8-Frame Colab Skeletal Sprite Sheet mode!');
      } else if (unitRenderMode === 'sheet8') {
        unitRenderMode = 'shapes';
        document.getElementById('btnRenderMode').textContent = 'Rig: Shapes';
        setBanner('Switched to Legacy Geometric Shape mode.');
      } else {
        unitRenderMode = 'limbs60';
        document.getElementById('btnRenderMode').textContent = 'Rig: 60FPS Limbs';
        setBanner('Switched to 60FPS Live 6-Limb Skeletal Animation mode!');
      }
    };
    document.getElementById('btnToggleLabels').onclick = () => {
      showUnitLabels = !showUnitLabels;
      document.getElementById('btnToggleLabels').textContent = `Tags: ${showUnitLabels ? 'ON' : 'OFF'}`;
    };"""

assert target_label_btn_binding in html, "target_label_btn_binding not found"
html = html.replace(target_label_btn_binding, replacement_label_btn_binding, 1)

# Save index.html (which loads epic_battle_2d_assets_bundle.js)
index_path = os.path.join(BASE_DIR, "index.html")
with open(index_path, "w", encoding="utf-8") as f:
    f.write(html)
print(f"Saved {index_path} ({os.path.getsize(index_path)/1024:.1f} KB)")

# Also save a 100% standalone single-file HTML with epic_battle_2d_assets_bundle.js inlined!
with open(BUNDLE_JS, "r", encoding="utf-8") as f:
    bundle_js_content = f.read()

standalone_html = html.replace(
    '<script src="epic_battle_2d_assets_bundle.js"></script>',
    f"<script>\n{bundle_js_content}\n  </script>",
    1
)
standalone_path = os.path.join(BASE_DIR, "epic_battle_2d_animated_soldiers.html")
with open(standalone_path, "w", encoding="utf-8") as f:
    f.write(standalone_html)
print(f"Saved standalone {standalone_path} ({os.path.getsize(standalone_path)/(1024*1024):.2f} MB)")
