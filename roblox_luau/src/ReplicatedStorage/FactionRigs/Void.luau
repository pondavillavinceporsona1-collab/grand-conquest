-- VoidRig.luau
-- High-Detail Corrupted Cosmic 3D Rigs for Void Faction
-- Detailed with intricate glowing corruption cracks, shattered obsidian plates,
-- floating crystal shards, and weaponless organic rift conduits!

local VoidRig = {}

local PITCH_BLACK     = Color3.fromRGB(14, 12, 20)
local OBSIDIAN_SLATE  = Color3.fromRGB(24, 20, 34)
local OBSIDIAN_HIGH   = Color3.fromRGB(42, 35, 58)
local DARK_CARAPACE   = Color3.fromRGB(30, 25, 42)
local VOID_VIOLET     = Color3.fromRGB(185, 75, 255)
local VOID_MAGENTA    = Color3.fromRGB(235, 95, 255)
local VOID_CRIMSON    = Color3.fromRGB(165, 25, 75)
local VOID_ELECTRIC   = Color3.fromRGB(140, 50, 255)
local VOID_CYAN       = Color3.fromRGB(90, 225, 255)
local VOID_CORE       = Color3.fromRGB(255, 135, 255)
local VOID_RUNIC      = Color3.fromRGB(240, 215, 255)

