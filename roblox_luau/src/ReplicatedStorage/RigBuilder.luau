local Workspace = game:GetService("Workspace")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local RigBuilder = {}

local function makePart(name, size, color, mat, parent, shape)
	local p = Instance.new("Part")
	p.Name = name
	if shape then
		p.Shape = shape
	end
	p.Size = size
	p.Color = color
	p.Material = mat or Enum.Material.SmoothPlastic
	p.Reflectance = 0.06
	p.Anchored = true
	p.CanCollide = false
	p.CanQuery = false
	p.CanTouch = false
	p.CastShadow = true
	p.TopSurface = Enum.SurfaceType.Smooth
	p.BottomSurface = Enum.SurfaceType.Smooth
	p.Parent = parent
	return p
end

local function makeWedge(name, size, color, mat, parent)
	local w = Instance.new("WedgePart")
	w.Name = name
	w.Size = size
	w.Color = color
	w.Material = mat or Enum.Material.SmoothPlastic
	w.Reflectance = 0.06
	w.Anchored = true
	w.CanCollide = false
	w.CanQuery = false
	w.CanTouch = false
	w.CastShadow = true
	w.TopSurface = Enum.SurfaceType.Smooth
	w.BottomSurface = Enum.SurfaceType.Smooth
	w.Parent = parent
	return w
end

-- Sculpted 3D Gothic Skull relief for Dread/Elite Knights & Demon/Void Juggernauts
local function attachSculptedSkull(attachFn, limbKey, model, baseCF, s, plateDark, plateLight, eyeVoid, mat)
	local disc = makePart("SkullShield", Vector3.new(0.76, 0.82, 0.16) * s, plateDark, mat, model)
	attachFn(limbKey, disc, baseCF)

	local cranium = makePart("SkullCranium", Vector3.new(0.58, 0.38, 0.22) * s, plateLight, mat, model)
	attachFn(limbKey, cranium, baseCF * CFrame.new(0, 0.14 * s, -0.06 * s))

	local brow = makePart("SkullBrow", Vector3.new(0.60, 0.12, 0.26) * s, plateLight, mat, model)
	attachFn(limbKey, brow, baseCF * CFrame.new(0, 0.04 * s, -0.08 * s))

	local eyeL = makePart("SkullEyeL", Vector3.new(0.18, 0.10, 0.25) * s, eyeVoid, Enum.Material.SmoothPlastic, model)
	attachFn(limbKey, eyeL, baseCF * CFrame.new(-0.15 * s, -0.02 * s, -0.08 * s) * CFrame.Angles(0, 0, math.rad(-18)))

	local eyeR = makePart("SkullEyeR", Vector3.new(0.18, 0.10, 0.25) * s, eyeVoid, Enum.Material.SmoothPlastic, model)
	attachFn(limbKey, eyeR, baseCF * CFrame.new(0.15 * s, -0.02 * s, -0.08 * s) * CFrame.Angles(0, 0, math.rad(18)))

	local jaw = makePart("SkullJaw", Vector3.new(0.38, 0.18, 0.20) * s, plateDark, mat, model)
	attachFn(limbKey, jaw, baseCF * CFrame.new(0, -0.22 * s, -0.05 * s))

	for t = -1, 1 do
		local tooth = makePart("SkullTooth", Vector3.new(0.07, 0.14, 0.22) * s, plateLight, mat, model)
		attachFn(limbKey, tooth, baseCF * CFrame.new(t * 0.11 * s, -0.20 * s, -0.06 * s))
	end
end

-- Determine distinct visual archetype based on the unit's function/role and name!
local function getUnitArchetype(factionName, unitDef)
	local uName = unitDef.Name or ""

	-- 1. Hovering Drones & Bots (No legs, anti-grav thruster ring)
	if uName == "Medic Bot" or uName == "Assault Drone" or uName == "Repair Bot" or uName == "Shield Bot" then
		return "HoverDrone"
	end

	-- 2. Feral Beasts, Crawlers & Imps
	if uName == "Hell Hound" or uName == "Alien Grunt" or uName == "Imp" or uName == "Shadowling" then
		return "FeralBeast"
	end

	-- 3. Healers & Clerics (Priest, High Priest)
	if unitDef.HealAmount then
		return "HealerCleric"
	end

	-- 4. Heavy Shield Tanks (Shield Man, Riot Shield, Alien Guard, Dark Sentinel, Shield Robot, Demon Soldier)
	if unitDef.WeaponType == "Shield" and unitDef.Tier ~= "A" then
		return "ShieldTank"
	end

	-- 5. Grim Reapers & Scythe Wielders
	if string.find(uName, "Reaper") or uName == "Void Stalker" then
		return "GrimReaper"
	end

	-- 6. Artillery & Grenadiers
	if uName == "Grenade Thrower" or uName == "Heavy Artillery Bot" then
		return "ArtillerySiege"
	end

	-- 7. Arcane Mages, Warlocks & Bio-Casters
	if string.find(uName, "Mage") or string.find(uName, "Sorcerer") or string.find(uName, "Caster")
		or uName == "Soul Drainer" or uName == "Acid Spitter" or uName == "Plasma Overlord" then
		return "ArcaneCaster"
	end

	-- 8. Ranged Archers, Riflemen, Police, Shotgunners & Heavy Gunners
	if (unitDef.Range or 6) >= 15 and not unitDef.SummonUnitId and not unitDef.AttackSpeedAura then
		return "RangedMarksman"
	end

	-- 9. A-Tier Titan Champions & Reference Winged/Horned Juggernauts
	if unitDef.Tier == "A" then
		return "TitanChampion"
	end

	-- 10. B-Tier Armored Vanguard (Knight, Blood Knight, Mutated Warrior, Warbot)
	if unitDef.Tier == "B" then
		return "VanguardKnight"
	end

	-- 11. C-Tier Light Infantry (Footman, Riot Trooper, Combat Bot)
	return "LightInfantry"
end

