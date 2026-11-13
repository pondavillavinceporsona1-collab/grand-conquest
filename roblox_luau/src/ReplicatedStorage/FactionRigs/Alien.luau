-- AlienRig.luau
-- High-Detail Bio-Evolutionary 3D Geometric Rigs for Alien Faction
-- Intricate chitinous carapaces, xenomorph cranial crests, dorsal breathing vents,
-- glowing bio-plasma conduits, and specialized organic weapon systems!

local AlienRig = {}

local CHITIN_DARK     = Color3.fromRGB(16, 42, 22)
local CHITIN_SLATE    = Color3.fromRGB(24, 60, 32)
local CHITIN_MID      = Color3.fromRGB(36, 92, 48)
local CHITIN_HIGH     = Color3.fromRGB(52, 130, 68)
local TOXIC_SKIN      = Color3.fromRGB(45, 115, 58)
local FLESH_MUTANT    = Color3.fromRGB(72, 38, 95)
local FLESH_DARK      = Color3.fromRGB(48, 24, 65)

local NEON_LIME       = Color3.fromRGB(120, 255, 45)
local NEON_ACID       = Color3.fromRGB(175, 255, 35)
local NEON_CYAN       = Color3.fromRGB(45, 235, 255)
local NEON_PLASMA     = Color3.fromRGB(80, 255, 200)
local NEON_CORE       = Color3.fromRGB(220, 255, 120)
local NEON_PURPLE     = Color3.fromRGB(195, 60, 255)

