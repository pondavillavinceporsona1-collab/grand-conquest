-- DemonRig.luau
-- High-Detail Anatomical 3D Rigs for Infernal Demon Faction
-- All 9 Units engineered with rich 3D anatomy matching demon_ref.png
-- Hell Hound engineered specifically as a cute "small human dog" companion!

local DemonRig = {}

local CRIMSON_SKIN    = Color3.fromRGB(205, 35, 35)
local BLOOD_RED       = Color3.fromRGB(165, 25, 30)
local DARK_CRIMSON    = Color3.fromRGB(115, 16, 22)
local HORN_BLACK      = Color3.fromRGB(20, 16, 20)
local OBSIDIAN_PLATE  = Color3.fromRGB(26, 22, 28)
local DARK_IRON       = Color3.fromRGB(38, 38, 45)
local STEEL_GREY      = Color3.fromRGB(130, 135, 145)
local MAGMA_ORANGE    = Color3.fromRGB(255, 95, 20)
local MOLTEN_GOLD     = Color3.fromRGB(255, 175, 40)
local HELLFIRE_YELLOW = Color3.fromRGB(255, 225, 60)
local ROYAL_GOLD      = Color3.fromRGB(245, 195, 55)
local IMPERIAL_GOLD   = Color3.fromRGB(255, 205, 50)
local SOUL_CYAN       = Color3.fromRGB(105, 235, 255)
local BONE_WHITE      = Color3.fromRGB(225, 225, 230)
local ABYSSAL_NAVY    = Color3.fromRGB(20, 22, 40)

-- Cute Dog Palette for Hell Hound
local DOG_COAT_DARK   = Color3.fromRGB(32, 26, 30)
local DOG_COAT_WARM   = Color3.fromRGB(48, 36, 40)
local DOG_NOSE_BLACK  = Color3.fromRGB(14, 14, 16)
local DOG_TONGUE_PINK = Color3.fromRGB(245, 120, 140)
local DOG_EAR_INNER   = Color3.fromRGB(215, 135, 145)
local DOG_COLLAR_RED  = Color3.fromRGB(195, 25, 30)

