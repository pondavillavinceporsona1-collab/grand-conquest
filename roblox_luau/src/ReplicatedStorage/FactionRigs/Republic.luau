-- RepublicRig.luau
-- High-Detail Anatomical & Tactical Republic Rig Builder matching Reference Artwork & Mechanica/Kingdom Craftsmanship
-- Core shrinking, layered tactical vests, ballistic shields, helmets, facial features & multi-part firearms
-- All accessories strictly attach to core limb keys: "Head", "Body", "LeftArm", "RightArm", "LeftLeg", "RightLeg"

local RepublicRig = {}

local TACTICAL_BLACK = Color3.fromRGB(24, 26, 32)
local UNDER_SUIT     = Color3.fromRGB(32, 35, 42)
local STEEL_DARK     = Color3.fromRGB(48, 54, 64)
local STEEL_GUNMETAL = Color3.fromRGB(72, 80, 94)
local STEEL_LIGHT    = Color3.fromRGB(175, 185, 200)
local WHITE_ARMOR    = Color3.fromRGB(242, 246, 252)
local POLICE_BLUE    = Color3.fromRGB(48, 102, 185)
local POLICE_NAVY    = Color3.fromRGB(24, 40, 74)
local OLIVE_DRAB     = Color3.fromRGB(68, 82, 54)
local CAMO_DARK      = Color3.fromRGB(45, 56, 38)
local NEON_CYAN      = Color3.fromRGB(65, 215, 255)
local NEON_AMBER     = Color3.fromRGB(255, 195, 50)
local MEDIC_RED      = Color3.fromRGB(230, 35, 45)
local GOLD_BRASS     = Color3.fromRGB(225, 185, 60)
local SKIN_TONE      = Color3.fromRGB(225, 175, 140)
local BEARD_DARK     = Color3.fromRGB(35, 30, 26)
local BEARD_GREY     = Color3.fromRGB(150, 155, 165)

local VALID_LIMBS = {
	Head = true,
	Body = true,
	LeftArm = true,
	RightArm = true,
	LeftLeg = true,
	RightLeg = true,
}