function AlienRig.build(ctx)
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

	-- Base limb bio-chitin defaults
	head.Color = CHITIN_DARK
	body.Color = CHITIN_DARK
	leftArm.Color = CHITIN_DARK
	rightArm.Color = CHITIN_DARK
	leftLeg.Color = CHITIN_DARK
	rightLeg.Color = CHITIN_DARK

	if uName == "Alien Grunt" then
		-- =====================================================================
		-- C-TIER 1: ALIEN GRUNT (Xenomorph Bio-Predator, Swarm Drone) ~50 parts
		-- =====================================================================
		local dome = P("XenoDome", Vector3.new(1.18, 0.58, 1.95), CHITIN_DARK)
		at("Head", dome, 0, 0.35, 0.45, -14, 0, 0)
		local domeRidge = P("DomeRidge", Vector3.new(0.24, 0.65, 1.85), CHITIN_MID)
		at("Head", domeRidge, 0, 0.38, 0.45, -14, 0, 0)
		local domeGlow = P("DomeGlow", Vector3.new(0.08, 0.68, 1.55), NEON_LIME, NEON)
		at("Head", domeGlow, 0, 0.40, 0.45, -14, 0, 0)

		-- Quad Compound Eyes & Mandibles
		for _, sign in ipairs({ -1, 1 }) do
			local eye1 = P("CompoundEye1", Vector3.new(0.24, 0.08, 0.18), NEON_LIME, NEON)
			at("Head", eye1, sign * 0.38, 0.12, -0.55, 0, sign * 25, -sign * 15)
			local eye2 = P("CompoundEye2", Vector3.new(0.20, 0.08, 0.16), NEON_ACID, NEON)
			at("Head", eye2, sign * 0.42, -0.05, -0.52, 0, sign * 20, -sign * 10)
			local mandible = W("Mandible", Vector3.new(0.12, 0.45, 0.35), CHITIN_SLATE)
			at("Head", mandible, sign * 0.45, -0.38, -0.48, -30, 0, -sign * 20)
			local mGlow = P("MandibleGlow", Vector3.new(0.06, 0.35, 0.08), NEON_LIME, NEON)
			at("Head", mGlow, sign * 0.46, -0.38, -0.49, -30, 0, -sign * 20)
		end

		-- Chitin Ribcage on Chest with glowing acid spine
		local chest = P("ChitinChest", Vector3.new(2.10, 1.85, 0.45), CHITIN_SLATE)
		at("Body", chest, 0, 0.15, -0.45)
		local bioSpine = P("AcidSpine", Vector3.new(0.14, 1.65, 0.22), NEON_ACID, NEON)
		at("Body", bioSpine, 0, 0.15, -0.62)
		for r = 1, 2 do
			local ribL = P("RibL_" .. r, Vector3.new(0.75, 0.12, 0.18), CHITIN_MID)
			at("Body", ribL, -0.45, 0.45 - (r - 1) * 0.45, -0.60, 0, 0, 15)
			local ribR = P("RibR_" .. r, Vector3.new(0.75, 0.12, 0.18), CHITIN_MID)
			at("Body", ribR, 0.45, 0.45 - (r - 1) * 0.45, -0.60, 0, 0, -15)
		end

		-- 3 Dorsal Breathing Chimneys on Back
		for sp = 1, 3 do
			local spine = W("DorsalTube_" .. sp, Vector3.new(0.16, 0.65 - sp * 0.06, 0.55), CHITIN_SLATE)
			at("Body", spine, 0, 0.65 - (sp - 1) * 0.38, 0.65, 42, 0, 0)
			local sVent = P("TubeVent_" .. sp, Vector3.new(0.08, 0.12, 0.35), NEON_LIME, NEON)
			at("Body", sVent, 0, 0.70 - (sp - 1) * 0.38, 0.68, 42, 0, 0)
		end

		-- Arms: Spiked Elbow Spurs & Dual Razor Claws
		for _, sign in ipairs({ -1, 1 }) do
			local aKey = sign == -1 and "LeftArm" or "RightArm"
			local shoulder = P("ArmPlate", Vector3.new(1.15, 0.45, 1.15), CHITIN_SLATE)
			at(aKey, shoulder, 0, 0.55, 0)
			local elbowSpur = W("ElbowSpur", Vector3.new(0.12, 0.45, 0.45), CHITIN_HIGH)
			at(aKey, elbowSpur, sign * 0.45, -0.15, 0.45, -25, 0, sign * 15)
			local forearmChitin = P("ForearmChitin", Vector3.new(0.95, 0.85, 0.25), CHITIN_SLATE)
			at(aKey, forearmChitin, 0, -0.35, -0.52)

			for c = 1, 2 do
				local cOffset = (c == 1 and -0.20 or 0.20)
				local clawName = (sign == 1 and c == 1) and "Weapon" or ("Claw_" .. sign .. "_" .. c)
				local talon = W(clawName, Vector3.new(0.08, 0.18, 0.75), CHITIN_DARK)
				at(aKey, talon, cOffset, -0.85, -0.42)
				local talonGlow = P(clawName .. "_Glow", Vector3.new(0.06, 0.08, 0.45), NEON_ACID, NEON)
				at(aKey, talonGlow, cOffset, -0.82, -0.60)
			end
		end

		-- Legs: Spiked Knee Plates, Shin Chitin & Raptor Talons
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = W("KneeSpike", Vector3.new(0.16, 0.45, 0.35), CHITIN_HIGH)
			at(lKey, knee, 0, 0.25, -0.55, -25, 0, 0)
			local shin = P("ShinChitin", Vector3.new(0.95, 1.15, 0.28), CHITIN_SLATE)
			at(lKey, shin, 0, -0.35, -0.48)
			local shinGlow = P("ShinGlow", Vector3.new(0.08, 0.85, 0.14), NEON_LIME, NEON)
			at(lKey, shinGlow, 0, -0.35, -0.58)
			local raptorToe = W("RaptorToe", Vector3.new(0.14, 0.25, 0.55), CHITIN_DARK)
			at(lKey, raptorToe, 0, -0.88, -0.55, -15, 0, 0)
		end

	elseif uName == "Plasma Shooter" then
		-- =====================================================================
		-- C-TIER 2: PLASMA SHOOTER (Bio-Plasma Rifle Trooper) ~46 parts
		-- Note: RangedMarksman right arm pitches +78°, handCF counter-rotated -78°!
		-- =====================================================================
		local helm = P("PlasmaHelm", Vector3.new(1.18, 1.18, 1.18), CHITIN_DARK)
		at("Head", helm, 0, 0.05, 0)
		local visor = P("PlasmaVisor", Vector3.new(0.85, 0.16, 0.18), NEON_CYAN, NEON)
		at("Head", visor, 0, 0.08, -0.58)
		for _, sign in ipairs({ -1, 1 }) do
			local cheekFilter = P("CheekFilter", Vector3.new(0.24, 0.28, 0.35), CHITIN_SLATE, SMOOTH, CYL)
			at("Head", cheekFilter, sign * 0.45, -0.15, -0.42, 0, 90, 0)
			local anten = P("Antenna", Vector3.new(0.06, 0.65, 0.06), CHITIN_HIGH)
			at("Head", anten, sign * 0.45, 0.65, 0, 15, 0, sign * 15)
			local antenTip = P("AntenTip", Vector3.new(0.12, 0.12, 0.12), NEON_CYAN, NEON, BALL)
			at("Head", antenTip, sign * 0.52, 0.95, 0.05)
		end

		-- Bio-Chitin Combat Vest with Central Plasma Core & Back Exhaust
		local vest = P("CombatVest", Vector3.new(1.95, 1.65, 0.45), CHITIN_SLATE)
		at("Body", vest, 0, 0.15, -0.45)
		local plasmaCore = P("PlasmaCore", Vector3.new(0.48, 0.48, 0.25), NEON_CYAN, NEON, BALL)
		at("Body", plasmaCore, 0, 0.18, -0.62)
		local coreRing = P("CoreRing", Vector3.new(0.65, 0.65, 0.15), CHITIN_HIGH)
		at("Body", coreRing, 0, 0.18, -0.60)
		for sp = 1, 2 do
			local sign = sp == 1 and -1 or 1
			local exh = P("Exhaust_" .. sp, Vector3.new(0.25, 0.75, 0.25), CHITIN_DARK, SMOOTH, CYL)
			at("Body", exh, sign * 0.45, 0.45, 0.65, -20, 0, 0)
			local exhGlow = P("ExhGlow_" .. sp, Vector3.new(0.15, 0.15, 0.15), NEON_PLASMA, NEON, BALL)
			at("Body", exhGlow, sign * 0.45, 0.82, 0.75)
			local backSpur = W("BackSpur_" .. sp, Vector3.new(0.12, 0.45, 0.45), CHITIN_HIGH)
			at("Body", backSpur, sign * 0.65, 0.25, 0.60, 30, 0, sign * 15)
		end

		-- Pauldrons with Bio-Plasma Vents
		for _, sign in ipairs({ -1, 1 }) do
			local pKey = sign == -1 and "LeftArm" or "RightArm"
			local pauldron = P("ShooterPauldron", Vector3.new(1.25, 0.45, 1.25), CHITIN_SLATE)
			at(pKey, pauldron, 0, 0.65, 0)
			local pVent = P("ShooterVent", Vector3.new(0.15, 0.15, 0.15), NEON_CYAN, NEON, BALL)
			at(pKey, pVent, 0, 0.88, 0)
		end

		-- HANDHELD BIO-PLASMA RIFLE (RightArm): Hand pitch counter-rotated -78°!
		local handCF = CFrame.new(0, -0.85 * s, -0.15 * s) * CFrame.Angles(math.rad(-78), 0, 0)
		local rifleBody = P("RifleReceiver", Vector3.new(0.35, 0.55, 1.75), CHITIN_DARK)
		attach("RightArm", rifleBody, handCF * CFrame.new(0, 0, -1.05 * s))
		local rifleBarrel = P("Weapon", Vector3.new(0.24, 0.24, 1.45), NEON_CYAN, NEON, CYL)
		attach("RightArm", rifleBarrel, handCF * CFrame.new(0, 0.12 * s, -2.15 * s) * CFrame.Angles(math.rad(90), 0, 0))
		local rifleGrip = P("RifleStock", Vector3.new(0.22, 0.45, 0.65), CHITIN_SLATE)
		attach("RightArm", rifleGrip, handCF * CFrame.new(0, -0.25 * s, -0.35 * s))
		local rifleCell = P("PlasmaCell", Vector3.new(0.28, 0.45, 0.35), NEON_PLASMA, NEON)
		attach("RightArm", rifleCell, handCF * CFrame.new(0, -0.28 * s, -1.15 * s))
		for fin = 1, 3 do
			local rFin = W("RifleFin_" .. fin, Vector3.new(0.08, 0.25, 0.35), CHITIN_HIGH)
			attach("RightArm", rFin, handCF * CFrame.new(0, 0.32 * s, (-1.35 - (fin - 1) * 0.32) * s) * CFrame.Angles(math.rad(15), 0, 0))
		end

		-- Left Arm: Forearm Sensor Unit & Shield Plate
		local leftSensor = P("SensorUnit", Vector3.new(0.45, 0.75, 0.25), CHITIN_SLATE)
		at("LeftArm", leftSensor, 0, -0.35, -0.52)
		local sensorGlow = P("SensorGlow", Vector3.new(0.25, 0.35, 0.12), NEON_CYAN, NEON)
		at("LeftArm", sensorGlow, 0, -0.35, -0.60)
		local lForearmSpur = W("ForearmSpurL", Vector3.new(0.12, 0.45, 0.35), CHITIN_HIGH)
		at("LeftArm", lForearmSpur, -0.45, -0.35, 0, 0, -90, 0)

		-- Legs: Chitin Greaves, Knee Guards & Combat Sabatons
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = W("ShooterKnee", Vector3.new(0.16, 0.35, 0.35), CHITIN_HIGH)
			at(lKey, knee, 0, 0.25, -0.55, -20, 0, 0)
			local greave = P("ShooterGreave", Vector3.new(0.95, 1.25, 0.28), CHITIN_SLATE)
			at(lKey, greave, 0, -0.25, -0.48)
			local gGlow = P("GreaveGlow", Vector3.new(0.08, 0.95, 0.14), NEON_CYAN, NEON)
			at(lKey, gGlow, 0, -0.25, -0.58)
			local boot = P("CombatBoot", Vector3.new(1.05, 0.38, 1.15), CHITIN_DARK)
			at(lKey, boot, 0, -0.78, -0.05)
		end

	elseif uName == "Alien Guard" then
		-- =====================================================================
		-- C-TIER 3: ALIEN GUARD (Heavy Bio-Shield Bastion, Chitin Bulwark) ~46 parts
		-- =====================================================================
		local helm = P("GuardHelm", Vector3.new(1.25, 1.25, 1.25), CHITIN_DARK)
		at("Head", helm, 0, 0.05, 0)
		local visorH = P("GuardVisorH", Vector3.new(0.85, 0.14, 0.18), NEON_LIME, NEON)
		at("Head", visorH, 0, 0.08, -0.62)
		local hornPlate = W("ForeheadHorn", Vector3.new(0.35, 0.65, 0.85), CHITIN_HIGH)
		at("Head", hornPlate, 0, 0.55, -0.15, 25, 0, 0)
		for _, sign in ipairs({ -1, 1 }) do
			local cheekShield = P("CheekShield", Vector3.new(0.25, 0.65, 0.55), CHITIN_SLATE)
			at("Head", cheekShield, sign * 0.55, -0.05, -0.25, 0, sign * 20, 0)
		end

		-- Heavy Interlocking Chitin Breastplate with Sternum Core
		local breastplate = P("GuardBreastplate", Vector3.new(2.25, 1.95, 0.48), CHITIN_SLATE)
		at("Body", breastplate, 0, 0.15, -0.45)
		local sternumCore = P("SternumCore", Vector3.new(0.45, 0.65, 0.22), NEON_ACID, NEON)
		at("Body", sternumCore, 0, 0.22, -0.65)
		for _, sign in ipairs({ -1, 1 }) do
			local pGlow = P("ChestStripe", Vector3.new(0.12, 1.25, 0.16), NEON_LIME, NEON)
			at("Body", pGlow, sign * 0.55, 0.15, -0.62, 0, 0, sign * 25)
		end
		local belt = P("GuardBelt", Vector3.new(2.28, 0.32, 1.30), CHITIN_HIGH)
		at("Body", belt, 0, -0.65, 0)
		-- 3 Dorsal Back Plates
		for sp = 1, 3 do
			local bPlate = P("BackPlating_" .. sp, Vector3.new(0.22, 0.45, 0.35), CHITIN_SLATE)
			at("Body", bPlate, 0, 0.55 - (sp - 1) * 0.38, 0.65, 25, 0, 0)
		end

		-- Heavy Spiked Chitin Pauldrons
		for _, sign in ipairs({ -1, 1 }) do
			local pauldron = P("GuardPauldron", Vector3.new(1.45, 0.65, 1.45), CHITIN_HIGH)
			at("Body", pauldron, sign * 1.55, 0.85, 0, 0, 0, sign * -20)
			local pSpike = W("PauldronSpike", Vector3.new(0.18, 0.45, 0.45), CHITIN_DARK)
			at("Body", pSpike, sign * 1.85, 1.15, 0, 0, sign * 90, 0)
			local pGlow = P("PauldronGlow", Vector3.new(0.10, 0.75, 0.14), NEON_LIME, NEON)
			at("Body", pGlow, sign * 1.62, 0.88, 0, 0, 0, sign * -20)
		end

		-- MASSIVE ORGANIC CHITIN BULWARK SHIELD (LeftArm)
		local shield = P("Weapon", Vector3.new(0.38, 2.75, 1.65), CHITIN_DARK)
		at("LeftArm", shield, -0.45, -0.15, 0)
		local shieldCore = P("ShieldCore", Vector3.new(0.16, 1.85, 1.05), NEON_ACID, NEON)
		at("LeftArm", shieldCore, -0.60, -0.15, 0)
		local sBoss = P("ShieldBoss", Vector3.new(0.45, 0.65, 0.65), CHITIN_HIGH)
		at("LeftArm", sBoss, -0.62, -0.15, 0)
		for _, bSign in ipairs({ -1, 1 }) do
			local sWedge = W("ShieldWedge", Vector3.new(0.20, 0.85, 0.45), CHITIN_SLATE)
			at("LeftArm", sWedge, -0.55, -0.15, bSign * 0.75, 0, bSign * 90, 0)
			local sSpur = W("ShieldSpur", Vector3.new(0.12, 0.45, 0.35), CHITIN_HIGH)
			at("LeftArm", sSpur, -0.55, bSign * 0.95, 0, bSign * 90, 0, 0)
		end

		-- Right Arm: Heavy Bio-Cleaver / Stinger Mace
		local maceGrip = P("MaceGrip", Vector3.new(0.18, 2.05, 0.18), CHITIN_DARK, SMOOTH, CYL)
		at("RightArm", maceGrip, 0, -0.55, -0.35, 45, 0, 0)
		local maceHead = P("MaceHead", Vector3.new(0.65, 0.65, 0.65), CHITIN_HIGH, SMOOTH, BALL)
		at("RightArm", maceHead, 0, -1.25, -1.05)
		local maceGlow = P("MaceGlow", Vector3.new(0.35, 0.35, 0.35), NEON_LIME, NEON, BALL)
		at("RightArm", maceGlow, 0, -1.25, -1.05)
		for f = 1, 2 do
			local flSign = (f == 1 and -1 or 1)
			local mFlange = W("MaceFlange_" .. f, Vector3.new(0.12, 0.45, 0.45), CHITIN_DARK)
			at("RightArm", mFlange, flSign * 0.35, -1.25, -1.05, 0, 0, flSign * 45)
		end

		-- Heavy Armored Knee Shields & Greaves
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = P("KneeShield", Vector3.new(1.05, 1.05, 0.38), CHITIN_HIGH)
			at(lKey, knee, 0, 0.35, -0.48)
			local kSpike = W("KneeSpike", Vector3.new(0.18, 0.35, 0.35), CHITIN_DARK)
			at(lKey, kSpike, 0, 0.35, -0.72, -25, 0, 0)
			local greave = P("HeavyGreave", Vector3.new(1.05, 1.25, 0.32), CHITIN_SLATE)
			at(lKey, greave, 0, -0.25, -0.48)
			local sabaton = P("HeavySabaton", Vector3.new(1.10, 0.40, 1.20), CHITIN_DARK)
			at(lKey, sabaton, 0, -0.78, -0.05)
		end

	elseif uName == "Mutated Warrior" then
		-- =====================================================================
		-- B-TIER 1: MUTATED WARRIOR (Brute Brawler, Asymmetric Mutation) ~48 parts
		-- =====================================================================
		local skull = P("MutantSkull", Vector3.new(1.28, 1.28, 1.28), FLESH_DARK)
		at("Head", skull, 0, 0.05, 0)
		-- Quad Mutated Eyes & Jagged Bone Tusks
		local eyeL1 = P("MutantEyeL1", Vector3.new(0.22, 0.12, 0.16), NEON_ACID, NEON)
		at("Head", eyeL1, -0.32, 0.15, -0.62)
		local eyeL2 = P("MutantEyeL2", Vector3.new(0.18, 0.10, 0.14), NEON_LIME, NEON)
		at("Head", eyeL2, -0.45, -0.05, -0.58)
		local eyeR1 = P("MutantEyeR1", Vector3.new(0.28, 0.16, 0.18), NEON_PURPLE, NEON)
		at("Head", eyeR1, 0.35, 0.12, -0.62)
		for _, sign in ipairs({ -1, 1 }) do
			local tusk = W("JawTusk", Vector3.new(0.16, 0.45, 0.35), CHITIN_HIGH)
			at("Head", tusk, sign * 0.45, -0.35, -0.55, -35, 0, -sign * 15)
			local crestSpike = W("CrestSpike", Vector3.new(0.14, 0.45, 0.45), CHITIN_HIGH)
			at("Head", crestSpike, sign * 0.45, 0.65, 0.15, 30, 0, sign * 15)
		end

		-- Bulging Mutated Chestplate with Glowing Mutagen Nodes
		local chest = P("MutantChest", Vector3.new(2.35, 1.95, 0.55), FLESH_MUTANT)
		at("Body", chest, 0, 0.15, -0.45)
		for n = 1, 3 do
			local node = P("MutantNode_" .. n, Vector3.new(0.35, 0.35, 0.22), NEON_ACID, NEON, BALL)
			at("Body", node, (n - 2) * 0.55, 0.25 - (n % 2) * 0.35, -0.68)
		end
		-- 4 Staggered Dorsal Bone Spikes on Back & Mutagen Conduits
		for sp = 1, 4 do
			local bSpike = W("MutantSpine_" .. sp, Vector3.new(0.18, 0.65 - sp * 0.05, 0.65), CHITIN_HIGH)
			at("Body", bSpike, (sp % 2 == 0 and 0.25 or -0.25), 0.65 - (sp - 1) * 0.35, 0.68, 35, 0, (sp % 2 == 0 and 15 or -15))
		end
		for v = 1, 2 do
			local vSign = v == 1 and -1 or 1
			local vein = P("MutagenVein_" .. v, Vector3.new(0.10, 1.25, 0.14), NEON_PURPLE, NEON)
			at("Body", vein, vSign * 0.75, 0.15, -0.62, 0, 0, vSign * 20)
		end
		-- 3 Dorsal Vertebrae Spikes
		for v = 1, 3 do
			local vert = W("DorsalVert_" .. v, Vector3.new(0.14, 0.40, 0.35), CHITIN_HIGH)
			at("Body", vert, 0, 0.65 - (v - 1) * 0.35, 0.65, 30, 0, 0)
		end
		-- 2 Mutated Hip Scales
		for _, sign in ipairs({ -1, 1 }) do
			local hip = P("HipScale", Vector3.new(0.25, 0.65, 0.65), CHITIN_SLATE)
			at("Body", hip, sign * 1.05, -0.65, 0, 0, 0, sign * 15)
		end

		-- ASYMMETRICAL LEFT ARM: Massive Mutated Crusher Claw (Pincer Weapon)
		local lShoulder = P("MutantShoulderL", Vector3.new(1.65, 0.85, 1.65), FLESH_MUTANT)
		at("LeftArm", lShoulder, -0.25, 0.65, 0)
		local lCrusherForearm = P("CrusherForearm", Vector3.new(1.45, 1.65, 1.45), CHITIN_DARK)
		at("LeftArm", lCrusherForearm, -0.25, -0.35, 0)
		local pincerBase = P("Weapon", Vector3.new(0.45, 0.45, 0.45), NEON_ACID, NEON, BALL)
		at("LeftArm", pincerBase, -0.25, -1.25, -0.25)
		for p = 1, 3 do
			local pAngle = (p - 1) * (2 * math.pi / 3)
			local pincer = W("Pincer_" .. p, Vector3.new(0.16, 0.35, 1.25), CHITIN_HIGH)
			at("LeftArm", pincer, -0.25 + math.cos(pAngle) * 0.45, -1.35 + math.sin(pAngle) * 0.45, -0.75, 25, 0, math.deg(pAngle))
		end
		local lPincerGlow = P("PincerGlow", Vector3.new(0.25, 0.25, 0.25), NEON_PURPLE, NEON, BALL)
		at("LeftArm", lPincerGlow, -0.25, -1.25, 0)

		-- Right Arm: Scythe Blades & Spiked Knuckles
		local rShoulder = P("ShoulderR", Vector3.new(1.25, 0.55, 1.25), CHITIN_SLATE)
		at("RightArm", rShoulder, 0, 0.55, 0)
		local rForearmPlate = P("ForearmPlateR", Vector3.new(1.10, 0.95, 0.35), CHITIN_DARK)
		at("RightArm", rForearmPlate, 0, -0.35, -0.45)
		local rScythe1 = W("RScythe1", Vector3.new(0.12, 1.65, 0.45), CHITIN_HIGH)
		at("RightArm", rScythe1, 0, -0.45, -0.65, 25, 0, 0)
		local rScytheEdge = P("RScytheEdge", Vector3.new(0.06, 1.55, 0.15), NEON_LIME, NEON)
		at("RightArm", rScytheEdge, 0, -0.45, -0.85, 25, 0, 0)
		local rElbowSpur = W("ElbowSpurR", Vector3.new(0.14, 0.45, 0.45), CHITIN_HIGH)
		at("RightArm", rElbowSpur, 0.45, -0.15, 0.45, -25, 0, 15)

		-- Heavy Spiked Knee Carapaces & Raptor Talons
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = W("KneeSpike", Vector3.new(0.22, 0.55, 0.45), CHITIN_HIGH)
			at(lKey, knee, 0, 0.25, -0.55, -25, 0, 0)
			local greave = P("MutantGreave", Vector3.new(1.10, 1.35, 0.35), CHITIN_SLATE)
			at(lKey, greave, 0, -0.25, -0.48)
			local clawFoot = W("ClawFoot", Vector3.new(0.18, 0.30, 0.65), CHITIN_DARK)
			at(lKey, clawFoot, 0, -0.88, -0.55, -15, 0, 0)
		end

	elseif uName == "Plasma Specialist" then
		-- =====================================================================
		-- B-TIER 2: PLASMA SPECIALIST (Heavy Bio-Plasma Artillery Gunner) ~48 parts
		-- Note: RangedMarksman right arm pitches +78°, handCF counter-rotated -78°!
		-- =====================================================================
		local helm = P("SpecialistHelm", Vector3.new(1.22, 1.22, 1.22), CHITIN_DARK)
		at("Head", helm, 0, 0.05, 0)
		-- Tri-Optic Targeter (3 Glowing Neon Plasma Nodes) & Antenna Horns
		local opt1 = P("TriOptic_1", Vector3.new(0.16, 0.16, 0.16), NEON_PLASMA, NEON, BALL)
		at("Head", opt1, 0, 0.22, -0.62)
		local opt2 = P("TriOptic_2", Vector3.new(0.14, 0.14, 0.14), NEON_CYAN, NEON, BALL)
		at("Head", opt2, -0.22, -0.05, -0.62)
		local opt3 = P("TriOptic_3", Vector3.new(0.14, 0.14, 0.14), NEON_CYAN, NEON, BALL)
		at("Head", opt3, 0.22, -0.05, -0.62)
		for _, sign in ipairs({ -1, 1 }) do
			local filter = P("SideCanister", Vector3.new(0.28, 0.55, 0.28), CHITIN_SLATE, SMOOTH, CYL)
			at("Head", filter, sign * 0.55, -0.05, -0.25, 0, 0, sign * 15)
			local sHorn = W("SensorHorn", Vector3.new(0.14, 0.45, 0.45), CHITIN_HIGH)
			at("Head", sHorn, sign * 0.45, 0.55, 0, 20, 0, sign * 15)
		end

		-- Heavy Blast Carapace with Dual Back Plasma Generators
		local chest = P("HeavyVest", Vector3.new(2.15, 1.85, 0.48), CHITIN_SLATE)
		at("Body", chest, 0, 0.15, -0.45)
		local coreReactor = P("ReactorCore", Vector3.new(0.65, 0.65, 0.25), NEON_PLASMA, NEON, BALL)
		at("Body", coreReactor, 0, 0.18, -0.62)
		for sp = 1, 2 do
			local sign = sp == 1 and -1 or 1
			local gen = P("PlasmaGenerator_" .. sp, Vector3.new(0.48, 1.25, 0.48), CHITIN_DARK, SMOOTH, CYL)
			at("Body", gen, sign * 0.65, 0.45, 0.65, -15, 0, sign * 10)
			local genCap = P("GenCap_" .. sp, Vector3.new(0.35, 0.25, 0.35), NEON_CYAN, NEON, BALL)
			at("Body", genCap, sign * 0.65, 1.05, 0.75)
			local dorsalVent = P("DorsalVent_" .. sp, Vector3.new(0.14, 0.45, 0.25), NEON_PLASMA, NEON)
			at("Body", dorsalVent, sign * 0.35, 0.45, 0.62)
		end

		-- Backpack Radiators & Hip Batteries
		for _, sign in ipairs({ -1, 1 }) do
			local rad = P("CoolingRadiator", Vector3.new(0.15, 0.85, 0.45), CHITIN_SLATE)
			at("Body", rad, sign * 0.95, 0.25, 0.65, 0, sign * 20, 0)
			local bat = P("HipBattery", Vector3.new(0.35, 0.55, 0.35), CHITIN_DARK, SMOOTH, CYL)
			at("Body", bat, sign * 1.05, -0.65, 0)
		end

		-- Heavy Shoulders with Plasma Emitters
		for _, sign in ipairs({ -1, 1 }) do
			local aKey = sign == -1 and "LeftArm" or "RightArm"
			local pauldron = P("SpecPauldron", Vector3.new(1.35, 0.55, 1.35), CHITIN_SLATE)
			at(aKey, pauldron, 0, 0.65, 0)
			local pCap = P("PauldronCap", Vector3.new(0.25, 0.25, 0.25), NEON_CYAN, NEON, BALL)
			at(aKey, pCap, 0, 0.95, 0)
		end

		-- HEAVY DUAL-BARREL PLASMA SCATTER-CANNON (RightArm): Hand pitch counter-rotated -78°!
		local handCF = CFrame.new(0, -0.85 * s, -0.15 * s) * CFrame.Angles(math.rad(-78), 0, 0)
		local cannonBody = P("CannonReceiver", Vector3.new(0.55, 0.75, 2.05), CHITIN_DARK)
		attach("RightArm", cannonBody, handCF * CFrame.new(0, 0, -1.25 * s))
		local barrelTop = P("Weapon", Vector3.new(0.24, 0.24, 1.65), NEON_PLASMA, NEON, CYL)
		attach("RightArm", barrelTop, handCF * CFrame.new(0, 0.22 * s, -2.45 * s) * CFrame.Angles(math.rad(90), 0, 0))
		local barrelBtm = P("BarrelLower", Vector3.new(0.24, 0.24, 1.65), NEON_CYAN, NEON, CYL)
		attach("RightArm", barrelBtm, handCF * CFrame.new(0, -0.22 * s, -2.45 * s) * CFrame.Angles(math.rad(90), 0, 0))
		for _, fSign in ipairs({ -1, 1 }) do
			local muzzleFin = W("MuzzleFin", Vector3.new(0.08, 0.35, 0.55), CHITIN_HIGH)
			attach("RightArm", muzzleFin, handCF * CFrame.new(fSign * 0.32 * s, 0, -2.65 * s) * CFrame.Angles(0, fSign * math.rad(90), 0))
		end

		-- RightArm Power Conduits
		for c = 1, 2 do
			local pCond = P("ArmConduit_" .. c, Vector3.new(0.10, 0.65, 0.10), NEON_PLASMA, NEON, CYL)
			at("RightArm", pCond, 0.25 * (c == 1 and 1 or -1), -0.25, -0.45, 15, 0, 0)
		end

		-- Left Arm: Power Regulator Gauntlet
		local lGauntlet = P("PowerGauntlet", Vector3.new(1.15, 1.25, 1.15), CHITIN_SLATE)
		at("LeftArm", lGauntlet, 0, -0.35, 0)
		local regNode = P("RegulatorNode", Vector3.new(0.35, 0.35, 0.35), NEON_PLASMA, NEON, BALL)
		at("LeftArm", regNode, -0.55, -0.35, 0)

		-- Recoil-Stabilizer Greaves & Anchored Sabatons
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local greave = P("SpecGreave", Vector3.new(1.05, 1.35, 0.35), CHITIN_SLATE)
			at(lKey, greave, 0, -0.25, -0.48)
			local gGlow = P("SpecGlow", Vector3.new(0.10, 1.05, 0.14), NEON_PLASMA, NEON)
			at(lKey, gGlow, 0, -0.25, -0.62)
			local heelAnchor = W("HeelAnchor", Vector3.new(0.16, 0.35, 0.55), CHITIN_DARK)
			at(lKey, heelAnchor, 0, -0.85, 0.55, -25, 0, 0)
			local sabaton = P("HeavySabaton", Vector3.new(1.10, 0.40, 1.20), CHITIN_DARK)
			at(lKey, sabaton, 0, -0.78, -0.05)
		end

	elseif uName == "Acid Spitter" then
		-- =====================================================================
		-- B-TIER 3: ACID SPITTER (Caustic Bio-Caster, Venom Glands) ~44 parts
		-- =====================================================================
		local headBase = P("SpitterHead", Vector3.new(1.22, 1.22, 1.22), CHITIN_DARK)
		at("Head", headBase, 0, 0.05, 0)
		-- Swollen Venom Gland Cheeks & Spiked Cranial Ridge
		for _, sign in ipairs({ -1, 1 }) do
			local gland = P("AcidGland", Vector3.new(0.48, 0.55, 0.65), NEON_ACID, NEON, BALL)
			at("Head", gland, sign * 0.55, -0.15, -0.25)
			local eye = P("SpitterEye", Vector3.new(0.22, 0.10, 0.14), NEON_LIME, NEON)
			at("Head", eye, sign * 0.35, 0.15, -0.58)
			local fang = W("SpitterFang", Vector3.new(0.10, 0.35, 0.25), CHITIN_HIGH)
			at("Head", fang, sign * 0.32, -0.35, -0.55, -30, 0, -sign * 15)
			local crest = W("CranialCrest", Vector3.new(0.14, 0.45, 0.55), CHITIN_HIGH)
			at("Head", crest, sign * 0.35, 0.65, 0.15, 30, 0, sign * 15)
		end
		local maw = P("AcidMaw", Vector3.new(0.45, 0.25, 0.25), NEON_LIME, NEON)
		at("Head", maw, 0, -0.25, -0.58)

		-- Swollen Abdomen with 2 Glowing Acid Storage Sacs & Conduits
		local chest = P("ThoracicPlate", Vector3.new(2.10, 1.85, 0.55), CHITIN_SLATE)
		at("Body", chest, 0, 0.15, -0.45)
		for _, sign in ipairs({ -1, 1 }) do
			local sac = P("AcidSac", Vector3.new(0.65, 0.85, 0.55), NEON_ACID, NEON, BALL)
			at("Body", sac, sign * 0.75, 0.15, 0.45)
			local conduit = P("AcidConduit", Vector3.new(0.12, 1.15, 0.14), NEON_LIME, NEON)
			at("Body", conduit, sign * 0.65, 0.25, 0.25, 20, 0, sign * 15)
		end
		-- 3 Dorsal Chimneys
		for sp = 1, 3 do
			local chim = P("AcidChimney_" .. sp, Vector3.new(0.24, 0.85, 0.24), CHITIN_DARK, SMOOTH, CYL)
			at("Body", chim, 0, 0.65 - (sp - 1) * 0.42, 0.68, 30, 0, 0)
			local cGlow = P("ChimneyGlow_" .. sp, Vector3.new(0.14, 0.14, 0.14), NEON_LIME, NEON, BALL)
			at("Body", cGlow, 0, 0.72 - (sp - 1) * 0.42, 0.75)
		end

		-- WEAPONLESS CASTING: Floating Caustic Globule with Orbiting Droplets
		local acidOrb = P("Weapon", Vector3.new(0.65, 0.65, 0.65), NEON_ACID, NEON, BALL)
		at("RightArm", acidOrb, -0.55, -0.85, -0.75)
		local acidCore = P("AcidCore", Vector3.new(0.35, 0.35, 0.35), NEON_LIME, NEON, BALL)
		at("RightArm", acidCore, -0.55, -0.85, -0.75)
		for d = 1, 3 do
			local dAng = (d - 1) * (2 * math.pi / 3)
			local droplet = P("AcidDroplet_" .. d, Vector3.new(0.18, 0.18, 0.18), NEON_ACID, NEON, BALL)
			at("RightArm", droplet, -0.55 + math.cos(dAng) * 0.55, -0.85 + math.sin(dAng) * 0.55, -0.75)
		end
		-- Arm Venom Spur
		local rVenomSpur = W("VenomSpurR", Vector3.new(0.14, 0.45, 0.35), CHITIN_HIGH)
		at("RightArm", rVenomSpur, 0.45, -0.35, 0, 0, 90, 0)

		-- Left Arm: Acid Emitter Pod
		local lPod = P("AcidPodL", Vector3.new(0.85, 0.95, 0.85), CHITIN_SLATE)
		at("LeftArm", lPod, 0, -0.35, 0)
		local lPodGlow = P("PodGlowL", Vector3.new(0.35, 0.35, 0.35), NEON_ACID, NEON, BALL)
		at("LeftArm", lPodGlow, -0.45, -0.35, 0)

		-- Insectoid Greaves & Climbing Spurs
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = W("SpitterKnee", Vector3.new(0.16, 0.45, 0.35), CHITIN_HIGH)
			at(lKey, knee, 0, 0.25, -0.55, -20, 0, 0)
			local greave = P("SpitterGreave", Vector3.new(0.95, 1.25, 0.28), CHITIN_SLATE)
			at(lKey, greave, 0, -0.25, -0.48)
			local gGlow = P("GreaveGlow", Vector3.new(0.08, 0.95, 0.14), NEON_ACID, NEON)
			at(lKey, gGlow, 0, -0.25, -0.58)
			local boot = P("SpitterBoot", Vector3.new(1.02, 0.35, 1.10), CHITIN_DARK)
			at(lKey, boot, 0, -0.78, -0.05)
		end

	elseif uName == "Alien Juggernaut" then
		-- =====================================================================
		-- A-TIER 1: ALIEN JUGGERNAUT (Colossal Bio-Titan, Chitin Behemoth) ~48 parts
		-- Note: This is the champion statue unit for Shrine_Alien!
		-- Weapon is held raised at a 45° aggressive angle so LowestY > 0!
		-- =====================================================================
		local helm = P("TitanHelm", Vector3.new(1.38, 1.38, 1.38), CHITIN_DARK)
		at("Head", helm, 0, 0.10, 0)
		-- 3 Massive Swept Titan Horns
		local hornCenter = W("HornCenter", Vector3.new(0.35, 0.65, 1.85), CHITIN_HIGH)
		at("Head", hornCenter, 0, 0.85, 0.25, 30, 0, 0)
		for _, sign in ipairs({ -1, 1 }) do
			local hornSide = W("HornSide", Vector3.new(0.28, 0.55, 1.65), CHITIN_SLATE)
			at("Head", hornSide, sign * 0.65, 0.75, 0.25, 30, 0, sign * 20)
			local eye = P("TitanEye", Vector3.new(0.28, 0.12, 0.18), NEON_LIME, NEON)
			at("Head", eye, sign * 0.42, 0.12, -0.72)
			local tusk = W("TitanTusk", Vector3.new(0.18, 0.45, 0.38), CHITIN_HIGH)
			at("Head", tusk, sign * 0.52, -0.35, -0.65, -35, 0, -sign * 18)
		end

		-- Supermassive Juggernaut Breastplate with Pulsating Bio-Core
		local breastplate = P("TitanPlate", Vector3.new(2.45, 2.15, 0.58), CHITIN_SLATE)
		at("Body", breastplate, 0, 0.15, -0.45)
		local titanCore = P("TitanCore", Vector3.new(0.85, 0.85, 0.35), NEON_ACID, NEON, BALL)
		at("Body", titanCore, 0, 0.22, -0.68)
		for c = 1, 4 do
			local cAngle = (c - 1) * (math.pi / 2)
			local clamp = W("CoreClamp_" .. c, Vector3.new(0.14, 0.28, 0.35), CHITIN_HIGH)
			at("Body", clamp, math.cos(cAngle) * 0.55, 0.22 + math.sin(cAngle) * 0.55, -0.72, 0, 0, math.deg(cAngle))
		end
		local belt = P("TitanBelt", Vector3.new(2.48, 0.35, 1.45), CHITIN_HIGH)
		at("Body", belt, 0, -0.75, 0)

		-- Colossal Double-Tiered Chitin Pauldrons
		for _, sign in ipairs({ -1, 1 }) do
			local pLower = P("PauldronLower", Vector3.new(1.80, 0.90, 1.80), CHITIN_DARK)
			at("Body", pLower, sign * 1.88, 0.95, 0, 0, 0, sign * -22)
			local pUpper = P("PauldronUpper", Vector3.new(1.40, 0.70, 1.40), CHITIN_HIGH)
			at("Body", pUpper, sign * 1.98, 1.28, 0, 0, 0, sign * -25)
			local pGlow = P("PauldronGlow", Vector3.new(0.14, 1.05, 0.20), NEON_LIME, NEON)
			at("Body", pGlow, sign * 2.08, 1.30, 0, 0, 0, sign * -25)
		end

		-- 4 Heavy Dorsal Shell Mantlets on Back
		for d = 1, 4 do
			local dSign = (d <= 2 and -1 or 1)
			local dY = (d % 2 == 1 and 0.65 or -0.15)
			local shell = P("BackShell_" .. d, Vector3.new(0.28, 1.25, 0.45), CHITIN_SLATE)
			at("Body", shell, dSign * 0.85, dY, 0.75, 20, 0, dSign * 15)
		end

		-- COLOSSAL TITAN CHITIN CLEAVER GREATBLADE (RightArm)
		-- Raised forward at -45° combat angle so the weapon stays high above ground (LowestY > 0)
		local bladeHandle = P("BladeHandle", Vector3.new(0.22, 1.85, 0.22), CHITIN_DARK, SMOOTH, CYL)
		at("RightArm", bladeHandle, 0, -0.35, -0.45, -45, 0, 0)
		local bladeCore = P("Weapon", Vector3.new(0.24, 3.25, 0.95), CHITIN_DARK)
		at("RightArm", bladeCore, 0, 0.85, -1.45, -45, 0, 0)
		local bladeEdge = P("BladeNeonEdge", Vector3.new(0.10, 3.15, 0.28), NEON_ACID, NEON)
		at("RightArm", bladeEdge, 0, 0.85, -1.85, -45, 0, 0)
		for s = 1, 3 do
			local serration = W("Serration_" .. s, Vector3.new(0.12, 0.35, 0.35), CHITIN_HIGH)
			at("RightArm", serration, 0, 0.35 + (s - 1) * 0.75, -1.05, -30, 0, 0)
		end

		-- Left Arm: Heavy Titan Chitin Shield
		local shield = P("TitanShield", Vector3.new(0.45, 2.85, 1.85), CHITIN_DARK)
		at("LeftArm", shield, -0.55, -0.15, 0)
		local shieldRune = P("ShieldRune", Vector3.new(0.18, 2.05, 1.15), NEON_LIME, NEON)
		at("LeftArm", shieldRune, -0.72, -0.15, 0)

		-- Heavy Tiered Greaves & Sabatons
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = P("TitanKnee", Vector3.new(1.30, 0.65, 0.48), CHITIN_HIGH)
			at(lKey, knee, 0, 0.30, -0.60)
			local greave = P("TitanGreave", Vector3.new(1.30, 1.80, 1.30), CHITIN_SLATE)
			at(lKey, greave, 0, -0.25, 0)
			local gGlow = P("GreaveGlow", Vector3.new(0.12, 1.45, 0.16), NEON_ACID, NEON)
			at(lKey, gGlow, 0, -0.25, -0.70, 0, 0, sign * -22)
			local sabaton = P("TitanSabaton", Vector3.new(1.32, 0.45, 1.35), CHITIN_DARK)
			at(lKey, sabaton, 0, -0.80, -0.05)
		end

	elseif uName == "Plasma Overlord" then
		-- =====================================================================
		-- A-TIER 2: PLASMA OVERLORD (Psionic Plasma Architect) ~48 parts
		-- =====================================================================
		local cowl = P("OverlordCowl", Vector3.new(1.25, 1.25, 1.25), CHITIN_DARK)
		at("Head", cowl, 0, 0.05, 0)
		local royalVisor = P("RoyalVisor", Vector3.new(0.95, 0.16, 0.18), NEON_CYAN, NEON)
		at("Head", royalVisor, 0, 0.08, -0.62)
		-- 3 Levitating Psionic Plasma Crystals Hovering Above Head
		for i = 1, 3 do
			local cAngle = (i - 2) * 0.65
			local crystal = P("CrownCrystal_" .. i, Vector3.new(0.22, 0.75, 0.22), NEON_PLASMA, NEON)
			at("Head", crystal, math.sin(cAngle) * 0.85, 0.95, -0.15, 15, 0, math.deg(-cAngle))
		end
		-- 3 Psionic Shards
		for i = 1, 3 do
			local sAng = (i - 1) * (2 * math.pi / 3) + 0.35
			local shard = P("CrownShard_" .. i, Vector3.new(0.14, 0.45, 0.14), NEON_CYAN, NEON)
			at("Head", shard, math.cos(sAng) * 0.95, 1.25, math.sin(sAng) * 0.65, 20, math.deg(sAng), 0)
		end
		for _, sign in ipairs({ -1, 1 }) do
			local cowlFin = W("CowlFin", Vector3.new(0.14, 0.65, 0.55), CHITIN_HIGH)
			at("Head", cowlFin, sign * 0.65, 0.45, 0.15, 25, 0, sign * 18)
		end

		-- Regal Robes with Radiant Central Plasma Heart
		local robe = P("OverlordRobe", Vector3.new(2.20, 2.05, 0.58), CHITIN_SLATE, FABRIC)
		at("Body", robe, 0, 0.15, -0.45)
		local plasmaHeart = P("PlasmaHeart", Vector3.new(0.70, 0.70, 0.38), NEON_PLASMA, NEON, BALL)
		at("Body", plasmaHeart, 0, 0.22, -0.65)
		local innerGlow = P("InnerHeartGlow", Vector3.new(0.35, 0.35, 0.35), NEON_CORE, NEON, BALL)
		at("Body", innerGlow, 0, 0.22, -0.65)
		for _, sign in ipairs({ -1, 1 }) do
			local seal = P("RobeSeal", Vector3.new(0.18, 0.45, 0.18), NEON_PLASMA, NEON, BALL)
			at("Body", seal, sign * 0.55, -0.25, -0.62)
		end

		-- Pauldrons with Levitating Energy Nodes
		for _, sign in ipairs({ -1, 1 }) do
			local pauldron = P("RoyalPauldron", Vector3.new(1.45, 0.65, 1.45), CHITIN_DARK)
			at("Body", pauldron, sign * 1.55, 0.85, 0, 0, 0, sign * -20)
			local pCrystal = P("PauldronCrystal", Vector3.new(0.28, 0.55, 0.28), NEON_CYAN, NEON)
			at("Body", pCrystal, sign * 1.75, 1.15, 0, 0, 0, sign * -20)
		end

		-- 4 Floating Plasma Conduit Fins Hovering Behind Back
		for f = 1, 4 do
			local fSign = (f <= 2 and -1 or 1)
			local fY = (f % 2 == 1 and 0.65 or -0.15)
			local fin = P("PlasmaFin_" .. f, Vector3.new(0.18, 1.15, 0.35), CHITIN_HIGH)
			at("Body", fin, fSign * 1.65, fY, 0.78, 15, 0, fSign * 15)
			local finGlow = P("FinGlow_" .. f, Vector3.new(0.08, 0.95, 0.14), NEON_CYAN, NEON)
			at("Body", finGlow, fSign * 1.65, fY, 0.85, 15, 0, fSign * 15)
		end
		-- 3 Levitating Dorsal Mantlet Prisms
		for m = 1, 3 do
			local mPrism = W("MantletPrism_" .. m, Vector3.new(0.14, 0.75, 0.35), CHITIN_HIGH)
			at("Body", mPrism, (m - 2) * 0.65, 0.95, 0.75, -25, 0, (m - 2) * 15)
		end

		-- WEAPONLESS CASTING: Floating Tri-Prism Plasma Singularity
		local singularity = P("Weapon", Vector3.new(0.65, 0.65, 0.65), NEON_PLASMA, NEON, BALL)
		at("RightArm", singularity, -0.55, -0.85, -0.75)
		local singCore = P("SingularityCore", Vector3.new(0.35, 0.35, 0.35), NEON_CORE, NEON, BALL)
		at("RightArm", singCore, -0.55, -0.85, -0.75)
		for p = 1, 3 do
			local pAngle = (p - 1) * (2 * math.pi / 3)
			local pPrism = W("PrismShard_" .. p, Vector3.new(0.12, 0.35, 0.55), CHITIN_HIGH)
			at("RightArm", pPrism, -0.55 + math.cos(pAngle) * 0.55, -0.85 + math.sin(pAngle) * 0.55, -0.75, 0, 0, math.deg(pAngle))
		end

		-- Left Arm: Royal Bio-Scepter
		local scepterShaft = P("ScepterShaft", Vector3.new(0.16, 2.55, 0.16), CHITIN_DARK, SMOOTH, CYL)
		at("LeftArm", scepterShaft, 0, -0.55, -0.35, 35, 0, 0)
		local scepterHead = P("ScepterHead", Vector3.new(0.45, 0.45, 0.45), NEON_CYAN, NEON, BALL)
		at("LeftArm", scepterHead, 0, -1.45, -0.85)

		-- Regal Skirt with Plasma Inlays
		local skirt = P("OverlordSkirt", Vector3.new(2.35, 1.90, 1.25), CHITIN_DARK, FABRIC)
		at("Body", skirt, 0, -1.25, 0)
		for sk = 1, 3 do
			local sGlow = P("SkirtStripe_" .. sk, Vector3.new(0.10, 1.55, 0.14), NEON_CYAN, NEON)
			at("Body", sGlow, (sk - 2) * 0.65, -1.25, -0.65, 0, 0, (sk - 2) * 15)
		end

	elseif uName == "Hive Commander" then
		-- =====================================================================
		-- A-TIER 3: HIVE COMMANDER (Supreme Synaptic Sovereign) ~54 parts
		-- =====================================================================
		local headCrown = P("QueenCowl", Vector3.new(1.35, 1.35, 1.35), CHITIN_DARK)
		at("Head", headCrown, 0, 0.10, 0)
		-- Regal Queen Crest sweeping upward and outward
		local crestCenter = W("CrestCenter", Vector3.new(0.35, 0.75, 2.05), CHITIN_HIGH)
		at("Head", crestCenter, 0, 0.95, 0.35, 35, 0, 0)
		for _, sign in ipairs({ -1, 1 }) do
			local crestSide = W("CrestSide", Vector3.new(0.25, 0.65, 1.75), CHITIN_SLATE)
			at("Head", crestSide, sign * 0.65, 0.85, 0.35, 35, 0, sign * 22)
			local crownNeon = P("CrownNeon", Vector3.new(0.08, 0.55, 1.45), NEON_LIME, NEON)
			at("Head", crownNeon, sign * 0.68, 0.88, 0.35, 35, 0, sign * 22)
			local eye = P("MonarchEye", Vector3.new(0.28, 0.12, 0.18), NEON_ACID, NEON)
			at("Head", eye, sign * 0.38, 0.12, -0.70)
		end

		-- Monarch Breastplate with Synaptic Core
		local breastplate = P("MonarchPlate", Vector3.new(2.40, 2.15, 0.58), CHITIN_SLATE)
		at("Body", breastplate, 0, 0.15, -0.45)
		local synapseCore = P("SynapseCore", Vector3.new(0.80, 0.80, 0.38), NEON_CORE, NEON, BALL)
		at("Body", synapseCore, 0, 0.22, -0.68)
		for r = 1, 3 do
			local ribL = P("MonarchRibL_" .. r, Vector3.new(0.85, 0.12, 0.18), NEON_LIME, NEON)
			at("Body", ribL, -0.55, 0.55 - (r - 1) * 0.45, -0.65, 0, 0, 15)
			local ribR = P("MonarchRibR_" .. r, Vector3.new(0.85, 0.12, 0.18), NEON_LIME, NEON)
			at("Body", ribR, 0.55, 0.55 - (r - 1) * 0.45, -0.65, 0, 0, -15)
		end

		-- 4 Segmented Synaptic Spider Appendages Curving Over Shoulders
		for a = 1, 4 do
			local aSign = (a <= 2 and -1 or 1)
			local aZ = (a % 2 == 1 and 0.55 or 0.75)
			local arm1 = W("SynapseSpur_" .. a, Vector3.new(0.18, 0.45, 1.45), CHITIN_HIGH)
			at("Body", arm1, aSign * 0.95, 0.95, aZ, 45, 0, aSign * 25)
			local armGlow = P("SynapseGlow_" .. a, Vector3.new(0.08, 0.08, 1.25), NEON_ACID, NEON)
			at("Body", armGlow, aSign * 0.95, 1.02, aZ, 45, 0, aSign * 25)
		end

		-- Colossal Monarch Pauldrons
		for _, sign in ipairs({ -1, 1 }) do
			local pauldron = P("MonarchPauldron", Vector3.new(1.65, 0.75, 1.65), CHITIN_HIGH)
			at("Body", pauldron, sign * 1.75, 0.95, 0, 0, 0, sign * -22)
			local pSpike = W("PauldronSpike", Vector3.new(0.22, 0.45, 0.85), CHITIN_DARK)
			at("Body", pSpike, sign * 1.95, 1.15, 0, 0, sign * 90, 0)
			local pGlow = P("PauldronGlow", Vector3.new(0.12, 0.85, 0.16), NEON_LIME, NEON)
			at("Body", pGlow, sign * 1.82, 0.98, 0, 0, 0, sign * -22)
		end

		-- ROYAL HIVE QUEEN SCEPTER (RightArm)
		local staffShaft = P("StaffShaft", Vector3.new(0.20, 3.85, 0.20), CHITIN_DARK, SMOOTH, CYL)
		at("RightArm", staffShaft, 0, -0.25, -0.65, 25, 0, 0)
		local eggPrism = P("Weapon", Vector3.new(0.75, 1.05, 0.75), NEON_ACID, NEON, BALL)
		at("RightArm", eggPrism, 0, -1.85, -1.45, 25, 0, 0)
		for c = 1, 3 do
			local cAngle = (c - 1) * (2 * math.pi / 3)
			local cProng = W("StaffProng_" .. c, Vector3.new(0.12, 0.35, 0.65), CHITIN_HIGH)
			at("RightArm", cProng, math.cos(cAngle) * 0.45, -2.05 + math.sin(cAngle) * 0.45, -1.55, 25, 0, math.deg(cAngle))
		end

		-- Left Arm: Razor Talon Gauntlet
		local lGauntlet = P("TalonGauntlet", Vector3.new(1.30, 1.45, 1.30), CHITIN_SLATE)
		at("LeftArm", lGauntlet, 0, -0.35, 0)
		for t = 1, 3 do
			local talon = W("MonarchTalon_" .. t, Vector3.new(0.12, 0.22, 0.65), CHITIN_HIGH)
			at("LeftArm", talon, (t - 2) * 0.32, -1.05, -0.55)
		end

		-- Colossal Ornate Greaves & Sabatons
		for _, sign in ipairs({ -1, 1 }) do
			local lKey = sign == -1 and "LeftLeg" or "RightLeg"
			local knee = P("MonarchKnee", Vector3.new(1.25, 0.65, 0.45), CHITIN_HIGH)
			at(lKey, knee, 0, 0.30, -0.55)
			local greave = P("MonarchGreave", Vector3.new(1.25, 1.75, 1.25), CHITIN_SLATE)
			at(lKey, greave, 0, -0.25, 0)
			local gGlow = P("GreaveGlow", Vector3.new(0.12, 1.35, 0.16), NEON_LIME, NEON)
			at(lKey, gGlow, 0, -0.25, -0.65, 0, 0, sign * -20)
			local sabaton = P("MonarchSabaton", Vector3.new(1.25, 0.42, 1.30), CHITIN_DARK)
			at(lKey, sabaton, 0, -0.80, -0.05)
		end
	end
end

return AlienRig