function RigBuilder.BuildUnitRig(side, factionName, unitDef, spawnPos, parentFolder, isStatue)
	local model = Instance.new("Model")
	model.Name = (side or "Statue") .. "_" .. unitDef.Name

	local tier = unitDef.Tier or "C"
	local archetype = getUnitArchetype(factionName, unitDef)
	local isImp = (unitDef.Name == "Imp")
	local isQuadruped = (unitDef.Name == "Hell Hound")
	local scaleMult = (tier == "A" and 1.24) or (tier == "B" and 1.08) or 0.94
	if isImp then
		scaleMult = isStatue and 1.15 or 0.70
	elseif isStatue then
		scaleMult = 1.62
	end

	local s = scaleMult
	local torsoSize = isQuadruped and Vector3.new(1.65, 1.35, 2.75) * s or Vector3.new(2.0, 2.0, 1.0) * s
	local limbSize = isQuadruped and Vector3.new(0.65, 1.45, 0.65) * s or Vector3.new(1.0, 2.0, 1.0) * s
	local headSize = isQuadruped and Vector3.new(0.95, 0.95, 1.15) * s or Vector3.new(1.16, 1.16, 1.16) * s
	local isHover = (archetype == "HoverDrone")
	local groundY
	if isStatue then
		groundY = spawnPos.Y
	elseif spawnPos.Y > 50 then
		-- In Showcase Room (pavilions at Y ≈ 302.5, pedestals at Y ≈ 303.75)
		groundY = spawnPos.Y + (isQuadruped and (limbSize.Y + 0.35) or (limbSize.Y + torsoSize.Y * 0.5 + (isHover and 1.45 or 0.55)))
	else
		-- On Battlefield Lane (LaneFloor surface at Y = 0.70)
		groundY = (isQuadruped and (limbSize.Y + 0.70) or (limbSize.Y + torsoSize.Y * 0.5 + (isHover and 1.45 or 0.70)))
	end

	local baseFactionColor = unitDef.Color or Color3.fromRGB(65, 110, 200)
	local accentColor = unitDef.Accent or Color3.fromRGB(255, 215, 75)

	-- Distinct Armor/Cloth Palette
	local underSuit = Color3.fromRGB(22, 24, 30):Lerp(baseFactionColor, 0.16)
	local armorDark = Color3.fromRGB(32, 36, 44):Lerp(baseFactionColor, 0.28)
	local armorMid = Color3.fromRGB(48, 54, 66):Lerp(baseFactionColor, 0.45)
	local armorHighlight = Color3.fromRGB(72, 78, 92):Lerp(baseFactionColor, 0.55)
	local eyeVoid = Color3.fromRGB(10, 12, 16)
	local armorMat = (factionName == "Mechanica" and Enum.Material.Metal) or Enum.Material.SmoothPlastic

	local facingDir = (side == "Enemy") and -1 or 1
	local rootPos = Vector3.new(spawnPos.X, groundY, spawnPos.Z)

	-- 6 Core Limbs
	local body = makePart("Body", torsoSize, underSuit, armorMat, model)
	body.CFrame = CFrame.lookAt(rootPos, rootPos + Vector3.new(facingDir, 0, 0))

	local head = makePart("Head", headSize, armorDark, armorMat, model)
	local leftLeg = makePart("LeftLeg", limbSize, underSuit, armorMat, model)
	local rightLeg = makePart("RightLeg", limbSize, underSuit, armorMat, model)
	local leftArm = makePart("LeftArm", limbSize, underSuit, armorMat, model)
	local rightArm = makePart("RightArm", limbSize, underSuit, armorMat, model)

	if isHover then
		leftLeg.Transparency = 1
		rightLeg.Transparency = 1
	end

	local attachments = {}
	local function attach(limbKey, part, offsetCF)
		table.insert(attachments, {
			Limb = limbKey,
			Part = part,
			Offset = offsetCF,
		})
	end

	-- =========================================================================
	-- ARCHETYPE-SPECIFIC HEAD, TORSO, ARM, LEG & WEAPON SILHOUETTES
	-- =========================================================================

	if factionName == "Kingdom" then
		local KingdomRig = require(ReplicatedStorage.FactionRigs.Kingdom)
		KingdomRig.build({
			unitDef = unitDef,
			uName = unitDef.Name,
			s = s,
			model = model,
			body = body,
			head = head,
			leftArm = leftArm,
			rightArm = rightArm,
			leftLeg = leftLeg,
			rightLeg = rightLeg,
			makePart = makePart,
			makeWedge = makeWedge,
			attach = attach,
		})
	elseif factionName == "Republic" then
		local RepublicRig = require(ReplicatedStorage.FactionRigs.Republic)
		RepublicRig.build({
			unitDef = unitDef,
			uName = unitDef.Name,
			s = s,
			model = model,
			body = body,
			head = head,
			leftArm = leftArm,
			rightArm = rightArm,
			leftLeg = leftLeg,
			rightLeg = rightLeg,
			makePart = makePart,
			makeWedge = makeWedge,
			attach = attach,
		})
	elseif factionName == "Alien" then
		local AlienRig = require(ReplicatedStorage.FactionRigs.Alien)
		AlienRig.build({
			unitDef = unitDef,
			uName = unitDef.Name,
			s = s,
			model = model,
			body = body,
			head = head,
			leftArm = leftArm,
			rightArm = rightArm,
			leftLeg = leftLeg,
			rightLeg = rightLeg,
			makePart = makePart,
			makeWedge = makeWedge,
			attach = attach,
		})
	elseif factionName == "Demon" then
		local DemonRig = require(ReplicatedStorage.FactionRigs.Demon)
		DemonRig.build({
			unitDef = unitDef,
			uName = unitDef.Name,
			s = s,
			model = model,
			body = body,
			head = head,
			leftArm = leftArm,
			rightArm = rightArm,
			leftLeg = leftLeg,
			rightLeg = rightLeg,
			makePart = makePart,
			makeWedge = makeWedge,
			attach = attach,
		})
	elseif factionName == "Void" then
		local VoidRig = require(ReplicatedStorage.FactionRigs.Void)
		VoidRig.build({
			unitDef = unitDef,
			uName = unitDef.Name,
			s = s,
			model = model,
			body = body,
			head = head,
			leftArm = leftArm,
			rightArm = rightArm,
			leftLeg = leftLeg,
			rightLeg = rightLeg,
			makePart = makePart,
			makeWedge = makeWedge,
			attach = attach,
		})
	elseif factionName == "Mechanica" then
		-- =====================================================================
		-- MECHANICA TERMINATOR-STYLE ENDOSKELETON RIGS
		-- Chrome skulls with red eyes, exposed ribcage & hydraulic pistons,
		-- heavy plasma weaponry. Units: Combat Bot, Assault Drone, Shield Robot,
		-- Warbot, Plasma Robot, Repair Bot, Destroyer Mech, Heavy Artillery Bot,
		-- Cybernetic Commander
		-- =====================================================================
		local uName = unitDef.Name
		local chromeDark = Color3.fromRGB(42, 45, 51)
		local chromeSteel = Color3.fromRGB(154, 161, 171)
		local chromeBright = Color3.fromRGB(205, 210, 218)
		local jointBlack = Color3.fromRGB(18, 18, 22)
		local hydraulic = Color3.fromRGB(95, 100, 108)
		local redEye = Color3.fromRGB(255, 35, 35)
		local redCore = Color3.fromRGB(255, 70, 40)
		local teethSteel = Color3.fromRGB(190, 192, 196)
		local leatherBlack = Color3.fromRGB(34, 27, 23)
		local denimDark = Color3.fromRGB(32, 36, 48)
		local skinTone = Color3.fromRGB(212, 168, 138)
		local hairDark = Color3.fromRGB(28, 24, 20)
		local brassAmmo = Color3.fromRGB(185, 145, 60)
		local repairGreen = Color3.fromRGB(70, 255, 130)
		local MetalM = Enum.Material.Metal
		local PlateM = Enum.Material.DiamondPlate
		local NeonM = Enum.Material.Neon
		local PlainM = Enum.Material.SmoothPlastic
		local LeatherM = Enum.Material.Leather
		local CYL = Enum.PartType.Cylinder
		local BALL = Enum.PartType.Ball

		local function P(name, size, color, mat, shape)
			return makePart(name, size * s, color, mat or MetalM, model, shape)
		end
		local function at(limb, part, x, y, z, rx, ry, rz)
			attach(limb, part, CFrame.new(x * s, y * s, z * s)
				* CFrame.Angles(math.rad(rx or 0), math.rad(ry or 0), math.rad(rz or 0)))
			return part
		end

		-- Dark inner frame; thin limbs expose the chrome "bones" around them
		local function setupCore(thin)
			for _, limb in ipairs({ body, leftArm, rightArm, leftLeg, rightLeg }) do
				limb.Color = jointBlack
				limb.Material = PlainM
			end
			head.Color = chromeSteel
			head.Material = MetalM
			if thin then
				body.Size = Vector3.new(1.55, 1.9, 0.75) * s
				leftArm.Size = Vector3.new(0.42, 2.0, 0.42) * s
				rightArm.Size = Vector3.new(0.42, 2.0, 0.42) * s
				leftLeg.Size = Vector3.new(0.45, 2.0, 0.45) * s
				rightLeg.Size = Vector3.new(0.45, 2.0, 0.45) * s
			end
		end

		-- Chrome endoskeleton skull (halfSkin = left side keeps torn synthetic skin)
		local function buildSkull(halfSkin)
			at("Head", P("Cranium", Vector3.new(1.08, 0.28, 1.12), chromeBright), 0, 0.62, 0.02)
			if halfSkin then
				at("Head", P("BrowRidge", Vector3.new(0.56, 0.16, 0.18), chromeBright), 0.28, 0.2, -0.6)
			else
				at("Head", P("BrowRidge", Vector3.new(1.12, 0.16, 0.18), chromeBright), 0, 0.2, -0.6)
			end
			at("Head", P("NoseCavity", Vector3.new(0.16, 0.2, 0.06), jointBlack, PlainM), 0, -0.13, -0.6)
			at("Head", P("UpperTeeth", Vector3.new(0.62, 0.12, 0.08), teethSteel, PlateM), 0, -0.36, -0.6)
			at("Head", P("Jaw", Vector3.new(0.86, 0.22, 0.8), chromeSteel), 0, -0.64, -0.1)
			at("Head", P("LowerTeeth", Vector3.new(0.6, 0.1, 0.08), teethSteel, PlateM), 0, -0.5, -0.52)
			for _, side in ipairs({ -1, 1 }) do
				if not (halfSkin and side == -1) then
					at("Head", P("EyeSocket", Vector3.new(0.34, 0.24, 0.06), jointBlack, PlainM), side * 0.25, 0.03, -0.6)
					at("Head", P(side == 1 and "RedEyeR" or "RedEyeL", Vector3.new(0.18, 0.12, 0.06), redEye, NeonM), side * 0.25, 0.03, -0.63)
					at("Head", P("Cheekbone", Vector3.new(0.2, 0.32, 0.26), chromeBright), side * 0.5, -0.14, -0.42, 0, 0, side * -10)
				end
			end
			if halfSkin then
				at("Head", P("SyntheticSkin", Vector3.new(0.58, 1.0, 0.08), skinTone, PlainM), -0.3, -0.06, -0.61)
				at("Head", P("SkinSide", Vector3.new(0.08, 1.0, 1.0), skinTone, PlainM), -0.6, -0.06, -0.05)
				at("Head", P("HumanEye", Vector3.new(0.14, 0.08, 0.04), Color3.fromRGB(40, 60, 90), PlainM), -0.25, 0.03, -0.66)
				at("Head", P("TornEdge", Vector3.new(0.06, 0.95, 0.1), Color3.fromRGB(120, 28, 28), PlainM), -0.01, -0.06, -0.62)
				at("Head", P("Hair", Vector3.new(0.62, 0.32, 1.2), hairDark, PlainM), -0.3, 0.66, 0.02)
			end
		end

		-- Exposed ribcage, sternum, abdominal power cell, clavicles, neck pistons, pelvis
		local function buildEndoTorso()
			at("Body", P("Sternum", Vector3.new(0.22, 1.25, 0.14), chromeBright), 0, 0.25, -0.42)
			for r = 1, 3 do
				at("Body", P("Rib_" .. r, Vector3.new(1.62 - r * 0.14, 0.11, 0.12), chromeSteel), 0, 0.72 - (r - 1) * 0.3, -0.42)
			end
			at("Body", P("CoreHousing", Vector3.new(0.52, 0.52, 0.1), chromeDark), 0, -0.42, -0.4)
			at("Body", P("PowerCore", Vector3.new(0.34, 0.34, 0.1), redCore, NeonM), 0, -0.42, -0.44)
			for _, side in ipairs({ -1, 1 }) do
				at("Body", P("Clavicle", Vector3.new(0.85, 0.14, 0.2), chromeBright), side * 0.55, 0.93, -0.22, 0, 0, side * -8)
				at("Body", P("ShoulderJoint", Vector3.new(0.68, 0.68, 0.68), chromeSteel, MetalM, BALL), side * 1.3, 0.85, 0)
				at("Body", P("NeckPiston", Vector3.new(0.5, 0.11, 0.11), hydraulic, MetalM, CYL), side * 0.2, 1.2, 0.05, 0, 0, 90)
			end
			at("Body", P("NeckColumn", Vector3.new(0.3, 0.4, 0.3), chromeSteel), 0, 1.15, 0.1)
			at("Body", P("Pelvis", Vector3.new(1.5, 0.3, 0.72), chromeSteel), 0, -0.98, 0)
			at("Body", P("BackSpine", Vector3.new(0.32, 1.8, 0.2), chromeSteel, PlateM), 0, 0, 0.42)
		end

		local function buildEndoArm(aKey, opts)
			opts = opts or {}
			if not opts.noUpper then
				at(aKey, P("Humerus", Vector3.new(0.56, 0.78, 0.56), chromeSteel), 0, 0.38, 0)
			end
			at(aKey, P("Elbow", Vector3.new(0.6, 0.6, 0.6), chromeDark, MetalM, BALL), 0, -0.12, 0)
			at(aKey, P("Forearm", Vector3.new(0.6, 0.72, 0.6), chromeSteel), 0, -0.56, 0)
			if opts.piston then
				at(aKey, P("ArmPiston", Vector3.new(0.9, 0.12, 0.12), hydraulic, MetalM, CYL), 0, -0.2, 0.36, 0, 0, 90)
			end
			at(aKey, P("Hand", Vector3.new(0.5, 0.38, 0.56), chromeDark), 0, -1.08, -0.02)
		end

		local function buildEndoLeg(lKey, opts)
			opts = opts or {}
			if not opts.noFemur then
				at(lKey, P("Femur", Vector3.new(0.6, 0.82, 0.6), chromeSteel), 0, 0.4, 0)
			end
			at(lKey, P("KneeCap", Vector3.new(0.46, 0.42, 0.22), chromeBright), 0, -0.08, -0.32)
			at(lKey, P("Shin", Vector3.new(0.56, 0.72, 0.56), chromeSteel), 0, -0.52, 0)
			if opts.piston then
				at(lKey, P("CalfPiston", Vector3.new(0.85, 0.12, 0.12), hydraulic, MetalM, CYL), 0, -0.45, 0.34, 0, 0, 90)
			end
			if not opts.noFoot then
				at(lKey, P("Foot", Vector3.new(0.66, 0.24, 1.0), chromeDark, PlateM), 0, -0.88, -0.16)
			end
		end

		local function buildFullEndo(armOpts, legOpts)
			setupCore(true)
			buildSkull(false)
			buildEndoTorso()
			buildEndoArm("LeftArm", armOpts)
			buildEndoArm("RightArm", armOpts)
			buildEndoLeg("LeftLeg", legOpts)
			buildEndoLeg("RightLeg", legOpts)
		end

		-- Small skull-like sensor head for hovering units
		local function buildSensorHead()
			head.Size = Vector3.new(0.95, 0.55, 1.05) * s
			head.Color = chromeSteel
			head.Material = MetalM
			at("Head", P("SensorVisor", Vector3.new(0.85, 0.22, 0.08), jointBlack, PlainM), 0, 0, -0.53)
			for _, side in ipairs({ -1, 1 }) do
				at("Head", P(side == 1 and "SensorEyeR" or "SensorEyeL", Vector3.new(0.2, 0.13, 0.06), redEye, NeonM), side * 0.22, 0, -0.57)
			end
		end

		-- C-TIER 1: COMBAT BOT (classic T-800 endoskeleton, extending wrist blade)
		if uName == "Combat Bot" then
			buildFullEndo()
			at("RightArm", P("BladeHousing", Vector3.new(0.22, 0.6, 0.34), chromeDark), 0.36, -0.6, 0)
			at("RightArm", P("Weapon", Vector3.new(0.08, 1.5, 0.3), chromeBright), 0.36, -1.5, -0.06)
			at("RightArm", P("BladeEdge", Vector3.new(0.05, 1.35, 0.06), redEye, NeonM), 0.36, -1.55, -0.24)

		-- C-TIER 2: ASSAULT DRONE (HK hunter-killer flyer, twin plasma cannons)
		elseif uName == "Assault Drone" then
			setupCore(false)
			leftArm.Transparency = 1
			rightArm.Transparency = 1
			body.Color = chromeDark
			body.Material = PlateM
			buildSensorHead()
			at("Body", P("UpperHull", Vector3.new(2.1, 0.45, 2.4), chromeSteel, PlateM), 0, 1.15, 0.1)
			at("Body", P("Fuselage", Vector3.new(1.7, 1.1, 2.8), chromeDark, PlateM), 0, 0.25, 0.3)
			at("Body", makeWedge("NoseCone", Vector3.new(1.7, 1.1, 1.0) * s, chromeSteel, MetalM, model), 0, 0.25, -1.6)
			at("Body", makeWedge("TailFin", Vector3.new(0.15, 0.8, 0.9) * s, chromeSteel, MetalM, model), 0, 1.6, 1.2)
			at("Body", P("BellyThruster", Vector3.new(0.1, 1.1, 1.1), redCore, NeonM, CYL), 0, -0.35, 0.5, 0, 0, 90)
			for _, side in ipairs({ -1, 1 }) do
				at("Body", P("TurbineStrut", Vector3.new(0.6, 0.2, 0.4), chromeSteel), side * 1.05, 0.4, 0.4)
				at("Body", P("Turbine", Vector3.new(0.6, 1.2, 1.2), chromeSteel, MetalM, CYL), side * 1.45, 0.35, 0.4, 0, 0, 90)
				at("Body", P("TurbineGlow", Vector3.new(0.08, 1.0, 1.0), redCore, NeonM, CYL), side * 1.45, 0.02, 0.4, 0, 0, 90)
				at("Body", P(side == 1 and "Weapon" or "PlasmaCannonL", Vector3.new(0.38, 0.38, 1.7), chromeDark), side * 0.75, -0.55, -0.9)
				at("Body", P("CannonBarrel", Vector3.new(0.9, 0.2, 0.2), chromeSteel, MetalM, CYL), side * 0.75, -0.55, -2.1, 0, 90, 0)
				at("Body", P("CannonMuzzle", Vector3.new(0.06, 0.18, 0.18), redEye, NeonM, CYL), side * 0.75, -0.55, -2.57, 0, 90, 0)
			end

		-- C-TIER 3: SHIELD ROBOT (armored endoskeleton, riveted riot shield, piston fist)
		elseif uName == "Shield Robot" then
			buildFullEndo({ piston = true })
			at("Body", P("ChestArmor", Vector3.new(1.85, 0.62, 0.22), chromeSteel, PlateM), 0, 0.68, -0.52)
			at("Body", P("AbdomenArmor", Vector3.new(1.3, 0.35, 0.2), chromeSteel, PlateM), 0, -0.86, -0.45)
			for _, side in ipairs({ -1, 1 }) do
				at("Body", P("Pauldron", Vector3.new(0.95, 0.38, 1.0), chromeSteel, PlateM), side * 1.4, 1.22, 0, 0, 0, side * -12)
			end
			at("LeftArm", P("Weapon", Vector3.new(1.7, 2.5, 0.24), chromeDark, PlateM), -0.05, -0.55, -0.52)
			at("LeftArm", P("ShieldRimTop", Vector3.new(1.82, 0.14, 0.3), chromeBright), -0.05, 0.7, -0.52)
			at("LeftArm", P("ShieldRimBottom", Vector3.new(1.82, 0.14, 0.3), chromeBright), -0.05, -1.8, -0.52)
			at("LeftArm", P("ShieldSlit", Vector3.new(0.95, 0.1, 0.05), redEye, NeonM), -0.05, 0.3, -0.66)
			at("RightArm", P("PistonFist", Vector3.new(0.72, 0.62, 0.72), chromeDark, PlateM), 0, -1.1, 0)
			at("RightArm", P("FistRam", Vector3.new(0.5, 0.45, 0.25), chromeBright), 0, -1.2, -0.45)

		-- B-TIER 1: WARBOT (hydraulic heavy endoskeleton, pneumatic warhammer)
		elseif uName == "Warbot" then
			buildFullEndo({ piston = true }, { piston = true })
			at("Body", P("WarPlate", Vector3.new(1.6, 0.5, 0.2), chromeDark, PlateM), 0, 0.75, -0.52)
			for _, side in ipairs({ -1, 1 }) do
				at("Body", P("HydraulicShoulder", Vector3.new(1.1, 0.6, 1.15), chromeSteel, PlateM), side * 1.45, 1.22, 0, 0, 0, side * -14)
				at("Body", P("ShoulderPiston", Vector3.new(0.8, 0.14, 0.14), hydraulic, MetalM, CYL), side * 0.85, 0.75, -0.4, 0, 0, side * 55)
			end
			at("RightArm", P("HammerShaft", Vector3.new(0.22, 0.22, 3.0), chromeDark), 0, -1.0, -1.3)
			at("RightArm", P("Weapon", Vector3.new(0.85, 1.3, 0.85), chromeSteel, PlateM), 0, -1.0, -2.85)
			at("RightArm", P("HammerVent", Vector3.new(0.9, 0.12, 0.9), redCore, NeonM), 0, -1.0, -2.85)
			at("RightArm", P("HammerRamTop", Vector3.new(0.55, 0.2, 0.55), chromeBright), 0, -0.25, -2.85)
			at("RightArm", P("HammerRamBottom", Vector3.new(0.55, 0.2, 0.55), chromeBright), 0, -1.75, -2.85)

		-- B-TIER 2: PLASMA ROBOT (phased plasma rifle, shoulder targeting sensor)
		elseif uName == "Plasma Robot" then
			buildFullEndo({ piston = true })
			-- Rifle runs along the arm so it points forward when the arm is raised to aim
			at("RightArm", P("Weapon", Vector3.new(0.32, 2.2, 0.52), chromeDark, PlateM), 0, -2.0, -0.1)
			at("RightArm", P("BarrelShroud", Vector3.new(1.1, 0.34, 0.34), chromeSteel, MetalM, CYL), 0, -3.6, -0.1, 0, 0, 90)
			for c = 1, 3 do
				at("RightArm", P("PlasmaCoil_" .. c, Vector3.new(0.09, 0.44, 0.44), redCore, NeonM, CYL), 0, -3.2 - (c - 1) * 0.32, -0.1, 0, 0, 90)
			end
			at("RightArm", P("MuzzleGlow", Vector3.new(0.08, 0.28, 0.28), redEye, NeonM, CYL), 0, -4.2, -0.1, 0, 0, 90)
			at("RightArm", P("PlasmaCell", Vector3.new(0.3, 0.55, 0.3), redCore, NeonM), 0, -1.7, 0.32)
			at("RightArm", P("RifleSight", Vector3.new(0.14, 0.5, 0.2), chromeSteel), 0, -2.2, -0.45)
			at("Body", P("TargetingSensor", Vector3.new(0.4, 0.32, 0.5), chromeDark), -1.25, 1.35, 0)
			at("Body", P("TargetingLens", Vector3.new(0.2, 0.12, 0.06), redEye, NeonM), -1.25, 1.35, -0.27)

		-- B-TIER 3: REPAIR BOT (hovering sensor drone, welder & repair arms, repair orb)
		elseif uName == "Repair Bot" or uName == "Shield Bot" then
			setupCore(false)
			leftArm.Transparency = 1
			rightArm.Transparency = 1
			body.Color = chromeDark
			buildSensorHead()
			at("Body", P("RepairChassis", Vector3.new(1.9, 1.4, 1.8), chromeDark, PlateM), 0, 0.15, 0)
			at("Body", P("ChassisDome", Vector3.new(1.4, 0.4, 1.4), chromeSteel), 0, 0.95, 0)
			at("Body", P("StatusLight", Vector3.new(1.2, 0.12, 0.06), repairGreen, NeonM), 0, 0.4, -0.92)
			at("Body", P("ThrusterHousing", Vector3.new(0.3, 1.5, 1.5), chromeDark, MetalM, CYL), 0, -0.5, 0, 0, 0, 90)
			at("Body", P("HoverThruster", Vector3.new(0.1, 1.3, 1.3), redCore, NeonM, CYL), 0, -0.68, 0, 0, 0, 90)
			for _, side in ipairs({ -1, 1 }) do
				at("Body", P("ArmMount", Vector3.new(0.5, 0.5, 0.5), chromeSteel, MetalM, BALL), side * 1.05, 0.35, -0.3)
				at("Body", P("RepairArmUpper", Vector3.new(0.22, 0.22, 1.0), chromeSteel), side * 1.15, 0.2, -0.85, -20, 0, 0)
				at("Body", P("RepairArmLower", Vector3.new(0.18, 0.18, 0.8), hydraulic), side * 1.15, -0.1, -1.45, 25, 0, 0)
				at("Body", P(side == 1 and "RepairEmitter" or "WelderTip", Vector3.new(0.2, 0.2, 0.3), side == 1 and repairGreen or redCore, NeonM), side * 1.15, -0.3, -1.85)
			end
			at("Body", P("Weapon", Vector3.new(0.6, 0.6, 0.6), repairGreen, NeonM, BALL), 0, -0.15, -2.0)
			at("Body", P("OrbRing", Vector3.new(0.08, 0.95, 0.95), chromeSteel, MetalM, CYL), 0, -0.15, -2.0, 0, 90, 0)

		-- A-TIER 1: DESTROYER MECH (T-1 style war machine, twin minigun, crusher fist, treads)
		elseif uName == "Destroyer Mech" then
			setupCore(false)
			head.Color = chromeDark
			at("Head", P("TurretCowl", Vector3.new(1.4, 0.75, 1.3), chromeDark, PlateM), 0, 0.12, 0.05)
			at("Head", P("SensorBar", Vector3.new(1.15, 0.2, 0.08), redEye, NeonM), 0, 0.05, -0.64)
			at("Head", P("SensorBrow", Vector3.new(1.3, 0.14, 0.25), chromeSteel), 0, 0.28, -0.6)
			at("Head", P("ChinGuard", Vector3.new(1.0, 0.3, 0.3), chromeSteel), 0, -0.4, -0.5)
			at("Head", P("Antenna", Vector3.new(0.08, 0.7, 0.08), chromeSteel), 0.45, 0.8, 0.3)
			at("Body", P("HeavyChassis", Vector3.new(2.5, 1.9, 1.5), chromeDark, PlateM), 0, 0.1, 0)
			at("Body", makeWedge("FrontGlacis", Vector3.new(2.3, 0.9, 0.6) * s, chromeSteel, MetalM, model), 0, 0.6, -1.05)
			for v = 1, 3 do
				at("Body", P("CoreVent_" .. v, Vector3.new(1.5, 0.1, 0.06), redCore, NeonM), 0, -0.05 - (v - 1) * 0.25, -0.78)
			end
			for _, side in ipairs({ -1, 1 }) do
				local aKey = side == -1 and "LeftArm" or "RightArm"
				local lKey = side == -1 and "LeftLeg" or "RightLeg"
				at("Body", P("MechShoulder", Vector3.new(1.2, 0.8, 1.3), chromeSteel, PlateM), side * 1.55, 1.05, 0, 0, 0, side * -10)
				at("Body", P("Exhaust", Vector3.new(0.9, 0.28, 0.28), chromeDark, MetalM, CYL), side * 0.6, 1.3, 0.75, 0, 0, 90)
				at("Body", P("ExhaustGlow", Vector3.new(0.06, 0.22, 0.22), redCore, NeonM, CYL), side * 0.6, 1.76, 0.75, 0, 0, 90)
				at(aKey, P("ArmArmor", Vector3.new(1.15, 1.0, 1.15), chromeDark, PlateM), 0, 0.4, 0)
				at(aKey, P("Elbow", Vector3.new(0.85, 0.85, 0.85), chromeSteel, MetalM, BALL), 0, -0.25, 0)
				at(lKey, P("TreadBand", Vector3.new(1.05, 2.0, 1.5), jointBlack, PlateM), 0, 0, -0.05)
				at(lKey, P("TreadWheelTop", Vector3.new(1.12, 0.7, 0.7), chromeSteel, MetalM, CYL), 0, 0.62, -0.42)
				at(lKey, P("TreadWheelBottom", Vector3.new(1.12, 0.7, 0.7), chromeSteel, MetalM, CYL), 0, -0.62, -0.42)
				at(lKey, P("LegSkirt", Vector3.new(0.14, 1.5, 1.3), chromeDark, PlateM), side * 0.6, 0.1, 0)
			end
			-- Left arm: twin minigun
			at("LeftArm", P("MinigunHousing", Vector3.new(0.8, 0.75, 1.0), chromeDark), 0, -0.95, -0.55)
			for _, gx in ipairs({ -0.22, 0.22 }) do
				at("LeftArm", P("MinigunBarrel", Vector3.new(1.8, 0.34, 0.34), chromeSteel, MetalM, CYL), gx, -0.95, -1.9, 0, 90, 0)
				at("LeftArm", P("MinigunMuzzle", Vector3.new(0.06, 0.3, 0.3), redEye, NeonM, CYL), gx, -0.95, -2.82, 0, 90, 0)
			end
			-- Right arm: hydraulic crusher fist
			at("RightArm", P("Weapon", Vector3.new(1.15, 1.0, 1.15), chromeSteel, PlateM), 0, -1.1, 0)
			for k = 1, 3 do
				at("RightArm", P("CrusherClaw_" .. k, Vector3.new(0.18, 0.55, 0.3), chromeBright), (k - 2) * 0.35, -1.85, -0.25, -15, 0, 0)
			end
			at("RightArm", P("FistPiston", Vector3.new(0.9, 0.14, 0.14), hydraulic, MetalM, CYL), 0, -0.4, 0.6, 0, 0, 90)

		-- A-TIER 2: HEAVY ARTILLERY BOT (shoulder rail cannon, ammo drum, stabilizer legs)
		elseif uName == "Heavy Artillery Bot" then
			setupCore(true)
			buildSkull(false)
			buildEndoTorso()
			buildEndoArm("LeftArm", { piston = true })
			buildEndoArm("RightArm", { piston = true })
			buildEndoLeg("LeftLeg", { piston = true, noFoot = true })
			buildEndoLeg("RightLeg", { piston = true, noFoot = true })
			for _, lKey in ipairs({ "LeftLeg", "RightLeg" }) do
				at(lKey, P("StabilizerFoot", Vector3.new(0.9, 0.24, 1.3), chromeDark, PlateM), 0, -0.88, -0.1)
			end
			at("Body", P("AmmoDrum", Vector3.new(1.4, 1.5, 0.8), chromeDark, PlateM), 0, 0.15, 0.85)
			for v = 1, 2 do
				at("Body", P("DrumVent_" .. v, Vector3.new(1.0, 0.08, 0.06), redCore, NeonM), 0, 0.45 - (v - 1) * 0.35, 1.26)
			end
			at("Body", P("AmmoFeed", Vector3.new(0.28, 0.14, 1.0), brassAmmo), 0.6, 1.25, 0.6, 30, 0, 0)
			at("Body", P("CannonMount", Vector3.new(0.7, 0.55, 0.9), chromeDark), 0.95, 1.3, 0.25)
			at("Body", P("Weapon", Vector3.new(0.55, 0.55, 3.4), chromeDark, PlateM), 0.95, 1.7, -1.0)
			at("Body", P("RailTop", Vector3.new(0.12, 0.1, 3.3), chromeBright), 0.95, 2.02, -1.1)
			at("Body", P("RailBottom", Vector3.new(0.12, 0.1, 3.3), chromeBright), 0.95, 1.38, -1.1)
			for c = 1, 3 do
				at("Body", P("ChargeCoil_" .. c, Vector3.new(0.12, 0.75, 0.75), redCore, NeonM, CYL), 0.95, 1.7, -0.5 - c * 0.7, 0, 90, 0)
			end
			at("Body", P("CannonMuzzle", Vector3.new(0.35, 0.65, 0.65), chromeSteel, MetalM, CYL), 0.95, 1.7, -2.85, 0, 90, 0)
			at("Body", P("MuzzleGlow", Vector3.new(0.06, 0.4, 0.4), redEye, NeonM, CYL), 0.95, 1.7, -3.05, 0, 90, 0)
			at("RightArm", P("TargetingGauntlet", Vector3.new(0.68, 0.7, 0.68), chromeDark, PlateM), 0, -0.6, 0)
			at("RightArm", P("TargetLens", Vector3.new(0.2, 0.08, 0.2), redEye, NeonM), 0, -1.3, 0)

		-- A-TIER 3: CYBERNETIC COMMANDER (half-skin infiltrator, trench coat, plasma minigun)
		elseif uName == "Cybernetic Commander" then
			setupCore(true)
			buildSkull(true)
			buildEndoTorso()
			buildEndoArm("LeftArm", { noUpper = true })
			buildEndoArm("RightArm", { noUpper = true })
			buildEndoLeg("LeftLeg", { noFemur = true, noFoot = true })
			buildEndoLeg("RightLeg", { noFemur = true, noFoot = true })
			-- Torn leather trench coat, open at the chest to show the chrome ribcage
			at("Body", P("CoatBack", Vector3.new(1.95, 1.95, 0.12), leatherBlack, LeatherM), 0, 0, 0.5)
			at("Body", P("CoatCollar", Vector3.new(1.8, 0.55, 0.15), leatherBlack, LeatherM), 0, 1.15, 0.5, -15, 0, 0)
			at("Body", P("CoatTail", Vector3.new(1.95, 1.8, 0.12), leatherBlack, LeatherM), 0, -1.85, 0.52, 6, 0, 0)
			for _, side in ipairs({ -1, 1 }) do
				local aKey = side == -1 and "LeftArm" or "RightArm"
				local lKey = side == -1 and "LeftLeg" or "RightLeg"
				at("Body", P("CoatFront", Vector3.new(0.55, 1.95, 0.12), leatherBlack, LeatherM), side * 0.7, 0, -0.46)
				at("Body", P("CoatSide", Vector3.new(0.12, 1.95, 0.95), leatherBlack, LeatherM), side * 0.98, 0, 0)
				at("Body", P("Lapel", Vector3.new(0.3, 0.75, 0.08), leatherBlack, LeatherM), side * 0.45, 0.6, -0.53, 0, 0, side * 18)
				at(aKey, P("CoatSleeve", Vector3.new(0.7, 1.05, 0.7), leatherBlack, LeatherM), 0, 0.4, 0)
				at(lKey, P("Pants", Vector3.new(0.66, 1.0, 0.66), denimDark, Enum.Material.Fabric), 0, 0.45, 0)
				at(lKey, P("CombatBoot", Vector3.new(0.7, 0.55, 1.0), jointBlack, LeatherM), 0, -0.73, -0.12)
			end
			at("Body", P("AmmoBox", Vector3.new(0.4, 0.55, 0.7), chromeDark, PlateM), 1.18, -0.8, 0.15)
			-- Back-mounted attack-speed aura emitter
			at("Body", P("AuraRing", Vector3.new(0.1, 1.25, 1.25), redCore, NeonM, CYL), 0, 0.3, 0.66, 0, 90, 0)
			at("Body", P("AuraHousing", Vector3.new(0.2, 0.95, 0.95), chromeDark, MetalM, CYL), 0, 0.3, 0.78, 0, 90, 0)
			-- Right arm: heavy plasma minigun held at the hip
			at("RightArm", P("Weapon", Vector3.new(0.6, 0.62, 1.3), chromeDark, PlateM), 0, -1.0, -0.75)
			at("RightArm", P("CarryHandle", Vector3.new(0.12, 0.3, 0.7), chromeSteel), 0, -0.55, -0.8)
			for b = 1, 4 do
				local ang = math.rad((b - 1) * 90 + 45)
				at("RightArm", P("MinigunBarrel_" .. b, Vector3.new(1.6, 0.13, 0.13), chromeSteel, MetalM, CYL),
					math.cos(ang) * 0.16, -1.0 + math.sin(ang) * 0.16, -2.1, 0, 90, 0)
			end
			at("RightArm", P("BarrelClamp", Vector3.new(0.1, 0.55, 0.55), chromeDark, MetalM, CYL), 0, -1.0, -2.4, 0, 90, 0)
			at("RightArm", P("MuzzleGlow", Vector3.new(0.05, 0.45, 0.45), redEye, NeonM, CYL), 0, -1.0, -2.92, 0, 90, 0)

		else
			-- Fallback for any future Mechanica unit
			buildFullEndo()
			at("RightArm", P("Weapon", Vector3.new(0.3, 0.5, 2.2), chromeDark), 0, -1.0, -1.1)
		end
	else
		-- Generic Minimal Fallback
		local wPart = makePart("Weapon", Vector3.new(0.24, 0.24, 2.85) * s, accentColor, Enum.Material.Neon, model)
		attach("RightArm", wPart, CFrame.new(0, -0.85 * s, -1.25 * s))
	end

	-- Ground Faction Ring & Overhead HP Bar (only for active combat units, not statues)
	local ring = nil
	local fill = nil
	if not isStatue and side ~= "Showcase" then
		local ringRadius = (unitDef.DamageReductionAura or unitDef.AttackSpeedAura) and (5.5 * s) or (2.9 * s)
		ring = makePart(
			"Ring",
			Vector3.new(0.16, ringRadius, ringRadius),
			side == "Player" and Color3.fromRGB(65, 185, 255) or Color3.fromRGB(255, 75, 110),
			Enum.Material.Neon,
			model,
			Enum.PartType.Cylinder
		)
		ring.Transparency = 0.45

		local bb = Instance.new("BillboardGui")
		bb.Name = "UnitHUD"
		bb.Size = UDim2.new(0, 84, 0, 20)
		bb.StudsOffset = Vector3.new(0, headSize.Y * 0.5 + 1.4 * s, 0)
		bb.AlwaysOnTop = false
		bb.MaxDistance = 110
		bb.Parent = head

		local bg = Instance.new("Frame")
		bg.Name = "Bg"
		bg.Size = UDim2.new(1, 0, 0.52, 0)
		bg.Position = UDim2.new(0, 0, 0.48, 0)
		bg.BackgroundColor3 = Color3.fromRGB(14, 16, 22)
		bg.BorderSizePixel = 0
		bg.Parent = bb

		fill = Instance.new("Frame")
		fill.Name = "Fill"
		fill.Size = UDim2.new(1, 0, 1, 0)
		fill.BackgroundColor3 = side == "Player" and Color3.fromRGB(70, 235, 120) or Color3.fromRGB(255, 80, 95)
		fill.BorderSizePixel = 0
		fill.Parent = bg

		local lbl = Instance.new("TextLabel")
		lbl.Size = UDim2.new(1, 0, 0.46, 0)
		lbl.BackgroundTransparency = 1
		lbl.Text = "[" .. tier .. "] " .. unitDef.Name
		lbl.TextColor3 = Color3.new(1, 1, 1)
		lbl.TextStrokeTransparency = 0.25
		lbl.Font = Enum.Font.GothamBold
		lbl.TextScaled = true
		lbl.Parent = bb
	end

	local weaponPart = model:FindFirstChild("Weapon") or rightArm
	model.PrimaryPart = body
	model.Parent = parentFolder

	local rigData = {
		Model = model,
		Body = body,
		Head = head,
		LeftLeg = leftLeg,
		RightLeg = rightLeg,
		LeftArm = leftArm,
		RightArm = rightArm,
		Weapon = weaponPart,
		Ring = ring,
		HPBarFill = fill,
		Attachments = attachments,
		Archetype = archetype,
		IsHover = isHover,
		IsQuadruped = isQuadruped,
		ScaleMult = s,
		TorsoSize = torsoSize,
		LimbSize = limbSize,
		HeadSize = headSize,
		GroundY = groundY,
		Position = rootPos,
		FacingCFrame = body.CFrame,
		AnimPhase = math.random() * 6.28,
		IsMoving = false,
		LastAttack = 0,
	}

	RigBuilder.PoseRig(rigData, 10.0, os.clock(), rootPos + Vector3.new(facingDir * 10, 0, 0))

	if not isStatue then
		local limbMap = {
			Body = body,
			Head = head,
			LeftLeg = leftLeg,
			RightLeg = rightLeg,
			LeftArm = leftArm,
			RightArm = rightArm,
		}
		for i = 1, #attachments do
			local att = attachments[i]
			local p = att.Part
			local parentLimb = limbMap[att.Limb]
			if p and parentLimb then
				p.Anchored = false
				p.Massless = true
				p.CanCollide = false
				p.CanQuery = false
				p.CanTouch = false
				local weld = Instance.new("WeldConstraint")
				weld.Name = "AttachmentWeld"
				weld.Part0 = parentLimb
				weld.Part1 = p
				weld.Parent = p
			end
		end
		rigData.WeldedAttachments = true
	end

	return rigData
