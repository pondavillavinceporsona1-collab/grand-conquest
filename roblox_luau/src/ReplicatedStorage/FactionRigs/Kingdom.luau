-- KingdomRig.luau
-- Anatomical High-Detail Kingdom Rig Builder matching Mechanica's craftsmanship & Reference Design
-- Core shrinking, fluted gothic plate, articulated sabatons & runic weapons
-- All accessories attach to core limb keys: "Head", "Body", "LeftArm", "RightArm", "LeftLeg", "RightLeg"

local KingdomRig = {}

local STEEL_PLATE     = Color3.fromRGB(195, 205, 218)
local STEEL_DARK      = Color3.fromRGB(120, 128, 138)
local CHAINMAIL       = Color3.fromRGB(42, 46, 54)
local ROYAL_GOLD      = Color3.fromRGB(235, 185, 55)
local GOLD_NEON       = Color3.fromRGB(255, 220, 90)
local ROYAL_BLUE      = Color3.fromRGB(26, 68, 158)
local LEATHER_BROWN   = Color3.fromRGB(98, 62, 40)
local WHITE_SILK      = Color3.fromRGB(242, 244, 250)
local CELESTIAL_CYAN  = Color3.fromRGB(80, 220, 255)

function KingdomRig.build(ctx)
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

	-- Part constructor helper
	local function P(name, size, color, mat, shape)
		return makePart(name, size * s, color, mat or Enum.Material.Metal, model, shape)
	end
	local function W(name, size, color, mat)
		return makeWedge(name, size * s, color, mat or Enum.Material.Metal, model)
	end
	local function at(limbKey, part, x, y, z, rx, ry, rz)
		attach(limbKey, part, CFrame.new(x * s, y * s, z * s) * CFrame.Angles(math.rad(rx or 0), math.rad(ry or 0), math.rad(rz or 0)))
		return part
	end

	-- 1. Core Shrinking (Just like Mechanica setupCore)
	-- Shrink base R6 limbs to dark chainmail under-armor so gothic plate has true 3D depth
	body.Size = Vector3.new(1.85, 1.85, 0.85) * s
	body.Color = CHAINMAIL
	body.Material = Enum.Material.Slate
	head.Size = Vector3.new(1.05, 1.05, 1.05) * s
	head.Color = CHAINMAIL
	head.Material = Enum.Material.Slate
	leftArm.Size = Vector3.new(0.85, 1.85, 0.85) * s
	leftArm.Color = CHAINMAIL
	leftArm.Material = Enum.Material.Slate
	rightArm.Size = Vector3.new(0.85, 1.85, 0.85) * s
	rightArm.Color = CHAINMAIL
	rightArm.Material = Enum.Material.Slate
	leftLeg.Size = Vector3.new(0.85, 1.85, 0.85) * s
	leftLeg.Color = CHAINMAIL
	leftLeg.Material = Enum.Material.Slate
	rightLeg.Size = Vector3.new(0.85, 1.85, 0.85) * s
	rightLeg.Color = CHAINMAIL
	rightLeg.Material = Enum.Material.Slate

	-- 2. Anatomical Gothic Helm Builder (Sallet, Armet, Greathelm)
	-- -Z is FRONT (face/visor), +Z is BACK (plumes/neck lames)
	local function buildGothicHelm(style, plumeColor, hasCrown, hasWings)
		-- Cranium Cap
		at("Head", P("Cranium", Vector3.new(1.22, 0.55, 1.25), STEEL_PLATE), 0, 0.35, 0)
		-- Crest Ridge / Comb along top
		at("Head", P("HelmComb", Vector3.new(0.12, 0.35, 1.3), ROYAL_GOLD), 0, 0.65, 0)
		-- Brow Ridge (front, -Z)
		at("Head", P("BrowRidge", Vector3.new(1.26, 0.22, 0.45), STEEL_PLATE), 0, 0.22, -0.45)
		-- Visored Faceplate (front, -Z)
		at("Head", P("VisorPlate", Vector3.new(1.24, 0.55, 0.35), STEEL_PLATE), 0, -0.05, -0.52)
		-- Horizontal Breath/Eye Slit with golden rune glow
		at("Head", P("EyeSlit", Vector3.new(1.05, 0.08, 0.12), GOLD_NEON, Enum.Material.Neon), 0, 0.08, -0.64)
		-- Pointed Chin Bevor (protecting throat/jaw)
		at("Head", P("Bevor", Vector3.new(1.22, 0.45, 0.35), STEEL_PLATE), 0, -0.38, -0.45)
		-- Neck Lobster-Tail Lames (back of neck protection, +Z)
		for i = 1, 3 do
			at("Head", P("NeckLame_" .. i, Vector3.new(1.18 - i * 0.05, 0.14, 0.35), STEEL_PLATE), 0, -0.15 - i * 0.14, 0.48 + i * 0.06, -i * 12, 0, 0)
		end
		-- Cheek Guards
		at("Head", P("LCheek", Vector3.new(0.15, 0.65, 0.95), STEEL_PLATE), -0.58, -0.15, -0.05)
		at("Head", P("RCheek", Vector3.new(0.15, 0.65, 0.95), STEEL_PLATE), 0.58, -0.15, -0.05)

		-- Feather Plume (back of helm, +Z)
		if plumeColor then
			at("Head", P("PlumeHolder", Vector3.new(0.25, 0.25, 0.25), ROYAL_GOLD), 0, 0.65, 0.45)
			for i = 1, 3 do
				at("Head", P("Plume_" .. i, Vector3.new(0.18, 0.75, 0.28), plumeColor, Enum.Material.SmoothPlastic), 0, 0.85 + i * 0.25, 0.55 + i * 0.15, 25 + i * 12, 0, 0)
			end
		end

		-- Royal Crown
		if hasCrown then
			at("Head", P("CrownRing", Vector3.new(1.35, 0.22, 1.35), ROYAL_GOLD), 0, 0.55, 0)
			for i = 1, 6 do
				local ang = math.rad(i * 60)
				at("Head", P("CrownSpire_" .. i, Vector3.new(0.12, 0.45, 0.12), GOLD_NEON, Enum.Material.Neon), math.cos(ang) * 0.65, 0.78, math.sin(ang) * 0.65)
			end
		end

		-- Valkyrie / Angelic Helm Wings
		if hasWings then
			for _, sign in ipairs({ -1, 1 }) do
				for w = 1, 3 do
					at("Head", P("HelmWing_" .. (sign < 0 and "L_" or "R_") .. w, Vector3.new(0.1, 0.85, 0.35), WHITE_SILK, Enum.Material.SmoothPlastic), sign * (0.68 + w * 0.12), 0.35 + w * 0.32, -0.15 + w * 0.08, -15, sign * 20, sign * (35 + w * 10))
				end
			end
		end
	end

	-- 3. Anatomical Gothic Cuirass & Tabard Builder
	-- -Z is FRONT (breastplate, buckle, tabard), +Z is BACK (backplate, spine)
	local function buildGothicCuirass(tabardColor, trimColor)
		-- High Armored Neck Gorget
		at("Body", P("Gorget", Vector3.new(1.45, 0.35, 1.25), STEEL_PLATE), 0, 0.95, 0)
		at("Body", P("GorgetTrim", Vector3.new(1.5, 0.1, 1.3), trimColor or ROYAL_GOLD), 0, 0.85, 0)

		-- Fluted Gothic Breastplate with Central Plackart Ridge (front, -Z)
		at("Body", P("Breastplate", Vector3.new(2.15, 1.45, 0.45), STEEL_PLATE), 0, 0.25, -0.45)
		at("Body", P("PlackartRidge", Vector3.new(0.18, 1.5, 0.52), trimColor or ROYAL_GOLD), 0, 0.25, -0.48)

		-- Left & Right Fluting Ridges (front, -Z)
		at("Body", P("Flute_L", Vector3.new(0.1, 1.2, 0.48), STEEL_DARK), -0.55, 0.25, -0.46, 0, 0, -12)
		at("Body", P("Flute_R", Vector3.new(0.1, 1.2, 0.48), STEEL_DARK), 0.55, 0.25, -0.46, 0, 0, 12)

		-- Backplate with Spinal Armor Lames (back, +Z)
		at("Body", P("Backplate", Vector3.new(2.1, 1.65, 0.4), STEEL_PLATE), 0, 0.15, 0.45)
		at("Body", P("BackSpine", Vector3.new(0.22, 1.65, 0.45), trimColor or ROYAL_GOLD), 0, 0.15, 0.48)

		-- Waist Belt & Faulds (Articulated Hip Plates)
		at("Body", P("WaistBelt", Vector3.new(2.18, 0.25, 1.15), LEATHER_BROWN, Enum.Material.SmoothPlastic), 0, -0.55, 0)
		at("Body", P("BeltBuckle", Vector3.new(0.45, 0.32, 0.18), ROYAL_GOLD), 0, -0.55, -0.58)

		-- Hanging Gothic Tassets (Thigh Plates, front, -Z)
		at("Body", P("Tasset_L", Vector3.new(0.85, 0.85, 0.25), STEEL_PLATE), -0.65, -0.95, -0.45, -12, 0, -8)
		at("Body", P("Tasset_R", Vector3.new(0.85, 0.85, 0.25), STEEL_PLATE), 0.65, -0.95, -0.45, -12, 0, 8)

		-- Royal Cloth Tabard hanging between tassets (front, -Z)
		if tabardColor then
			at("Body", P("TabardCloth", Vector3.new(0.85, 1.85, 0.12), tabardColor, Enum.Material.SmoothPlastic), 0, -0.65, -0.52)
			at("Body", P("TabardTrim", Vector3.new(0.9, 0.15, 0.14), trimColor or ROYAL_GOLD), 0, -1.5, -0.52)
		end
	end

	-- 4. Articulated Gothic Plate Arm Builder
	local function buildPlateArm(armKey, isLeft, trimColor)
		local sign = isLeft and -1 or 1
		-- 3-Layer Cascading Gothic Pauldron
		at(armKey, P("PauldronTop_" .. (isLeft and "L" or "R"), Vector3.new(1.35, 0.45, 1.35), STEEL_PLATE), sign * 0.25, 0.85, 0, 0, 0, sign * -12)
		at(armKey, P("PauldronTrim_" .. (isLeft and "L" or "R"), Vector3.new(1.4, 0.12, 1.4), trimColor or ROYAL_GOLD), sign * 0.25, 0.85, 0, 0, 0, sign * -12)
		at(armKey, P("PauldronMid_" .. (isLeft and "L" or "R"), Vector3.new(1.25, 0.42, 1.25), STEEL_PLATE), sign * 0.28, 0.55, 0, 0, 0, sign * -10)
		at(armKey, P("PauldronLow_" .. (isLeft and "L" or "R"), Vector3.new(1.15, 0.38, 1.15), STEEL_PLATE), sign * 0.3, 0.28, 0, 0, 0, sign * -8)

		-- Winged Elbow Couter (Pointed guard protecting elbow joint, back of arm +Z)
		at(armKey, P("ElbowCouter_" .. (isLeft and "L" or "R"), Vector3.new(0.55, 0.55, 0.45), STEEL_PLATE), sign * 0.45, -0.25, 0.25, 0, sign * 15, sign * -15)
		at(armKey, P("CouterSpike_" .. (isLeft and "L" or "R"), Vector3.new(0.25, 0.25, 0.4), trimColor or ROYAL_GOLD), sign * 0.55, -0.25, 0.38)

		-- Vambrace (Forearm armor plate with double banded rivets)
		at(armKey, P("Vambrace_" .. (isLeft and "L" or "R"), Vector3.new(0.95, 0.95, 0.95), STEEL_PLATE), 0, -0.65, 0)
		at(armKey, P("VambraceBand1_" .. (isLeft and "L" or "R"), Vector3.new(1.0, 0.12, 1.0), trimColor or ROYAL_GOLD), 0, -0.45, 0)
		at(armKey, P("VambraceBand2_" .. (isLeft and "L" or "R"), Vector3.new(1.0, 0.12, 1.0), trimColor or ROYAL_GOLD), 0, -0.85, 0)

		-- Segmented Gothic Gauntlet on hand
		at(armKey, P("Gauntlet_" .. (isLeft and "L" or "R"), Vector3.new(0.98, 0.45, 0.98), STEEL_PLATE), 0, -1.05, 0)
	end

	-- 5. Articulated Gothic Plate Leg Builder
	local function buildPlateLeg(legKey, isLeft, trimColor)
		local sign = isLeft and -1 or 1
		-- Thigh Cuisse
		at(legKey, P("Cuisse_" .. (isLeft and "L" or "R"), Vector3.new(0.98, 0.85, 0.98), STEEL_PLATE), 0, 0.45, 0)

		-- Winged Knee Poleyn (front of knee, -Z)
		at(legKey, P("KneePoleyn_" .. (isLeft and "L" or "R"), Vector3.new(0.95, 0.45, 0.45), STEEL_PLATE), 0, 0, -0.45)
		at(legKey, P("PoleynBoss_" .. (isLeft and "L" or "R"), Vector3.new(0.35, 0.35, 0.22), trimColor or ROYAL_GOLD), 0, 0, -0.62)

		-- Layered Greave (Shin Armor)
		at(legKey, P("Greave_" .. (isLeft and "L" or "R"), Vector3.new(0.96, 0.95, 0.96), STEEL_PLATE), 0, -0.55, 0)
		at(legKey, P("GreaveRidge_" .. (isLeft and "L" or "R"), Vector3.new(0.12, 0.95, 0.25), trimColor or ROYAL_GOLD), 0, -0.55, -0.48)

		-- Pointed Gothic Sabaton (Segmented foot shoe, extends FORWARD to -Z)
		at(legKey, P("Sabaton_" .. (isLeft and "L" or "R"), Vector3.new(0.96, 0.35, 1.25), STEEL_PLATE), 0, -0.95, -0.15)
		at(legKey, P("SabatonTip_" .. (isLeft and "L" or "R"), Vector3.new(0.45, 0.25, 0.45), trimColor or ROYAL_GOLD), 0, -1.0, -0.75, -20, 0, 0)
	end

	-- Helper to make a detailed Gothic Runic Sword held angled naturally down/forward
	-- Base anchor at hand: (0, -0.85, -0.2), pitchAngle default -38 deg
	local function buildRunicSword(gripKey, lengthScale, runeColor, pitchAngle)
		local sc = lengthScale or 1
		local pitch = pitchAngle or -38
		local radP = math.rad(pitch)
		local sinP = math.sin(radP)
		local cosP = math.cos(radP)

		-- Helper to calculate rotated position along blade axis (d > 0 extends forward along blade)
		local function swordPos(d)
			return 0, -0.85 + d * sinP, -0.2 - d * cosP
		end

		-- Pommel behind hand
		local px, py, pz = swordPos(-0.45 * sc)
		at(gripKey, P("SwordPommel", Vector3.new(0.38, 0.38, 0.28), ROYAL_GOLD), px, py, pz, pitch, 0, 0)

		-- Leather Hilt Grip in hand
		local gx, gy, gz = swordPos(-0.1 * sc)
		at(gripKey, P("SwordGrip", Vector3.new(0.20, 0.20, 0.75 * sc), LEATHER_BROWN, Enum.Material.SmoothPlastic), gx, gy, gz, pitch, 0, 0)

		-- Crossguard with curved quillons
		local qx, qy, qz = swordPos(0.25 * sc)
		at(gripKey, P("SwordGuard", Vector3.new(1.45 * sc, 0.25, 0.32), ROYAL_GOLD), qx, qy, qz, pitch, 0, 0)
		at(gripKey, P("GuardGem", Vector3.new(0.26, 0.26, 0.36), runeColor or GOLD_NEON, Enum.Material.Neon), qx, qy, qz, pitch, 0, 0)

		-- Double-Edged Fluted Steel Blade (Named "Weapon" for combat detection)
		local bx, by, bz = swordPos((0.25 + 1.25 * sc))
		at(gripKey, P("Weapon", Vector3.new(0.28 * sc, 0.48 * sc, 2.5 * sc), STEEL_PLATE), bx, by, bz, pitch, 0, 0)

		-- Central Fuller with Runic Inlay
		at(gripKey, P("SwordFuller", Vector3.new(0.30 * sc, 0.25 * sc, 2.2 * sc), runeColor or GOLD_NEON, Enum.Material.Neon), bx, by, bz, pitch, 0, 0)

		-- Pointed Blade Tip
		local tx, ty, tz = swordPos((0.25 + 2.55 * sc))
		at(gripKey, P("BladeTip", Vector3.new(0.22 * sc, 0.40 * sc, 0.45 * sc), STEEL_PLATE), tx, ty, tz, pitch, 0, 0)
	end

	-- Helper to make a towering Gothic Kite / Tower Shield on LeftArm
	local function buildKiteShield(mountKey)
		at(mountKey, P("ShieldBody", Vector3.new(0.28, 3.0, 1.75), ROYAL_BLUE, Enum.Material.SmoothPlastic), -0.55, -0.3, -0.2)
		at(mountKey, P("ShieldRimTop", Vector3.new(0.32, 0.25, 1.85), STEEL_PLATE), -0.55, 1.1, -0.2)
		at(mountKey, P("ShieldRimL", Vector3.new(0.32, 3.05, 0.22), STEEL_PLATE), -0.55, -0.3, -1.0)
		at(mountKey, P("ShieldRimR", Vector3.new(0.32, 3.05, 0.22), STEEL_PLATE), -0.55, -0.3, 0.6)
		at(mountKey, P("ShieldCrossV", Vector3.new(0.34, 2.0, 0.35), ROYAL_GOLD), -0.59, -0.3, -0.2)
		at(mountKey, P("ShieldCrossH", Vector3.new(0.34, 0.35, 1.25), ROYAL_GOLD), -0.59, 0.05, -0.2)
		at(mountKey, P("ShieldCenterBoss", Vector3.new(0.42, 0.55, 0.55), GOLD_NEON, Enum.Material.Neon), -0.61, 0.05, -0.2)
	end

	-- -------------------------------------------------------------------------
	-- UNIT 1: FOOTMAN (C-Tier Gothic Line Infantry)
	-- Visored Sallet, Quartered Surcoat, Arming Sword angled forward/down
	-- -------------------------------------------------------------------------
	if uName == "Footman" then
		buildGothicHelm("Sallet", ROYAL_BLUE, false, false)
		buildGothicCuirass(ROYAL_BLUE, ROYAL_GOLD)
		buildPlateArm("LeftArm", true, STEEL_PLATE)
		buildPlateArm("RightArm", false, STEEL_PLATE)
		buildPlateLeg("LeftLeg", true, STEEL_PLATE)
		buildPlateLeg("RightLeg", false, STEEL_PLATE)

		-- Arming Sword held angled naturally down/forward at -38 degrees
		buildRunicSword("RightArm", 0.95, ROYAL_GOLD, -38)

		-- Heater Shield slung on back (+Z)
		at("Body", P("HeaterShield", Vector3.new(0.22, 2.2, 1.55), ROYAL_BLUE, Enum.Material.SmoothPlastic), 0, 0.1, 0.65, -15, 0, 15)
		at("Body", P("HeaterCross", Vector3.new(0.26, 1.6, 0.28), ROYAL_GOLD), 0, 0.1, 0.68, -15, 0, 15)

	-- -------------------------------------------------------------------------
	-- UNIT 2: ARCHER (C-Tier Royal Longbow Marksman)
	-- Green Hood / Kettle Hat, Brigandine, Upright Recurve Bow, Side Quiver
	-- -------------------------------------------------------------------------
	elseif uName == "Archer" then
		at("Head", P("KettleHat", Vector3.new(1.65, 0.35, 1.65), STEEL_PLATE), 0, 0.45, 0)
		at("Head", P("NasalGuard", Vector3.new(0.15, 0.65, 0.22), STEEL_PLATE), 0, 0.05, -0.55)
		at("Head", P("ArcherEyes", Vector3.new(0.85, 0.12, 0.15), GOLD_NEON, Enum.Material.Neon), 0, 0.15, -0.52)

		-- Leather Brigandine with brass rivets (front, -Z)
		at("Body", P("Brigandine", Vector3.new(2.1, 1.85, 0.45), LEATHER_BROWN, Enum.Material.SmoothPlastic), 0, 0, -0.45)
		for r = 1, 3 do
			for c = 1, 3 do
				at("Body", P("Rivet_" .. r .. "_" .. c, Vector3.new(0.12, 0.12, 0.12), ROYAL_GOLD), (c - 2) * 0.55, (r - 2) * 0.45, -0.68)
			end
		end

		-- Composite Recurve Longbow in Left Hand (held upright)
		at("LeftArm", P("Weapon", Vector3.new(0.22, 1.2, 0.35), LEATHER_BROWN, Enum.Material.Wood), -0.45, -0.4, -0.45, 0, 0, 15)
		at("LeftArm", P("BowLimbTop", Vector3.new(0.16, 1.5, 0.25), ROYAL_GOLD), -0.45, 0.85, -0.65, 25, 0, 15)
		at("LeftArm", P("BowLimbBot", Vector3.new(0.16, 1.5, 0.25), ROYAL_GOLD), -0.45, -1.65, -0.65, -25, 0, 15)
		at("LeftArm", P("BowString", Vector3.new(0.04, 3.4, 0.04), WHITE_SILK, Enum.Material.SmoothPlastic), -0.45, -0.4, -0.25, 0, 0, 15)

		-- Side Quiver with fletched arrows on Right Hip / Back (+Z)
		at("Body", P("Quiver", Vector3.new(0.55, 1.8, 0.55), LEATHER_BROWN, Enum.Material.SmoothPlastic), 0.85, -0.2, 0.45, -25, 0, 15)
		for i = 1, 4 do
			at("Body", P("Fletching_" .. i, Vector3.new(0.08, 0.45, 0.18), WHITE_SILK, Enum.Material.SmoothPlastic), 0.85 + (i % 2 == 0 and 0.15 or -0.15), 0.75, 0.45 + (i > 2 and 0.15 or -0.15), -25, 0, 15)
		end

	-- -------------------------------------------------------------------------
	-- UNIT 3: SHIELD MAN (C-Tier Heavy Bastion)
	-- Tower Kite Shield on Left Arm, Arming Sword in Right Hand
	-- -------------------------------------------------------------------------
	elseif uName == "Shield Man" then
		buildGothicHelm("Sallet", ROYAL_BLUE, false, false)
		buildGothicCuirass(ROYAL_BLUE, ROYAL_GOLD)
		buildPlateArm("LeftArm", true, ROYAL_GOLD)
		buildPlateArm("RightArm", false, ROYAL_GOLD)
		buildPlateLeg("LeftLeg", true, ROYAL_GOLD)
		buildPlateLeg("RightLeg", false, ROYAL_GOLD)

		buildKiteShield("LeftArm")
		buildRunicSword("RightArm", 0.85, ROYAL_GOLD, -38)

	-- -------------------------------------------------------------------------
	-- UNIT 4: KNIGHT (B-Tier Gothic Armored Vanguard)
	-- Armet Helm, Blue Plume, Blue/White Cuirass, Broadsword angled down
	-- -------------------------------------------------------------------------
	elseif uName == "Knight" then
		buildGothicHelm("Armet", ROYAL_BLUE, false, false)
		buildGothicCuirass(ROYAL_BLUE, ROYAL_GOLD)
		buildPlateArm("LeftArm", true, ROYAL_GOLD)
		buildPlateArm("RightArm", false, ROYAL_GOLD)
		buildPlateLeg("LeftLeg", true, ROYAL_GOLD)
		buildPlateLeg("RightLeg", false, ROYAL_GOLD)

		buildRunicSword("RightArm", 1.15, GOLD_NEON, -38)

	-- -------------------------------------------------------------------------
	-- UNIT 5: MAGE (B-Tier Celestial Arcanist)
	-- Pointed Wizard Hat, Arcane Robes, Upright Conduit Staff with Cyan Core
	-- -------------------------------------------------------------------------
	elseif uName == "Mage" then
		-- Pointed Wizard Hat & Mantle
		at("Head", P("WizardBrim", Vector3.new(1.85, 0.18, 1.85), ROYAL_BLUE, Enum.Material.SmoothPlastic), 0, 0.45, 0)
		at("Head", P("WizardCone", Vector3.new(1.15, 1.45, 1.15), ROYAL_BLUE, Enum.Material.SmoothPlastic), 0, 1.15, -0.08, -12, 0, 0)
		at("Head", P("WizardTip", Vector3.new(0.65, 0.95, 0.65), ROYAL_BLUE, Enum.Material.SmoothPlastic), 0, 1.95, -0.28, -25, 0, 0)
		at("Head", P("FaceShadow", Vector3.new(0.95, 0.75, 0.25), Color3.fromRGB(15, 18, 25), Enum.Material.SmoothPlastic), 0, 0, -0.52)
		at("Head", P("ArcaneEyes", Vector3.new(0.75, 0.12, 0.15), CELESTIAL_CYAN, Enum.Material.Neon), 0, 0.08, -0.62)

		-- Flowing Arcane Robes Cuirass (front, -Z)
		at("Body", P("MageRobe", Vector3.new(2.15, 2.3, 0.55), ROYAL_BLUE, Enum.Material.SmoothPlastic), 0, -0.15, -0.45)
		at("Body", P("RobeStole", Vector3.new(0.75, 2.4, 0.15), WHITE_SILK, Enum.Material.SmoothPlastic), 0, -0.15, -0.55)

		-- Celestial Crystal Conduit Staff in Right Hand (stands upright)
		at("RightArm", P("MageStaff", Vector3.new(0.18, 4.4, 0.18), ROYAL_GOLD), 0, 0.6, -0.45, 5, 0, 0)
		at("RightArm", P("StaffHeadMount", Vector3.new(0.65, 0.55, 0.65), ROYAL_GOLD), 0, 2.7, -0.45)
		at("RightArm", P("Weapon", Vector3.new(0.75, 1.0, 0.75), CELESTIAL_CYAN, Enum.Material.Neon), 0, 3.3, -0.45, 0, 45, 45)

		-- Floating Arcane Ring behind head (+Z)
		for i = 1, 8 do
			local ang = math.rad(i * 45)
			at("Head", P("RuneShard_" .. i, Vector3.new(0.12, 0.35, 0.12), CELESTIAL_CYAN, Enum.Material.Neon), math.cos(ang) * 0.95, math.sin(ang) * 0.95, 0.55)
		end

		-- Grimoire Tome on Left Hip
		at("Body", P("Grimoire", Vector3.new(0.75, 1.1, 0.35), LEATHER_BROWN, Enum.Material.SmoothPlastic), -0.95, -0.65, 0, 25, -15, 0)
		at("Body", P("GrimoireClasp", Vector3.new(0.35, 0.35, 0.38), ROYAL_GOLD), -0.97, -0.65, 0, 25, -15, 0)

	-- -------------------------------------------------------------------------
	-- UNIT 6: PRIEST (B-Tier Holy Crusader Inquisitor)
	-- White Mitre / Cowl, Holy Crosier with Sun Disc in Right Hand
	-- -------------------------------------------------------------------------
	elseif uName == "Priest" then
		buildGothicHelm("Armet", WHITE_SILK, false, false)
		buildGothicCuirass(WHITE_SILK, ROYAL_GOLD)
		buildPlateArm("LeftArm", true, ROYAL_GOLD)
		buildPlateArm("RightArm", false, ROYAL_GOLD)
		buildPlateLeg("LeftLeg", true, ROYAL_GOLD)
		buildPlateLeg("RightLeg", false, ROYAL_GOLD)

		-- Golden Sun Crosier / Holy Staff in Right Hand (stands upright)
		at("RightArm", P("PriestStaff", Vector3.new(0.18, 4.4, 0.18), ROYAL_GOLD), 0, 0.6, -0.45, 5, 0, 0)
		at("RightArm", P("Weapon", Vector3.new(1.15, 1.15, 0.30), GOLD_NEON, Enum.Material.Neon), 0, 2.8, -0.45)
		at("RightArm", P("SunCrossH", Vector3.new(1.25, 0.22, 0.34), ROYAL_GOLD), 0, 2.8, -0.45)
		at("RightArm", P("SunCrossV", Vector3.new(0.22, 1.25, 0.34), ROYAL_GOLD), 0, 2.8, -0.45)

		-- Holy Radiant Halo floating behind head (+Z)
		for i = 1, 10 do
			local ang = math.rad(i * 36)
			at("Head", P("HolyHalo_" .. i, Vector3.new(0.12, 0.28, 0.12), GOLD_NEON, Enum.Material.Neon), math.cos(ang) * 0.95, math.sin(ang) * 0.95, 0.55)
		end

	-- -------------------------------------------------------------------------
	-- UNIT 7: ELITE KNIGHT (A-Tier Supreme Champion - Matches Reference Design!)
	-- Winged Greathelm, Upright Royal Lance/Spear in Right Hand, Shield on Left Arm
	-- -------------------------------------------------------------------------
	elseif uName == "Elite Knight" then
		-- Winged Celestial Armet with Royal Gold Crown
		buildGothicHelm("Armet", ROYAL_BLUE, true, true)
		buildGothicCuirass(ROYAL_BLUE, ROYAL_GOLD)
		buildPlateArm("LeftArm", true, ROYAL_GOLD)
		buildPlateArm("RightArm", false, ROYAL_GOLD)
		buildPlateLeg("LeftLeg", true, ROYAL_GOLD)
		buildPlateLeg("RightLeg", false, ROYAL_GOLD)

				-- Royal Celestial Broadsword in Right Hand (Angled naturally down and forward at hip, matching Knight)
		buildRunicSword("RightArm", 1.15, GOLD_NEON, -38)

		-- Royal Blue Heater Shield on Left Arm
		at("LeftArm", P("HeaterShield", Vector3.new(0.25, 2.6, 1.8), ROYAL_BLUE, Enum.Material.SmoothPlastic), -0.55, -0.3, -0.2)
		at("LeftArm", P("HeaterCrossV", Vector3.new(0.30, 2.4, 0.30), ROYAL_GOLD), -0.58, -0.3, -0.2)
		at("LeftArm", P("HeaterCrossH", Vector3.new(0.30, 0.30, 1.4), ROYAL_GOLD), -0.58, 0.1, -0.2)

		-- Grand Lion-Embossed Royal Pauldron Bosses
		at("LeftArm", P("LionBoss_L", Vector3.new(0.75, 0.75, 0.45), ROYAL_GOLD), -0.75, 0.75, 0, 0, -35, 0)
		at("RightArm", P("LionBoss_R", Vector3.new(0.75, 0.75, 0.45), ROYAL_GOLD), 0.75, 0.75, 0, 0, 35, 0)

		-- Floating Crown Halo of the High King behind head (+Z)
		for i = 1, 12 do
			local ang = math.rad(i * 30)
			at("Head", P("CrownHalo_" .. i, Vector3.new(0.12, 0.35, 0.15), GOLD_NEON, Enum.Material.Neon), math.cos(ang) * 1.25, math.sin(ang) * 1.25, 0.65)
		end

	-- -------------------------------------------------------------------------
	-- UNIT 8: ELITE MAGE (A-Tier Celestial Arch-Mage)
	-- Glowing Crown & Wizard Hat, Double-Headed Celestial Staff, Open Spellbook
	-- -------------------------------------------------------------------------
	elseif uName == "Elite Mage" then
		-- Pointed Wizard Hat with Arch-Mage Crown
		at("Head", P("ArchWizardBrim", Vector3.new(1.95, 0.20, 1.95), ROYAL_BLUE, Enum.Material.SmoothPlastic), 0, 0.45, 0)
		at("Head", P("ArchWizardCone", Vector3.new(1.25, 1.55, 1.25), ROYAL_BLUE, Enum.Material.SmoothPlastic), 0, 1.20, -0.08, -12, 0, 0)
		at("Head", P("ArchWizardTip", Vector3.new(0.70, 1.05, 0.70), ROYAL_BLUE, Enum.Material.SmoothPlastic), 0, 2.05, -0.28, -25, 0, 0)
		at("Head", P("ArchMageCrown", Vector3.new(1.45, 0.25, 1.45), ROYAL_GOLD), 0, 0.55, 0)
		at("Head", P("ArchEyes", Vector3.new(0.85, 0.15, 0.18), CELESTIAL_CYAN, Enum.Material.Neon), 0, 0.08, -0.65)

		-- Arcane Chasuble Robes with Celestial Emblems (front, -Z)
		at("Body", P("ArchRobes", Vector3.new(2.2, 2.4, 0.65), ROYAL_BLUE, Enum.Material.SmoothPlastic), 0, -0.15, -0.45)
		at("Body", P("CenterRune", Vector3.new(0.85, 0.85, 0.18), CELESTIAL_CYAN, Enum.Material.Neon), 0, 0.25, -0.55)

		-- Arch-Mage Celestial Orb Staff in Right Hand (stands upright)
		at("RightArm", P("ArchStaff", Vector3.new(0.20, 5.0, 0.20), ROYAL_GOLD), 0, 0.8, -0.45, 5, 0, 0)
		at("RightArm", P("Weapon", Vector3.new(1.1, 1.1, 1.1), CELESTIAL_CYAN, Enum.Material.Neon), 0, 3.3, -0.45)
		-- 4 Orbiting Mana Shards around the orb
		for i = 1, 4 do
			local ang = math.rad(i * 90)
			at("RightArm", P("OrbitShard_" .. i, Vector3.new(0.18, 0.45, 0.18), GOLD_NEON, Enum.Material.Neon), math.cos(ang) * 0.95, 3.3 + math.sin(ang) * 0.95, -0.45)
		end

		-- Dual Concentric Floating Halos behind head (+Z)
		for i = 1, 8 do
			local ang = math.rad(i * 45)
			at("Head", P("HaloInner_" .. i, Vector3.new(0.1, 0.25, 0.1), CELESTIAL_CYAN, Enum.Material.Neon), math.cos(ang) * 0.85, math.sin(ang) * 0.85, 0.55)
		end
		for i = 1, 12 do
			local ang = math.rad(i * 30)
			at("Head", P("HaloOuter_" .. i, Vector3.new(0.12, 0.35, 0.12), GOLD_NEON, Enum.Material.Neon), math.cos(ang) * 1.35, math.sin(ang) * 1.35, 0.55)
		end

		-- Hovering Open Spell Tome in Left Hand
		at("LeftArm", P("OpenBookCover", Vector3.new(1.15, 0.15, 0.85), LEATHER_BROWN, Enum.Material.SmoothPlastic), -0.55, -0.65, -0.35, 25, 0, 15)
		at("LeftArm", P("OpenBookPages", Vector3.new(1.05, 0.18, 0.75), CELESTIAL_CYAN, Enum.Material.Neon), -0.55, -0.6, -0.35, 25, 0, 15)

	-- -------------------------------------------------------------------------
	-- UNIT 9: HIGH PRIEST (A-Tier Supreme Pontiff - Matches Reference Design!)
	-- Papal Mitre Crown, Sun Scepter, Radiant 16-Ray Halo, Sacred Reliquary
	-- -------------------------------------------------------------------------
	elseif uName == "High Priest" then
		-- Ornate Papal Mitre Crown with gold sun cross
		at("Head", P("PapalMitre", Vector3.new(1.25, 1.45, 1.15), WHITE_SILK, Enum.Material.SmoothPlastic), 0, 0.75, 0)
		at("Head", P("MitreCross", Vector3.new(0.18, 0.75, 0.18), ROYAL_GOLD), 0, 1.0, -0.52)
		at("Head", P("MitreGem", Vector3.new(0.35, 0.35, 0.25), GOLD_NEON, Enum.Material.Neon), 0, 1.15, -0.57)

		-- Regal Chasuble & Stole Vestments (front, -Z)
		at("Body", P("Chasuble", Vector3.new(2.25, 2.4, 0.65), WHITE_SILK, Enum.Material.SmoothPlastic), 0, -0.15, -0.45)
		at("Body", P("GoldStoleL", Vector3.new(0.35, 2.45, 0.12), ROYAL_GOLD), -0.55, -0.15, -0.55)
		at("Body", P("GoldStoleR", Vector3.new(0.35, 2.45, 0.12), ROYAL_GOLD), 0.55, -0.15, -0.55)

		-- Golden Sun-Cross Scepter in Right Hand (stands upright)
		at("RightArm", P("SunScepter", Vector3.new(0.20, 4.6, 0.20), ROYAL_GOLD), 0, 0.7, -0.45, 5, 0, 0)
		at("RightArm", P("Weapon", Vector3.new(1.25, 1.25, 0.35), GOLD_NEON, Enum.Material.Neon), 0, 3.0, -0.45)
		for i = 1, 8 do
			local ang = math.rad(i * 45)
			at("RightArm", P("SunRay_" .. i, Vector3.new(0.12, 0.55, 0.12), ROYAL_GOLD), math.cos(ang) * 0.85, 3.0 + math.sin(ang) * 0.85, -0.45, 0, 0, math.deg(ang))
		end

		-- Sacred Golden Reliquary in Left Hand
		at("LeftArm", P("Reliquary", Vector3.new(0.75, 0.95, 0.75), ROYAL_GOLD), -0.45, -0.75, -0.25)
		at("LeftArm", P("ReliquaryGlow", Vector3.new(0.55, 0.75, 0.55), GOLD_NEON, Enum.Material.Neon), -0.45, -0.75, -0.25)

		-- Radiant Multi-Ring Halo of the Heavens behind head (+Z)
		for i = 1, 16 do
			local ang = math.rad(i * 22.5)
			at("Head", P("HeavenHalo_" .. i, Vector3.new(0.12, 0.45, 0.12), GOLD_NEON, Enum.Material.Neon), math.cos(ang) * 1.45, math.sin(ang) * 1.45, 0.65)
		end
	end
end

return KingdomRig