function DemonRig.build(ctx)
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
	local METAL = Enum.Material.Metal
	local SMOOTH = Enum.Material.SmoothPlastic
	local NEON = Enum.Material.Neon
	local FABRIC = Enum.Material.Fabric

	local function P(name, size, color, mat, shape)
		return makePart(name, size * s, color, mat or METAL, model, shape)
	end
	local function W(name, size, color, mat)
		return makeWedge(name, size * s, color, mat or METAL, model)
	end
	local function at(limbKey, part, x, y, z, rx, ry, rz)
		local cf = CFrame.new(x * s, y * s, z * s)
		if rx or ry or rz then
			cf = cf * CFrame.Angles(math.rad(rx or 0), math.rad(ry or 0), math.rad(rz or 0))
		end
		attach(limbKey, part, cf)
		return part
	end
		local function makeHandGripCF(limbKey, pitchDeg, rollDeg, yawDeg, extraY, extraZ)
		local yOff = (-0.95 + (extraY or 0)) * s
		local zOff = (-0.15 + (extraZ or 0)) * s
		local cf = CFrame.new(0, yOff, zOff)
		if pitchDeg or rollDeg or yawDeg then
			cf = cf * CFrame.Angles(math.rad(pitchDeg or 0), math.rad(yawDeg or 0), math.rad(rollDeg or 0))
		end
		return cf
	end

	local function buildDemonSword(gripKey, lengthScale, bladeColor, edgeColor, guardColor, pommelColor, pitchAngle, guardWidth, bladeWidth, tipStyle)
		local sc = lengthScale or 1
		local pitch = pitchAngle or -38
		local radP = math.rad(pitch)
		local sinP = math.sin(radP)
		local cosP = math.cos(radP)

		-- Helper to calculate rotated position along blade axis (d > 0 extends down and forward along blade)
		local function swordPos(d)
			return 0, -0.85 + d * sinP, -0.2 - d * cosP
		end

		local bW = (bladeWidth or 0.65) * sc
		local gW = (guardWidth or 1.85) * sc

		-- Pommel behind hand
		local px, py, pz = swordPos(-0.45 * sc)
		at(gripKey, P("SwordPommel", Vector3.new(0.38, 0.38, 0.38) * sc, pommelColor or DARK_IRON, NEON, BALL), px, py, pz, pitch, 0, 0)

		-- Hilt Grip in hand
		local gx, gy, gz = swordPos(-0.10 * sc)
		at(gripKey, P("SwordGrip", Vector3.new(0.20, 0.20, 0.75 * sc), DARK_IRON, SMOOTH), gx, gy, gz, pitch, 0, 0)

		-- Crossguard
		local qx, qy, qz = swordPos(0.25 * sc)
		at(gripKey, P("Crossguard", Vector3.new(gW, 0.28 * sc, 0.35 * sc), guardColor or DARK_IRON, METAL), qx, qy, qz, pitch, 0, 0)
		at(gripKey, P("CrossGemL", Vector3.new(0.26, 0.26, 0.38) * sc, edgeColor or MAGMA_ORANGE, NEON), -gW * 0.44, qy, qz, pitch, 0, 0)
		at(gripKey, P("CrossGemR", Vector3.new(0.26, 0.26, 0.38) * sc, edgeColor or MAGMA_ORANGE, NEON), gW * 0.44, qy, qz, pitch, 0, 0)
		at(gripKey, P("GuardGem", Vector3.new(0.30, 0.30, 0.40) * sc, pommelColor or HELLFIRE_YELLOW, NEON), 0, qy, qz, pitch, 0, 0)

		-- Double-Edged Blade (Named "Weapon" for combat detection, length along Z!)
		local bladeLen = 2.50 * sc
		local bx, by, bz = swordPos(0.25 * sc + bladeLen * 0.5)
		at(gripKey, P("Weapon", Vector3.new(0.28 * sc, bW, bladeLen), bladeColor or OBSIDIAN_PLATE, METAL), bx, by, bz, pitch, 0, 0)

		-- Central Glowing Core / Fuller
		at(gripKey, P("HellfireCore", Vector3.new(0.32 * sc, bW * 0.48, bladeLen * 0.90), edgeColor or MAGMA_ORANGE, NEON), bx, by, bz, pitch, 0, 0)

		-- Blade Tip
		local tx, ty, tz = swordPos(0.25 * sc + bladeLen + 0.22 * sc)
		if tipStyle == "Executioner" then
			at(gripKey, P("BladeTip", Vector3.new(0.26 * sc, bW * 0.95, 0.45 * sc), bladeColor or OBSIDIAN_PLATE, METAL), tx, ty, tz, pitch, 0, 0)
			at(gripKey, P("BladeTipGlow", Vector3.new(0.12 * sc, bW * 0.60, 0.38 * sc), edgeColor or MAGMA_ORANGE, NEON), tx, ty, tz, pitch, 0, 0)
		else
			at(gripKey, P("BladeTip", Vector3.new(0.24 * sc, bW * 0.80, 0.45 * sc), bladeColor or OBSIDIAN_PLATE, METAL), tx, ty, tz, pitch, 0, 0)
		end
	end

	-- Default base limb coloring
	head.Color = OBSIDIAN_PLATE
	body.Color = OBSIDIAN_PLATE
	leftArm.Color = OBSIDIAN_PLATE
	rightArm.Color = OBSIDIAN_PLATE
	leftLeg.Color = OBSIDIAN_PLATE
	rightLeg.Color = OBSIDIAN_PLATE

	if uName == "Imp" then
		-- =====================================================================
		-- C-TIER 1: IMP (Small Stature, Crimson Skin, Dual Swept Horns, Claw Daggers)
		-- =====================================================================
		head.Color = CRIMSON_SKIN
		body.Color = CRIMSON_SKIN
		leftArm.Color = CRIMSON_SKIN
		rightArm.Color = CRIMSON_SKIN
		leftLeg.Color = CRIMSON_SKIN
		rightLeg.Color = CRIMSON_SKIN

		-- Dual Swept Demon Horns & Brow Ridges
		for _, sign in ipairs({ -1, 1 }) do
			local horn1 = W("HornBase", Vector3.new(0.20, 0.70, 0.35), HORN_BLACK, SMOOTH)
			at("Head", horn1, sign * 0.42, 0.65, 0.10, 25, 0, -sign * 18)
			local horn2 = W("HornTip", Vector3.new(0.14, 0.50, 0.25), HORN_BLACK, SMOOTH)
			at("Head", horn2, sign * 0.52, 1.05, 0.22, 45, 0, -sign * 25)

			local ear = W("ImpEar", Vector3.new(0.12, 0.40, 0.25), CRIMSON_SKIN, SMOOTH)
			at("Head", ear, sign * 0.62, 0.15, 0.10, -10, 0, -sign * 35)

			local eye = P("ImpEye", Vector3.new(0.24, 0.14, 0.15), MOLTEN_GOLD, NEON)
			at("Head", eye, sign * 0.30, 0.08, -0.52)
			local brow = P("ImpBrow", Vector3.new(0.30, 0.08, 0.18), HORN_BLACK, SMOOTH)
			at("Head", brow, sign * 0.30, 0.20, -0.54, 0, 0, -sign * 15)
			local fang = W("ImpFang", Vector3.new(0.08, 0.18, 0.12), BONE_WHITE, SMOOTH)
			at("Head", fang, sign * 0.22, -0.28, -0.55, 180, 0, 0)
		end

		-- Muscular Red Torso + Abdominal Ridges + Chest Chitin
		local chest = P("ImpChest", Vector3.new(2.10, 1.85, 1.15), CRIMSON_SKIN, SMOOTH)
		at("Body", chest, 0, 0, 0)
		local abs = P("ImpAbs", Vector3.new(1.20, 0.85, 0.18), DARK_CRIMSON, SMOOTH)
		at("Body", abs, 0, -0.35, -0.60)
		local belt = P("ImpBelt", Vector3.new(2.15, 0.25, 1.20), HORN_BLACK, SMOOTH)
		at("Body", belt, 0, -0.75, 0)
		local buckle = P("ImpBuckle", Vector3.new(0.35, 0.35, 0.15), MAGMA_ORANGE, NEON)
		at("Body", buckle, 0, -0.75, -0.62)

		for sp = 1, 4 do
			local spine = W("ImpSpine_" .. sp, Vector3.new(0.16, 0.55 - sp * 0.06, 0.45), HORN_BLACK, SMOOTH)
			at("Body", spine, 0, 0.65 - sp * 0.35, 0.65, 45, 0, 0)
		end

		-- Shoulders & Dual Spiked Claw Daggers
		for _, sideKey in ipairs({ "LeftArm", "RightArm" }) do
			local pauldron = P("ImpPauldron", Vector3.new(1.15, 0.35, 1.15), HORN_BLACK, SMOOTH)
			at(sideKey, pauldron, 0, 0.55, 0)
			local pSpike = W("PauldronSpike", Vector3.new(0.14, 0.40, 0.30), HORN_BLACK, SMOOTH)
			at(sideKey, pSpike, 0, 0.85, 0)
			local spur = W("ElbowSpur", Vector3.new(0.16, 0.45, 0.35), HORN_BLACK, SMOOTH)
			at(sideKey, spur, 0, -0.35, 0.55, 45, 0, 0)
			local bracer = P("ArmBracer", Vector3.new(1.08, 0.55, 1.08), DARK_CRIMSON, SMOOTH)
			at(sideKey, bracer, 0, -0.45, 0)

			-- Unified Claw Dagger Assembly locked into hand palm
			local gripCF = makeHandGripCF(sideKey, 25, 0, 0)
			local grip = P("DaggerGrip_" .. sideKey, Vector3.new(0.14, 0.65, 0.14), DARK_IRON, METAL, CYL)
			attach(sideKey, grip, gripCF)
			local dagger = P(sideKey == "RightArm" and "Weapon" or "OffhandClaw", Vector3.new(0.16, 1.35, 0.35), DARK_IRON, METAL)
			attach(sideKey, dagger, gripCF * CFrame.new(0, 0.75 * s, -0.15 * s) * CFrame.Angles(math.rad(20), 0, 0))
			local bladeGlow = P("BladeGlow_" .. sideKey, Vector3.new(0.08, 1.15, 0.15), MAGMA_ORANGE, NEON)
			attach(sideKey, bladeGlow, gripCF * CFrame.new(0, 0.75 * s, -0.28 * s) * CFrame.Angles(math.rad(20), 0, 0))
		end

		-- Legs: Spiked Knees & Red Boots
		for _, legKey in ipairs({ "LeftLeg", "RightLeg" }) do
			local knee = W("KneeSpike", Vector3.new(0.18, 0.42, 0.35), HORN_BLACK, SMOOTH)
			at(legKey, knee, 0, 0.25, -0.55, -25, 0, 0)
			local boot = P("ImpBoot", Vector3.new(1.08, 0.45, 1.08), DARK_CRIMSON, SMOOTH)
			at(legKey, boot, 0, -0.75, 0)
			local toeClaw = W("ToeClaw", Vector3.new(0.15, 0.18, 0.28), HORN_BLACK, SMOOTH)
			at(legKey, toeClaw, 0, -0.85, -0.65, -35, 0, 0)
		end

	elseif uName == "Hell Hound" then
		-- =====================================================================
		-- C-TIER 2: HELL HOUND ("Small Human Dog" - Cute Quadruped Companion Dog!)
		-- Adorable domestic canine anatomy: snout, button nose, lower jaw, floppy ears,
		-- collar with gold tag, perky wagging tail, cute paws with pad toes!
		-- =====================================================================
		head.Transparency = 1
		body.Transparency = 1
		leftArm.Transparency = 1
		rightArm.Transparency = 1
		leftLeg.Transparency = 1
		rightLeg.Transparency = 1

		-- Cute Dog Head: Rounded Head Base
		local dogHeadBase = P("DogHeadBase", Vector3.new(1.25, 1.05, 1.15), DOG_COAT_DARK, SMOOTH)
		at("Head", dogHeadBase, 0, 0.05, 0)

		-- Cute Dog Muzzle / Snout (-Z is forward)
		local snout = P("DogSnout", Vector3.new(0.68, 0.42, 0.70), DOG_COAT_WARM, SMOOTH)
		at("Head", snout, 0, -0.15, -0.75)
		local snoutBridge = P("SnoutBridge", Vector3.new(0.50, 0.20, 0.60), DOG_COAT_DARK, SMOOTH)
		at("Head", snoutBridge, 0, 0.02, -0.70)

		-- Cute Glossy Black Dog Button Nose on tip of snout
		local dogNose = P("DogNose", Vector3.new(0.26, 0.18, 0.16), DOG_NOSE_BLACK, SMOOTH)
		at("Head", dogNose, 0, -0.06, -1.15)

		-- Dog Lower Jaw & Mouth with little pink puppy tongue
		local jaw = P("DogLowerJaw", Vector3.new(0.50, 0.16, 0.60), DOG_COAT_DARK, SMOOTH)
		at("Head", jaw, 0, -0.36, -0.70)
		local tongue = P("DogTongue", Vector3.new(0.22, 0.08, 0.30), DOG_TONGUE_PINK, SMOOTH)
		at("Head", tongue, 0, -0.30, -0.92)

		-- Cute Floppy / Folded Dog Ears on sides of head
		for _, sign in ipairs({ -1, 1 }) do
			local earBase = P("EarBase", Vector3.new(0.20, 0.40, 0.32), DOG_COAT_DARK, SMOOTH)
			at("Head", earBase, sign * 0.60, 0.42, 0.10, 0, 0, -sign * 25)
			local earFlap = P("EarFlap", Vector3.new(0.18, 0.50, 0.28), DOG_COAT_DARK, SMOOTH)
			at("Head", earFlap, sign * 0.70, 0.08, 0.12, -15, 0, sign * 20)
			local earInner = P("EarInner", Vector3.new(0.08, 0.36, 0.18), DOG_EAR_INNER, SMOOTH)
			at("Head", earInner, sign * 0.68, 0.08, 0.08)

			-- Large Warm Expressive Puppy Eyes with Highlights
			local eye = P("PuppyEye", Vector3.new(0.24, 0.24, 0.14), MOLTEN_GOLD, NEON)
			at("Head", eye, sign * 0.34, 0.12, -0.58)
			local pupil = P("EyePupil", Vector3.new(0.12, 0.15, 0.15), HORN_BLACK, SMOOTH)
			at("Head", pupil, sign * 0.34, 0.12, -0.62)

			-- Subtle tiny cute devil puppy horns peeking between ears
			local miniHorn = W("MiniHorn", Vector3.new(0.10, 0.32, 0.16), MAGMA_ORANGE, NEON)
			at("Head", miniHorn, sign * 0.28, 0.60, 0.05, 25, 0, -sign * 15)
		end

		-- Cute Dog Collar around neck with shiny golden bone / medallion tag
		local collar = P("DogCollar", Vector3.new(1.10, 0.22, 1.10), DOG_COLLAR_RED, SMOOTH)
		at("Head", collar, 0, -0.30, 0.32)
		local dogTag = P("GoldDogTag", Vector3.new(0.25, 0.25, 0.08), ROYAL_GOLD, METAL)
		at("Head", dogTag, 0, -0.48, -0.20)
		local tagGem = P("TagGem", Vector3.new(0.12, 0.12, 0.10), MAGMA_ORANGE, NEON)
		at("Head", tagGem, 0, -0.48, -0.23)

		-- Compact Canine Body: Rounded Dog Chest & Warm Belly
		local dogChest = P("DogChest", Vector3.new(1.55, 1.35, 1.75), DOG_COAT_DARK, SMOOTH)
		at("Body", dogChest, 0, 0, 0)
		local dogBelly = P("DogBelly", Vector3.new(1.15, 0.20, 1.35), DOG_COAT_WARM, SMOOTH)
		at("Body", dogBelly, 0, -0.68, 0)
		local backRidge = P("BackFluff", Vector3.new(0.40, 0.20, 1.45), DOG_COAT_DARK, SMOOTH)
		at("Body", backRidge, 0, 0.68, 0)

		-- Adorable Wagging Puppy Tail (curling up happily!)
		local tailBase = P("DogTailBase", Vector3.new(0.20, 0.20, 0.55), DOG_COAT_DARK, SMOOTH)
		at("Body", tailBase, 0, 0.32, 0.95, 45, 0, 0)
		local tailMid = P("DogTailMid", Vector3.new(0.18, 0.18, 0.45), DOG_COAT_DARK, SMOOTH)
		at("Body", tailMid, 0, 0.65, 1.25, 75, 0, 0)
		local tailTip = P("DogTailTip", Vector3.new(0.16, 0.16, 0.30), MAGMA_ORANGE, NEON)
		at("Body", tailTip, 0, 0.95, 1.20, 105, 0, 0)

		-- Canine Legs & Cute Little Paws (Lifted cleanly to avoid floor clipping)
		for _, sideKey in ipairs({ "LeftArm", "RightArm" }) do
			local frontLeg = P("FrontLeg", Vector3.new(0.70, 0.75, 0.70), DOG_COAT_DARK, SMOOTH)
			at(sideKey, frontLeg, 0, -0.25, 0)
			local frontPaw = P("FrontPaw", Vector3.new(0.80, 0.28, 0.85), DOG_COAT_DARK, SMOOTH)
			at(sideKey, frontPaw, 0, -0.52, -0.10)
			for t = -1, 1 do
				local toe = P("FrontToe_" .. t, Vector3.new(0.18, 0.15, 0.20), DOG_COAT_WARM, SMOOTH)
				at(sideKey, toe, t * 0.24, -0.55, -0.50)
			end
		end
		for _, legKey in ipairs({ "LeftLeg", "RightLeg" }) do
			local rearThigh = P("RearThigh", Vector3.new(0.85, 0.85, 0.85), DOG_COAT_DARK, SMOOTH)
			at(legKey, rearThigh, 0, -0.22, 0)
			local rearPaw = P("RearPaw", Vector3.new(0.80, 0.28, 0.85), DOG_COAT_DARK, SMOOTH)
			at(legKey, rearPaw, 0, -0.52, -0.10)
			for t = -1, 1 do
				local toe = P("RearToe_" .. t, Vector3.new(0.18, 0.15, 0.20), DOG_COAT_WARM, SMOOTH)
				at(legKey, toe, t * 0.24, -0.55, -0.50)
			end
		end

	elseif uName == "Demon Soldier" then
		-- =====================================================================
		-- C-TIER 3: DEMON SOLDIER (Horned Barbute Helm, Bull Horns, Blood Armor, Cleaver & Shield)
		-- =====================================================================
		head.Color = BLOOD_RED
		body.Color = BLOOD_RED
		leftArm.Color = BLOOD_RED
		rightArm.Color = BLOOD_RED
		leftLeg.Color = BLOOD_RED
		rightLeg.Color = BLOOD_RED

		-- Horned Barbute Helmet
		local helm = P("DemonHelm", Vector3.new(1.22, 0.55, 1.22), BLOOD_RED, METAL)
		at("Head", helm, 0, 0.45, 0)
		local noseGuard = P("NoseGuard", Vector3.new(0.22, 0.45, 0.22), DARK_IRON, METAL)
		at("Head", noseGuard, 0, 0.05, -0.58)
		local helmRim = P("HelmRim", Vector3.new(1.28, 0.12, 1.28), DARK_IRON, METAL)
		at("Head", helmRim, 0, 0.22, 0)

		-- Curved Bull Horns (Layered for 3D sweep)
		for _, sign in ipairs({ -1, 1 }) do
			local horn1 = W("BullHorn1", Vector3.new(0.22, 0.85, 0.45), HORN_BLACK, SMOOTH)
			at("Head", horn1, sign * 0.62, 0.55, -0.10, 15, 0, -sign * 35)
			local horn2 = W("BullHorn2", Vector3.new(0.16, 0.65, 0.32), HORN_BLACK, SMOOTH)
			at("Head", horn2, sign * 0.95, 1.05, -0.12, 25, 0, -sign * 55)

			local eyeSlit = P("EyeSlit", Vector3.new(0.35, 0.12, 0.18), MAGMA_ORANGE, NEON)
			at("Head", eyeSlit, sign * 0.28, 0.08, -0.55)
		end

		-- Blood-Red Heavy Chestplate & Dark Iron Belt
		local chest = P("SoldierChest", Vector3.new(2.15, 1.85, 1.20), BLOOD_RED, METAL)
		at("Body", chest, 0, 0, 0)
		local chestPlaque = P("ChestPlaque", Vector3.new(1.10, 1.10, 0.18), DARK_CRIMSON, METAL)
		at("Body", chestPlaque, 0, 0.15, -0.66)
		local belt = P("SoldierBelt", Vector3.new(2.18, 0.28, 1.25), DARK_IRON, METAL)
		at("Body", belt, 0, -0.65, 0)
		local buckle = P("BeltBuckle", Vector3.new(0.42, 0.35, 0.22), MAGMA_ORANGE, NEON)
		at("Body", buckle, 0, -0.65, -0.65)
		for t = -1, 1, 2 do
			local tasset = P("Tasset_" .. t, Vector3.new(0.55, 0.65, 0.18), BLOOD_RED, METAL)
			at("Body", tasset, t * 0.65, -0.95, -0.60)
		end

		-- Spiked Pauldrons & Vambraces
		for _, sideKey in ipairs({ "LeftArm", "RightArm" }) do
			local pauldron = P("SoldierPauldron", Vector3.new(1.30, 0.55, 1.30), BLOOD_RED, METAL)
			at(sideKey, pauldron, 0, 0.65, 0)
			local pRim = P("PauldronRim", Vector3.new(1.38, 0.14, 1.38), DARK_IRON, METAL)
			at(sideKey, pRim, 0, 0.55, 0)
			local pSpike = W("PauldronSpike", Vector3.new(0.18, 0.45, 0.35), HORN_BLACK, SMOOTH)
			at(sideKey, pSpike, 0, 0.95, 0)

			local bracer = P("Vambrace", Vector3.new(1.10, 0.65, 1.10), DARK_CRIMSON, METAL)
			at(sideKey, bracer, 0, -0.35, 0)
			local elbow = W("ElbowPlate", Vector3.new(0.18, 0.35, 0.30), DARK_IRON, METAL)
			at(sideKey, elbow, 0, -0.25, 0.55, 45, 0, 0)
		end

		-- Weapons: Serrated Demon Cleaver (RightArm) + Jagged Kite Shield (LeftArm)
				-- Weapon: Serrated Demon Cleaver (RightArm, held at hip angled down & forward at -38°, matching Knight)
		buildDemonSword("RightArm", 1.05, BLOOD_RED, MAGMA_ORANGE, DARK_IRON, MAGMA_ORANGE, -38, 1.45, 0.60, "Cleaver")

		-- LeftArm: Jagged Demon Kite Shield strapped to forearm
		local shieldStrap1 = P("ShieldStrap1", Vector3.new(1.15, 0.15, 1.15), DARK_IRON, METAL)
		at("LeftArm", shieldStrap1, 0, -0.25, 0)
		local shieldStrap2 = P("ShieldStrap2", Vector3.new(1.15, 0.15, 1.15), DARK_IRON, METAL)
		at("LeftArm", shieldStrap2, 0, -0.55, 0)
		local shield = P("DemonShield", Vector3.new(0.35, 2.85, 1.65), BLOOD_RED, METAL)
		at("LeftArm", shield, -0.65, -0.35, 0)
		local shieldCrest = P("ShieldCrest", Vector3.new(0.15, 2.05, 0.45), MAGMA_ORANGE, NEON)
		at("LeftArm", shieldCrest, -0.85, -0.35, 0)
		local shieldBoss = P("ShieldBoss", Vector3.new(0.25, 0.55, 0.55), DARK_IRON, METAL, BALL)
		at("LeftArm", shieldBoss, -0.88, -0.35, 0)

		-- Greaves & Sabatons
		for _, legKey in ipairs({ "LeftLeg", "RightLeg" }) do
			local knee = P("KneeGuard", Vector3.new(1.10, 0.45, 0.35), DARK_IRON, METAL)
			at(legKey, knee, 0, 0.25, -0.55)
			local greave = P("SoldierGreave", Vector3.new(1.08, 0.75, 1.08), DARK_CRIMSON, METAL)
			at(legKey, greave, 0, -0.45, 0)
			local boot = P("Sabaton", Vector3.new(1.08, 0.42, 1.15), BLOOD_RED, METAL)
			at(legKey, boot, 0, -0.75, -0.05)
		end

	elseif uName == "Blood Knight" then
		-- =====================================================================
		-- B-TIER 1: BLOOD KNIGHT (Executioner Helm, Ribbed Armor, Blood Claymore)
		-- =====================================================================
		local helm = P("ExecutionerHelm", Vector3.new(1.25, 0.55, 1.25), OBSIDIAN_PLATE, METAL)
		at("Head", helm, 0, 0.45, 0)
		local centerSpike = W("CenterSpike", Vector3.new(0.20, 1.05, 0.38), BLOOD_RED, METAL)
		at("Head", centerSpike, 0, 1.05, -0.15, 20, 0, 0)
		for _, sign in ipairs({ -1, 1 }) do
			local cheekHorn = W("CheekHorn", Vector3.new(0.16, 0.75, 0.32), BLOOD_RED, METAL)
			at("Head", cheekHorn, sign * 0.58, 0.65, -0.15, 0, 0, -sign * 28)
			local cheekPlate = P("CheekPlate", Vector3.new(0.18, 0.45, 0.35), DARK_CRIMSON, METAL)
			at("Head", cheekPlate, sign * 0.55, 0.05, -0.35)

			local eye = P("BloodVisor", Vector3.new(0.35, 0.12, 0.18), Color3.fromRGB(255, 35, 45), NEON)
			at("Head", eye, sign * 0.28, 0.08, -0.56)
		end

		-- Gothic Ribbed Chestplate with 3 Skeletal Rib Pairs
		local chest = P("BloodChest", Vector3.new(2.18, 1.95, 1.25), OBSIDIAN_PLATE, METAL)
		at("Body", chest, 0, 0, 0)
		for r = 1, 3 do
			local rib = P("BloodRib_" .. r, Vector3.new(1.85 - r * 0.20, 0.18, 0.22), BLOOD_RED, METAL)
			at("Body", rib, 0, 0.45 - r * 0.32, -0.66)
		end
		local spineBack = P("DorsalSpine", Vector3.new(0.25, 1.65, 0.25), BLOOD_RED, METAL)
		at("Body", spineBack, 0, 0, 0.66)
		local belt = P("BloodBelt", Vector3.new(2.20, 0.30, 1.28), DARK_IRON, METAL)
		at("Body", belt, 0, -0.65, 0)
		for t = -1, 1, 2 do
			local tasset = P("GothicTasset_" .. t, Vector3.new(0.60, 0.70, 0.18), OBSIDIAN_PLATE, METAL)
			at("Body", tasset, t * 0.65, -0.95, -0.62)
		end

		-- Heavy Spiked Pauldrons with Glowing Crimson Trim
		for _, sideKey in ipairs({ "LeftArm", "RightArm" }) do
			local pauldron = P("SpikedPauldron", Vector3.new(1.38, 0.65, 1.38), OBSIDIAN_PLATE, METAL)
			at(sideKey, pauldron, 0, 0.65, 0)
			local pauldronTrim = P("PauldronTrim", Vector3.new(1.45, 0.16, 1.45), Color3.fromRGB(255, 35, 45), NEON)
			at(sideKey, pauldronTrim, 0, 0.55, 0)
			local pSpike = W("PauldronBlade", Vector3.new(0.18, 0.55, 0.45), BLOOD_RED, METAL)
			at(sideKey, pSpike, 0, 1.05, 0)

			local gauntlet = P("Gauntlet", Vector3.new(1.15, 0.75, 1.15), DARK_CRIMSON, METAL)
			at(sideKey, gauntlet, 0, -0.45, 0)
			local knuckle = P("KnuckleGuard", Vector3.new(0.35, 0.20, 0.35), BLOOD_RED, METAL)
			at(sideKey, knuckle, 0, -0.85, -0.45)
		end

		-- Weapon: Massive Jagged Blood Claymore Greatsword (RightArm, locked into hand palm!)
				-- Weapon: Massive Jagged Blood Claymore (RightArm, held at hip angled down & forward at -38°, matching Knight)
		buildDemonSword("RightArm", 1.25, OBSIDIAN_PLATE, Color3.fromRGB(255, 35, 45), DARK_IRON, BLOOD_RED, -38, 1.85, 0.70, "Claymore")

		-- Gothic Greaves & Knee Guards
		for _, legKey in ipairs({ "LeftLeg", "RightLeg" }) do
			local kneePlate = P("GothicKnee", Vector3.new(1.15, 0.45, 0.38), BLOOD_RED, METAL)
			at(legKey, kneePlate, 0, 0.25, -0.55)
			local greave = P("GothicGreave", Vector3.new(1.10, 0.85, 1.10), OBSIDIAN_PLATE, METAL)
			at(legKey, greave, 0, -0.45, 0)
			local sabaton = P("GothicSabaton", Vector3.new(1.12, 0.38, 1.20), DARK_CRIMSON, METAL)
			at(legKey, sabaton, 0, -0.80, -0.06)
		end

	elseif uName == "Flame Caster" then
		-- =====================================================================
		-- B-TIER 2: FLAME CASTER (Horned Ram Cowl, Magma Robes, Blazing Torch Staff & Orb)
		-- =====================================================================
		local cowl = P("RamCowl", Vector3.new(1.28, 0.65, 1.28), OBSIDIAN_PLATE, FABRIC)
		at("Head", cowl, 0, 0.35, 0)
		local cowlHem = P("CowlHem", Vector3.new(1.35, 0.15, 1.35), BLOOD_RED, FABRIC)
		at("Head", cowlHem, 0, 0.15, 0)
		for _, sign in ipairs({ -1, 1 }) do
			local horn1 = W("RamHorn1", Vector3.new(0.22, 0.75, 0.45), HORN_BLACK, SMOOTH)
			at("Head", horn1, sign * 0.58, 0.65, -0.15, -15, 0, -sign * 32)
			local horn2 = W("RamHorn2", Vector3.new(0.16, 0.55, 0.32), HORN_BLACK, SMOOTH)
			at("Head", horn2, sign * 0.85, 0.95, 0.05, -35, 0, -sign * 45)
			local hornSpur = W("HornSpur", Vector3.new(0.12, 0.45, 0.25), HORN_BLACK, SMOOTH)
			at("Head", hornSpur, sign * 0.95, 1.25, 0.15, -45, 0, -sign * 55)

			local eye = P("FlameEye", Vector3.new(0.26, 0.14, 0.15), MAGMA_ORANGE, NEON)
			at("Head", eye, sign * 0.32, 0.08, -0.54)
		end

		-- Magma Robes & Glowing Runic Sigil
		local robe = P("CasterRobe", Vector3.new(2.18, 1.95, 1.25), OBSIDIAN_PLATE, FABRIC)
		at("Body", robe, 0, 0, 0)
		local sigil = P("MagmaSigil", Vector3.new(0.85, 0.85, 0.15), MAGMA_ORANGE, NEON, BALL)
		at("Body", sigil, 0, 0.25, -0.66)
		local sash = P("RobeSash", Vector3.new(2.22, 0.30, 1.28), BLOOD_RED, FABRIC)
		at("Body", sash, 0, -0.55, 0)
		local sashKnot = P("SashKnot", Vector3.new(0.35, 0.55, 0.20), MAGMA_ORANGE, NEON)
		at("Body", sashKnot, -0.45, -0.75, -0.66)
		-- Floating Rune Arc on back
		for r = -2, 2 do
			local rAngle = r * 0.35
			local rune = P("FlameRune_" .. r, Vector3.new(0.18, 0.35, 0.12), MAGMA_ORANGE, NEON)
			at("Body", rune, math.sin(rAngle) * 1.35, 0.65 + math.cos(rAngle) * 0.45, 0.75)
		end

		-- Shoulder Mantle & Cuffs
		for _, sideKey in ipairs({ "LeftArm", "RightArm" }) do
			local mantle = P("RobeShoulder", Vector3.new(1.25, 0.45, 1.25), DARK_CRIMSON, FABRIC)
			at(sideKey, mantle, 0, 0.65, 0)
			local cuff = P("RobeCuff", Vector3.new(1.15, 0.35, 1.15), MAGMA_ORANGE, NEON)
			at(sideKey, cuff, 0, -0.55, 0)
			local wristGlow = P("WristGlow", Vector3.new(1.20, 0.12, 1.20), HELLFIRE_YELLOW, NEON)
			at(sideKey, wristGlow, 0, -0.45, 0)
			local bracer = P("BrazierBracer", Vector3.new(1.22, 0.35, 1.22), DARK_IRON, METAL)
			at(sideKey, bracer, 0, -0.35, 0)
		end

		-- Weapon: Blazing Hellfire Torch Staff (Held in LeftArm matching demon_ref.png!)
		local torchGripCF = makeHandGripCF("LeftArm", -12, 8, 0)
		local staffShaft = P("Weapon", Vector3.new(0.22, 4.25, 0.22), DARK_IRON, METAL, CYL)
		attach("LeftArm", staffShaft, torchGripCF * CFrame.new(0, 1.15 * s, 0))
		local staffPommel = P("StaffPommel", Vector3.new(0.32, 0.32, 0.32), DARK_IRON, METAL, BALL)
		attach("LeftArm", staffPommel, torchGripCF * CFrame.new(0, -1.05 * s, 0))
		local brazier = P("TorchBrazier", Vector3.new(0.55, 0.45, 0.55), DARK_IRON, METAL)
		attach("LeftArm", brazier, torchGripCF * CFrame.new(0, 3.15 * s, 0))
		local flameHead = P("TorchFlame", Vector3.new(1.15, 1.45, 1.15), MAGMA_ORANGE, NEON, BALL)
		attach("LeftArm", flameHead, torchGripCF * CFrame.new(0, 3.75 * s, 0))
		local flameCore = P("FlameCore", Vector3.new(0.68, 0.95, 0.68), HELLFIRE_YELLOW, NEON, BALL)
		attach("LeftArm", flameCore, torchGripCF * CFrame.new(0, 3.75 * s, 0))

		-- Right Arm: Swirling Hellfire Casting Orb in Hand
		local rightOrb = P("HellfireOrb", Vector3.new(0.85, 0.85, 0.85), MAGMA_ORANGE, NEON, BALL)
		at("RightArm", rightOrb, 0, -0.95, -0.25)
		local orbCore = P("OrbCore", Vector3.new(0.48, 0.48, 0.48), HELLFIRE_YELLOW, NEON, BALL)
		at("RightArm", orbCore, 0, -0.95, -0.25)

		-- Skirt / Robe Trim
		for _, legKey in ipairs({ "LeftLeg", "RightLeg" }) do
			local skirt = P("RobeSkirt", Vector3.new(1.12, 1.25, 1.12), OBSIDIAN_PLATE, FABRIC)
			at(legKey, skirt, 0, 0, 0)
			local hem = P("SkirtHem", Vector3.new(1.18, 0.15, 1.18), BLOOD_RED, FABRIC)
			at(legKey, hem, 0, -0.60, 0)
		end

	elseif uName == "Soul Reaper" then
		-- =====================================================================
		-- B-TIER 3: SOUL REAPER (Grim Reaper Cowl, Soul Eyes, Chains, Colossal Scythe)
		-- =====================================================================
		head.Color = HORN_BLACK
		body.Color = HORN_BLACK
		leftArm.Color = HORN_BLACK
		rightArm.Color = HORN_BLACK
		leftLeg.Color = HORN_BLACK
		rightLeg.Color = HORN_BLACK

		-- Grim Cowl & Shadow Hood
		local cowl = P("ReaperCowl", Vector3.new(1.38, 1.35, 1.35), HORN_BLACK, FABRIC)
		at("Head", cowl, 0, 0.15, 0.05)
		local hoodPeak = W("HoodPeak", Vector3.new(0.25, 0.45, 0.55), HORN_BLACK, FABRIC)
		local cowlFlap1 = W("CowlFlap1", Vector3.new(0.18, 0.65, 0.45), HORN_BLACK, FABRIC)
		at("Head", cowlFlap1, -0.55, -0.25, 0.15, 15, 0, 25)
		local cowlFlap2 = W("CowlFlap2", Vector3.new(0.18, 0.65, 0.45), HORN_BLACK, FABRIC)
		at("Head", cowlFlap2, 0.55, -0.25, 0.15, 15, 0, -25)
		at("Head", hoodPeak, 0, 0.85, -0.35, -15, 0, 0)
		local shadowFace = P("ShadowFace", Vector3.new(0.95, 0.95, 0.18), Color3.fromRGB(10, 8, 12), SMOOTH)
		at("Head", shadowFace, 0, 0, -0.52)

		-- Glowing Spectral Cyan Soul Eyes
		for _, sign in ipairs({ -1, 1 }) do
			local eye = P("SoulEye", Vector3.new(0.24, 0.12, 0.15), SOUL_CYAN, NEON)
			at("Head", eye, sign * 0.32, 0.05, -0.58)
		end

		-- Tattered Robe & Ragged Cloak
		local robe = P("ReaperRobe", Vector3.new(2.18, 1.95, 1.25), HORN_BLACK, FABRIC)
		at("Body", robe, 0, 0, 0)
		-- Exposed Skeletal Ribs on Chest
		for r = 1, 3 do
			local boneRib = P("BoneRib_" .. r, Vector3.new(1.65 - r * 0.20, 0.15, 0.18), BONE_WHITE, SMOOTH)
			at("Body", boneRib, 0, 0.45 - r * 0.32, -0.66)
		end
		local cloak = P("ReaperCloak", Vector3.new(2.45, 3.25, 0.28), HORN_BLACK, FABRIC)
		at("Body", cloak, 0, -0.75, 0.68, 8, 0, 0)

		-- Hanging Soul Chains Draped Across Torso
		for ch = 1, 2 do
			local sign = (ch == 1) and 1 or -1
			local chain = P("SoulChain_" .. ch, Vector3.new(2.35, 0.22, 0.18), DARK_IRON, METAL)
			at("Body", chain, 0, 0.15, -0.66, 0, 0, sign * 35)
		end
		local chestPendant = P("SoulPendant", Vector3.new(0.35, 0.35, 0.15), SOUL_CYAN, NEON, BALL)
		at("Body", chestPendant, 0, 0.15, -0.70)
		local gorget = P("SoulGorget", Vector3.new(1.85, 0.25, 1.15), DARK_IRON, METAL)
		at("Body", gorget, 0, 0.75, 0)
		-- Soul Lantern on hip
		local lantern = P("SoulLantern", Vector3.new(0.45, 0.75, 0.45), DARK_IRON, METAL)
		at("Body", lantern, -1.05, -0.65, 0)
		local lanternCore = P("LanternCore", Vector3.new(0.30, 0.45, 0.30), SOUL_CYAN, NEON)
		at("Body", lanternCore, -1.05, -0.65, 0)
		for sp = 1, 2 do
			local wisp = P("SoulWisp_" .. sp, Vector3.new(0.25, 0.25, 0.25), SOUL_CYAN, NEON, BALL)
			at("Body", wisp, (sp == 1 and -1.25 or 1.25), 0.85, 0.35)
		end
		for r = 4, 5 do
			local boneRib = P("BoneRib_" .. r, Vector3.new(1.45 - r * 0.15, 0.15, 0.18), BONE_WHITE, SMOOTH)
			at("Body", boneRib, 0, 0.45 - r * 0.28, -0.66)
		end

		-- Armored Gauntlets
		for _, sideKey in ipairs({ "LeftArm", "RightArm" }) do
			local pauldron = P("TatteredPauldron", Vector3.new(1.25, 0.55, 1.25), HORN_BLACK, FABRIC)
			at(sideKey, pauldron, 0, 0.65, 0)
			local gauntlet = P("SoulGauntlet", Vector3.new(1.12, 0.75, 1.12), DARK_IRON, METAL)
			at(sideKey, gauntlet, 0, -0.45, 0)
			local gem = P("GauntletGem", Vector3.new(0.18, 0.18, 0.18), SOUL_CYAN, NEON)
			at(sideKey, gem, 0, -0.45, -0.58)
		end

		-- Weapon: Colossal Curved Soul Scythe (Held in LeftArm matching demon_ref.png!)
		local scytheGripCF = makeHandGripCF("LeftArm", -15, 12, 0)
		local scytheShaft = P("Weapon", Vector3.new(0.22, 5.05, 0.22), DARK_IRON, METAL, CYL)
		attach("LeftArm", scytheShaft, scytheGripCF * CFrame.new(0, 1.45 * s, 0))
		local scythePommel = P("ScythePommel", Vector3.new(0.35, 0.45, 0.35), BONE_WHITE, SMOOTH)
		attach("LeftArm", scythePommel, scytheGripCF * CFrame.new(0, -1.15 * s, 0))
		local scytheSocket = P("ScytheSocket", Vector3.new(0.42, 0.32, 0.42), DARK_IRON, METAL)
		attach("LeftArm", scytheSocket, scytheGripCF * CFrame.new(0, 3.85 * s, 0))
		local scytheBlade = W("ScytheBlade", Vector3.new(0.15, 2.35, 1.65), BONE_WHITE, METAL)
		attach("LeftArm", scytheBlade, scytheGripCF * CFrame.new(0, 4.05 * s, -0.75 * s) * CFrame.Angles(math.rad(75), 0, 0))
		local soulEdge = P("SoulEdge", Vector3.new(0.08, 2.15, 0.22), SOUL_CYAN, NEON)
		attach("LeftArm", soulEdge, scytheGripCF * CFrame.new(0, 4.05 * s, -1.55 * s) * CFrame.Angles(math.rad(75), 0, 0))
		local backHook = W("BackHook", Vector3.new(0.12, 0.95, 0.65), DARK_IRON, METAL)
		attach("LeftArm", backHook, scytheGripCF * CFrame.new(0, 3.95 * s, 0.45 * s) * CFrame.Angles(math.rad(-45), 0, 0))

		-- Right Arm: Spectral Soul Plasma Orb in Hand
		local soulHandOrb = P("SoulHandOrb", Vector3.new(0.75, 0.75, 0.75), SOUL_CYAN, NEON, BALL)
		at("RightArm", soulHandOrb, 0, -0.95, -0.25)
		local soulCore = P("SoulCore", Vector3.new(0.38, 0.38, 0.38), Color3.fromRGB(255, 255, 255), NEON, BALL)
		at("RightArm", soulCore, 0, -0.95, -0.25)

		-- Legs: Flowing Robe Skirt
		for _, legKey in ipairs({ "LeftLeg", "RightLeg" }) do
			local skirt = P("ReaperSkirt", Vector3.new(1.12, 1.25, 1.12), HORN_BLACK, FABRIC)
			at(legKey, skirt, 0, 0, 0)
			local boot = P("ReaperBoot", Vector3.new(1.08, 0.45, 1.08), DARK_IRON, METAL)
			at(legKey, boot, 0, -0.75, 0)
		end

	elseif uName == "Demon Brute" then
		-- =====================================================================
		-- A-TIER 1: DEMON BRUTE (Titan Colossus, Sweeping Horns, Magma Armor, Molten Claw)
		-- =====================================================================
		local helm = P("TitanHelm", Vector3.new(1.28, 0.65, 1.28), OBSIDIAN_PLATE, METAL)
		at("Head", helm, 0, 0.45, 0)
		local jawGuard = P("TitanJaw", Vector3.new(1.15, 0.35, 0.45), DARK_IRON, METAL)
		at("Head", jawGuard, 0, -0.35, -0.55)
		for _, sign in ipairs({ -1, 1 }) do
			local horn1 = W("TitanHorn1", Vector3.new(0.26, 1.55, 0.55), HORN_BLACK, SMOOTH)
			at("Head", horn1, sign * 0.68, 0.75, -0.10, 15, 0, -sign * 32)
			local horn2 = W("TitanHorn2", Vector3.new(0.18, 0.85, 0.35), HORN_BLACK, SMOOTH)
			at("Head", horn2, sign * 1.15, 1.55, 0.10, 35, 0, -sign * 45)
			local tusk = W("TitanTusk", Vector3.new(0.14, 0.45, 0.25), BONE_WHITE, SMOOTH)
			at("Head", tusk, sign * 0.45, -0.25, -0.65, 180, 0, -sign * 15)

			local eye = P("TitanEye", Vector3.new(0.35, 0.14, 0.18), Color3.fromRGB(255, 45, 45), NEON)
			at("Head", eye, sign * 0.35, 0.08, -0.56)
		end

		-- Colossal Magma-Cracked Chestplate + Exposed Core + Cape
		local chest = P("TitanChest", Vector3.new(2.45, 2.15, 1.45), OBSIDIAN_PLATE, METAL)
		at("Body", chest, 0, 0, 0)
		local magmaCore = P("MagmaVein", Vector3.new(1.35, 1.25, 0.20), MAGMA_ORANGE, NEON)
		at("Body", magmaCore, 0, 0.15, -0.76)
		local cape = P("BruteCape", Vector3.new(2.65, 3.45, 0.30), BLOOD_RED, FABRIC)
		at("Body", cape, 0, -0.85, 0.78, 10, 0, 0)
		local belt = P("TitanBelt", Vector3.new(2.48, 0.35, 1.48), DARK_IRON, METAL)
		at("Body", belt, 0, -0.75, 0)
		-- Back exhaust pipes
		for _, sign in ipairs({ -1, 1 }) do
			local vent = P("TitanVent", Vector3.new(0.35, 0.95, 0.35), DARK_IRON, METAL, CYL)
			at("Body", vent, sign * 0.75, 0.75, 0.75, 25, 0, 0)
		end

		-- Colossal Spiked Titan Pauldrons
		for _, sideKey in ipairs({ "LeftArm", "RightArm" }) do
			local pauldron = P("TitanPauldron", Vector3.new(1.70, 0.75, 1.70), OBSIDIAN_PLATE, METAL)
			at(sideKey, pauldron, 0, 0.75, 0)
			local pauldronTrim = P("PauldronNeon", Vector3.new(1.78, 0.18, 1.78), BLOOD_RED, METAL)
			at(sideKey, pauldronTrim, 0, 0.65, 0)
			local pSpike = W("TitanSpike", Vector3.new(0.25, 0.75, 0.55), HORN_BLACK, SMOOTH)
			at(sideKey, pSpike, 0, 1.25, 0)
		end

		-- Right Arm: RADIANT MOLTEN CLAW GAUNTLET
		local fist = P("Weapon", Vector3.new(1.45, 1.55, 1.75), MAGMA_ORANGE, NEON)
		at("RightArm", fist, 0, -0.75, -0.35)
		for c = 1, 3 do
			local cAngle = (c - 2) * 0.45
			local claw = W("MoltenClaw_" .. c, Vector3.new(0.20, 0.95, 0.48), DARK_IRON, METAL)
			local cf = CFrame.new(math.sin(cAngle) * 0.58 * s, -1.45 * s, -0.58 * s) * CFrame.Angles(math.rad(-35), 0, cAngle)
			attach("RightArm", claw, cf)
		end

		-- Left Arm: Heavy Iron Shield Gauntlet
		local lShield = P("BruteGuard", Vector3.new(0.45, 1.85, 1.45), OBSIDIAN_PLATE, METAL)
		at("LeftArm", lShield, -0.65, -0.35, 0)
		local lTrim = P("GuardTrim", Vector3.new(0.15, 1.45, 0.45), BLOOD_RED, METAL)
		at("LeftArm", lTrim, -0.85, -0.35, 0)
		local lSpike = W("GuardSpike", Vector3.new(0.22, 0.65, 0.45), HORN_BLACK, SMOOTH)
		at("LeftArm", lSpike, -0.95, -0.35, 0, 0, 0, -90)

		-- Titan Greaves & Plated Sabatons
		for _, legKey in ipairs({ "LeftLeg", "RightLeg" }) do
			local knee = P("TitanKnee", Vector3.new(1.25, 0.55, 0.45), BLOOD_RED, METAL)
			at(legKey, knee, 0, 0.25, -0.58)
			local greave = P("TitanGreave", Vector3.new(1.18, 0.85, 1.18), OBSIDIAN_PLATE, METAL)
			at(legKey, greave, 0, -0.45, 0)
			local boot = P("TitanSabaton", Vector3.new(1.18, 0.45, 1.25), OBSIDIAN_PLATE, METAL)
			at(legKey, boot, 0, -0.75, -0.05)
		end

	elseif uName == "Infernal Archmage" then
		-- =====================================================================
		-- A-TIER 2: INFERNAL ARCHMAGE (Horned Mitre, Imperial Robes, Abyss Scepter, Tome)
		-- Note: Archetype is RangedMarksman (arm pitches +78°), so staff is counter-rotated -78°!
		-- =====================================================================
		head.Color = CRIMSON_SKIN
		local mitre = P("ArchmageMitre", Vector3.new(1.28, 0.95, 1.28), BLOOD_RED, FABRIC)
		at("Head", mitre, 0, 0.65, 0)
		local crownRim = P("MitreGoldRim", Vector3.new(1.35, 0.22, 1.35), ROYAL_GOLD, METAL)
		at("Head", crownRim, 0, 0.45, 0)
		for _, gsign in ipairs({ -1, 1 }) do
			local gem = P("CrownRuby", Vector3.new(0.18, 0.18, 0.14), BLOOD_RED, NEON, BALL)
			at("Head", gem, gsign * 0.45, 0.65, -0.62)
		end
		local thirdEye = P("ThirdEye", Vector3.new(0.30, 0.30, 0.18), MOLTEN_GOLD, NEON, BALL)
		at("Head", thirdEye, 0, 0.75, -0.66)
		for _, sign in ipairs({ -1, 1 }) do
			local horn = W("ArchmageHorn", Vector3.new(0.24, 1.25, 0.45), ROYAL_GOLD, METAL)
			at("Head", horn, sign * 0.58, 0.95, -0.15, 20, 0, -sign * 28)
			local eye = P("ArchEye", Vector3.new(0.26, 0.14, 0.15), MOLTEN_GOLD, NEON)
			at("Head", eye, sign * 0.32, 0.08, -0.54)
			local ear = W("ArchEar", Vector3.new(0.12, 0.45, 0.25), CRIMSON_SKIN, SMOOTH)
			at("Head", ear, sign * 0.65, 0.15, 0.10, -15, 0, -sign * 35)
		end

		-- Imperial Warlock Robes + High Standing Collar + Cape
		local collar = P("StandingCollar", Vector3.new(2.55, 1.35, 0.35), BLOOD_RED, FABRIC)
		at("Body", collar, 0, 0.95, 0.48, 15, 0, 0)
		local robe = P("ArchmageRobe", Vector3.new(2.18, 1.95, 1.25), ABYSSAL_NAVY, FABRIC)
		at("Body", robe, 0, 0, 0)
		local robeTrim = P("RobeTrim", Vector3.new(0.38, 1.95, 0.16), ROYAL_GOLD, METAL)
		at("Body", robeTrim, 0, 0, -0.66)
		local sash = P("ImperialSash", Vector3.new(2.25, 0.35, 1.30), ROYAL_GOLD, METAL)
		at("Body", sash, 0, -0.55, 0)
		local cape = P("ArchmageCape", Vector3.new(2.45, 3.35, 0.28), BLOOD_RED, FABRIC)
		at("Body", cape, 0, -0.85, 0.68, 10, 0, 0)
		for sp = 1, 2 do
			local mantleSpur = W("MantleSpur_" .. sp, Vector3.new(0.15, 0.65, 0.35), ROYAL_GOLD, METAL)
			at("Body", mantleSpur, (sp == 1 and -1.15 or 1.15), 1.05, 0.35, 25, 0, (sp == 1 and 25 or -25))
		end
		-- Floating Pyre Sigils on Back
		for o = 1, 3 do
			local orb = P("ArchOrb_" .. o, Vector3.new(0.42, 0.42, 0.42), MAGMA_ORANGE, NEON, BALL)
			at("Body", orb, (o - 2) * 0.95, 0.95, 0.85)
		end

		-- Pauldrons with Gold Crests
		for _, sideKey in ipairs({ "LeftArm", "RightArm" }) do
			local pauldron = P("ArchPauldron", Vector3.new(1.35, 0.55, 1.35), ABYSSAL_NAVY, FABRIC)
			at(sideKey, pauldron, 0, 0.65, 0)
			local pTrim = P("ArchTrim", Vector3.new(1.42, 0.15, 1.42), ROYAL_GOLD, METAL)
			at(sideKey, pTrim, 0, 0.55, 0)
			local cuff = P("ArchCuff", Vector3.new(1.15, 0.40, 1.15), ROYAL_GOLD, METAL)
			at(sideKey, cuff, 0, -0.55, 0)
		end

		-- Weapon: Blazing Scepter of the Abyss (RightArm, held upright matching demon_ref.png!)
		local scepterGripCF = makeHandGripCF("RightArm", -12, 0, 0)
		local staff = P("Weapon", Vector3.new(0.22, 4.45, 0.22), ROYAL_GOLD, METAL, CYL)
		attach("RightArm", staff, scepterGripCF * CFrame.new(0, 1.25 * s, 0))
		local scepterPommel = P("ScepterPommel", Vector3.new(0.35, 0.35, 0.35), ROYAL_GOLD, METAL, BALL)
		attach("RightArm", scepterPommel, scepterGripCF * CFrame.new(0, -1.05 * s, 0))
		local scepterCollar = P("ScepterCollar", Vector3.new(0.38, 0.22, 0.38), ROYAL_GOLD, METAL, CYL)
		attach("RightArm", scepterCollar, scepterGripCF * CFrame.new(0, 3.35 * s, 0))
		local scepterHead = P("ScepterFlame", Vector3.new(1.15, 1.15, 1.15), MAGMA_ORANGE, NEON, BALL)
		attach("RightArm", scepterHead, scepterGripCF * CFrame.new(0, 3.85 * s, 0))
		local scepterCore = P("ScepterCore", Vector3.new(0.65, 0.65, 0.65), HELLFIRE_YELLOW, NEON, BALL)
		attach("RightArm", scepterCore, scepterGripCF * CFrame.new(0, 3.85 * s, 0))
		local scepterRing = P("ScepterRing", Vector3.new(1.45, 0.08, 1.45), ROYAL_GOLD, METAL)
		attach("RightArm", scepterRing, scepterGripCF * CFrame.new(0, 3.85 * s, 0) * CFrame.Angles(math.rad(35), math.rad(45), 0))

		-- Left Hand: Floating Flaming Grimoire / Tome (Locked to Left Forearm)
		local tome = P("InfernalTome", Vector3.new(0.35, 1.25, 0.95), BLOOD_RED, FABRIC)
		at("LeftArm", tome, -0.45, -0.65, -0.25, 0, 90, 0)
		local tomeGlow = P("TomeFlame", Vector3.new(0.18, 0.95, 0.75), MAGMA_ORANGE, NEON)
		at("LeftArm", tomeGlow, -0.45, -0.35, -0.25)
		local clasp1 = P("TomeClasp1", Vector3.new(0.08, 0.22, 0.22), ROYAL_GOLD, METAL)
		at("LeftArm", clasp1, -0.45, -0.85, 0.22)
		local clasp2 = P("TomeClasp2", Vector3.new(0.08, 0.22, 0.22), ROYAL_GOLD, METAL)
		at("LeftArm", clasp2, -0.45, -0.45, 0.22)

		-- Legs: Regal Robe Skirt
		for _, legKey in ipairs({ "LeftLeg", "RightLeg" }) do
			local skirt = P("ArchSkirt", Vector3.new(1.12, 1.25, 1.12), ABYSSAL_NAVY, FABRIC)
			at(legKey, skirt, 0, 0, 0)
			local hem = P("ArchHem", Vector3.new(1.18, 0.16, 1.18), ROYAL_GOLD, METAL)
			at(legKey, hem, 0, -0.60, 0)
		end

	elseif uName == "Demon Lord" then
		-- =====================================================================
		-- A-TIER 3: DEMON LORD (Supreme Monarch, Majestic Bat Wings, Crown, Greatsword)
		-- =====================================================================
		local crownRim = P("LordCrownRim", Vector3.new(1.28, 0.35, 1.28), IMPERIAL_GOLD, METAL)
		at("Head", crownRim, 0, 0.55, 0)
		local centerCrownSpire = W("CrownCenter", Vector3.new(0.26, 1.05, 0.38), IMPERIAL_GOLD, METAL)
		at("Head", centerCrownSpire, 0, 1.05, -0.25)
		for _, sign in ipairs({ -1, 1 }) do
			local horn1 = W("LordHorn1", Vector3.new(0.26, 1.45, 0.55), IMPERIAL_GOLD, METAL)
			at("Head", horn1, sign * 0.68, 0.95, -0.15, 15, 0, -sign * 32)
			local horn2 = W("LordHorn2", Vector3.new(0.18, 0.75, 0.35), IMPERIAL_GOLD, METAL)
			at("Head", horn2, sign * 1.15, 1.65, 0.05, 30, 0, -sign * 45)

			local eye = P("LordEye", Vector3.new(0.35, 0.14, 0.18), HELLFIRE_YELLOW, NEON)
			at("Head", eye, sign * 0.35, 0.08, -0.56)
		end

		-- Emperor Armor + Royal Cape
		local chest = P("LordChest", Vector3.new(2.38, 2.05, 1.35), OBSIDIAN_PLATE, METAL)
		at("Body", chest, 0, 0, 0)
		local chestGold = P("ChestTrim", Vector3.new(1.55, 1.55, 0.18), IMPERIAL_GOLD, METAL)
		at("Body", chestGold, 0, 0.15, -0.72)
		local belt = P("LordSash", Vector3.new(2.42, 0.35, 1.38), IMPERIAL_GOLD, METAL)
		at("Body", belt, 0, -0.55, 0)
		local cape = P("LordCape", Vector3.new(2.55, 3.55, 0.30), BLOOD_RED, FABRIC)
		at("Body", cape, 0, -0.85, 0.72, 12, 0, 0)
		for t = -1, 1, 2 do
			local tasset = P("EmperorTasset_" .. t, Vector3.new(0.65, 0.75, 0.18), IMPERIAL_GOLD, METAL)
			at("Body", tasset, t * 0.70, -0.95, -0.65)
		end

		-- MAJESTIC BAT WINGS (Spanning wide from back)
		for _, sign in ipairs({ -1, 1 }) do
			local wingBone = P("WingBone", Vector3.new(0.28, 2.65, 0.28), HORN_BLACK, SMOOTH)
			at("Body", wingBone, sign * 1.55, 1.25, 0.78, 0, 0, -sign * 55)
			local wingMembrane1 = W("WingMembrane1", Vector3.new(0.14, 2.35, 1.95), BLOOD_RED, FABRIC)
			at("Body", wingMembrane1, sign * 2.35, 0.85, 0.78, 0, sign * 15, -sign * 35)
			local wingMembrane2 = W("WingMembrane2", Vector3.new(0.14, 1.95, 1.55), BLOOD_RED, FABRIC)
			at("Body", wingMembrane2, sign * 1.65, -0.15, 0.78, 0, sign * 15, -sign * 20)
		end

		-- Royal Pauldrons
		for _, sideKey in ipairs({ "LeftArm", "RightArm" }) do
			local pauldron = P("LordPauldron", Vector3.new(1.45, 0.65, 1.45), IMPERIAL_GOLD, METAL)
			at(sideKey, pauldron, 0, 0.65, 0)
			local pSpike = W("PauldronHorn", Vector3.new(0.20, 0.65, 0.45), HORN_BLACK, SMOOTH)
			at(sideKey, pSpike, 0, 1.05, 0)
			local gauntlet = P("LordGauntlet", Vector3.new(1.15, 0.75, 1.15), OBSIDIAN_PLATE, METAL)
			at(sideKey, gauntlet, 0, -0.45, 0)
			local gTrim = P("GauntletTrim", Vector3.new(1.22, 0.14, 1.22), IMPERIAL_GOLD, METAL)
			at(sideKey, gTrim, 0, -0.25, 0)
		end

		-- Weapon: Colossal Hellfire Executioner Greatsword (RightArm, locked into hand palm!)
				-- Weapon: Colossal Hellfire Executioner Greatsword (RightArm, held at hip angled down & forward at -38°, matching Knight)
		buildDemonSword("RightArm", 1.35, OBSIDIAN_PLATE, MAGMA_ORANGE, IMPERIAL_GOLD, HELLFIRE_YELLOW, -38, 2.25, 0.82, "Executioner")

		-- Greaves & Sabatons
		for _, legKey in ipairs({ "LeftLeg", "RightLeg" }) do
			local knee = P("LordKnee", Vector3.new(1.22, 0.48, 0.42), IMPERIAL_GOLD, METAL)
			at(legKey, knee, 0, 0.25, -0.56)
			local greave = P("LordGreave", Vector3.new(1.12, 0.85, 1.12), OBSIDIAN_PLATE, METAL)
			at(legKey, greave, 0, -0.45, 0)
			local sabaton = P("LordSabaton", Vector3.new(1.14, 0.40, 1.22), DARK_CRIMSON, METAL)
			at(legKey, sabaton, 0, -0.80, -0.06)
		end
	end
end

return DemonRig