function VoidRig.build(ctx)
	local unitDef = ctx.unitDef
	local uName = ctx.uName
	local s = ctx.s
	local model = ctx.model
	local body = ctx.body
	local head = ctx.head
	local leftArm = ctx.leftArm
	local rightArm = ctx.rightArm
	local leftLeg = ctx.leftLeg
	local rightLeg = ctx.rightLeg
	local makePart = ctx.makePart
	local makeWedge = ctx.makeWedge
	local attach = ctx.attach

	local CYL = Enum.PartType.Cylinder
	local BALL = Enum.PartType.Ball
	local SMOOTH = Enum.Material.SmoothPlastic
	local NEON = Enum.Material.Neon
	local METAL = Enum.Material.Metal
	local FABRIC = Enum.Material.Fabric

	local function P(name, size, color, mat, shape)
		return makePart(name, size * s, color, mat or SMOOTH, model, shape)
	end
	local function W(name, size, color, mat)
		return makeWedge(name, size * s, color, mat or SMOOTH, model)
	end
	local function at(limbKey, part, x, y, z, rx, ry, rz)
		local cf = CFrame.new(x * s, y * s, z * s)
		if rx or ry or rz then
			cf = cf * CFrame.Angles(math.rad(rx or 0), math.rad(ry or 0), math.rad(rz or 0))
		end
		attach(limbKey, part, cf)
		return part
	end

	-- =====================================================================
	-- ICONIC CORRUPTED VOID HELPER SUITE (Crown, Chest Diamond, Crystal Clusters)
	-- =====================================================================
	local function makeVoidDiamondCore(parentLimb, centerY, centerZ, scaleMult)
		local dW = 0.55 * scaleMult
		local dH = 0.85 * scaleMult
		local dD = 0.22 * scaleMult
		local socket = P("CoreSocket", Vector3.new(dW + 0.22, dH + 0.22, 0.16), PITCH_BLACK)
		at(parentLimb, socket, 0, centerY, centerZ)
		local topDiamond = W("CoreDiamondTop", Vector3.new(dW, dH * 0.5, dD), VOID_MAGENTA, NEON)
		at(parentLimb, topDiamond, 0, centerY + dH * 0.25, centerZ - 0.08, 0, 0, 180)
		local botDiamond = W("CoreDiamondBot", Vector3.new(dW, dH * 0.5, dD), VOID_MAGENTA, NEON)
		at(parentLimb, botDiamond, 0, centerY - dH * 0.25, centerZ - 0.08, 0, 0, 0)
		local innerCore = P("CorePrism", Vector3.new(dW * 0.52, dH * 0.65, dD * 1.25), VOID_CORE, NEON)
		at(parentLimb, innerCore, 0, centerY, centerZ - 0.09)
		local v1 = P("CoreVein1", Vector3.new(0.08, 0.75, 0.12), VOID_VIOLET, NEON)
		at(parentLimb, v1, -0.45, centerY + 0.35, centerZ - 0.05, 0, 0, 35)
		local v2 = P("CoreVein2", Vector3.new(0.08, 0.75, 0.12), VOID_VIOLET, NEON)
		at(parentLimb, v2, 0.45, centerY + 0.35, centerZ - 0.05, 0, 0, -35)
		local v3 = P("CoreVein3", Vector3.new(0.08, 0.65, 0.12), VOID_CRIMSON, NEON)
		at(parentLimb, v3, -0.42, centerY - 0.35, centerZ - 0.05, 0, 0, -42)
		local v4 = P("CoreVein4", Vector3.new(0.08, 0.65, 0.12), VOID_CRIMSON, NEON)
		at(parentLimb, v4, 0.42, centerY - 0.35, centerZ - 0.05, 0, 0, 42)
	end

	local function makeVoidSpikedCrown(crownRadius, crownY, spikeHeight, points)
		local crownRing = P("VoidCrownRing", Vector3.new(crownRadius * 2.1, 0.25, crownRadius * 2.1), PITCH_BLACK)
		at("Head", crownRing, 0, crownY, 0)
		points = points or 8
		for i = 1, points do
			local angle = (i - 1) * (2 * math.pi / points)
			local cX = math.cos(angle) * crownRadius
			local cZ = math.sin(angle) * crownRadius
			local rotY = math.deg(-angle) + 90
			local isFront = math.cos(angle) < -0.5
			local sH = spikeHeight * (isFront and 1.25 or 0.95)
			local tooth = W("CrownTooth_" .. i, Vector3.new(0.22, sH, 0.35), OBSIDIAN_SLATE)
			at("Head", tooth, cX, crownY + sH * 0.45, cZ, 12, rotY, 0)
			local toothGlow = P("ToothGlow_" .. i, Vector3.new(0.06, sH * 0.85, 0.10), isFront and VOID_MAGENTA or VOID_CRIMSON, NEON)
			at("Head", toothGlow, cX * 1.05, crownY + sH * 0.45, cZ * 1.05, 12, rotY, 0)
		end
		local frontHorn = W("CrownFrontHorn", Vector3.new(0.28, spikeHeight * 1.45, 0.45), PITCH_BLACK)
		at("Head", frontHorn, 0, crownY + spikeHeight * 0.70, -crownRadius * 1.05, 18, 0, 0)
		local frontGlow = P("FrontHornGlow", Vector3.new(0.08, spikeHeight * 1.15, 0.12), VOID_MAGENTA, NEON)
		at("Head", frontGlow, 0, crownY + spikeHeight * 0.70, -crownRadius * 1.15, 18, 0, 0)
	end

	local function makeProtrudingShards(limbKey, shardDefs)
		for _, def in ipairs(shardDefs) do
			local shard = W("ProtrudingShard_" .. def.name, def.size, OBSIDIAN_HIGH)
			at(limbKey, shard, def.x, def.y, def.z, def.rx or 0, def.ry or 0, def.rz or 0)
			if def.glow then
				local g = P("ShardGlow_" .. def.name, Vector3.new(def.size.X * 0.35, def.size.Y * 0.85, 0.08), VOID_MAGENTA, NEON)
				at(limbKey, g, def.x, def.y, def.z - 0.05, def.rx or 0, def.ry or 0, def.rz or 0)
			end
		end
	end

	-- Base limb dark obsidian defaults
	head.Color = PITCH_BLACK
	body.Color = PITCH_BLACK
	leftArm.Color = PITCH_BLACK
	rightArm.Color = PITCH_BLACK
	leftLeg.Color = PITCH_BLACK
	rightLeg.Color = PITCH_BLACK

	if uName == "Shadowling" then
		-- =====================================================================
		-- C-TIER 1: SHADOWLING (Void Stalker Assassin, Sculpted Cowl, Void Dagger)
		-- =====================================================================
		-- 1. Sculpted Assassin Cowl & Predator Faceplate
		local cowlBase = P("ShadowCowl", Vector3.new(1.24, 1.20, 1.24), PITCH_BLACK)
		at("Head", cowlBase, 0, 0.05, 0)
		-- Sloped hood crown (tapering back aerodynamically)
		local hoodSlope = W("HoodSlope", Vector3.new(1.26, 0.35, 1.15), OBSIDIAN_SLATE)
		at("Head", hoodSlope, 0, 0.58, 0.15, -15, 0, 0)
		-- Hood front peak overhanging the forehead
		local hoodPeak = W("HoodPeak", Vector3.new(0.55, 0.32, 0.55), OBSIDIAN_HIGH)
		at("Head", hoodPeak, 0, 0.42, -0.68, 25, 0, 0)
		local peakGlow = P("PeakGlow", Vector3.new(0.12, 0.12, 0.35), VOID_MAGENTA, NEON)
		at("Head", peakGlow, 0, 0.45, -0.72, 25, 0, 0)

		-- Aerodynamic Swept Obsidian Horns (Signature Corrupted Crown from reference)
		local centerHorn = W("CenterHorn", Vector3.new(0.22, 0.95, 0.75), OBSIDIAN_HIGH)
		at("Head", centerHorn, 0, 0.82, 0.22, -28, 0, 0)
		local centerHornGlow = P("CenterHornGlow", Vector3.new(0.08, 0.75, 0.18), VOID_CORE, NEON)
		at("Head", centerHornGlow, 0, 0.88, 0.15, -28, 0, 0)

		for _, sign in ipairs({ -1, 1 }) do
			local sideHorn = W("SideHorn_" .. sign, Vector3.new(0.18, 0.75, 0.95), OBSIDIAN_SLATE)
			at("Head", sideHorn, sign * 0.55, 0.65, 0.32, -22, sign * 20, sign * 22)
			local hornGlow = P("SideHornGlow_" .. sign, Vector3.new(0.08, 0.15, 0.80), VOID_MAGENTA, NEON)
			at("Head", hornGlow, sign * 0.55, 0.70, 0.32, -22, sign * 20, sign * 22)

			-- Angular Assassin Mask Cheek Guards
			local cheekGuard = W("CheekGuard_" .. sign, Vector3.new(0.22, 0.65, 0.45), OBSIDIAN_HIGH)
			at("Head", cheekGuard, sign * 0.52, -0.15, -0.52, 0, sign * -15, sign * 12)

			-- Predator Slit Eyes (Bright glowing Cyan with Violet undertones)
			local eyeSlot = P("EyeSlot_" .. sign, Vector3.new(0.32, 0.14, 0.18), PITCH_BLACK)
			at("Head", eyeSlot, sign * 0.30, 0.08, -0.56, 0, 0, sign * -14)
			local eyeCore = P("EyeCore_" .. sign, Vector3.new(0.26, 0.09, 0.20), VOID_CYAN, NEON)
			at("Head", eyeCore, sign * 0.30, 0.08, -0.57, 0, 0, sign * -14)

			-- Crimson Fissure Veins branching across face
			local cheekVein = P("CheekVein_" .. sign, Vector3.new(0.07, 0.55, 0.12), VOID_CRIMSON, NEON)
			at("Head", cheekVein, sign * 0.38, -0.16, -0.58, 0, 0, sign * 32)
		end

		-- Jaw / Respirator Chitin Plate
		local jawPlate = P("JawPlate", Vector3.new(0.72, 0.35, 0.35), OBSIDIAN_SLATE)
		at("Head", jawPlate, 0, -0.42, -0.52)
		local jawGlow = P("JawGlow", Vector3.new(0.35, 0.08, 0.12), VOID_VIOLET, NEON)
		at("Head", jawGlow, 0, -0.42, -0.62)

		-- 2. Sleek Carapace Cuirass with Iconic Glowing Void Diamond Core
		local cuirass = P("AssassinCuirass", Vector3.new(2.15, 1.95, 0.52), OBSIDIAN_SLATE)
		at("Body", cuirass, 0, 0.15, -0.45)
		local cuirassBevel = W("CuirassBevel", Vector3.new(2.20, 0.45, 0.35), OBSIDIAN_HIGH)
		at("Body", cuirassBevel, 0, 0.85, -0.50, 180, 0, 0)
		local cuirassBelt = P("AssassinBelt", Vector3.new(2.25, 0.22, 1.25), OBSIDIAN_HIGH)
		at("Body", cuirassBelt, 0, -0.72, 0)

		-- Signature Void Diamond Core (Scaled for Stalker)
		makeVoidDiamondCore("Body", 0.18, -0.62, 0.95)

		-- Protruding Obsidian Rib & Waist Crystal Shards (Signature Void Corruption from image)
		makeProtrudingShards("Body", {
			{ name = "RibShardL", size = Vector3.new(0.26, 0.72, 0.38), x = -0.88, y = 0.28, z = -0.68, rx = 18, ry = 28, rz = 35, glow = true },
			{ name = "RibShardR", size = Vector3.new(0.26, 0.72, 0.38), x = 0.88, y = 0.28, z = -0.68, rx = 18, ry = -28, rz = -35, glow = true },
			{ name = "WaistShardL", size = Vector3.new(0.22, 0.60, 0.32), x = -0.82, y = -0.35, z = -0.62, rx = 22, ry = 22, rz = 28, glow = true },
			{ name = "WaistShardR", size = Vector3.new(0.22, 0.60, 0.32), x = 0.82, y = -0.35, z = -0.62, rx = 22, ry = -22, rz = -28, glow = true },
		})

		-- 4 Tiered Dorsal Spines on Back
		for sp = 1, 4 do
			local spine = W("DorsalSpine_" .. sp, Vector3.new(0.16, 0.58 - sp * 0.06, 0.60), OBSIDIAN_HIGH)
			at("Body", spine, 0, 0.65 - (sp - 1) * 0.38, 0.65, 30, 0, 0)
			local sGlow = P("SpineGlow_" .. sp, Vector3.new(0.08, 0.12, 0.45), VOID_MAGENTA, NEON)
			at("Body", sGlow, 0, 0.70 - (sp - 1) * 0.38, 0.68, 30, 0, 0)
		end

		-- 3. Left Arm: Assassin Bracer & Parrying Shadow Talon
		local lShoulder = P("BracerL", Vector3.new(1.24, 0.58, 1.24), OBSIDIAN_SLATE)
		at("LeftArm", lShoulder, 0, 0.55, 0)
		local lElbowCrystal = W("ElbowCrystalL", Vector3.new(0.24, 0.48, 0.72), OBSIDIAN_HIGH)
		at("LeftArm", lElbowCrystal, -0.52, 0.15, 0.38, -25, 0, 0)
		local lElbowGlow = P("ElbowGlowL", Vector3.new(0.08, 0.15, 0.52), VOID_MAGENTA, NEON)
		at("LeftArm", lElbowGlow, -0.52, 0.18, 0.40, -25, 0, 0)
		local lCrack = P("ArmCrackL", Vector3.new(0.10, 1.35, 0.16), VOID_VIOLET, NEON)
		at("LeftArm", lCrack, 0, 0, -0.54, 0, 0, -15)
		-- Triple curved parrying claws on left hand
		for c = -1, 1 do
			local talon = W("ParryClaw_" .. c, Vector3.new(0.10, 0.22, 0.90), OBSIDIAN_HIGH)
			at("LeftArm", talon, c * 0.24, -0.95, -0.42)
			local talonGlow = P("ParryClawGlow_" .. c, Vector3.new(0.06, 0.08, 0.60), VOID_MAGENTA, NEON)
			at("LeftArm", talonGlow, c * 0.24, -0.92, -0.68)
		end

		-- 4. Right Arm: Assassin Bracer & THE VOID DAGGER (Holding Dagger)
		local rShoulder = P("BracerR", Vector3.new(1.24, 0.58, 1.24), OBSIDIAN_SLATE)
		at("RightArm", rShoulder, 0, 0.55, 0)
		local rElbowCrystal = W("ElbowCrystalR", Vector3.new(0.24, 0.48, 0.72), OBSIDIAN_HIGH)
		at("RightArm", rElbowCrystal, 0.52, 0.15, 0.38, -25, 0, 0)
		local rElbowGlow = P("ElbowGlowR", Vector3.new(0.08, 0.15, 0.52), VOID_MAGENTA, NEON)
		at("RightArm", rElbowGlow, 0.52, 0.18, 0.40, -25, 0, 0)
		local rCrack = P("ArmCrackR", Vector3.new(0.10, 1.35, 0.16), VOID_MAGENTA, NEON)
		at("RightArm", rCrack, 0, 0, -0.54, 0, 0, 15)

		-- === THE VOID DAGGER ===
		-- Wrapped dark leather handle held firmly in right fist
		local daggerGrip = P("DaggerGrip", Vector3.new(0.22, 0.65, 0.22), Color3.fromRGB(34, 25, 45))
		at("RightArm", daggerGrip, 0, -0.95, -0.22, 30, -18, 0)

		-- Faceted Glowing Void Diamond Pommel Gem at base of handle
		local daggerPommel = P("DaggerPommel", Vector3.new(0.35, 0.35, 0.35), VOID_CORE, NEON)
		at("RightArm", daggerPommel, 0, -1.32, -0.04, 75, -18, 0)

		-- Curved Obsidian Crescent Crossguard with sharp quillons
		local crossguard = P("DaggerCrossguard", Vector3.new(0.78, 0.20, 0.32), OBSIDIAN_HIGH)
		at("RightArm", crossguard, 0, -0.62, -0.38, 30, -18, 0)
		local guardGlowL = P("GuardGlowL", Vector3.new(0.18, 0.12, 0.16), VOID_MAGENTA, NEON)
		at("RightArm", guardGlowL, -0.35, -0.62, -0.40, 30, -18, 0)
		local guardGlowR = P("GuardGlowR", Vector3.new(0.18, 0.12, 0.16), VOID_MAGENTA, NEON)
		at("RightArm", guardGlowR, 0.35, -0.62, -0.40, 30, -18, 0)

		-- Primary Serrated Obsidian Dagger Blade (Named "Weapon" for rig recognition)
		local daggerBlade = W("Weapon", Vector3.new(0.20, 0.48, 1.85), OBSIDIAN_SLATE)
		at("RightArm", daggerBlade, 0, -0.32, -1.25, 30, -18, 0)

		-- Glowing Void Energy Core / Fuller running down blade center
		local bladeCore = P("DaggerCore", Vector3.new(0.08, 0.20, 1.65), VOID_MAGENTA, NEON)
		at("RightArm", bladeCore, 0, -0.32, -1.15, 30, -18, 0)

		-- Glowing Prism Razor Edge
		local bladeEdge = P("DaggerEdge", Vector3.new(0.05, 0.12, 1.55), VOID_CORE, NEON)
		at("RightArm", bladeEdge, 0, -0.32, -1.20, 30, -18, 0)

		-- Jagged Serrated Barb along blade spine
		local bladeBarb = W("DaggerBarb", Vector3.new(0.16, 0.35, 0.60), OBSIDIAN_HIGH)
		at("RightArm", bladeBarb, 0, -0.10, -0.90, 210, -18, 0)

		-- Sharp Diamond Piercing Dagger Tip
		local daggerTip = W("DaggerTip", Vector3.new(0.16, 0.42, 0.72), OBSIDIAN_SLATE)
		at("RightArm", daggerTip, 0, -0.02, -2.25, 30, -18, 0)
		local tipGlow = P("DaggerTipGlow", Vector3.new(0.08, 0.18, 0.55), VOID_CYAN, NEON)
		at("RightArm", tipGlow, 0, -0.02, -2.22, 30, -18, 0)

		-- 5. Legs: Angular Shin Guards, Knee Spikes & Crystal Spurs
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = W("KneeSpike", Vector3.new(0.26, 0.60, 0.48), OBSIDIAN_HIGH)
			at(lKey, knee, 0, 0.25, -0.58, -25, 0, 0)
			local kneeGlow = P("KneeGlow", Vector3.new(0.09, 0.42, 0.18), VOID_MAGENTA, NEON)
			at(lKey, kneeGlow, 0, 0.28, -0.70, -25, 0, 0)

			local shin = P("ShinGuard", Vector3.new(1.08, 1.30, 0.38), OBSIDIAN_SLATE)
			at(lKey, shin, 0, -0.35, -0.52)
			local shinCrack = P("ShinCrack", Vector3.new(0.10, 1.05, 0.18), VOID_VIOLET, NEON)
			at(lKey, shinCrack, 0, -0.35, -0.65, 0, 0, sign * 25)

			-- Rear crystal spur on ankle for agile predator silhouette
			local heelSpur = W("HeelSpur", Vector3.new(0.18, 0.36, 0.58), OBSIDIAN_HIGH)
			at(lKey, heelSpur, 0, -0.75, 0.54, 30, 0, 0)
			local spurGlow = P("SpurGlow", Vector3.new(0.07, 0.14, 0.38), VOID_MAGENTA, NEON)
			at(lKey, spurGlow, 0, -0.75, 0.58, 30, 0, 0)
		end

	elseif uName == "Void Archer" then
		-- =====================================================================
		-- C-TIER 2: VOID ARCHER (Sleek Rift Marksman, Smooth Armor, Back Spines Array)
		-- Requirements:
		-- 1. Exact same weapon design preserved!
		-- 2. Less spike on body/limbs/head (sleek, smooth armor).
		-- 3. Spikes concentrated on the BACK (Majestic Obsidian Dorsal Spine Array & Void Quiver).
		-- =====================================================================
		-- 1. Sleek, Smooth Marksman Hood & Precision Sniper Visor (Zero Spikes)
		local hoodBase = P("ArcherHood", Vector3.new(1.22, 1.20, 1.22), PITCH_BLACK)
		at("Head", hoodBase, 0, 0.05, 0)
		local hoodSlope = W("HoodSlope", Vector3.new(1.24, 0.32, 1.15), OBSIDIAN_SLATE)
		at("Head", hoodSlope, 0, 0.55, 0.12, -15, 0, 0)
		local hoodBrow = W("HoodBrow", Vector3.new(1.26, 0.22, 0.45), OBSIDIAN_HIGH)
		at("Head", hoodBrow, 0, 0.38, -0.62, 22, 0, 0)

		-- Smooth Assassin Mask with Precision Horizontal Optical Aiming Visor
		local maskPlate = P("ArcherMask", Vector3.new(1.10, 0.75, 0.22), OBSIDIAN_SLATE)
		at("Head", maskPlate, 0, 0.02, -0.52)
		local sniperVisor = P("SniperVisor", Vector3.new(0.92, 0.10, 0.16), VOID_CYAN, NEON)
		at("Head", sniperVisor, 0, 0.08, -0.64)
		local visorCore = P("VisorCore", Vector3.new(0.18, 0.18, 0.18), VOID_CORE, NEON)
		at("Head", visorCore, 0, 0.08, -0.65)
		local cheekTrimL = P("CheekTrimL", Vector3.new(0.06, 0.45, 0.12), VOID_VIOLET, NEON)
		at("Head", cheekTrimL, -0.32, -0.15, -0.62, 0, 0, 20)
		local cheekTrimR = P("CheekTrimR", Vector3.new(0.06, 0.45, 0.12), VOID_VIOLET, NEON)
		at("Head", cheekTrimR, 0.32, -0.15, -0.62, 0, 0, -20)
		local chinGuard = P("ChinGuard", Vector3.new(0.65, 0.25, 0.32), OBSIDIAN_HIGH)
		at("Head", chinGuard, 0, -0.42, -0.50)

		-- 2. Sleek Segmented Chest Cuirass & Cross-Strap (Smooth, No Spikes)
		local cuirass = P("ArcherCuirass", Vector3.new(2.05, 1.85, 0.42), OBSIDIAN_SLATE)
		at("Body", cuirass, 0, 0.15, -0.45)
		local sternumPlate = P("SternumPlate", Vector3.new(0.85, 1.45, 0.16), OBSIDIAN_HIGH)
		at("Body", sternumPlate, 0, 0.18, -0.66)
		local sternumGlow = P("SternumGlow", Vector3.new(0.08, 1.25, 0.12), VOID_MAGENTA, NEON)
		at("Body", sternumGlow, 0, 0.18, -0.74)

		-- Diagonal Quiver Harness Strap across chest
		local harness = P("QuiverHarness", Vector3.new(0.32, 2.15, 0.12), Color3.fromRGB(34, 25, 45))
		at("Body", harness, 0, 0.15, -0.68, 0, 0, 32)
		local harnessBrooch = P("HarnessBrooch", Vector3.new(0.35, 0.35, 0.16), VOID_CORE, NEON)
		at("Body", harnessBrooch, -0.15, 0.25, -0.76)

		-- Smooth archer belt & side pouches
		local belt = P("ArcherBelt", Vector3.new(2.15, 0.20, 1.15), OBSIDIAN_HIGH)
		at("Body", belt, 0, -0.72, 0)
		local pouchL = P("PouchL", Vector3.new(0.22, 0.35, 0.42), Color3.fromRGB(28, 22, 38))
		at("Body", pouchL, -1.08, -0.68, 0)
		local pouchR = P("PouchR", Vector3.new(0.22, 0.35, 0.42), Color3.fromRGB(28, 22, 38))
		at("Body", pouchR, 1.08, -0.68, 0)

		-- 3. THE BACK: MAJESTIC OBSIDIAN DORSAL SPINES & CRYSTAL QUIVER ARRAY
		-- "except it's back" -> All spikes concentrated on back in a spectacular array!
		-- Central Tiered Dorsal Spines (5 cascading obsidian horn spikes)
		local spineDefs = {
			{ name = "DorsalSpine_1", sz = Vector3.new(0.20, 0.85, 1.05), y = 0.85, z = 0.65, ang = 35, glow = true },
			{ name = "DorsalSpine_2", sz = Vector3.new(0.24, 1.15, 1.35), y = 0.50, z = 0.72, ang = 42, glow = true },
			{ name = "DorsalSpine_3", sz = Vector3.new(0.22, 0.95, 1.15), y = 0.15, z = 0.70, ang = 38, glow = true },
			{ name = "DorsalSpine_4", sz = Vector3.new(0.18, 0.75, 0.95), y = -0.20, z = 0.68, ang = 32, glow = true },
			{ name = "DorsalSpine_5", sz = Vector3.new(0.16, 0.55, 0.75), y = -0.55, z = 0.65, ang = 26, glow = false },
		}
		for _, sp in ipairs(spineDefs) do
			local spine = W(sp.name, sp.sz, OBSIDIAN_HIGH)
			at("Body", spine, 0, sp.y, sp.z, sp.ang, 0, 0)
			if sp.glow then
				local sRidge = P(sp.name .. "_Glow", Vector3.new(0.08, sp.sz.Y * 0.82, sp.sz.Z * 0.85), VOID_MAGENTA, NEON)
				at("Body", sRidge, 0, sp.y + 0.06, sp.z + 0.05, sp.ang, 0, 0)
			end
		end

		-- Flanking Shoulder Blade Crystal Spikes (Wings of Spikes)
		for _, sSign in ipairs({ -1, 1 }) do
			local wingSpike1 = W("WingSpike1_" .. sSign, Vector3.new(0.16, 0.75, 0.95), OBSIDIAN_SLATE)
			at("Body", wingSpike1, sSign * 0.65, 0.65, 0.68, 35, sSign * 25, sSign * 22)
			local wingGlow1 = P("WingGlow1_" .. sSign, Vector3.new(0.06, 0.12, 0.80), VOID_VIOLET, NEON)
			at("Body", wingGlow1, sSign * 0.65, 0.68, 0.70, 35, sSign * 25, sSign * 22)

			local wingSpike2 = W("WingSpike2_" .. sSign, Vector3.new(0.14, 0.65, 0.85), OBSIDIAN_HIGH)
			at("Body", wingSpike2, sSign * 0.62, 0.25, 0.66, 30, sSign * 20, sSign * 18)
			local wingGlow2 = P("WingGlow2_" .. sSign, Vector3.new(0.06, 0.10, 0.70), VOID_CORE, NEON)
			at("Body", wingGlow2, sSign * 0.62, 0.28, 0.68, 30, sSign * 20, sSign * 18)
		end

		-- Void Crystal Quiver Battery (4 docked crystalline arrows)
		local quiverHousing = P("QuiverHousing", Vector3.new(0.85, 0.85, 0.32), PITCH_BLACK)
		at("Body", quiverHousing, 0, 0.35, 0.62)
		for a = 1, 4 do
			local arrow = P("VoidArrow_" .. a, Vector3.new(0.09, 1.25, 0.09), VOID_CYAN, NEON)
			at("Body", arrow, (a - 2.5) * 0.22, 0.95, 0.82, 18, 0, 0)
			local arrowHead = W("ArrowHead_" .. a, Vector3.new(0.14, 0.25, 0.18), VOID_CORE, NEON)
			at("Body", arrowHead, (a - 2.5) * 0.22, 1.55, 0.92, 18, 0, 0)
		end

		-- 4. Left Arm: Smooth Bracer & Focus Orb (Smooth, No Spikes)
		local lShoulder = P("LShoulderCap", Vector3.new(1.18, 0.45, 1.18), OBSIDIAN_SLATE)
		at("LeftArm", lShoulder, 0, 0.55, 0)
		local lBracer = P("LBracer", Vector3.new(1.12, 1.15, 1.12), OBSIDIAN_SLATE)
		at("LeftArm", lBracer, 0, -0.35, 0)
		local lRune = P("LBracerRune", Vector3.new(0.08, 0.95, 0.12), VOID_CYAN, NEON)
		at("LeftArm", lRune, 0, -0.35, -0.58)
		local leftFocus = P("FocusOrb", Vector3.new(0.38, 0.38, 0.38), VOID_MAGENTA, NEON, BALL)
		at("LeftArm", leftFocus, 0, -0.85, -0.25)

		-- 5. Right Arm: Smooth Shoulder Cap & EXACT SAME ORIGINAL WEAPON DESIGN (Preserved 100%)
		local rShoulder = P("RShoulderCap", Vector3.new(1.18, 0.45, 1.18), OBSIDIAN_SLATE)
		at("RightArm", rShoulder, 0, 0.55, 0)

		-- WEAPONLESS RANGED CONDUIT (RightArm): Hand pitch counter-rotated -78° for horizontal aiming!
		local handCF = CFrame.new(0, -0.85 * s, -0.15 * s) * CFrame.Angles(math.rad(-78), 0, 0)
		local conduitRing = P("ConduitRing", Vector3.new(0.55, 0.55, 0.18), OBSIDIAN_SLATE)
		attach("RightArm", conduitRing, handCF * CFrame.new(0, 0, -1.05 * s))
		local conduitCore = P("Weapon", Vector3.new(0.35, 0.35, 0.35), VOID_CYAN, NEON, BALL)
		attach("RightArm", conduitCore, handCF * CFrame.new(0, 0, -1.05 * s))
		for p = 1, 4 do
			local angle = (p - 1) * (math.pi / 2)
			local prong = W("Prong_" .. p, Vector3.new(0.08, 0.15, 0.65), VOID_VIOLET, NEON)
			attach("RightArm", prong, handCF * CFrame.new(math.cos(angle) * 0.35 * s, math.sin(angle) * 0.35 * s, -1.35 * s) * CFrame.Angles(0, 0, angle))
		end
		-- Upper/Lower Bow Arc Tips
		for _, bSign in ipairs({ -1, 1 }) do
			local bowArc = W("BowArc_" .. bSign, Vector3.new(0.10, 0.85, 0.22), OBSIDIAN_HIGH)
			attach("RightArm", bowArc, handCF * CFrame.new(0, bSign * 0.75 * s, -1.05 * s) * CFrame.Angles(bSign * 25, 0, 0))
		end

		-- 6. Legs: Smooth Archer Greaves, Clean Rounded Knee Caps & Sabatons (Smooth, No Spikes)
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local greave = P("Greave", Vector3.new(1.02, 1.25, 0.32), OBSIDIAN_SLATE)
			at(lKey, greave, 0, -0.25, -0.48)
			local greaveRune = P("GreaveRune", Vector3.new(0.08, 0.95, 0.12), VOID_MAGENTA, NEON)
			at(lKey, greaveRune, 0, -0.25, -0.62)
			-- Smooth beveled knee cap (rounded/beveled, no spike)
			local kneeCap = P("KneeCap", Vector3.new(0.85, 0.38, 0.22), OBSIDIAN_HIGH)
			at(lKey, kneeCap, 0, 0.32, -0.50)
			local kneeTrim = P("KneeTrim", Vector3.new(0.08, 0.25, 0.10), VOID_CYAN, NEON)
			at(lKey, kneeTrim, 0, 0.32, -0.60)
			local sabaton = P("Sabaton", Vector3.new(1.02, 0.32, 1.10), OBSIDIAN_HIGH)
			at(lKey, sabaton, 0, -0.80, -0.05)
		end

	elseif uName == "Dark Sentinel" then
		-- =====================================================================
		-- C-TIER 3: DARK SENTINEL (Monolithic Bastion, Obsidian Crown, Aegis Carapace)
		-- =====================================================================
		local helm = P("BastionHelm", Vector3.new(1.28, 1.28, 1.28), PITCH_BLACK)
		at("Head", helm, 0, 0.05, 0)
		makeVoidSpikedCrown(0.62, 0.58, 0.65, 8)
		local crossH = P("CrossCrackH", Vector3.new(0.90, 0.12, 0.16), VOID_MAGENTA, NEON)
		at("Head", crossH, 0, 0.05, -0.66)
		local crossV = P("CrossCrackV", Vector3.new(0.12, 0.90, 0.16), VOID_VIOLET, NEON)
		at("Head", crossV, 0, 0.05, -0.66)
		for _, sign in ipairs({ -1, 1 }) do
			local cheekCrack = P("CheekCrack", Vector3.new(0.08, 0.45, 0.14), VOID_CRIMSON, NEON)
			at("Head", cheekCrack, sign * 0.45, -0.15, -0.64, 0, 0, sign * 25)
		end

		-- Heavy Tectonic Breastplate with Iconic Central Void Diamond Core
		local breastplate = P("TectonicPlate", Vector3.new(2.35, 2.05, 0.52), OBSIDIAN_SLATE)
		at("Body", breastplate, 0, 0.15, -0.45)
		makeVoidDiamondCore("Body", 0.22, -0.68, 1.15)

		-- Protruding Obsidian Crystal Shards on Torso
		makeProtrudingShards("Body", {
			{ name = "RibL", size = Vector3.new(0.25, 0.65, 0.35), x = -0.75, y = -0.15, z = -0.65, rx = 15, ry = 25, rz = 35, glow = true },
			{ name = "RibR", size = Vector3.new(0.25, 0.65, 0.35), x = 0.75, y = -0.15, z = -0.65, rx = 15, ry = -25, rz = -35, glow = true },
			{ name = "AbL", size = Vector3.new(0.22, 0.55, 0.30), x = -0.55, y = -0.48, z = -0.62, rx = 20, ry = 15, rz = 25 },
			{ name = "AbR", size = Vector3.new(0.22, 0.55, 0.30), x = 0.55, y = -0.48, z = -0.62, rx = 20, ry = -15, rz = -25 },
		})

		local backShell = P("BackShell", Vector3.new(2.20, 1.95, 0.42), OBSIDIAN_SLATE)
		at("Body", backShell, 0, 0.15, 0.50)
		local backCrack1 = P("BackCrack1", Vector3.new(0.10, 1.45, 0.14), VOID_VIOLET, NEON)
		at("Body", backCrack1, -0.35, 0.15, 0.68, 0, 0, -25)
		local backCrack2 = P("BackCrack2", Vector3.new(0.10, 1.45, 0.14), VOID_MAGENTA, NEON)
		at("Body", backCrack2, 0.35, 0.15, 0.68, 0, 0, 25)

		local belt = P("BastionBelt", Vector3.new(2.35, 0.32, 1.35), OBSIDIAN_HIGH)
		at("Body", belt, 0, -0.68, 0)

		-- Massive Double-Tiered Cracked Pauldrons with Upward/Backward Void Spikes
		for _, sign in ipairs({ -1, 1 }) do
			local pauldron = P("BastionPauldron", Vector3.new(1.50, 0.70, 1.50), OBSIDIAN_HIGH)
			at("Body", pauldron, sign * 1.58, 0.88, 0, 0, 0, sign * -20)
			local pCrack = P("PauldronCrack", Vector3.new(0.10, 0.85, 0.14), VOID_VIOLET, NEON)
			at("Body", pCrack, sign * 1.66, 0.92, 0, 0, 0, sign * -20)
			-- Signature Upward/Backward Swept Void Lance Spike
			local lanceSpike = W("LanceSpike", Vector3.new(0.26, 0.45, 1.65), PITCH_BLACK)
			at("Body", lanceSpike, sign * 1.75, 1.35, 0.45, -35, sign * 15, sign * -18)
			local lanceGlow = P("LanceGlow", Vector3.new(0.08, 0.12, 1.45), VOID_MAGENTA, NEON)
			at("Body", lanceGlow, sign * 1.78, 1.38, 0.45, -35, sign * 15, sign * -18)

			local pSubPlate = P("PauldronSub", Vector3.new(1.20, 0.38, 1.20), OBSIDIAN_SLATE)
			at("Body", pSubPlate, sign * 1.52, 0.50, 0)
		end

		-- LeftArm: Massive Aegis Carapace Shield Plate + Radiating Fissures
		local aegisPlate = P("Weapon", Vector3.new(0.42, 2.95, 1.75), PITCH_BLACK)
		at("LeftArm", aegisPlate, -0.48, -0.15, 0)
		local aegisCore = P("AegisRune", Vector3.new(0.18, 2.15, 1.15), VOID_MAGENTA, NEON)
		at("LeftArm", aegisCore, -0.64, -0.15, 0)
		local aegisCrack1 = P("AegisCrack1", Vector3.new(0.08, 1.95, 0.14), VOID_CYAN, NEON)
		at("LeftArm", aegisCrack1, -0.66, -0.15, -0.38, 15, 0, 0)
		local aegisCrack2 = P("AegisCrack2", Vector3.new(0.08, 1.95, 0.14), VOID_CYAN, NEON)
		at("LeftArm", aegisCrack2, -0.66, -0.15, 0.38, -15, 0, 0)

		-- Right Arm: Cracked Tectonic Crush Gauntlet + Knuckle Spikes
		local gauntlet = P("CrushGauntlet", Vector3.new(1.25, 1.50, 1.25), OBSIDIAN_SLATE)
		at("RightArm", gauntlet, 0, -0.35, 0)
		local gCrack = P("GauntletCrack", Vector3.new(0.10, 1.20, 0.14), VOID_VIOLET, NEON)
		at("RightArm", gCrack, 0, -0.35, -0.60, 0, 0, 15)
		local gSpike = W("GauntletSpike", Vector3.new(0.22, 0.55, 0.45), OBSIDIAN_HIGH)
		at("RightArm", gSpike, 0.55, -0.35, 0, 0, 90, 0)

		-- Heavy Cracked Knee Shields & Greaves with Protruding Crystal Spikes
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = P("KneeShield", Vector3.new(1.10, 1.10, 0.42), OBSIDIAN_HIGH)
			at(lKey, knee, 0, 0.35, -0.50)
			local kneeSpike = W("KneeSpike", Vector3.new(0.22, 0.55, 0.35), OBSIDIAN_SLATE)
			at(lKey, kneeSpike, 0, 0.35, -0.72, 25, 0, 0)
			local kneeGlow = P("KneeGlow", Vector3.new(0.08, 0.45, 0.12), VOID_MAGENTA, NEON)
			at(lKey, kneeGlow, 0, 0.35, -0.76, 25, 0, 0)

			local sabaton = P("HeavySabaton", Vector3.new(1.15, 0.42, 1.25), OBSIDIAN_SLATE)
			at(lKey, sabaton, 0, -0.78, -0.05)
		end

	elseif uName == "Void Stalker" then
		-- =====================================================================
		-- B-TIER 1: VOID STALKER (Elite Shadow Assassin, Tri-Eye Visor, Twin Void Kamas)
		-- Requirements:
		-- 1. Improve weapon design to look distinctly like a lethal assassin (Twin Void Sickle-Kamas).
		-- 2. Sculpted stealth hood, tri-eye predator visor, chest diamond core, and crystal spurs.
		-- =====================================================================
		-- 1. Sculpted Assassin Cowl & Tri-Eye Predator Visor
		local cowl = P("StalkerCowl", Vector3.new(1.24, 1.22, 1.24), PITCH_BLACK)
		at("Head", cowl, 0, 0.05, 0)
		local cowlPeak = W("CowlPeak", Vector3.new(0.55, 0.28, 0.48), OBSIDIAN_HIGH)
		at("Head", cowlPeak, 0, 0.42, -0.68, 25, 0, 0)

		-- Swept Assassin Horns Curling Back like Aerodynamic Scythe Blades
		for _, sign in ipairs({ -1, 1 }) do
			local horn = W("StalkerHorn_" .. sign, Vector3.new(0.18, 0.42, 1.45), OBSIDIAN_SLATE)
			at("Head", horn, sign * 0.52, 0.48, 0.28, 28, sign * 15, sign * 18)
			local hCrack = P("HornCrack_" .. sign, Vector3.new(0.08, 0.10, 1.25), VOID_MAGENTA, NEON)
			at("Head", hCrack, sign * 0.52, 0.52, 0.28, 28, sign * 15, sign * 18)

			-- Angular Cheek Chitin Plates
			local cheekArmor = W("CheekArmor_" .. sign, Vector3.new(0.20, 0.60, 0.38), OBSIDIAN_HIGH)
			at("Head", cheekArmor, sign * 0.48, -0.15, -0.52, 0, sign * -12, sign * 15)
		end

		-- Sculpted Tri-Eye Optical Visor (Sinister Predator Pattern)
		local visorPlate = P("VisorPlate", Vector3.new(0.95, 0.72, 0.18), OBSIDIAN_SLATE)
		at("Head", visorPlate, 0, 0.05, -0.56)
		local eyeTop = P("EyeTop", Vector3.new(0.26, 0.14, 0.16), VOID_CORE, NEON)
		at("Head", eyeTop, 0, 0.22, -0.66)
		local eyeL = P("EyeL", Vector3.new(0.26, 0.10, 0.16), VOID_CYAN, NEON)
		at("Head", eyeL, -0.26, -0.02, -0.66, 0, 0, -18)
		local eyeR = P("EyeR", Vector3.new(0.26, 0.10, 0.16), VOID_CYAN, NEON)
		at("Head", eyeR, 0.26, -0.02, -0.66, 0, 0, 18)
		local mouthFracture = P("MouthFracture", Vector3.new(0.48, 0.08, 0.14), VOID_VIOLET, NEON)
		at("Head", mouthFracture, 0, -0.25, -0.64)

		-- 2. Interlocking Rib Carapace with Iconic Void Diamond Core
		local chest = P("RibCarapace", Vector3.new(2.15, 1.90, 0.48), OBSIDIAN_SLATE)
		at("Body", chest, 0, 0.15, -0.45)
		local chestPlateBevel = W("ChestBevel", Vector3.new(2.20, 0.42, 0.35), OBSIDIAN_HIGH)
		at("Body", chestPlateBevel, 0, 0.85, -0.50, 180, 0, 0)

		-- Signature Void Diamond Core (Scaled for Assassin)
		makeVoidDiamondCore("Body", 0.18, -0.60, 0.95)

		-- Protruding Obsidian Rib Shards (Signature Void Corruption from image)
		makeProtrudingShards("Body", {
			{ name = "RibShardL", size = Vector3.new(0.24, 0.65, 0.35), x = -0.88, y = 0.25, z = -0.65, rx = 18, ry = 28, rz = 35, glow = true },
			{ name = "RibShardR", size = Vector3.new(0.24, 0.65, 0.35), x = 0.88, y = 0.25, z = -0.65, rx = 18, ry = -28, rz = -35, glow = true },
			{ name = "FlankShardL", size = Vector3.new(0.20, 0.55, 0.30), x = -0.80, y = -0.32, z = -0.60, rx = 22, ry = 22, rz = 28 },
			{ name = "FlankShardR", size = Vector3.new(0.20, 0.55, 0.30), x = 0.80, y = -0.32, z = -0.60, rx = 22, ry = -22, rz = -28 },
		})

		-- Assassin Belt & Scarf/Cloak
		local belt = P("StalkerBelt", Vector3.new(2.25, 0.22, 1.25), OBSIDIAN_HIGH)
		at("Body", belt, 0, -0.72, 0)
		local cloak = P("ShadowCloak", Vector3.new(2.05, 2.85, 0.18), PITCH_BLACK, FABRIC)
		at("Body", cloak, 0, -0.35, 0.65, 12, 0, 0)

		-- 4 Cascading Obsidian Dorsal Spine Needles protruding through cloak
		for sp = 1, 4 do
			local spine = W("StalkerSpine_" .. sp, Vector3.new(0.16, 0.65 - sp * 0.08, 0.65), OBSIDIAN_HIGH)
			at("Body", spine, 0, 0.65 - (sp - 1) * 0.40, 0.70, 32, 0, 0)
			local sGlow = P("StalkerSpineGlow_" .. sp, Vector3.new(0.06, 0.10, 0.50), VOID_MAGENTA, NEON)
			at("Body", sGlow, 0, 0.70 - (sp - 1) * 0.40, 0.72, 32, 0, 0)
		end

		-- 3. Forearms & TWIN VOID ASSASSIN SICKLE-BLADES (KAMAS)
		-- Dual-wielded hooked crescent assassin sickles with serrated inner curves & glowing cores!
		for _, aSign in ipairs({ -1, 1 }) do
			local aKey = (aSign == -1 and "LeftArm" or "RightArm")
			local pName = (aSign == 1 and "Weapon" or "OffhandKama")

			-- Reinforced Assassin Bracer & Elbow Spike
			local bracer = P("Bracer_" .. aSign, Vector3.new(1.24, 0.55, 1.24), OBSIDIAN_SLATE)
			at(aKey, bracer, 0, 0.55, 0)
			local elbowSpike = W("ElbowSpike_" .. aSign, Vector3.new(0.20, 0.45, 0.65), OBSIDIAN_HIGH)
			at(aKey, elbowSpike, aSign * 0.52, 0.15, 0.38, -25, 0, 0)
			local elbowGlow = P("ElbowGlow_" .. aSign, Vector3.new(0.08, 0.14, 0.48), VOID_MAGENTA, NEON)
			at(aKey, elbowGlow, aSign * 0.52, 0.18, 0.40, -25, 0, 0)
			local armCrack = P("ArmCrack_" .. aSign, Vector3.new(0.10, 1.35, 0.16), (aSign == 1 and VOID_MAGENTA or VOID_VIOLET), NEON)
			at(aKey, armCrack, 0, 0, -0.54, 0, 0, aSign * 15)

			-- === THE VOID ASSASSIN KAMA / SICKLE ===
			-- Grip held firmly in fist, angled forward and outward in combat assassin strike stance
			local kamaGrip = P("KamaGrip_" .. aSign, Vector3.new(0.22, 0.68, 0.22), Color3.fromRGB(34, 25, 45))
			at(aKey, kamaGrip, 0, -0.95, -0.20, 32, aSign * -18, 0)

			-- Spiked Pommel Gem at base of handle
			local pommel = P("KamaPommel_" .. aSign, Vector3.new(0.32, 0.32, 0.32), VOID_CORE, NEON)
			at(aKey, pommel, 0, -1.35, -0.02, 70, aSign * -18, 0)

			-- Crescent Obsidian Crossguard with sharp quillons
			local guard = P("KamaGuard_" .. aSign, Vector3.new(0.72, 0.18, 0.28), OBSIDIAN_HIGH)
			at(aKey, guard, 0, -0.62, -0.35, 32, aSign * -18, 0)
			local guardGlowL = P("GuardGlowL_" .. aSign, Vector3.new(0.14, 0.10, 0.14), VOID_MAGENTA, NEON)
			at(aKey, guardGlowL, aSign * -0.32, -0.62, -0.36, 32, aSign * -18, 0)
			local guardGlowR = P("GuardGlowR_" .. aSign, Vector3.new(0.14, 0.10, 0.14), VOID_CYAN, NEON)
			at(aKey, guardGlowR, aSign * 0.32, -0.62, -0.36, 32, aSign * -18, 0)

			-- Shaft connecting guard to hooked scythe head
			local shaft = P("KamaShaft_" .. aSign, Vector3.new(0.18, 0.38, 1.35), OBSIDIAN_SLATE)
			at(aKey, shaft, 0, -0.35, -1.05, 32, aSign * -18, 0)
			local shaftGlow = P("ShaftGlow_" .. aSign, Vector3.new(0.08, 0.18, 1.15), VOID_MAGENTA, NEON)
			at(aKey, shaftGlow, 0, -0.35, -0.98, 32, aSign * -18, 0)

			-- The Curved Assassin Sickle Beak / Head (Named pName so RightArm binds as "Weapon")
			local kamaBeak = W(pName, Vector3.new(0.18, 1.15, 0.72), OBSIDIAN_HIGH)
			at(aKey, kamaBeak, 0, -0.68, -1.82, -45, aSign * -18, 0)

			-- Inner Curved Gut Hook / Serrated Sickle Edge
			local innerEdge = P("KamaInnerEdge_" .. aSign, Vector3.new(0.08, 1.05, 0.16), VOID_MAGENTA, NEON)
			at(aKey, innerEdge, 0, -0.68, -1.55, -45, aSign * -18, 0)

			-- Razor Core Spine
			local kamaCore = P("KamaCore_" .. aSign, Vector3.new(0.06, 0.85, 0.12), VOID_CORE, NEON)
			at(aKey, kamaCore, 0, -0.68, -1.68, -45, aSign * -18, 0)

			-- Curved Beak Piercing Tip (Lethal curved point)
			local beakTip = W("BeakTip_" .. aSign, Vector3.new(0.14, 0.55, 0.42), OBSIDIAN_SLATE)
			at(aKey, beakTip, 0, -1.25, -2.15, -60, aSign * -18, 0)
			local tipGlow = P("BeakTipGlow_" .. aSign, Vector3.new(0.07, 0.35, 0.18), VOID_CYAN, NEON)
			at(aKey, tipGlow, 0, -1.25, -2.08, -60, aSign * -18, 0)
		end

		-- 4. Legs: Stealth Greaves, Angled Knee Guards & Heel Spurs
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = W("StalkerKnee", Vector3.new(0.24, 0.55, 0.45), OBSIDIAN_HIGH)
			at(lKey, knee, 0, 0.25, -0.58, -25, 0, 0)
			local kneeGlow = P("StalkerKneeGlow", Vector3.new(0.08, 0.38, 0.15), VOID_MAGENTA, NEON)
			at(lKey, kneeGlow, 0, 0.28, -0.68, -25, 0, 0)

			local shin = P("StalkerGreave", Vector3.new(1.05, 1.25, 0.36), OBSIDIAN_SLATE)
			at(lKey, shin, 0, -0.35, -0.50)
			local shinCrack = P("StalkerGreaveCrack", Vector3.new(0.10, 0.95, 0.16), VOID_VIOLET, NEON)
			at(lKey, shinCrack, 0, -0.35, -0.62, 0, 0, sign * 22)

			local heelSpur = W("StalkerHeelSpur", Vector3.new(0.16, 0.35, 0.65), OBSIDIAN_HIGH)
			at(lKey, heelSpur, 0, -0.75, 0.52, 30, 0, 0)
			local spurGlow = P("StalkerSpurGlow", Vector3.new(0.07, 0.16, 0.45), VOID_MAGENTA, NEON)
			at(lKey, spurGlow, 0, -0.75, 0.56, 30, 0, 0)
		end

	elseif uName == "Abyss Mage" then
		-- =====================================================================
		-- B-TIER 2: ABYSS MAGE (Singularity Caster, Orbiting Crown, Event Horizon Staff)
		-- =====================================================================
		-- 1. Warlock Cowl & Orbiting Crystal Crown (6 Floating Shards)
		local cowl = P("AbyssCowl", Vector3.new(1.24, 1.22, 1.24), PITCH_BLACK)
		at("Head", cowl, 0, 0.05, 0)
		local cowlRim = P("CowlRim", Vector3.new(1.30, 0.16, 1.30), OBSIDIAN_HIGH)
		at("Head", cowlRim, 0, -0.45, 0)

		-- 6 Cracked Obsidian Shards Orbiting Crown
		for i = 1, 6 do
			local angle = (i - 1) * (2 * math.pi / 6)
			local sX = math.cos(angle) * 0.95
			local sZ = math.sin(angle) * 0.95
			local shard = W("HornShard_" .. i, Vector3.new(0.18, 0.85, 0.22), OBSIDIAN_SLATE)
			at("Head", shard, sX, 0.95, sZ, 15, math.deg(angle), 0)
			local shardGlow = P("HornShardGlow_" .. i, Vector3.new(0.08, 0.65, 0.10), VOID_MAGENTA, NEON)
			at("Head", shardGlow, sX, 0.98, sZ, 15, math.deg(angle), 0)
		end

		-- Gaping Tear-Drop Cosmic Corruption Mask
		local mask = P("AbyssMask", Vector3.new(1.05, 0.75, 0.18), OBSIDIAN_HIGH)
		at("Head", mask, 0, 0.02, -0.56)
		local tearL = P("TearCrackL", Vector3.new(0.08, 0.85, 0.16), VOID_VIOLET, NEON)
		at("Head", tearL, -0.28, -0.08, -0.66, 0, 0, -18)
		local tearR = P("TearCrackR", Vector3.new(0.08, 0.85, 0.16), VOID_MAGENTA, NEON)
		at("Head", tearR, 0.28, -0.08, -0.66, 0, 0, 18)
		local eyeSlotL = P("EyeSlotL", Vector3.new(0.22, 0.09, 0.16), VOID_CYAN, NEON)
		at("Head", eyeSlotL, -0.28, 0.12, -0.66)
		local eyeSlotR = P("EyeSlotR", Vector3.new(0.22, 0.09, 0.16), VOID_CYAN, NEON)
		at("Head", eyeSlotR, 0.28, 0.12, -0.66)

		-- 2. Layered Warlock Robe with Flared Collar & Void Diamond Core
		local robe = P("AbyssRobe", Vector3.new(2.15, 1.95, 0.52), OBSIDIAN_SLATE, FABRIC)
		at("Body", robe, 0, 0.15, -0.45)
		local collar = P("WarlockCollar", Vector3.new(2.35, 0.65, 1.35), OBSIDIAN_HIGH)
		at("Body", collar, 0, 0.95, 0.15)
		local collarTrim = P("CollarTrim", Vector3.new(2.40, 0.14, 1.40), VOID_MAGENTA, NEON)
		at("Body", collarTrim, 0, 0.92, 0.15)

		-- Signature Void Diamond Core (Gravitational Focus)
		makeVoidDiamondCore("Body", 0.18, -0.62, 0.95)

		-- Protruding Rib Shards
		makeProtrudingShards("Body", {
			{ name = "RibShardL", size = Vector3.new(0.22, 0.60, 0.32), x = -0.85, y = 0.22, z = -0.65, rx = 18, ry = 28, rz = 32, glow = true },
			{ name = "RibShardR", size = Vector3.new(0.22, 0.60, 0.32), x = 0.85, y = 0.22, z = -0.65, rx = 18, ry = -28, rz = -32, glow = true },
		})

		-- Robe Skirt with Cascading Corruption Rifts
		local skirt = P("RobeSkirt", Vector3.new(2.35, 1.95, 1.30), OBSIDIAN_SLATE, FABRIC)
		at("Body", skirt, 0, -1.25, 0)
		for sk = 1, 3 do
			local sCrack = P("SkirtCrack_" .. sk, Vector3.new(0.10, 1.65, 0.16), (sk % 2 == 0 and VOID_VIOLET or VOID_MAGENTA), NEON)
			at("Body", sCrack, (sk - 2) * 0.65, -1.25, -0.68, 0, 0, (sk - 2) * 14)
		end
		local skirtTrim = P("SkirtTrim", Vector3.new(2.40, 0.16, 1.35), VOID_CORE, NEON)
		at("Body", skirtTrim, 0, -2.18, 0)

		-- 3. Left Hand: Floating Gravitational Anchor Sphere (Slow Spell Conduit)
		local focusOrb = P("FocusOrb", Vector3.new(0.48, 0.48, 0.48), VOID_CYAN, NEON, BALL)
		at("LeftArm", focusOrb, 0, -0.95, -0.35)
		local focusRing = P("FocusRing", Vector3.new(0.68, 0.08, 0.68), VOID_MAGENTA, NEON)
		at("LeftArm", focusRing, 0, -0.95, -0.35, 35, 0, 0)

		-- 4. Right Hand: THE SINGULARITY EVENT HORIZON STAFF
		-- Tall twisted obsidian staff crowned with an orbiting black hole singularity!
		local staffShaft = P("StaffShaft", Vector3.new(0.18, 3.45, 0.18), Color3.fromRGB(28, 22, 38))
		at("RightArm", staffShaft, 0, -0.45, -0.25, 20, 0, 0)

		-- Staff Diamond Pommel
		local staffPommel = P("StaffPommel", Vector3.new(0.32, 0.45, 0.32), VOID_MAGENTA, NEON)
		at("RightArm", staffPommel, 0, -2.05, 0.28, 20, 0, 0)

		-- Staff Head Framing Housing (4 Obsidian Quillons)
		local headHousing = P("StaffHousing", Vector3.new(0.45, 0.45, 0.45), OBSIDIAN_HIGH)
		at("RightArm", headHousing, 0, 1.15, -0.78, 20, 0, 0)
		for q = 1, 4 do
			local qAng = (q - 1) * (math.pi / 2)
			local qX = math.cos(qAng) * 0.45
			local qZ = math.sin(qAng) * 0.45
			local quillon = W("Quillon_" .. q, Vector3.new(0.12, 0.65, 0.28), OBSIDIAN_SLATE)
			at("RightArm", quillon, qX, 1.45, -0.78 + qZ, 20, 0, math.deg(qAng))
		end

		-- Orbiting Black Hole Singularity Core (Named "Weapon")
		local singularityCore = P("Weapon", Vector3.new(0.55, 0.55, 0.55), PITCH_BLACK, Enum.Material.SmoothPlastic, BALL)
		at("RightArm", singularityCore, 0, 1.55, -0.92)

		-- Dual Perpendicular Event Horizon Rings
		local horizonRing1 = P("HorizonRing1", Vector3.new(0.85, 0.08, 0.85), VOID_MAGENTA, NEON)
		at("RightArm", horizonRing1, 0, 1.55, -0.92, 25, 45, 0)
		local horizonRing2 = P("HorizonRing2", Vector3.new(0.85, 0.08, 0.85), VOID_CYAN, NEON)
		at("RightArm", horizonRing2, 0, 1.55, -0.92, -45, -25, 0)

	elseif uName == "Soul Drainer" then
		-- =====================================================================
		-- B-TIER 3: SOUL DRAINER (Necro-Void Leech, Triple Dorsal Siphons, Palm Siphons)
		-- =====================================================================
		-- 1. Faceted Obsidian Skull Mask with Spiderweb Necrotic Fissures
		local cowl = P("DrainerCowl", Vector3.new(1.24, 1.22, 1.24), PITCH_BLACK)
		at("Head", cowl, 0, 0.05, 0)
		local skullPlate = P("SkullFace", Vector3.new(1.05, 0.95, 0.22), OBSIDIAN_HIGH)
		at("Head", skullPlate, 0, 0.02, -0.54)

		-- Hollow Glowing Cyan Eye Sockets
		for _, sign in ipairs({ -1, 1 }) do
			local eye = P("HollowEye_" .. sign, Vector3.new(0.24, 0.16, 0.20), VOID_CYAN, NEON)
			at("Head", eye, sign * 0.28, 0.12, -0.62)

			-- Spiderweb Necrotic Fissure Veins
			local web1 = P("CheekWeb1_" .. sign, Vector3.new(0.45, 0.08, 0.14), VOID_MAGENTA, NEON)
			at("Head", web1, sign * 0.38, 0.02, -0.64, 0, 0, sign * 25)
			local web2 = P("CheekWeb2_" .. sign, Vector3.new(0.08, 0.52, 0.14), VOID_VIOLET, NEON)
			at("Head", web2, sign * 0.45, -0.18, -0.64)

			-- Swept Chitin Ear-Horns
			local earHorn = W("EarHorn_" .. sign, Vector3.new(0.16, 0.45, 1.15), OBSIDIAN_SLATE)
			at("Head", earHorn, sign * 0.52, 0.45, 0.25, 25, 0, sign * 18)
		end

		-- Jaw Necro-Filter
		local jawPlate = P("DrainerJaw", Vector3.new(0.68, 0.28, 0.35), OBSIDIAN_SLATE)
		at("Head", jawPlate, 0, -0.38, -0.52)
		local jawVein = P("JawVein", Vector3.new(0.35, 0.08, 0.14), VOID_CYAN, NEON)
		at("Head", jawVein, 0, -0.38, -0.64)

		-- 2. Emaciated Carapace with Central Siphon Well & Diamond Core
		local chest = P("SiphonChest", Vector3.new(2.10, 1.85, 0.48), OBSIDIAN_SLATE)
		at("Body", chest, 0, 0.15, -0.45)
		makeVoidDiamondCore("Body", 0.18, -0.60, 0.95)

		-- Protruding Rib Crystal Shards
		makeProtrudingShards("Body", {
			{ name = "RibShardL", size = Vector3.new(0.22, 0.65, 0.32), x = -0.85, y = 0.22, z = -0.65, rx = 18, ry = 28, rz = 32, glow = true },
			{ name = "RibShardR", size = Vector3.new(0.22, 0.65, 0.32), x = 0.85, y = 0.22, z = -0.65, rx = 18, ry = -28, rz = -32, glow = true },
			{ name = "WaistShardL", size = Vector3.new(0.20, 0.55, 0.28), x = -0.80, y = -0.32, z = -0.60, rx = 22, ry = 22, rz = 28 },
			{ name = "WaistShardR", size = Vector3.new(0.20, 0.55, 0.28), x = 0.80, y = -0.32, z = -0.60, rx = 22, ry = -22, rz = -28 },
		})

		-- 3. THE SIGNATURE TRIPLE DORSAL SIPHON EXTRACTION TUBES (Back Feature)
		-- 3 colossal ribbed obsidian tubes channeling stolen souls with glowing neon energy cores!
		local tubeDefs = {
			{ name = "SiphonTube_Center", sz = Vector3.new(0.28, 1.45, 0.42), x = 0, y = 0.70, z = 0.72, ang = 26, color = VOID_CYAN },
			{ name = "SiphonTube_L", sz = Vector3.new(0.24, 1.25, 0.36), x = -0.58, y = 0.55, z = 0.68, ang = 24, outAng = -18, color = VOID_MAGENTA },
			{ name = "SiphonTube_R", sz = Vector3.new(0.24, 1.25, 0.36), x = 0.58, y = 0.55, z = 0.68, ang = 24, outAng = 18, color = VOID_MAGENTA },
		}
		for _, tb in ipairs(tubeDefs) do
			local tube = P(tb.name, tb.sz, OBSIDIAN_HIGH)
			at("Body", tube, tb.x, tb.y, tb.z, tb.ang, 0, tb.outAng or 0)
			local tubeCore = P(tb.name .. "_Core", Vector3.new(tb.sz.X * 0.45, tb.sz.Y * 0.85, tb.sz.Z * 0.45), tb.color, NEON)
			at("Body", tubeCore, tb.x, tb.y, tb.z + 0.08, tb.ang, 0, tb.outAng or 0)
		end

		-- Shoulder Vents with Pulsing Soul Plasma Orbs
		for _, sign in ipairs({ -1, 1 }) do
			local sVent = P("ShoulderVent_" .. sign, Vector3.new(0.48, 0.48, 0.48), OBSIDIAN_HIGH)
			at("Body", sVent, sign * 1.38, 0.78, 0.35, 25, 0, sign * 20)
			local sOrb = P("VentOrb_" .. sign, Vector3.new(0.28, 0.28, 0.28), VOID_CYAN, NEON, BALL)
			at("Body", sOrb, sign * 1.38, 0.85, 0.38)
		end

		-- 4. Arms: TWIN VAMPIRIC SOUL SIPHONS on Both Hands
		for _, aSign in ipairs({ -1, 1 }) do
			local aKey = (aSign == -1 and "LeftArm" or "RightArm")
			local wKey = (aSign == 1 and "Weapon" or "OffhandSiphon")

			local bracer = P("DrainBracer_" .. aSign, Vector3.new(1.22, 0.55, 1.22), OBSIDIAN_SLATE)
			at(aKey, bracer, 0, 0.55, 0)
			local armCrack = P("DrainCrack_" .. aSign, Vector3.new(0.10, 1.35, 0.16), (aSign == 1 and VOID_CYAN or VOID_MAGENTA), NEON)
			at(aKey, armCrack, 0, 0, -0.54, 0, 0, aSign * 15)

			-- Siphon Aperture Nozzle on Palm
			local siphonOrb = P(wKey, Vector3.new(0.55, 0.55, 0.55), VOID_CYAN, NEON, BALL)
			at(aKey, siphonOrb, 0, -0.95, -0.35)

			-- 3 Curved Chitin Claws Framing Siphon
			for c = -1, 1 do
				local sClaw = W("SiphonClaw_" .. aSign .. "_" .. c, Vector3.new(0.10, 0.22, 0.85), OBSIDIAN_HIGH)
				at(aKey, sClaw, c * 0.24, -0.95, -0.45, 22, 0, 0)
				local sClawGlow = P("SClawGlow_" .. aSign .. "_" .. c, Vector3.new(0.06, 0.08, 0.60), VOID_MAGENTA, NEON)
				at(aKey, sClawGlow, c * 0.24, -0.92, -0.70, 22, 0, 0)
			end
		end

		-- 5. Legs: Siphon Greaves with Energy Conduits & Heel Spurs
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local greave = P("DrainGreave", Vector3.new(1.05, 1.25, 0.36), OBSIDIAN_SLATE)
			at(lKey, greave, 0, -0.25, -0.50)
			local gCrack = P("DrainCrack", Vector3.new(0.08, 0.95, 0.14), VOID_CYAN, NEON)
			at(lKey, gCrack, 0, -0.25, -0.62, 0, 0, sign * 18)
			local kneeSpur = W("KneeSpur", Vector3.new(0.20, 0.45, 0.40), OBSIDIAN_HIGH)
			at(lKey, kneeSpur, 0, 0.25, -0.58, -25, 0, 0)
			local heelSpur = W("HeelSpur", Vector3.new(0.16, 0.32, 0.55), OBSIDIAN_HIGH)
			at(lKey, heelSpur, 0, -0.75, 0.52, 30, 0, 0)
		end

	elseif uName == "Void Reaper" then
		-- =====================================================================
		-- A-TIER 1: VOID REAPER (Titan Executioner, Jagged Spiked Crown, Organic Scythe)
		-- =====================================================================
		local helm = P("ReaperHelm", Vector3.new(1.42, 1.42, 1.42), PITCH_BLACK)
		at("Head", helm, 0, 0.10, 0)
		makeVoidSpikedCrown(0.70, 0.70, 0.85, 8)
		local visorFracture = P("VisorFracture", Vector3.new(1.15, 0.14, 0.22), VOID_MAGENTA, NEON)
		at("Head", visorFracture, 0, 0.08, -0.72)
		for _, sign in ipairs({ -1, 1 }) do
			local horn = W("ReaperHorn", Vector3.new(0.38, 0.48, 2.45), OBSIDIAN_SLATE)
			at("Head", horn, sign * 0.70, 0.72, 0.38, 35, 0, sign * 22)
			local hornCrack = P("HornCrack", Vector3.new(0.10, 0.16, 2.05), VOID_VIOLET, NEON)
			at("Head", hornCrack, sign * 0.74, 0.78, 0.38, 35, 0, sign * 22)
		end

		-- Shattered Tectonic Chestplate with Iconic Central Void Diamond Core
		local chestL = P("ChestPlateL", Vector3.new(1.25, 2.15, 0.60), OBSIDIAN_SLATE)
		at("Body", chestL, -0.60, 0.15, -0.45)
		local chestR = P("ChestPlateR", Vector3.new(1.25, 2.15, 0.60), OBSIDIAN_SLATE)
		at("Body", chestR, 0.60, 0.15, -0.45)
		makeVoidDiamondCore("Body", 0.25, -0.70, 1.30)

		-- Protruding Obsidian Crystal Shards on Ribs & Waist
		makeProtrudingShards("Body", {
			{ name = "ChestShardL", size = Vector3.new(0.28, 0.75, 0.38), x = -0.85, y = 0.05, z = -0.68, rx = 18, ry = 30, rz = 32, glow = true },
			{ name = "ChestShardR", size = Vector3.new(0.28, 0.75, 0.38), x = 0.85, y = 0.05, z = -0.68, rx = 18, ry = -30, rz = -32, glow = true },
			{ name = "MidShardL", size = Vector3.new(0.25, 0.65, 0.32), x = -0.65, y = -0.38, z = -0.65, rx = 22, ry = 20, rz = 25 },
			{ name = "MidShardR", size = Vector3.new(0.25, 0.65, 0.32), x = 0.65, y = -0.38, z = -0.65, rx = 22, ry = -20, rz = -25 },
		})

		-- Dorsal Spines on Back
		for sp = 1, 4 do
			local bSpine = W("ReaperDorsal_" .. sp, Vector3.new(0.20, 0.60, 0.95), OBSIDIAN_HIGH)
			at("Body", bSpine, 0, 0.85 - (sp - 1) * 0.42, 0.78, 30, 0, 0)
		end

		-- Spiked Tectonic Blast Pauldrons with Colossal Backward Lance Spikes
		for _, sign in ipairs({ -1, 1 }) do
			local pauldron = P("BlastPauldron", Vector3.new(1.75, 0.85, 1.75), OBSIDIAN_HIGH)
			at("Body", pauldron, sign * 1.82, 0.98, 0, 0, 0, sign * -22)
			local pCrack = P("PauldronCrack", Vector3.new(0.12, 0.95, 0.16), VOID_MAGENTA, NEON)
			at("Body", pCrack, sign * 1.90, 1.02, 0, 0, 0, sign * -22)
			-- Signature Giant Backward/Upward Swept Obsidian Lance Spike
			local lance = W("ReaperLanceSpike", Vector3.new(0.32, 0.52, 2.25), PITCH_BLACK)
			at("Body", lance, sign * 1.95, 1.45, 0.55, -38, sign * 18, sign * -15)
			local lanceGlow = P("ReaperLanceGlow", Vector3.new(0.10, 0.16, 2.00), VOID_MAGENTA, NEON)
			at("Body", lanceGlow, sign * 1.98, 1.50, 0.55, -38, sign * 18, sign * -15)
		end

		-- WEAPONLESS ORGANIC SCYTHE ARM: Terminating in Colossal Scythe-Blade
		local scytheBase = P("Weapon", Vector3.new(0.26, 4.85, 1.30), PITCH_BLACK)
		at("RightArm", scytheBase, 0, -0.45, -1.10, 30, 0, 0)
		local scytheEdge = P("ScytheGlowEdge", Vector3.new(0.14, 4.65, 0.42), VOID_MAGENTA, NEON)
		at("RightArm", scytheEdge, 0, -0.45, -1.55, 30, 0, 0)
		local scytheSpineCrack = P("ScytheSpineCrack", Vector3.new(0.10, 4.25, 0.16), VOID_VIOLET, NEON)
		at("RightArm", scytheSpineCrack, 0, -0.45, -0.80, 30, 0, 0)
		for sc = 1, 4 do
			local serration = W("ScytheSerration_" .. sc, Vector3.new(0.12, 0.38, 0.50), VOID_MAGENTA, NEON)
			at("RightArm", serration, 0, -0.35 - (sc - 1) * 0.85, -1.72, 45, 0, 0)
		end

		-- Left Arm: Spiked Titan Fist with Knuckle Spikes
		local leftFist = P("TitanFist", Vector3.new(1.35, 1.55, 1.35), OBSIDIAN_HIGH)
		at("LeftArm", leftFist, 0, -0.45, 0)
		for k = 1, 3 do
			local knuckleSpike = W("KnuckleSpike_" .. k, Vector3.new(0.16, 0.28, 0.55), VOID_VIOLET, NEON)
			at("LeftArm", knuckleSpike, (k - 2) * 0.38, -1.18, -0.70)
		end
		local fistShard = W("FistShard", Vector3.new(0.25, 0.65, 0.35), OBSIDIAN_SLATE)
		at("LeftArm", fistShard, -0.68, -0.45, 0, 0, -90, 0)

		-- Heavy Cracked Tectonic Greaves with Protruding Knee/Thigh Spikes
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = P("TitanKnee", Vector3.new(1.20, 0.60, 0.42), OBSIDIAN_HIGH)
			at(lKey, knee, 0, 0.32, -0.55)
			local legSpike = W("LegShardSpike", Vector3.new(0.24, 0.65, 0.38), OBSIDIAN_SLATE)
			at(lKey, legSpike, sign * 0.62, 0.15, 0, 0, sign * 90, sign * 20)
			local sabaton = P("TitanSabaton", Vector3.new(1.22, 0.40, 1.32), OBSIDIAN_SLATE)
			at(lKey, sabaton, 0, -0.80, -0.05)
		end

	elseif uName == "Abyssal Sorcerer" then
		-- =====================================================================
		-- A-TIER 2: ABYSSAL SORCERER (Supreme Void Archmage, 7-Shard Monolith Halo, Grand Lance)
		-- =====================================================================
		-- 1. Towering Ceremonial Arch-Mitre with Third-Eye Celestial Fissure
		local mitre = P("SorcererMitre", Vector3.new(1.35, 1.75, 1.35), PITCH_BLACK)
		at("Head", mitre, 0, 0.32, 0)
		local mitreRim = P("MitreRim", Vector3.new(1.42, 0.18, 1.42), OBSIDIAN_HIGH)
		at("Head", mitreRim, 0, -0.42, 0)

		-- Radiant Celestial Third-Eye Fissure (Diamond Prism)
		local thirdEye = P("ThirdEyePrism", Vector3.new(0.32, 0.55, 0.22), VOID_CYAN, NEON)
		at("Head", thirdEye, 0, 0.42, -0.72)
		local thirdEyeGlow = P("ThirdEyeCore", Vector3.new(0.18, 0.32, 0.25), VOID_CORE, NEON)
		at("Head", thirdEyeGlow, 0, 0.42, -0.74)

		-- Sweeping Archmage Horn Crests
		for _, sign in ipairs({ -1, 1 }) do
			local horn = W("ArchHorn_" .. sign, Vector3.new(0.20, 0.65, 1.55), OBSIDIAN_HIGH)
			at("Head", horn, sign * 0.55, 0.65, 0.35, 30, sign * 15, sign * 20)
			local hGlow = P("ArchHornGlow_" .. sign, Vector3.new(0.08, 0.15, 1.35), VOID_MAGENTA, NEON)
			at("Head", hGlow, sign * 0.55, 0.70, 0.35, 30, sign * 15, sign * 20)

			local mitreCrack = P("MitreCrack_" .. sign, Vector3.new(0.08, 0.95, 0.14), VOID_VIOLET, NEON)
			at("Head", mitreCrack, sign * 0.48, 0.35, -0.70, 0, 0, sign * -22)
		end

		-- 2. GRAND 7-SHARD MONOLITH HALO (Signature Archmage Halo floating in a crescent arch)
		for h = 1, 7 do
			local hAngle = (h - 1) * (math.pi / 3.0) - (math.pi / 2)
			local sX = math.cos(hAngle) * 1.55
			local sY = 0.65 + math.sin(hAngle) * 1.15
			local shard = P("HaloMonolith_" .. h, Vector3.new(0.22, 0.95, 0.48), OBSIDIAN_SLATE)
			at("Head", shard, sX, sY, 0.68, 15, 0, math.deg(hAngle) * 0.65)
			local shardRune = P("HaloRune_" .. h, Vector3.new(0.10, 0.75, 0.18), VOID_MAGENTA, NEON)
			at("Head", shardRune, sX, sY, 0.75, 15, 0, math.deg(hAngle) * 0.65)
			local shardCore = P("HaloCore_" .. h, Vector3.new(0.06, 0.45, 0.22), VOID_CORE, NEON)
			at("Head", shardCore, sX, sY, 0.78, 15, 0, math.deg(hAngle) * 0.65)
		end

		-- 3. Imperial Archmage Robes with Colossal Pauldrons & Supreme Diamond Core
		local robe = P("ImperialRobe", Vector3.new(2.35, 2.15, 0.65), PITCH_BLACK, FABRIC)
		at("Body", robe, 0, 0.15, -0.45)

		-- Supreme Radiant Void Diamond Core framed in ceremonial socket
		makeVoidDiamondCore("Body", 0.22, -0.68, 1.25)

		-- Protruding 3D Obsidian Crystal Shards from Ribs
		makeProtrudingShards("Body", {
			{ name = "RibShardL1", size = Vector3.new(0.28, 0.75, 0.38), x = -0.92, y = 0.22, z = -0.68, rx = 18, ry = 28, rz = 35, glow = true },
			{ name = "RibShardR1", size = Vector3.new(0.28, 0.75, 0.38), x = 0.92, y = 0.22, z = -0.68, rx = 18, ry = -28, rz = -35, glow = true },
			{ name = "RibShardL2", size = Vector3.new(0.24, 0.65, 0.32), x = -0.82, y = -0.25, z = -0.65, rx = 22, ry = 22, rz = 28 },
			{ name = "RibShardR2", size = Vector3.new(0.24, 0.65, 0.32), x = 0.82, y = -0.25, z = -0.65, rx = 22, ry = -22, rz = -28 },
		})

		-- Double-Tiered Obsidian Pauldrons with Rearward-Swept Lance Spikes
		for _, sign in ipairs({ -1, 1 }) do
			local pLower = P("SorcererPauldron_" .. sign, Vector3.new(1.65, 0.75, 1.65), PITCH_BLACK)
			at("Body", pLower, sign * 1.85, 0.95, 0, 0, 0, sign * -20)
			local pUpper = P("SorcererPauldronUpper_" .. sign, Vector3.new(1.35, 0.55, 1.35), OBSIDIAN_HIGH)
			at("Body", pUpper, sign * 1.95, 1.25, 0, 0, 0, sign * -22)
			local pGlow = P("PauldronGlow_" .. sign, Vector3.new(0.12, 0.95, 0.16), VOID_MAGENTA, NEON)
			at("Body", pGlow, sign * 2.05, 1.28, 0, 0, 0, sign * -22)
			local pLance = W("PauldronLance_" .. sign, Vector3.new(0.32, 0.48, 2.25), OBSIDIAN_SLATE)
			at("Body", pLance, sign * 2.05, 1.55, 0.75, 25, 0, sign * -15)
		end

		-- Imperial Skirt with Hieroglyphic Fissures & Neon Hem
		local skirt = P("ImperialSkirt", Vector3.new(2.45, 1.95, 1.35), OBSIDIAN_SLATE, FABRIC)
		at("Body", skirt, 0, -1.25, 0)
		for sk = 1, 3 do
			local sCrack = P("SkirtCrack_" .. sk, Vector3.new(0.10, 1.65, 0.16), (sk % 2 == 0 and VOID_VIOLET or VOID_MAGENTA), NEON)
			at("Body", sCrack, (sk - 2) * 0.70, -1.25, -0.72, 0, 0, (sk - 2) * 15)
		end
		local skirtTrim = P("SkirtTrim", Vector3.new(2.50, 0.18, 1.40), VOID_CORE, NEON)
		at("Body", skirtTrim, 0, -2.20, 0)

		-- 4. Left Hand: Cosmic Gravitational Well Orb
		local gravOrb = P("SorcererOrb", Vector3.new(0.55, 0.55, 0.55), VOID_CORE, NEON, BALL)
		at("LeftArm", gravOrb, 0, -0.95, -0.35)
		local gravRing = P("GravRing", Vector3.new(0.78, 0.08, 0.78), VOID_CYAN, NEON)
		at("LeftArm", gravRing, 0, -0.95, -0.35, 45, 25, 0)

		-- 5. Right Hand: THE CATACLYSMIC VOID LANCE / RIFT SPIRE
		-- Colossal faceted floating catalyst spearhead suspended between jagged obsidian magnetic arc clamps!
		local lanceShaft = P("LanceShaft", Vector3.new(0.20, 3.65, 0.20), Color3.fromRGB(34, 25, 45))
		at("RightArm", lanceShaft, 0, -0.35, -0.30, 25, -15, 0)

		-- Spiked Diamond Pommel
		local lancePommel = P("LancePommel", Vector3.new(0.35, 0.55, 0.35), VOID_CORE, NEON)
		at("RightArm", lancePommel, 0, -2.15, 0.35, 25, -15, 0)

		-- Colossal Faceted Prism Diamond Spearhead (Named "Weapon" for 65 DMG Void Lance attacks!)
		local lanceHead = P("Weapon", Vector3.new(0.48, 2.15, 0.48), VOID_CORE, NEON)
		at("RightArm", lanceHead, 0, 1.65, -0.95, 25, -15, 0)
		local innerLanceCore = P("LancePrismCore", Vector3.new(0.28, 1.75, 0.28), VOID_CYAN, NEON)
		at("RightArm", innerLanceCore, 0, 1.65, -0.95, 25, -15, 0)

		-- Dual Jagged Obsidian Magnetic Arc Clamps framing the floating diamond head
		for mc = -1, 1, 2 do
			local arcClamp = W("ArcClamp_" .. mc, Vector3.new(0.16, 1.65, 0.55), OBSIDIAN_SLATE)
			at("RightArm", arcClamp, mc * 0.38, 1.55, -0.95, 25, -15, mc * 20)
			local clampGlow = P("ClampGlow_" .. mc, Vector3.new(0.08, 1.35, 0.16), VOID_MAGENTA, NEON)
			at("RightArm", clampGlow, mc * 0.38, 1.55, -0.98, 25, -15, mc * 20)
		end

		-- Orbiting Cosmic Rift Rings around the lance
		local riftRing1 = P("RiftRing1", Vector3.new(0.95, 0.08, 0.95), VOID_MAGENTA, NEON)
		at("RightArm", riftRing1, 0, 1.65, -0.95, 45, 0, 0)
		local riftRing2 = P("RiftRing2", Vector3.new(0.85, 0.08, 0.85), VOID_CYAN, NEON)
		at("RightArm", riftRing2, 0, 1.65, -0.95, -45, 30, 0)

	elseif uName == "Void Warden" then
		-- =====================================================================
		-- A-TIER 3: VOID WARDEN (Supreme Monarch, Full Spiked Crown, Diamond Core, Shards)
		-- =====================================================================
		local crownBase = P("CrownBase", Vector3.new(1.42, 0.38, 1.42), PITCH_BLACK)
		at("Head", crownBase, 0, 0.65, 0)
		-- Iconic 10-Point Jagged Obsidian Crown wrapping 360° around head (1:1 with user reference)
		makeVoidSpikedCrown(0.72, 0.65, 0.95, 10)

		-- 4 Towering Cracked Obsidian Horns
		for i = 1, 4 do
			local angle = (i - 1) * (math.pi / 2) + math.pi / 4
			local hX = math.cos(angle) * 0.70
			local hZ = math.sin(angle) * 0.70
			local horn = W("WardenHorn_" .. i, Vector3.new(0.35, 0.55, 2.45), OBSIDIAN_SLATE)
			at("Head", horn, hX, 0.98, hZ, 30, math.deg(angle), 0)
			local hCrack = P("WardenHornCrack_" .. i, Vector3.new(0.10, 0.14, 2.05), VOID_VIOLET, NEON)
			at("Head", hCrack, hX, 1.05, hZ, 30, math.deg(angle), 0)
		end

		-- Shattered Monolith Mask with Cross-Fissures
		local mask = P("WardenMask", Vector3.new(1.30, 1.30, 1.30), PITCH_BLACK)
		at("Head", mask, 0, 0.08, 0)
		local maskH = P("MaskFractureH", Vector3.new(1.10, 0.14, 0.20), VOID_MAGENTA, NEON)
		at("Head", maskH, 0, 0.08, -0.70)
		local maskV = P("MaskFractureV", Vector3.new(0.14, 1.10, 0.20), VOID_CYAN, NEON)
		at("Head", maskV, 0, 0.08, -0.70)

		-- Supermassive Tectonic Juggernaut Breastplate with Iconic Void Diamond Core
		local chestPlate = P("JuggernautPlate", Vector3.new(2.48, 2.25, 0.62), OBSIDIAN_SLATE)
		at("Body", chestPlate, 0, 0.15, -0.45)
		makeVoidDiamondCore("Body", 0.28, -0.72, 1.45)

		-- Protruding Obsidian Crystal Shards sprouting from Chest, Ribs & Waist (exact match to image)
		makeProtrudingShards("Body", {
			{ name = "RibShardL1", size = Vector3.new(0.30, 0.85, 0.42), x = -0.92, y = 0.15, z = -0.70, rx = 15, ry = 28, rz = 35, glow = true },
			{ name = "RibShardR1", size = Vector3.new(0.30, 0.85, 0.42), x = 0.92, y = 0.15, z = -0.70, rx = 15, ry = -28, rz = -35, glow = true },
			{ name = "RibShardL2", size = Vector3.new(0.26, 0.72, 0.36), x = -0.78, y = -0.25, z = -0.68, rx = 20, ry = 22, rz = 28 },
			{ name = "RibShardR2", size = Vector3.new(0.26, 0.72, 0.36), x = 0.78, y = -0.25, z = -0.68, rx = 20, ry = -22, rz = -28 },
			{ name = "WaistShardL", size = Vector3.new(0.28, 0.80, 0.38), x = -0.85, y = -0.60, z = -0.65, rx = 25, ry = 18, rz = 30, glow = true },
			{ name = "WaistShardR", size = Vector3.new(0.28, 0.80, 0.38), x = 0.85, y = -0.60, z = -0.65, rx = 25, ry = -18, rz = -30, glow = true },
		})

		local belt = P("WardenBelt", Vector3.new(2.55, 0.38, 1.50), OBSIDIAN_HIGH)
		at("Body", belt, 0, -0.78, 0)

		-- Colossal Double-Tiered Bastion Pauldrons with Signature Massive Rearward Spikes
		for _, sign in ipairs({ -1, 1 }) do
			local pLower = P("PauldronLower", Vector3.new(1.85, 0.95, 1.85), PITCH_BLACK)
			at("Body", pLower, sign * 1.92, 0.98, 0, 0, 0, sign * -22)
			local pUpper = P("PauldronUpper", Vector3.new(1.45, 0.75, 1.45), OBSIDIAN_HIGH)
			at("Body", pUpper, sign * 2.02, 1.32, 0, 0, 0, sign * -25)
			local pGlow = P("PauldronGlow", Vector3.new(0.14, 1.10, 0.20), VOID_MAGENTA, NEON)
			at("Body", pGlow, sign * 2.12, 1.34, 0, 0, 0, sign * -25)
			-- Colossal Backward/Upward Swept Void Lance Spike (iconic shoulder spike from image)
			local shoulderLance = W("ShoulderLance", Vector3.new(0.38, 0.58, 2.55), PITCH_BLACK)
			at("Body", shoulderLance, sign * 2.15, 1.65, 0.65, -40, sign * 20, sign * -18)
			local lanceGlow = P("ShoulderLanceGlow", Vector3.new(0.12, 0.18, 2.25), VOID_MAGENTA, NEON)
			at("Body", lanceGlow, sign * 2.18, 1.70, 0.65, -40, sign * 20, sign * -18)
		end

		-- FLOATING BACK RING: 6 Cracked Obsidian Monolith Shards Orbiting in Circular Halo
		for r = 1, 6 do
			local rAngle = (r - 1) * (2 * math.pi / 6)
			local rRad = 2.30
			local rX = math.cos(rAngle) * rRad
			local rY = math.sin(rAngle) * rRad + 0.25
			local mono = P("BackMonolith_" .. r, Vector3.new(0.28, 1.50, 0.60), PITCH_BLACK)
			at("Body", mono, rX, rY, 1.05, 0, 0, math.deg(rAngle))
			local monoCrack = P("BackMonolithCrack_" .. r, Vector3.new(0.10, 1.30, 0.16), VOID_MAGENTA, NEON)
			at("Body", monoCrack, rX, rY, 1.15, 0, 0, math.deg(rAngle))
		end

		-- Right Arm: Titan Gauntlet with Protruding Crystal Shards & Miniature Singularity Orb
		local rightGauntlet = P("TitanGauntletR", Vector3.new(1.45, 1.65, 1.45), OBSIDIAN_SLATE)
		at("RightArm", rightGauntlet, 0, -0.45, 0)
		local rArmShard = W("ArmShardR", Vector3.new(0.25, 0.75, 0.35), OBSIDIAN_HIGH)
		at("RightArm", rArmShard, 0.72, -0.45, 0, 0, 90, 0)
		local rightGlowKnuckles = P("KnuckleGlowR", Vector3.new(1.10, 0.16, 0.22), VOID_MAGENTA, NEON)
		at("RightArm", rightGlowKnuckles, 0, -1.08, -0.74)
		local singularityBlackHole = P("Weapon", Vector3.new(0.90, 0.90, 0.90), VOID_CORE, NEON, BALL)
		at("RightArm", singularityBlackHole, 0, -1.18, -0.48)
		local holeCore = P("HoleCore", Vector3.new(0.48, 0.48, 0.48), PITCH_BLACK, SMOOTH, BALL)
		at("RightArm", holeCore, 0, -1.18, -0.48)

		-- Left Arm: Heavy Cracked Bastion Gauntlet with Active Void Defense Rune & Arm Shard
		local leftGauntlet = P("TitanGauntletL", Vector3.new(1.45, 1.65, 1.45), OBSIDIAN_SLATE)
		at("LeftArm", leftGauntlet, 0, -0.45, 0)
		local lArmShard = W("ArmShardL", Vector3.new(0.25, 0.75, 0.35), OBSIDIAN_HIGH)
		at("LeftArm", lArmShard, -0.72, -0.45, 0, 0, -90, 0)
		local leftDefenseRune = P("DefenseRuneL", Vector3.new(0.22, 1.30, 0.95), VOID_CYAN, NEON)
		at("LeftArm", leftDefenseRune, -0.74, -0.45, 0)

		-- Colossal Cracked Tectonic Greaves & Knee Shields with Protruding Crystal Spikes
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = P("WardenKnee", Vector3.new(1.35, 0.70, 0.50), OBSIDIAN_HIGH)
			at(lKey, knee, 0, 0.30, -0.62)
			local kCrack = P("WardenKneeCrack", Vector3.new(0.10, 0.48, 0.14), VOID_VIOLET, NEON)
			at(lKey, kCrack, 0, 0.30, -0.72)
			-- Thigh & Knee Crystal Shards (matching leg spikes in user image)
			local legShard1 = W("ThighShard", Vector3.new(0.28, 0.85, 0.40), OBSIDIAN_SLATE)
			at(lKey, legShard1, sign * 0.72, 0.25, 0, 0, sign * 90, sign * 25)
			local legShardGlow = P("ThighShardGlow", Vector3.new(0.08, 0.65, 0.12), VOID_MAGENTA, NEON)
			at(lKey, legShardGlow, sign * 0.76, 0.25, 0, 0, sign * 90, sign * 25)

			local greave = P("WardenGreave", Vector3.new(1.35, 1.85, 1.35), OBSIDIAN_SLATE)
			at(lKey, greave, 0, -0.25, 0)
			local gCrack = P("GreaveCrack", Vector3.new(0.12, 1.50, 0.16), VOID_MAGENTA, NEON)
			at(lKey, gCrack, 0, -0.25, -0.72, 0, 0, sign * -22)
		end
	end
end

return VoidRig