function RepublicRig.build(ctx)
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

	local function P(name, size, color, mat, shape)
		return makePart(name, size * s, color, mat or Enum.Material.Metal, model, shape)
	end
	local function W(name, size, color, mat)
		return makeWedge(name, size * s, color, mat or Enum.Material.Metal, model)
	end
	local function at(limbKey, part, x, y, z, rx, ry, rz)
		assert(VALID_LIMBS[limbKey], "Invalid limbKey in RepublicRig: " .. tostring(limbKey))
		attach(limbKey, part, CFrame.new(x * s, y * s, z * s) * CFrame.Angles(math.rad(rx or 0), math.rad(ry or 0), math.rad(rz or 0)))
		return part
	end

	-- Helper for firearms on RightArm: compensates for PoseRig's +78 deg arm pitch on ranged/artillery units
	-- so (gx, gy, gz) is in level horizontal gun-space (gz < 0 is forward barrel, gy > 0 is top sight, gy < 0 is mag/grip)
	local function gunAt(armPitchDeg, part, gx, gy, gz, rx, ry, rz)
		local handCF = CFrame.new(0, -0.92 * s, -0.08 * s) * CFrame.Angles(math.rad(-armPitchDeg), 0, 0)
		local offset = handCF * CFrame.new(gx * s, gy * s, gz * s) * CFrame.Angles(math.rad(rx or 0), math.rad(ry or 0), math.rad(rz or 0))
		attach("RightArm", part, offset)
		return part
	end

	-- 1. Core Shrinking & Base Setup
	body.Size = Vector3.new(1.85, 1.85, 0.85) * s
	body.Color = UNDER_SUIT
	body.Material = Enum.Material.SmoothPlastic
	head.Size = Vector3.new(1.05, 1.05, 1.05) * s
	head.Color = SKIN_TONE
	head.Material = Enum.Material.SmoothPlastic
	leftArm.Size = Vector3.new(0.85, 1.85, 0.85) * s
	leftArm.Color = UNDER_SUIT
	leftArm.Material = Enum.Material.SmoothPlastic
	rightArm.Size = Vector3.new(0.85, 1.85, 0.85) * s
	rightArm.Color = UNDER_SUIT
	rightArm.Material = Enum.Material.SmoothPlastic
	leftLeg.Size = Vector3.new(0.85, 1.85, 0.85) * s
	leftLeg.Color = UNDER_SUIT
	leftLeg.Material = Enum.Material.SmoothPlastic
	rightLeg.Size = Vector3.new(0.85, 1.85, 0.85) * s
	rightLeg.Color = UNDER_SUIT
	rightLeg.Material = Enum.Material.SmoothPlastic

	-- 2. Shared Sub-Assembly Builders
	local function buildHumanFace(hasBeard, beardColor, hasGlasses)
		head.Color = SKIN_TONE
		for _, sign in ipairs({ -1, 1 }) do
			at("Head", P("Eye_" .. (sign < 0 and "L" or "R"), Vector3.new(0.16, 0.12, 0.08), TACTICAL_BLACK, Enum.Material.SmoothPlastic), sign * 0.22, 0.08, -0.52)
			at("Head", P("Brow_" .. (sign < 0 and "L" or "R"), Vector3.new(0.24, 0.07, 0.10), BEARD_DARK, Enum.Material.SmoothPlastic), sign * 0.22, 0.20, -0.53, 0, 0, sign * -6)
		end
		if hasBeard then
			local bCol = beardColor or BEARD_DARK
			at("Head", P("BeardJaw", Vector3.new(1.08, 0.42, 0.65), bCol, Enum.Material.SmoothPlastic), 0, -0.32, -0.25)
			at("Head", P("Mustache", Vector3.new(0.65, 0.14, 0.16), bCol, Enum.Material.SmoothPlastic), 0, -0.10, -0.54)
		end
		if hasGlasses then
			at("Head", P("TacticalShades", Vector3.new(1.08, 0.24, 0.18), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.10, -0.54)
			at("Head", P("ShadeRim", Vector3.new(1.10, 0.06, 0.20), STEEL_LIGHT, Enum.Material.Metal), 0, 0.22, -0.54)
		end
	end

	local function buildTacticalBootsAndLegs(legColor, kneeColor, trimColor)
		for _, isLeft in ipairs({ true, false }) do
			local lKey = isLeft and "LeftLeg" or "RightLeg"
			local sfx = isLeft and "L" or "R"
			at(lKey, P("ThighArmor_" .. sfx, Vector3.new(0.96, 0.82, 0.96), legColor, Enum.Material.SmoothPlastic), 0, 0.42, 0)
			at(lKey, P("KneePad_" .. sfx, Vector3.new(0.88, 0.42, 0.35), kneeColor or TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.02, -0.42)
			if trimColor then
				at(lKey, P("KneeTrim_" .. sfx, Vector3.new(0.65, 0.10, 0.38), trimColor, Enum.Material.Metal), 0, 0.02, -0.44)
			end
			at(lKey, P("ShinGuard_" .. sfx, Vector3.new(0.94, 0.85, 0.94), legColor, Enum.Material.SmoothPlastic), 0, -0.52, 0)
			at(lKey, P("CombatBoot_" .. sfx, Vector3.new(0.96, 0.38, 1.20), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -0.92, -0.12)
			at(lKey, P("BootToe_" .. sfx, Vector3.new(0.88, 0.28, 0.35), STEEL_DARK, Enum.Material.Metal), 0, -0.96, -0.65)
		end
	end

	local function buildTacticalArm(armKey, isLeft, pauldronColor, sleeveColor, bareForearm, trimColor)
		local sign = isLeft and -1 or 1
		local sfx = isLeft and "L" or "R"
		at(armKey, P("UpperSleeve_" .. sfx, Vector3.new(0.95, 0.85, 0.95), sleeveColor, Enum.Material.SmoothPlastic), 0, 0.45, 0)
		if pauldronColor then
			at(armKey, P("Pauldron_" .. sfx, Vector3.new(1.28, 0.58, 1.22), pauldronColor, Enum.Material.Metal), sign * 0.20, 0.72, 0, 0, 0, sign * -8)
			if trimColor then
				at(armKey, P("PauldronStripe_" .. sfx, Vector3.new(1.32, 0.14, 1.25), trimColor, Enum.Material.SmoothPlastic), sign * 0.20, 0.72, 0, 0, 0, sign * -8)
			end
		end
		if bareForearm then
			at(armKey, P("ForearmSkin_" .. sfx, Vector3.new(0.88, 0.75, 0.88), SKIN_TONE, Enum.Material.SmoothPlastic), 0, -0.45, 0)
			at(armKey, P("WristWrap_" .. sfx, Vector3.new(0.92, 0.25, 0.92), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -0.78, 0)
		else
			at(armKey, P("Vambrace_" .. sfx, Vector3.new(0.96, 0.82, 0.96), pauldronColor or TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -0.52, 0)
		end
		at(armKey, P("CombatGlove_" .. sfx, Vector3.new(0.94, 0.42, 0.94), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -1.02, 0)
	end

	local function buildDutyBelt(beltColor, buckleColor, hasHolster)
		at("Body", P("DutyBelt", Vector3.new(2.12, 0.28, 1.12), beltColor or TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -0.62, 0)
		at("Body", P("BeltBuckle", Vector3.new(0.42, 0.30, 0.18), buckleColor or STEEL_LIGHT, Enum.Material.Metal), 0, -0.62, -0.54)
		at("Body", P("BeltPouchL", Vector3.new(0.28, 0.42, 0.45), TACTICAL_BLACK, Enum.Material.SmoothPlastic), -0.98, -0.62, 0)
		if hasHolster then
			at("Body", P("SideHolster", Vector3.new(0.32, 0.68, 0.48), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0.98, -0.72, 0)
		else
			at("Body", P("BeltPouchR", Vector3.new(0.28, 0.42, 0.45), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0.98, -0.62, 0)
		end
	end

	local function buildBallisticShield(armPitchDeg, mainColor, rimColor, crossColor, isLarge)
		local h = isLarge and 3.05 or 2.80
		local w = isLarge and 1.80 or 1.60
		local baseCF = CFrame.new(-0.58 * s, -0.25 * s, -0.35 * s) * CFrame.Angles(math.rad(-armPitchDeg), 0, 0)
		local function shAt(part, sx, sy, sz)
			attach("LeftArm", part, baseCF * CFrame.new(sx * s, sy * s, sz * s))
		end
		shAt(P("ShieldBody", Vector3.new(0.24, h, w), mainColor, Enum.Material.SmoothPlastic), 0, 0, 0)
		shAt(P("ShieldRimTop", Vector3.new(0.28, 0.22, w + 0.08), rimColor), 0, h * 0.48, 0)
		shAt(P("ShieldRimBot", Vector3.new(0.28, 0.22, w + 0.08), rimColor), 0, -h * 0.48, 0)
		shAt(P("ShieldRimL", Vector3.new(0.28, h, 0.20), rimColor), 0, 0, -w * 0.48)
		shAt(P("ShieldRimR", Vector3.new(0.28, h, 0.20), rimColor), 0, 0, w * 0.48)
		shAt(P("ShieldViewport", Vector3.new(0.30, 0.35, w * 0.62), NEON_CYAN, Enum.Material.Neon), -0.02, h * 0.26, 0)
		shAt(P("ShieldCrossV", Vector3.new(0.30, 0.90, 0.26), crossColor, Enum.Material.Neon), -0.02, -0.15, 0)
		shAt(P("ShieldCrossH", Vector3.new(0.30, 0.26, 0.82), crossColor, Enum.Material.Neon), -0.02, -0.15, 0)
	end

	-- -------------------------------------------------------------------------
	-- UNIT 1: RIOT TROOPER (C-Tier Frontline Enforcer - LightInfantry, pitch=0)
	-- -------------------------------------------------------------------------
	if uName == "Riot Trooper" then
		at("Head", P("RiotHelmetDome", Vector3.new(1.25, 0.55, 1.25), WHITE_ARMOR, Enum.Material.SmoothPlastic), 0, 0.38, 0)
		at("Head", P("HelmetRedStripe", Vector3.new(0.35, 0.12, 0.15), MEDIC_RED, Enum.Material.SmoothPlastic), 0, 0.48, -0.58)
		at("Head", P("GasMaskFace", Vector3.new(1.12, 0.68, 0.35), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -0.08, -0.45)
		at("Head", P("RespiratorSnout", Vector3.new(0.45, 0.38, 0.35), STEEL_DARK, Enum.Material.Metal), 0, -0.22, -0.62)
		for _, sign in ipairs({ -1, 1 }) do
			at("Head", P("AmberLens_" .. (sign < 0 and "L" or "R"), Vector3.new(0.32, 0.14, 0.12), NEON_AMBER, Enum.Material.Neon), sign * 0.24, 0.06, -0.62)
		end

		at("Body", P("RiotChestBase", Vector3.new(2.05, 1.55, 1.05), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.15, 0)
		at("Body", P("RiotPlateL", Vector3.new(0.88, 0.78, 0.22), WHITE_ARMOR, Enum.Material.SmoothPlastic), -0.48, 0.35, -0.48)
		at("Body", P("RiotPlateR", Vector3.new(0.88, 0.78, 0.22), STEEL_DARK, Enum.Material.Metal), 0.48, 0.35, -0.48)
		at("Body", P("AbGuard", Vector3.new(1.15, 0.65, 0.20), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -0.35, -0.48)
		buildDutyBelt(WHITE_ARMOR, TACTICAL_BLACK, false)

		buildTacticalArm("LeftArm", true, TACTICAL_BLACK, WHITE_ARMOR, false, nil)
		buildTacticalArm("RightArm", false, TACTICAL_BLACK, WHITE_ARMOR, false, nil)
		buildTacticalBootsAndLegs(TACTICAL_BLACK, WHITE_ARMOR, nil)

		-- Upright Conductive Stun Baton in Right Hand
		at("RightArm", P("BatonGrip", Vector3.new(0.22, 0.85, 0.22), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -0.85, -0.35)
		at("RightArm", P("BatonGuard", Vector3.new(0.35, 0.15, 0.45), STEEL_DARK, Enum.Material.Metal), 0, -0.45, -0.35)
		at("RightArm", P("Weapon", Vector3.new(0.20, 2.25, 0.20), STEEL_DARK, Enum.Material.Metal), 0, 0.55, -0.35, 10, 0, 0)
		at("RightArm", P("StunTip", Vector3.new(0.24, 0.45, 0.24), NEON_CYAN, Enum.Material.Neon), 0, 1.65, -0.52, 10, 0, 0)

	-- -------------------------------------------------------------------------
	-- UNIT 2: POLICE (C-Tier Patrol Marksman - RangedMarksman, pitch=78)
	-- -------------------------------------------------------------------------
	elseif uName == "Police" then
		buildHumanFace(false, nil, false)
		at("Head", P("PeakedCapCrown", Vector3.new(1.32, 0.42, 1.32), POLICE_NAVY, Enum.Material.SmoothPlastic), 0, 0.52, 0)
		at("Head", P("CapBand", Vector3.new(1.24, 0.20, 1.24), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.34, 0)
		at("Head", P("CapVisor", Vector3.new(1.25, 0.10, 0.55), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.26, -0.58, 12, 0, 0)
		at("Head", P("CapBadge", Vector3.new(0.28, 0.28, 0.12), GOLD_BRASS, Enum.Material.Metal), 0, 0.50, -0.65)

		at("Body", P("PatrolShirt", Vector3.new(2.02, 1.65, 1.02), POLICE_BLUE, Enum.Material.SmoothPlastic), 0, 0.12, 0)
		at("Body", P("CollarTie", Vector3.new(0.24, 1.05, 0.12), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.15, -0.50)
		at("Body", P("SilverBadge", Vector3.new(0.30, 0.35, 0.12), GOLD_BRASS, Enum.Material.Metal), -0.52, 0.42, -0.50)
		at("Body", P("PocketL", Vector3.new(0.45, 0.42, 0.10), POLICE_NAVY, Enum.Material.SmoothPlastic), -0.52, 0.12, -0.50)
		at("Body", P("PocketR", Vector3.new(0.45, 0.42, 0.10), POLICE_NAVY, Enum.Material.SmoothPlastic), 0.52, 0.12, -0.50)
		buildDutyBelt(TACTICAL_BLACK, GOLD_BRASS, true)

		buildTacticalArm("LeftArm", true, nil, POLICE_BLUE, true, nil)
		buildTacticalArm("RightArm", false, nil, POLICE_BLUE, true, nil)
		buildTacticalBootsAndLegs(POLICE_NAVY, TACTICAL_BLACK, nil)

		-- Level Police Tactical Carbine (compensating 78 deg RightArm pitch)
		gunAt(78, P("Weapon", Vector3.new(0.22, 0.46, 1.85), TACTICAL_BLACK, Enum.Material.Metal), 0, 0.08, -0.85)
		gunAt(78, P("CarbineBarrel", Vector3.new(0.85, 0.16, 0.16), STEEL_DARK, Enum.Material.Metal, CYL), 0, 0.16, -2.05, 0, 90, 0)
		gunAt(78, P("CarbineMag", Vector3.new(0.18, 0.58, 0.30), STEEL_GUNMETAL, Enum.Material.Metal), 0, -0.32, -0.75, 12, 0, 0)
		gunAt(78, P("CarbineOptic", Vector3.new(0.18, 0.22, 0.42), NEON_CYAN, Enum.Material.Neon), 0, 0.38, -0.85)
		gunAt(78, P("CarbineStock", Vector3.new(0.20, 0.48, 0.62), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.04, 0.32)

	-- -------------------------------------------------------------------------
	-- UNIT 3: MEDIC BOT (C-Tier Autonomous Medical Droid - HoverDrone, pitch=0)
	-- -------------------------------------------------------------------------
	elseif uName == "Medic Bot" then
		-- Make legs visible so Medic Bot has its full bipedal droid chassis from republic_ref.png
		leftLeg.Transparency = 0
		rightLeg.Transparency = 0
		head.Color = WHITE_ARMOR
		at("Head", P("MedicHelmDome", Vector3.new(1.22, 0.42, 1.22), WHITE_ARMOR, Enum.Material.SmoothPlastic), 0, 0.38, 0)
		at("Head", P("ForeheadCross", Vector3.new(0.45, 0.14, 0.14), MEDIC_RED, Enum.Material.SmoothPlastic), 0, 0.42, -0.56)
		at("Head", P("DroidFaceplate", Vector3.new(1.05, 0.58, 0.25), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -0.02, -0.46)
		for _, sign in ipairs({ -1, 1 }) do
			at("Head", P("DroidOptic_" .. (sign < 0 and "L" or "R"), Vector3.new(0.25, 0.18, 0.12), NEON_CYAN, Enum.Material.Neon), sign * 0.24, 0.05, -0.56)
		end

		at("Body", P("MedicChassis", Vector3.new(2.18, 1.85, 1.18), WHITE_ARMOR, Enum.Material.SmoothPlastic), 0, 0.05, 0)
		at("Body", P("RedCrossV", Vector3.new(0.36, 1.10, 0.14), MEDIC_RED, Enum.Material.SmoothPlastic), 0, 0.12, -0.56)
		at("Body", P("RedCrossH", Vector3.new(1.10, 0.36, 0.14), MEDIC_RED, Enum.Material.SmoothPlastic), 0, 0.12, -0.56)
		at("Body", P("PelvisArmor", Vector3.new(1.85, 0.38, 1.12), STEEL_GUNMETAL, Enum.Material.Metal), 0, -0.78, 0)

		for _, sign in ipairs({ -1, 1 }) do
			at("Body", P("SerumTank_" .. (sign < 0 and "L" or "R"), Vector3.new(1.35, 0.42, 0.42), NEON_CYAN, Enum.Material.Neon, CYL), sign * 0.48, 0.15, 0.72, 0, 0, 90)
			at("Body", P("TankCap_" .. (sign < 0 and "L" or "R"), Vector3.new(0.22, 0.46, 0.46), STEEL_DARK, Enum.Material.Metal, CYL), sign * 0.48, 0.82, 0.72, 0, 0, 90)
		end

		buildTacticalArm("LeftArm", true, WHITE_ARMOR, STEEL_GUNMETAL, false, MEDIC_RED)
		buildTacticalArm("RightArm", false, WHITE_ARMOR, STEEL_GUNMETAL, false, POLICE_BLUE)
		buildTacticalBootsAndLegs(WHITE_ARMOR, STEEL_GUNMETAL, MEDIC_RED)

		gunAt(0, P("Weapon", Vector3.new(0.24, 0.48, 1.55), WHITE_ARMOR, Enum.Material.SmoothPlastic), 0, 0.05, -0.85)
		gunAt(0, P("SerumChamber", Vector3.new(0.62, 0.32, 0.32), NEON_CYAN, Enum.Material.Neon, CYL), 0, 0.32, -0.85, 0, 90, 0)
		gunAt(0, P("InjectorTip", Vector3.new(0.12, 0.26, 0.75), STEEL_LIGHT, Enum.Material.Metal), 0, 0.05, -1.85)
		at("LeftArm", P("HealEmitter", Vector3.new(0.45, 0.45, 0.45), NEON_CYAN, Enum.Material.Neon, BALL), 0, -1.15, -0.25)

	-- -------------------------------------------------------------------------
	-- UNIT 4: RIFLEMAN (B-Tier Tactical Infantry - RangedMarksman, pitch=78)
	-- -------------------------------------------------------------------------
	elseif uName == "Rifleman" then
		buildHumanFace(false, nil, false)
		at("Head", P("CombatHelmet", Vector3.new(1.28, 0.52, 1.28), OLIVE_DRAB, Enum.Material.SmoothPlastic), 0, 0.40, 0)
		at("Head", P("HelmetRim", Vector3.new(1.32, 0.14, 1.32), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.20, 0)

		at("Body", P("PlateCarrier", Vector3.new(2.08, 1.65, 1.12), OLIVE_DRAB, Enum.Material.SmoothPlastic), 0, 0.12, 0)
		for m = 1, 3 do
			at("Body", P("MagPouch_" .. m, Vector3.new(0.42, 0.62, 0.22), CAMO_DARK, Enum.Material.SmoothPlastic), (-0.55 + (m - 1) * 0.55), -0.12, -0.58)
		end
		at("Body", P("ChestPack", Vector3.new(1.35, 1.45, 0.55), CAMO_DARK, Enum.Material.SmoothPlastic), 0, 0.15, 0.68)
		buildDutyBelt(TACTICAL_BLACK, GOLD_BRASS, false)

		buildTacticalArm("LeftArm", true, OLIVE_DRAB, OLIVE_DRAB, true, nil)
		buildTacticalArm("RightArm", false, OLIVE_DRAB, OLIVE_DRAB, true, nil)
		buildTacticalBootsAndLegs(OLIVE_DRAB, TACTICAL_BLACK, nil)

		-- Level M4A1 Tactical Assault Rifle (compensating 78 deg RightArm pitch)
		gunAt(78, P("Weapon", Vector3.new(0.24, 0.48, 2.15), TACTICAL_BLACK, Enum.Material.Metal), 0, 0.08, -0.95)
		gunAt(78, P("RifleBarrel", Vector3.new(1.05, 0.16, 0.16), STEEL_DARK, Enum.Material.Metal, CYL), 0, 0.18, -2.35, 0, 90, 0)
		gunAt(78, P("FlashSuppressor", Vector3.new(0.22, 0.22, 0.30), STEEL_LIGHT, Enum.Material.Metal), 0, 0.18, -2.92)
		gunAt(78, P("CurvedMag", Vector3.new(0.20, 0.68, 0.38), STEEL_GUNMETAL, Enum.Material.Metal), 0, -0.35, -0.88, 14, 0, 0)
		gunAt(78, P("Foregrip", Vector3.new(0.18, 0.45, 0.20), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -0.26, -1.65)
		gunAt(78, P("HoloSight", Vector3.new(0.22, 0.26, 0.50), TACTICAL_BLACK, Enum.Material.Metal), 0, 0.40, -0.88)
		gunAt(78, P("HoloLens", Vector3.new(0.16, 0.18, 0.10), NEON_CYAN, Enum.Material.Neon), 0, 0.40, -1.12)
		gunAt(78, P("RifleStock", Vector3.new(0.22, 0.52, 0.68), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.04, 0.38)

	-- -------------------------------------------------------------------------
	-- UNIT 5: RIOT SHIELD (B-Tier Heavy Breacher - ShieldTank, LeftArm pitch=35)
	-- -------------------------------------------------------------------------
	elseif uName == "Riot Shield" then
		buildHumanFace(true, BEARD_DARK, true)
		at("Head", P("SwatHelmet", Vector3.new(1.28, 0.52, 1.28), POLICE_BLUE, Enum.Material.SmoothPlastic), 0, 0.40, 0)
		at("Head", P("HelmetVisorBand", Vector3.new(1.32, 0.18, 1.32), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.22, 0)

		at("Body", P("SwatVest", Vector3.new(2.15, 1.75, 1.18), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.10, 0)
		at("Body", P("ChestRigPlate", Vector3.new(1.45, 1.15, 0.22), POLICE_NAVY, Enum.Material.SmoothPlastic), 0, 0.18, -0.55)
		at("Body", P("StrobeMount", Vector3.new(0.25, 0.45, 0.25), STEEL_DARK, Enum.Material.Metal), 0.75, 1.05, 0.25, 0, 0, -15)
		at("Body", P("ArcStrobe", Vector3.new(0.38, 0.38, 0.38), NEON_CYAN, Enum.Material.Neon, BALL), 0.85, 1.32, 0.25)
		buildDutyBelt(TACTICAL_BLACK, STEEL_LIGHT, true)

		buildTacticalArm("LeftArm", true, TACTICAL_BLACK, POLICE_NAVY, true, nil)
		buildTacticalArm("RightArm", false, TACTICAL_BLACK, POLICE_NAVY, true, nil)
		buildTacticalBootsAndLegs(TACTICAL_BLACK, POLICE_NAVY, NEON_CYAN)

		-- Left Arm: Towering Ballistic Shield (compensating 35 deg LeftArm pitch)
		buildBallisticShield(35, TACTICAL_BLACK, STEEL_GUNMETAL, NEON_CYAN, false)

		-- Right Arm: Heavy Tactical Sidearm (RightArm pitch=0)
		gunAt(0, P("Weapon", Vector3.new(0.22, 0.45, 1.35), STEEL_GUNMETAL, Enum.Material.Metal), 0, 0.05, -0.75)
		gunAt(0, P("SidearmLight", Vector3.new(0.16, 0.16, 0.35), NEON_CYAN, Enum.Material.Neon), 0, -0.08, -1.20)

	-- -------------------------------------------------------------------------
	-- UNIT 6: GRENADE THROWER (B-Tier Heavy Demolitions - ArtillerySiege, pitch=78)
	-- -------------------------------------------------------------------------
	elseif uName == "Grenade Thrower" then
		buildHumanFace(true, BEARD_GREY, false)
		at("Head", P("DemoHelmet", Vector3.new(1.28, 0.52, 1.28), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.40, 0)
		at("Head", P("HelmetOptic", Vector3.new(0.28, 0.22, 0.35), NEON_AMBER, Enum.Material.Neon), -0.28, 0.32, -0.58)

		at("Body", P("FlakArmor", Vector3.new(2.15, 1.75, 1.18), CAMO_DARK, Enum.Material.SmoothPlastic), 0, 0.10, 0)
		at("Body", P("CollarGuard", Vector3.new(1.65, 0.35, 1.15), STEEL_GUNMETAL, Enum.Material.Metal), 0, 0.88, 0)
		for g = 1, 5 do
			at("Body", P("Grenade40mm_" .. g, Vector3.new(0.48, 0.24, 0.24), GOLD_BRASS, Enum.Material.Metal, CYL), (-0.64 + (g - 1) * 0.32), (0.52 - (g - 1) * 0.24), -0.58, 0, 0, 35)
		end
		buildDutyBelt(TACTICAL_BLACK, GOLD_BRASS, false)

		buildTacticalArm("LeftArm", true, STEEL_GUNMETAL, CAMO_DARK, false, nil)
		buildTacticalArm("RightArm", false, STEEL_GUNMETAL, CAMO_DARK, false, nil)
		buildTacticalBootsAndLegs(STEEL_GUNMETAL, TACTICAL_BLACK, nil)

		-- Level Olive & Gunmetal 6-Shot Rotary Grenade Launcher (compensating 78 deg RightArm pitch)
		gunAt(78, P("Weapon", Vector3.new(0.78, 0.75, 0.75), OLIVE_DRAB, Enum.Material.Metal, CYL), 0, 0.05, -0.95, 0, 90, 0)
		gunAt(78, P("MGLFrame", Vector3.new(0.30, 0.55, 1.95), OLIVE_DRAB, Enum.Material.SmoothPlastic), 0, 0.12, -1.10)
		gunAt(78, P("MGLBarrel", Vector3.new(1.05, 0.34, 0.34), STEEL_DARK, Enum.Material.Metal, CYL), 0, 0.12, -2.20, 0, 90, 0)
		gunAt(78, P("MGLForegrip", Vector3.new(0.20, 0.48, 0.22), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -0.25, -1.85)
		gunAt(78, P("MGLSight", Vector3.new(0.18, 0.24, 0.42), TACTICAL_BLACK, Enum.Material.Metal), 0, 0.45, -1.10)

	-- -------------------------------------------------------------------------
	-- UNIT 7: ELITE RIOT (A-Tier Supreme Juggernaut Champion - TitanChampion, pitch=0)
	-- -------------------------------------------------------------------------
	elseif uName == "Elite Riot" then
		head.Color = STEEL_DARK
		at("Head", P("JugHelmDome", Vector3.new(1.34, 0.55, 1.34), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.38, 0)
		at("Head", P("JugFaceplate", Vector3.new(1.22, 0.78, 0.35), STEEL_LIGHT, Enum.Material.Metal), 0, -0.05, -0.46)
		at("Head", P("VisorSlitH", Vector3.new(0.98, 0.12, 0.14), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.08, -0.60)
		at("Head", P("VisorGlow", Vector3.new(0.88, 0.08, 0.12), NEON_CYAN, Enum.Material.Neon), 0, 0.08, -0.62)
		at("Head", P("VisorSlitV", Vector3.new(0.14, 0.48, 0.14), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, -0.15, -0.60)

		at("Body", P("JugChestCore", Vector3.new(2.25, 1.85, 1.25), STEEL_GUNMETAL, Enum.Material.Metal), 0, 0.08, 0)
		at("Body", P("JugBreastplate", Vector3.new(1.85, 1.25, 0.28), STEEL_LIGHT, Enum.Material.Metal), 0, 0.22, -0.58)
		at("Body", P("JugCollar", Vector3.new(1.75, 0.42, 1.28), TACTICAL_BLACK, Enum.Material.Metal), 0, 0.92, 0)
		at("Body", P("JugGroinPlate", Vector3.new(1.05, 0.75, 0.24), STEEL_GUNMETAL, Enum.Material.Metal), 0, -0.95, -0.52)
		at("Body", P("JugBackReactor", Vector3.new(1.45, 1.45, 0.55), STEEL_DARK, Enum.Material.Metal), 0, 0.18, 0.72)
		at("Body", P("ReactorCore", Vector3.new(0.45, 0.85, 0.18), NEON_CYAN, Enum.Material.Neon), 0, 0.18, 0.96)
		buildDutyBelt(TACTICAL_BLACK, NEON_CYAN, false)

		buildTacticalArm("LeftArm", true, STEEL_GUNMETAL, STEEL_LIGHT, false, NEON_CYAN)
		buildTacticalArm("RightArm", false, STEEL_GUNMETAL, STEEL_LIGHT, false, NEON_CYAN)
		buildTacticalBootsAndLegs(STEEL_GUNMETAL, TACTICAL_BLACK, NEON_CYAN)

		-- Left Arm: Colossal Titanium Breaching Shield with Glowing Cyan Cross (LeftArm pitch=0)
		buildBallisticShield(0, TACTICAL_BLACK, STEEL_LIGHT, NEON_CYAN, true)

		-- Right Arm: Heavy Automatic Assault Shotgun / SMG with Drum Magazine (RightArm pitch=0)
		gunAt(0, P("Weapon", Vector3.new(0.28, 0.55, 2.15), TACTICAL_BLACK, Enum.Material.Metal), 0, 0.08, -1.05)
		gunAt(0, P("HeavyBarrel", Vector3.new(0.95, 0.22, 0.22), STEEL_LIGHT, Enum.Material.Metal, CYL), 0, 0.18, -2.35, 0, 90, 0)
		gunAt(0, P("DrumMag", Vector3.new(0.38, 0.62, 0.62), STEEL_GUNMETAL, Enum.Material.Metal, CYL), 0, -0.32, -0.95)
		gunAt(0, P("TopRailOptic", Vector3.new(0.22, 0.24, 0.52), NEON_CYAN, Enum.Material.Neon), 0, 0.42, -0.95)

	-- -------------------------------------------------------------------------
	-- UNIT 8: SHOTGUNNER (A-Tier Heavy Shock Breacher - RangedMarksman, pitch=78)
	-- -------------------------------------------------------------------------
	elseif uName == "Shotgunner" then
		buildHumanFace(true, BEARD_DARK, false)
		at("Head", P("BreacherHelmTop", Vector3.new(1.28, 0.45, 1.28), WHITE_ARMOR, Enum.Material.SmoothPlastic), 0, 0.44, 0)
		at("Head", P("BreacherHelmRim", Vector3.new(1.32, 0.18, 1.32), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.22, 0)

		at("Body", P("BreacherVest", Vector3.new(2.15, 1.75, 1.18), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.10, 0)
		at("Body", P("WhiteShoulderYoke", Vector3.new(2.18, 0.55, 1.22), WHITE_ARMOR, Enum.Material.SmoothPlastic), 0, 0.65, 0)
		at("Body", P("CenterChestPlate", Vector3.new(1.25, 0.95, 0.22), STEEL_GUNMETAL, Enum.Material.Metal), 0, 0.12, -0.55)
		for sIdx = 1, 5 do
			at("Body", P("Shell12G_" .. sIdx, Vector3.new(0.38, 0.16, 0.16), MEDIC_RED, Enum.Material.SmoothPlastic, CYL), (-0.52 + (sIdx - 1) * 0.26), 0.18, -0.66, 0, 0, 90)
		end
		for _, sign in ipairs({ -1, 1 }) do
			at("Body", P("HipTasset_" .. (sign < 0 and "L" or "R"), Vector3.new(0.45, 0.85, 0.85), TACTICAL_BLACK, Enum.Material.SmoothPlastic), sign * 0.92, -0.88, 0, 0, 0, sign * -10)
		end
		buildDutyBelt(TACTICAL_BLACK, STEEL_LIGHT, false)

		buildTacticalArm("LeftArm", true, WHITE_ARMOR, TACTICAL_BLACK, true, nil)
		buildTacticalArm("RightArm", false, WHITE_ARMOR, TACTICAL_BLACK, true, nil)
		buildTacticalBootsAndLegs(TACTICAL_BLACK, STEEL_GUNMETAL, WHITE_ARMOR)

		-- Level Tactical Pump-Action Combat Shotgun (compensating 78 deg RightArm pitch)
		gunAt(78, P("Weapon", Vector3.new(0.24, 0.48, 2.25), TACTICAL_BLACK, Enum.Material.Metal), 0, 0.08, -1.05)
		gunAt(78, P("ShotgunBarrel", Vector3.new(1.25, 0.20, 0.20), STEEL_DARK, Enum.Material.Metal, CYL), 0, 0.22, -2.45, 0, 90, 0)
		gunAt(78, P("MagTube", Vector3.new(1.15, 0.18, 0.18), STEEL_GUNMETAL, Enum.Material.Metal, CYL), 0, 0.02, -2.35, 0, 90, 0)
		gunAt(78, P("PumpGrip", Vector3.new(0.28, 0.30, 0.72), OLIVE_DRAB, Enum.Material.SmoothPlastic), 0, 0.00, -2.05)
		gunAt(78, P("HeatShield", Vector3.new(0.26, 0.12, 1.15), STEEL_LIGHT, Enum.Material.Metal), 0, 0.32, -2.25)
		gunAt(78, P("ShotgunStock", Vector3.new(0.22, 0.52, 0.72), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.04, 0.35)

	-- -------------------------------------------------------------------------
	-- UNIT 9: HEAVY GUNNER (A-Tier Rapid-Fire Devastator - RangedMarksman, pitch=78)
	-- -------------------------------------------------------------------------
	elseif uName == "Heavy Gunner" then
		buildHumanFace(true, BEARD_DARK, false)
		at("Head", P("GunnerHelmet", Vector3.new(1.30, 0.52, 1.30), TACTICAL_BLACK, Enum.Material.SmoothPlastic), 0, 0.40, 0)
		at("Head", P("HelmetBrim", Vector3.new(1.34, 0.14, 1.34), STEEL_DARK, Enum.Material.Metal), 0, 0.20, 0)

		at("Body", P("GunnerVest", Vector3.new(2.22, 1.82, 1.22), STEEL_DARK, Enum.Material.SmoothPlastic), 0, 0.10, 0)
		at("Body", P("ChestPlate", Vector3.new(1.65, 1.25, 0.24), TACTICAL_BLACK, Enum.Material.Metal), 0, 0.18, -0.55)
		at("Body", P("AmmoBandolier", Vector3.new(2.15, 0.32, 1.26), GOLD_BRASS, Enum.Material.Metal), 0, 0.15, 0, 0, 0, 38)
		at("Body", P("AmmoBackpack", Vector3.new(1.55, 1.65, 0.75), STEEL_DARK, Enum.Material.Metal), 0, 0.15, 0.75)
		at("Body", P("AmmoChute", Vector3.new(0.35, 0.25, 0.95), GOLD_BRASS, Enum.Material.Metal), 0.78, -0.35, 0.45)
		buildDutyBelt(TACTICAL_BLACK, GOLD_BRASS, false)

		buildTacticalArm("LeftArm", true, STEEL_DARK, STEEL_DARK, true, nil)
		buildTacticalArm("RightArm", false, STEEL_DARK, STEEL_DARK, true, nil)
		buildTacticalBootsAndLegs(STEEL_DARK, TACTICAL_BLACK, nil)

		-- Level 6-Barrel Rotary Vulcan Minigun (compensating 78 deg RightArm pitch)
		gunAt(78, P("Weapon", Vector3.new(0.68, 0.68, 1.35), TACTICAL_BLACK, Enum.Material.Metal), 0, 0.05, -0.95)
		gunAt(78, P("TopCarryHandle", Vector3.new(0.18, 0.38, 0.95), STEEL_GUNMETAL, Enum.Material.Metal), 0, 0.48, -0.95)
		gunAt(78, P("AmmoDrum", Vector3.new(0.68, 0.72, 0.72), STEEL_DARK, Enum.Material.Metal, CYL), -0.45, -0.10, -0.95)
		for b = 1, 6 do
			local ang = math.rad((b - 1) * 60)
			local bx = math.cos(ang) * 0.22
			local by = 0.05 + math.sin(ang) * 0.22
			gunAt(78, P("VulcanBarrel_" .. b, Vector3.new(1.75, 0.12, 0.12), STEEL_GUNMETAL, Enum.Material.Metal, CYL), bx, by, -2.35, 0, 90, 0)
		end
		gunAt(78, P("BarrelClampMid", Vector3.new(0.20, 0.64, 0.64), TACTICAL_BLACK, Enum.Material.Metal, CYL), 0, 0.05, -2.25, 0, 90, 0)
		gunAt(78, P("MuzzleClamp", Vector3.new(0.24, 0.68, 0.68), TACTICAL_BLACK, Enum.Material.Metal, CYL), 0, 0.05, -3.12, 0, 90, 0)
	end
end

return RepublicRig