end

local singleParts = {}
local singleCFrames = {}

function RigBuilder.PoseRig(u, dt, now, lookTarget, outParts, outCFrames)
	local timeSinceAttack = now - (u.LastAttack or 0)
	local isSwinging = (timeSinceAttack < 0.28)

	-- Skip redundant CFrame math if unit is stationary, not swinging, not hovering, and already posed
	if not u.IsMoving and not isSwinging and not u.IsHover and u.WasStationaryPosed then
		local flatCheck = Vector3.new(lookTarget.X, u.Position.Y, lookTarget.Z)
		if u.LastLookPos and (flatCheck - u.LastLookPos).Magnitude < 0.25 then
			return
		end
	end

	local flatLook = Vector3.new(lookTarget.X, u.Position.Y, lookTarget.Z)
	u.LastLookPos = flatLook
	if (flatLook - u.Position).Magnitude > 0.15 then
		local desiredRot = CFrame.lookAt(u.Position, flatLook)
		u.FacingCFrame = u.FacingCFrame.Rotation:Lerp(desiredRot.Rotation, math.clamp(dt * 12, 0, 1)) + u.Position
	else
		u.FacingCFrame = u.FacingCFrame.Rotation + u.Position
	end

	local strideAngle = 0
	local bobOffset = 0
	if u.IsMoving then
		u.WasStationaryPosed = false
		local moveSpeed = (u.Def and u.Def.Speed) or 12
		u.AnimPhase = u.AnimPhase + dt * moveSpeed * 0.92
		strideAngle = math.sin(u.AnimPhase) * 0.55
		bobOffset = u.IsHover and (math.sin(u.AnimPhase * 0.8) * 0.35) or (math.abs(math.cos(u.AnimPhase)) * 0.20)
	elseif u.IsHover then
		u.WasStationaryPosed = false
		u.AnimPhase = u.AnimPhase + dt * 3.5
		bobOffset = math.sin(u.AnimPhase) * 0.28
	else
		u.WasStationaryPosed = not isSwinging
	end

	local attackSwing = 0
	if isSwinging then
		attackSwing = math.sin((timeSinceAttack / 0.28) * math.pi) * 1.05
	end

	local s = u.ScaleMult or 1
	local forwardLean = (u.IsQuadruped and math.rad(-2)) or ((u.Archetype == "FeralBeast" and math.rad(-22)) or 0)
	local rootCF = u.FacingCFrame * CFrame.new(0, bobOffset, 0) * CFrame.Angles(forwardLean, 0, 0)
	local headCF = rootCF * CFrame.new(0, 1.58 * s, 0) * CFrame.Angles(-forwardLean * 0.6, 0, 0)

	local leftLegCF, rightLegCF, leftArmCF, rightArmCF
	if u.IsQuadruped then
		-- Quadruped Canine Stance & Natural Trotting Gait (Hell Hound)
		local headBob = u.IsMoving and (math.sin(u.AnimPhase) * math.rad(4.5)) or 0
		local hipSway = u.IsMoving and (math.sin(u.AnimPhase) * math.rad(3.5)) or 0
		local dogBob = u.IsMoving and (math.abs(math.sin(u.AnimPhase)) * 0.16 * s) or 0

		rootCF = u.FacingCFrame * CFrame.new(0, dogBob, 0) * CFrame.Angles(forwardLean, hipSway, 0)
		headCF = rootCF * CFrame.new(0, 0.55 * s, -1.45 * s) * CFrame.Angles(math.rad(6) + headBob, 0, 0)

		if u.IsMoving then
			local phase1 = u.AnimPhase
			local phase2 = u.AnimPhase + math.pi

			local pitchFL = math.sin(phase1) * 0.48
			local liftFL = math.max(0, math.sin(phase1)) * 0.25 * s

			local pitchRR = math.sin(phase1 + 0.12) * 0.45
			local liftRR = math.max(0, math.sin(phase1 + 0.12)) * 0.22 * s

			local pitchFR = math.sin(phase2) * 0.48
			local liftFR = math.max(0, math.sin(phase2)) * 0.25 * s

			local pitchRL = math.sin(phase2 + 0.12) * 0.45
			local liftRL = math.max(0, math.sin(phase2 + 0.12)) * 0.22 * s

			-- Front legs (LeftArm = FL, RightArm = FR)
			leftArmCF = rootCF
				* CFrame.new(-0.65 * s, -0.55 * s + liftFL, -0.95 * s)
				* CFrame.Angles(pitchFL, 0, 0)
				* CFrame.new(0, -0.55 * s, 0)
			rightArmCF = rootCF
				* CFrame.new(0.65 * s, -0.55 * s + liftFR, -0.95 * s)
				* CFrame.Angles(pitchFR, 0, 0)
				* CFrame.new(0, -0.55 * s, 0)

			-- Rear legs (LeftLeg = RL, RightLeg = RR)
			leftLegCF = rootCF
				* CFrame.new(-0.65 * s, -0.55 * s + liftRL, 0.95 * s)
				* CFrame.Angles(pitchRL, 0, 0)
				* CFrame.new(0, -0.55 * s, 0)
			rightLegCF = rootCF
				* CFrame.new(0.65 * s, -0.55 * s + liftRR, 0.95 * s)
				* CFrame.Angles(pitchRR, 0, 0)
				* CFrame.new(0, -0.55 * s, 0)
		else
			-- Stationary balanced canine stance
			leftLegCF = rootCF
				* CFrame.new(-0.65 * s, -0.55 * s, 0.95 * s)
				* CFrame.new(0, -0.55 * s, 0)
			rightLegCF = rootCF
				* CFrame.new(0.65 * s, -0.55 * s, 0.95 * s)
				* CFrame.new(0, -0.55 * s, 0)
			leftArmCF = rootCF
				* CFrame.new(-0.65 * s, -0.55 * s, -0.95 * s)
				* CFrame.new(0, -0.55 * s, 0)
			rightArmCF = rootCF
				* CFrame.new(0.65 * s, -0.55 * s, -0.95 * s)
				* CFrame.new(0, -0.55 * s, 0)
		end
	else
		leftLegCF = rootCF
			* CFrame.new(-0.52 * s, -1.0 * s, 0)
			* CFrame.Angles(strideAngle, 0, 0)
			* CFrame.new(0, -1.0 * s, 0)
		rightLegCF = rootCF
			* CFrame.new(0.52 * s, -1.0 * s, 0)
			* CFrame.Angles(-strideAngle, 0, 0)
			* CFrame.new(0, -1.0 * s, 0)

		local leftArmPitch = (u.Archetype == "ShieldTank" and math.rad(35))
			or (u.Archetype == "FeralBeast" and (strideAngle * 0.75 + attackSwing))
			or (-strideAngle * 0.75)

		local rightArmPitch = (u.Archetype == "RangedMarksman" or u.Archetype == "ArtillerySiege")
			and (math.rad(78) + attackSwing * 0.25)
			or (strideAngle * 0.75 + attackSwing)

		leftArmCF = rootCF
			* CFrame.new(-1.52 * s, 0.85 * s, 0)
			* CFrame.Angles(leftArmPitch, 0, math.rad(-4))
			* CFrame.new(0, -0.85 * s, 0)
		rightArmCF = rootCF
			* CFrame.new(1.52 * s, 0.85 * s, 0)
			* CFrame.Angles(rightArmPitch, 0, math.rad(4))
			* CFrame.new(0, -0.85 * s, 0)
	end

	local parts = outParts or singleParts
	local cframes = outCFrames or singleCFrames
	if not outParts then
		table.clear(parts)
		table.clear(cframes)
	end

	local n = #parts
	parts[n + 1] = u.Body;     cframes[n + 1] = rootCF
	parts[n + 2] = u.Head;     cframes[n + 2] = headCF
	parts[n + 3] = u.LeftLeg;  cframes[n + 3] = leftLegCF
	parts[n + 4] = u.RightLeg; cframes[n + 4] = rightLegCF
	parts[n + 5] = u.LeftArm;  cframes[n + 5] = leftArmCF
	parts[n + 6] = u.RightArm; cframes[n + 6] = rightArmCF
	n += 6

	if u.Ring then
		n += 1
		parts[n] = u.Ring
		local ringY = (u.GroundY and (u.GroundY - u.LimbSize.Y + 0.05)) or 0.78
		cframes[n] = CFrame.new(u.Position.X, ringY, u.Position.Z) * CFrame.Angles(0, 0, math.rad(90))
	end

	if not u.WeldedAttachments then
		local atts = u.Attachments
		for i = 1, #atts do
			local att = atts[i]
			local p = att.Part
			if p and p.Parent then
				local limb = att.Limb
				local parentCF = (limb == "Body" and rootCF)
					or (limb == "Head" and headCF)
					or (limb == "RightArm" and rightArmCF)
					or (limb == "LeftArm" and leftArmCF)
					or (limb == "LeftLeg" and leftLegCF)
					or (limb == "RightLeg" and rightLegCF)
				if parentCF then
					n += 1
					parts[n] = p
					cframes[n] = parentCF * att.Offset
				end
			end
		end
	end

	if not outParts then
		Workspace:BulkMoveTo(parts, cframes, Enum.BulkMoveMode.FireCFrameChanged)
	end
end

return RigBuilder
