local FactionConfig = {}

FactionConfig.Order = { "Kingdom", "Republic", "Alien", "Demon", "Void", "Mechanica" }

-- Verified clean Roblox built-in & combat SFX (NO laugh sound!)
FactionConfig.SFX = {
	Sword = "rbxassetid://12222216",      -- swordslash.wav
	Lunge = "rbxassetid://12222208",      -- swordlunge.wav
	Bow = "rbxassetid://12222200",        -- swoosh.wav
	Rifle = "rbxassetid://131070686",     -- Gun Shot (9mm)
	Laser = "rbxassetid://12222084",      -- Rocket shot.wav (Clean energy/plasma shot)
	Turret = "rbxassetid://12222084",     -- Rocket shot.wav
	Explosion = "rbxassetid://142070127", -- Explosion
	Spell = "rbxassetid://12222095",      -- Rocket whoosh 01.wav
	WaveHorn = "rbxassetid://12222253",   -- victory.wav
	Heal = "rbxassetid://139481162475902", -- Fast healing magic sound (magical heal chime)
	Click = "rbxassetid://12221967",      -- button.wav
}

-- Dynamic Pricing Rule:
-- Price stays flat until every 3 troops bought (0,1,2 = base; 3,4,5 = +1 step; 6,7,8 = +2 steps).
-- C Tier = Low increase (+2G per 3 troops), B Tier = Medium increase (+5G per 3 troops), A Tier = High increase (+12G per 3 troops).
FactionConfig.TierPriceStep = {
	C = 2,
	B = 5,
	A = 12,
}

function FactionConfig.GetPriceBucketSize(unitDef)
	if unitDef and unitDef.PriceBucketSize then
		return unitDef.PriceBucketSize
	end
	return 2
end

function FactionConfig.GetDynamicCost(unitDef, ownedCount)
	local bucketSize = FactionConfig.GetPriceBucketSize(unitDef)
	local steps = math.floor((ownedCount or 0) / bucketSize)
	local inc = FactionConfig.TierPriceStep[unitDef.Tier or "C"] or 2
	return unitDef.Cost + (steps * inc)
end

-- Default Loadout Indices per Faction (2 C-tier, 2 B-tier, 2 A-tier = 6 cards!)
FactionConfig.DefaultLoadout = { 1, 2, 4, 5, 7, 8 }

FactionConfig.Factions = {
	-- ============================================================
	-- 1. KINGDOM
	-- ============================================================
	Kingdom = {
		DisplayName = "Solaris Empire",
		Badge = "👑",
		Subtitle = "Knights, Mages & Holy Priests",
		MainColor = Color3.fromRGB(45, 95, 200),
		AccentColor = Color3.fromRGB(255, 215, 75),
		Units = {
			-- C TIER (Pick 2)
			{
				Id = 1, Tier = "C", Name = "Footman", Role = "15 DMG",
				Cost = 6, HP = 90, Damage = 15, Range = 5.5, AttackCooldown = 1.0, Speed = 14,
				Scale = Vector3.new(2.1, 3.5, 1.5), WeaponType = "Sword", SoundType = "Sword",
				Color = Color3.fromRGB(65, 120, 230), Accent = Color3.fromRGB(240, 200, 70),
			},
			{
				Id = 2, Tier = "C", Name = "Archer", Role = "12 DMG (Ranged)",
				Cost = 9, HP = 50, Damage = 12, Range = 24, AttackCooldown = 1.05, Speed = 13,
				Scale = Vector3.new(1.9, 3.4, 1.4), WeaponType = "Crossbow", SoundType = "Bow",
				Color = Color3.fromRGB(50, 95, 190), Accent = Color3.fromRGB(255, 230, 110),
			},
			{
				Id = 3, Tier = "C", Name = "Shield Man", Role = "8 DMG (Tank)",
				Cost = 10, HP = 130, Damage = 8, Range = 5.5, AttackCooldown = 1.1, Speed = 12,
				Scale = Vector3.new(2.4, 3.6, 1.8), WeaponType = "Shield", SoundType = "Sword",
				Color = Color3.fromRGB(40, 80, 175), Accent = Color3.fromRGB(220, 230, 250),
			},
			-- B TIER (Pick 2)
			{
				Id = 4, Tier = "B", Name = "Knight", Role = "22 DMG (-25% DEF)",
				Cost = 16, HP = 160, Damage = 22, Range = 6.0, DamageReduction = 0.25, AttackCooldown = 1.0, Speed = 14.5,
				Scale = Vector3.new(2.5, 3.9, 1.8), WeaponType = "Sword", SoundType = "Sword",
				Color = Color3.fromRGB(80, 135, 245), Accent = Color3.fromRGB(255, 215, 70),
			},
			{
				Id = 5, Tier = "B", Name = "Mage", Role = "30 DMG (4th Hit: 45)",
				Cost = 20, HP = 85, Damage = 30, FourthHitDamage = 45, Range = 25, AttackCooldown = 1.3, Speed = 12,
				Scale = Vector3.new(2.1, 3.7, 1.6), WeaponType = "Staff", SoundType = "Laser",
				Color = Color3.fromRGB(105, 65, 215), Accent = Color3.fromRGB(130, 210, 255),
			},
			{
				Id = 6, Tier = "B", Name = "Priest", Role = "20 HP Heal / 3s",
				Cost = 32, PriceBucketSize = 2, HP = 80, Damage = 8, Range = 28, AttackCooldown = 1.5, Speed = 12,
				HealAmount = 20, HealInterval = 3.0, HealMaxTargets = 5,
				Scale = Vector3.new(2.0, 3.7, 1.5), WeaponType = "Staff", SoundType = "Heal",
				Color = Color3.fromRGB(235, 235, 245), Accent = Color3.fromRGB(255, 230, 95),
			},
			-- A TIER (Pick 2)
			{
				Id = 7, Tier = "A", Name = "Elite Knight", Role = "35 DMG (-25% DEF • +10% Aura)",
				Cost = 35, HP = 280, Damage = 35, Range = 6.5, DamageReduction = 0.25, AttackSpeedAura = 0.10, AttackCooldown = 1.0, Speed = 13.5,
				Scale = Vector3.new(3.1, 4.5, 2.3), WeaponType = "Hammer", SoundType = "Lunge",
				Color = Color3.fromRGB(235, 190, 55), Accent = Color3.fromRGB(255, 245, 160),
			},
			{
				Id = 8, Tier = "A", Name = "Elite Mage", Role = "50 DMG (Arcane Burst)",
				Cost = 45, HP = 175, Damage = 50, Range = 28, AttackCooldown = 1.4, Speed = 11.5,
				Scale = Vector3.new(2.5, 4.2, 1.9), WeaponType = "Orb", SoundType = "Explosion",
				Color = Color3.fromRGB(125, 50, 225), Accent = Color3.fromRGB(255, 125, 45),
			},
			{
				Id = 9, Tier = "A", Name = "High Priest", Role = "30 HP Heal / 4s",
				Cost = 55, PriceBucketSize = 2, HP = 160, Damage = 12, Range = 30, AttackCooldown = 1.5, Speed = 11.5,
				HealAmount = 30, HealInterval = 4.0, HealMaxTargets = 99,
				Scale = Vector3.new(2.5, 4.2, 1.9), WeaponType = "Staff", SoundType = "Heal",
				Color = Color3.fromRGB(250, 245, 215), Accent = Color3.fromRGB(255, 225, 70),
			},
		},
		Spells = {
			Q = { Name = "Excalibur Sky-Strike", Desc = "Calls down a giant golden blade (115 AoE DMG)", Cooldown = 60 },
			E = { Name = "Royal Aegis", Desc = "Heals nearby allied units +75 HP & boosts speed", Cooldown = 60 },
		},
	},

	-- ============================================================
	-- 2. REPUBLIC
	-- ============================================================
	Republic = {
		DisplayName = "Aegis Coalition",
		Badge = "🦅",
		Subtitle = "Modern Tactical Strike Force",
		MainColor = Color3.fromRGB(55, 110, 88),
		AccentColor = Color3.fromRGB(80, 235, 255),
		Units = {
			-- C TIER (Pick 2)
			{
				Id = 1, Tier = "C", Name = "Riot Trooper", Role = "12 DMG",
				Cost = 7, HP = 120, Damage = 12, Range = 5.5, AttackCooldown = 1.0, Speed = 13.5,
				Scale = Vector3.new(2.2, 3.6, 1.6), WeaponType = "Sword", SoundType = "Sword",
				Color = Color3.fromRGB(65, 95, 110), Accent = Color3.fromRGB(90, 225, 255),
			},
			{
				Id = 2, Tier = "C", Name = "Police", Role = "12 DMG (Ranged)",
				Cost = 9, HP = 50, Damage = 12, Range = 23, AttackCooldown = 1.0, Speed = 13.5,
				Scale = Vector3.new(1.9, 3.5, 1.4), WeaponType = "Rifle", SoundType = "Rifle",
				Color = Color3.fromRGB(45, 75, 125), Accent = Color3.fromRGB(110, 210, 255),
			},
			{
				Id = 3, Tier = "C", Name = "Medic Bot", Role = "10 HP Heal / 3s",
				Cost = 12, PriceBucketSize = 2, HP = 60, Damage = 5, Range = 26, AttackCooldown = 1.4, Speed = 13,
				HealAmount = 10, HealInterval = 3.0, HealMaxTargets = 3,
				Scale = Vector3.new(1.8, 2.8, 1.8), WeaponType = "Orb", SoundType = "Heal",
				Color = Color3.fromRGB(210, 235, 230), Accent = Color3.fromRGB(75, 255, 165),
			},
			-- B TIER (Pick 2)
			{
				Id = 4, Tier = "B", Name = "Rifleman", Role = "20 DMG (3rd Attack: 3x10 Burst)",
				Cost = 18, HP = 85, Damage = 20, Range = 24, AttackCooldown = 0.95, Speed = 13,
				BurstInterval = 3, BurstHits = 3, BurstDamage = 10,
				Scale = Vector3.new(2.1, 3.6, 1.5), WeaponType = "Rifle", SoundType = "Rifle",
				Color = Color3.fromRGB(70, 120, 90), Accent = Color3.fromRGB(90, 235, 255),
			},
			{
				Id = 5, Tier = "B", Name = "Riot Shield", Role = "12 DMG (-25% DEF)",
				Cost = 20, HP = 200, Damage = 12, Range = 5.8, DamageReduction = 0.25, AttackCooldown = 1.05, Speed = 11.5,
				Scale = Vector3.new(2.7, 3.9, 2.0), WeaponType = "Shield", SoundType = "Sword",
				Color = Color3.fromRGB(50, 75, 95), Accent = Color3.fromRGB(70, 215, 255),
			},
			{
				Id = 6, Tier = "B", Name = "Grenade Thrower", Role = "35 DMG (1st Hit: 50)",
				Cost = 25, HP = 95, Damage = 35, FirstStrikeDamage = 50, Range = 25, AttackCooldown = 1.4, Speed = 12,
				Scale = Vector3.new(2.0, 3.5, 1.5), WeaponType = "Orb", SoundType = "Explosion",
				Color = Color3.fromRGB(85, 115, 75), Accent = Color3.fromRGB(255, 175, 55),
			},
			-- A TIER (Pick 2)
			{
				Id = 7, Tier = "A", Name = "Elite Riot", Role = "20 DMG (-15% DEF Aura)",
				Cost = 35, HP = 300, Damage = 20, Range = 6.2, DamageReduction = 0.15, DamageReductionAura = 0.15, AttackCooldown = 1.0, Speed = 12,
				Scale = Vector3.new(3.2, 4.5, 2.4), WeaponType = "Shield", SoundType = "Lunge",
				Color = Color3.fromRGB(40, 65, 85), Accent = Color3.fromRGB(0, 245, 255),
			},
			{
				Id = 8, Tier = "A", Name = "Shotgunner", Role = "35 Point-Blank / 15 Range",
				Cost = 40, HP = 160, Damage = 35, CloseRangeDamage = 35, LongRangeDamage = 15, CloseRangeThreshold = 8.5, Range = 17, AttackCooldown = 1.05, Speed = 13.5,
				Scale = Vector3.new(2.5, 4.0, 1.8), WeaponType = "Railgun", SoundType = "Rifle",
				Color = Color3.fromRGB(60, 95, 80), Accent = Color3.fromRGB(255, 190, 60),
			},
			{
				Id = 9, Tier = "A", Name = "Heavy Gunner", Role = "17 DMG (Rapid Gatling • Slow)",
				Cost = 45, HP = 220, Damage = 17, Range = 26, AttackCooldown = 0.40, Speed = 8.5,
				Scale = Vector3.new(2.8, 4.2, 2.1), WeaponType = "Railgun", SoundType = "Rifle",
				Color = Color3.fromRGB(48, 82, 70), Accent = Color3.fromRGB(255, 215, 80),
			},
		},
		Spells = {
			Q = { Name = "Orbital Airstrike", Desc = "Drops precision tactical missiles (115 AoE DMG)", Cooldown = 60 },
			E = { Name = "Tactical Overclock", Desc = "Nano-stim heals allies +75 HP & boosts speed", Cooldown = 60 },
		},
	},

	-- ============================================================
	-- 3. ALIEN
	-- ============================================================
	Alien = {
		DisplayName = "Hive Mind",
		Badge = "👽",
		Subtitle = "Bio-Swarm, Acid & Hive Overlords",
		MainColor = Color3.fromRGB(45, 130, 85),
		AccentColor = Color3.fromRGB(140, 255, 75),
		Units = {
			-- C TIER (Pick 2)
			{
				Id = 1, Tier = "C", Name = "Alien Grunt", Role = "18 DMG",
				Cost = 6, HP = 65, Damage = 18, Range = 5.5, AttackCooldown = 0.95, Speed = 15.5,
				Scale = Vector3.new(2.0, 3.2, 1.8), WeaponType = "Claws", SoundType = "Sword",
				Color = Color3.fromRGB(50, 145, 90), Accent = Color3.fromRGB(150, 255, 80),
			},
			{
				Id = 2, Tier = "C", Name = "Plasma Shooter", Role = "15 DMG (Ranged)",
				Cost = 10, HP = 45, Damage = 15, Range = 23, AttackCooldown = 1.0, Speed = 13.5,
				Scale = Vector3.new(1.9, 3.3, 1.5), WeaponType = "Plasma", SoundType = "Laser",
				Color = Color3.fromRGB(40, 120, 80), Accent = Color3.fromRGB(120, 255, 110),
			},
			{
				Id = 3, Tier = "C", Name = "Alien Guard", Role = "8 DMG (Carapace Tank)",
				Cost = 9, HP = 110, Damage = 8, Range = 5.5, AttackCooldown = 1.1, Speed = 12,
				Scale = Vector3.new(2.4, 3.6, 1.9), WeaponType = "Shield", SoundType = "Sword",
				Color = Color3.fromRGB(35, 100, 65), Accent = Color3.fromRGB(165, 255, 95),
			},
			-- B TIER (Pick 2)
			{
				Id = 4, Tier = "B", Name = "Mutated Warrior", Role = "25 DMG (Undying: +30 HP)",
				Cost = 15, HP = 140, Damage = 25, Range = 6.0, AttackCooldown = 1.0, Speed = 14,
				ReviveHP = 30,
				Scale = Vector3.new(2.5, 3.9, 1.9), WeaponType = "Claws", SoundType = "Lunge",
				Color = Color3.fromRGB(55, 150, 95), Accent = Color3.fromRGB(175, 255, 70),
			},
			{
				Id = 5, Tier = "B", Name = "Plasma Specialist", Role = "35 DMG (3rd Shot: 55 DMG)",
				Cost = 22, HP = 80, Damage = 35, Range = 25, AttackCooldown = 1.25, Speed = 12.5,
				ThirdHitDamage = 55,
				Scale = Vector3.new(2.2, 3.6, 1.7), WeaponType = "Plasma", SoundType = "Laser",
				Color = Color3.fromRGB(45, 125, 85), Accent = Color3.fromRGB(95, 255, 155),
			},
			{
				Id = 6, Tier = "B", Name = "Acid Spitter", Role = "20 DMG + 5 Poison/sec",
				Cost = 24, HP = 90, Damage = 20, PoisonDPS = 5, PoisonDuration = 4.0, Range = 23, AttackCooldown = 1.2, Speed = 12.5,
				Scale = Vector3.new(2.1, 3.6, 1.7), WeaponType = "Orb", SoundType = "Laser",
				Color = Color3.fromRGB(70, 175, 95), Accent = Color3.fromRGB(190, 255, 65),
			},
			-- A TIER (Pick 2)
			{
				Id = 7, Tier = "A", Name = "Alien Juggernaut", Role = "17 DMG · Regens 60 HP / 4.5s (Slow Colossus)",
				Cost = 35, HP = 370, Damage = 17, Range = 6.5, AttackCooldown = 1.05, Speed = 8.5,
				RegenHP = 60, RegenInterval = 4.5,
				Scale = Vector3.new(3.3, 4.6, 2.5), WeaponType = "Hammer", SoundType = "Lunge",
				Color = Color3.fromRGB(35, 95, 65), Accent = Color3.fromRGB(165, 255, 70),
			},
			{
				Id = 8, Tier = "A", Name = "Plasma Overlord", Role = "40 DMG · +40 Shield at ≤30% HP",
				Cost = 45, HP = 190, Damage = 40, Range = 27, AttackCooldown = 1.45, Speed = 11.5,
				EmergencyShieldAmount = 40, EmergencyShieldThreshold = 0.30,
				Scale = Vector3.new(2.7, 4.3, 2.3), WeaponType = "Plasma", SoundType = "Laser",
				Color = Color3.fromRGB(60, 155, 110), Accent = Color3.fromRGB(130, 255, 200),
			},
			{
				Id = 9, Tier = "A", Name = "Hive Commander", Role = "Summons 2 Grunts / 11s",
				Cost = 45, HP = 240, Damage = 28, Range = 24, AttackCooldown = 1.15, Speed = 12.0,
				SummonUnitId = 1, SummonCount = 2, SummonInterval = 11.0,
				Scale = Vector3.new(2.8, 4.4, 2.2), WeaponType = "Staff", SoundType = "Laser",
				Color = Color3.fromRGB(85, 195, 130), Accent = Color3.fromRGB(210, 255, 100),
			},
		},
		Spells = {
			Q = { Name = "Mothership Plasma Beam", Desc = "Sweeps an orbital bio-laser (115 AoE DMG)", Cooldown = 60 },
			E = { Name = "Hive Frenzy", Desc = "Heals nearby Aliens +75 HP & surges speed", Cooldown = 60 },
		},
	},

	-- ============================================================
	-- 4. DEMON
	-- ============================================================
	Demon = {
		DisplayName = "Hellfire Legion",
		Badge = "🔥",
		Subtitle = "Hellfire, Lifesteal & Archmages",
		MainColor = Color3.fromRGB(175, 32, 38),
		AccentColor = Color3.fromRGB(255, 145, 40),
		Units = {
			-- C TIER (Pick 2)
			{
				Id = 1, Tier = "C", Name = "Imp", Role = "20 DMG",
				Cost = 6, HP = 45, Damage = 20, Range = 5.5, AttackCooldown = 0.9, Speed = 16,
				Scale = Vector3.new(1.9, 3.1, 1.5), WeaponType = "Claws", SoundType = "Sword",
				Color = Color3.fromRGB(190, 40, 40), Accent = Color3.fromRGB(255, 160, 45),
			},
			{
				Id = 2, Tier = "C", Name = "Hell Hound", Role = "18 DMG (Fast Beast)",
				Cost = 8, HP = 70, Damage = 18, Range = 5.5, AttackCooldown = 0.9, Speed = 16.5,
				Scale = Vector3.new(2.2, 2.9, 2.1), WeaponType = "Claws", SoundType = "Lunge",
				Color = Color3.fromRGB(165, 32, 35), Accent = Color3.fromRGB(255, 140, 40),
			},
			{
				Id = 3, Tier = "C", Name = "Demon Soldier", Role = "15 DMG",
				Cost = 9, HP = 100, Damage = 15, Range = 5.8, AttackCooldown = 1.0, Speed = 13,
				Scale = Vector3.new(2.3, 3.6, 1.8), WeaponType = "Sword", SoundType = "Sword",
				Color = Color3.fromRGB(135, 25, 30), Accent = Color3.fromRGB(255, 120, 35),
			},
			-- B TIER (Pick 2)
			{
				Id = 4, Tier = "B", Name = "Blood Knight", Role = "20 DMG + 30 Lifesteal",
				Cost = 18, HP = 200, Damage = 20, HealOnHit = 30, Lifesteal = 30, Range = 6.0, AttackCooldown = 1.0, Speed = 14,
				Scale = Vector3.new(2.5, 3.9, 1.9), WeaponType = "Sword", SoundType = "Sword",
				Color = Color3.fromRGB(185, 30, 42), Accent = Color3.fromRGB(255, 95, 95),
			},
			{
				Id = 5, Tier = "B", Name = "Flame Caster", Role = "35 DMG + Burning",
				Cost = 22, HP = 80, Damage = 35, BurnDPS = 6, BurnDuration = 3.0, Range = 24, AttackCooldown = 1.3, Speed = 12,
				Scale = Vector3.new(2.1, 3.6, 1.6), WeaponType = "Staff", SoundType = "Laser",
				Color = Color3.fromRGB(205, 55, 35), Accent = Color3.fromRGB(255, 180, 50),
			},
			{
				Id = 6, Tier = "B", Name = "Soul Reaper", Role = "25 DMG + 10 Heal on Kill",
				Cost = 24, HP = 110, Damage = 25, HealOnKill = 10, Range = 6.2, AttackCooldown = 1.0, Speed = 14,
				Scale = Vector3.new(2.3, 3.8, 1.7), WeaponType = "Sword", SoundType = "Lunge",
				Color = Color3.fromRGB(125, 20, 45), Accent = Color3.fromRGB(255, 110, 150),
			},
			-- A TIER (Pick 2)
			{
				Id = 7, Tier = "A", Name = "Demon Brute", Role = "40 DMG",
				Cost = 38, HP = 350, Damage = 40, Range = 6.5, AttackCooldown = 1.1, Speed = 12,
				Scale = Vector3.new(3.4, 4.7, 2.5), WeaponType = "Hammer", SoundType = "Lunge",
				Color = Color3.fromRGB(125, 22, 28), Accent = Color3.fromRGB(255, 110, 30),
			},
			{
				Id = 8, Tier = "A", Name = "Infernal Archmage", Role = "60 DMG (Hellfire Bolt)",
				Cost = 48, HP = 195, Damage = 60, Range = 27, AttackCooldown = 1.45, Speed = 11.5,
				Scale = Vector3.new(2.7, 4.3, 2.2), WeaponType = "Orb", SoundType = "Explosion",
				Color = Color3.fromRGB(205, 45, 35), Accent = Color3.fromRGB(255, 200, 55),
			},
			{
				Id = 9, Tier = "A", Name = "Demon Lord", Role = "40 DMG + 50 Lifesteal",
				Cost = 50, HP = 280, Damage = 40, HealOnHit = 50, Lifesteal = 50, Range = 6.5, AttackCooldown = 1.0, Speed = 13,
				Scale = Vector3.new(3.1, 4.5, 2.3), WeaponType = "Sword", SoundType = "Lunge",
				Color = Color3.fromRGB(155, 20, 35), Accent = Color3.fromRGB(255, 165, 50),
			},
		},
		Spells = {
			Q = { Name = "Hellfire Cataclysm", Desc = "Drops a molten meteor pillar (115 AoE DMG)", Cooldown = 60 },
			E = { Name = "Bloodlust Pact", Desc = "Heals allies +75 HP & ignites furious speed", Cooldown = 60 },
		},
	},

	-- ============================================================
	-- 5. VOID
	-- ============================================================
	Void = {
		DisplayName = "Abyssal Void",
		Badge = "🌌",
		Subtitle = "Shadows, Soul Drain & Void Auras",
		MainColor = Color3.fromRGB(85, 38, 165),
		AccentColor = Color3.fromRGB(215, 95, 255),
		Units = {
			-- C TIER (Pick 2)
			{
				Id = 1, Tier = "C", Name = "Shadowling", Role = "18 DMG",
				Cost = 6, HP = 50, Damage = 18, Range = 5.5, AttackCooldown = 0.95, Speed = 15.5,
				Scale = Vector3.new(2.0, 3.4, 1.5), WeaponType = "Sword", SoundType = "Sword",
				Color = Color3.fromRGB(70, 30, 140), Accent = Color3.fromRGB(220, 100, 255),
			},
			{
				Id = 2, Tier = "C", Name = "Void Archer", Role = "14 DMG (Ranged)",
				Cost = 10, HP = 40, Damage = 14, Range = 24, AttackCooldown = 1.0, Speed = 13,
				Scale = Vector3.new(1.9, 3.4, 1.4), WeaponType = "Crossbow", SoundType = "Bow",
				Color = Color3.fromRGB(85, 35, 160), Accent = Color3.fromRGB(195, 115, 255),
			},
			{
				Id = 3, Tier = "C", Name = "Dark Sentinel", Role = "8 DMG (Shield)",
				Cost = 9, HP = 100, Damage = 8, Range = 5.5, AttackCooldown = 1.1, Speed = 12,
				Scale = Vector3.new(2.3, 3.6, 1.8), WeaponType = "Shield", SoundType = "Sword",
				Color = Color3.fromRGB(55, 22, 115), Accent = Color3.fromRGB(210, 130, 255),
			},
			-- B TIER (Pick 2)
			{
				Id = 4, Tier = "B", Name = "Void Stalker", Role = "30 DMG",
				Cost = 17, HP = 120, Damage = 30, Range = 6.0, AttackCooldown = 1.0, Speed = 14.5,
				Scale = Vector3.new(2.4, 3.9, 1.8), WeaponType = "Sword", SoundType = "Lunge",
				Color = Color3.fromRGB(95, 42, 175), Accent = Color3.fromRGB(235, 115, 255),
			},
			{
				Id = 5, Tier = "B", Name = "Abyss Mage", Role = "40 DMG + -25% Enemy ATK",
				Cost = 23, HP = 75, Damage = 40, AttackDebuff = 0.25, Range = 25, AttackCooldown = 1.35, Speed = 12,
				Scale = Vector3.new(2.1, 3.7, 1.6), WeaponType = "Staff", SoundType = "Laser",
				Color = Color3.fromRGB(110, 48, 195), Accent = Color3.fromRGB(205, 135, 255),
			},
			{
				Id = 6, Tier = "B", Name = "Soul Drainer", Role = "15 DMG + 1s Death Immune",
				Cost = 24, HP = 100, Damage = 15, DeathImmunity = 1.0, Range = 28, AttackCooldown = 0.5, Speed = 12.5,
				Scale = Vector3.new(2.1, 3.7, 1.6), WeaponType = "Orb", SoundType = "Laser",
				Color = Color3.fromRGB(135, 55, 215), Accent = Color3.fromRGB(245, 150, 255),
			},
			-- A TIER (Pick 2)
			{
				Id = 7, Tier = "A", Name = "Void Reaper", Role = "45 DMG",
				Cost = 38, HP = 260, Damage = 45, Range = 6.5, AttackCooldown = 1.0, Speed = 13.5,
				Scale = Vector3.new(3.0, 4.4, 2.2), WeaponType = "Sword", SoundType = "Lunge",
				Color = Color3.fromRGB(65, 25, 130), Accent = Color3.fromRGB(235, 115, 255),
			},
			{
				Id = 8, Tier = "A", Name = "Abyssal Sorcerer", Role = "65 DMG (Void Lance)",
				Cost = 48, HP = 190, Damage = 65, Range = 28, AttackCooldown = 1.5, Speed = 11.5,
				Scale = Vector3.new(2.7, 4.3, 2.3), WeaponType = "Orb", SoundType = "Explosion",
				Color = Color3.fromRGB(115, 45, 200), Accent = Color3.fromRGB(250, 140, 255),
			},
			{
				Id = 9, Tier = "A", Name = "Void Warden", Role = "35 DMG + 20% DEF Aura",
				Cost = 50, HP = 270, Damage = 35, DamageReductionAura = 0.20, AuraRadius = 24, Range = 6.5, AttackCooldown = 1.05, Speed = 12,
				Scale = Vector3.new(3.2, 4.5, 2.4), WeaponType = "Shield", SoundType = "Lunge",
				Color = Color3.fromRGB(50, 18, 105), Accent = Color3.fromRGB(255, 180, 255),
			},
		},
		Spells = {
			Q = { Name = "Event Horizon", Desc = "Implodes a singularity black hole (115 AoE DMG)", Cooldown = 60 },
			E = { Name = "Phase Veil", Desc = "Restores +75 HP to Void allies & surges speed", Cooldown = 60 },
		},
	},

	-- ============================================================
	-- 6. MECHANICA (Robot Faction)
	-- ============================================================
	Mechanica = {
		DisplayName = "Mechanica Corps",
		Badge = "🤖",
		Subtitle = "Autonomous Warbots, Mechs & Artillery",
		MainColor = Color3.fromRGB(70, 95, 125),
		AccentColor = Color3.fromRGB(255, 175, 45),
		Units = {
			-- C TIER (Pick 2)
			{
				Id = 1, Tier = "C", Name = "Combat Bot", Role = "15 DMG",
				Cost = 8, HP = 100, Damage = 15, Range = 5.5, AttackCooldown = 1.0, Speed = 13.5,
				Scale = Vector3.new(2.2, 3.6, 1.6), WeaponType = "Sword", SoundType = "Sword",
				Color = Color3.fromRGB(85, 110, 140), Accent = Color3.fromRGB(255, 185, 55),
			},
			{
				Id = 2, Tier = "C", Name = "Assault Drone", Role = "12 DMG (Ranged)",
				Cost = 10, HP = 50, Damage = 12, Range = 24, AttackCooldown = 0.95, Speed = 14.5,
				Scale = Vector3.new(1.9, 3.2, 1.5), WeaponType = "Rifle", SoundType = "Laser",
				Color = Color3.fromRGB(75, 100, 130), Accent = Color3.fromRGB(100, 235, 255),
			},
			{
				Id = 3, Tier = "C", Name = "Shield Robot", Role = "8 DMG (Armor Tank)",
				Cost = 12, HP = 150, Damage = 8, Range = 5.5, AttackCooldown = 1.1, Speed = 12,
				Scale = Vector3.new(2.5, 3.7, 1.9), WeaponType = "Shield", SoundType = "Sword",
				Color = Color3.fromRGB(60, 82, 110), Accent = Color3.fromRGB(255, 195, 70),
			},
			-- B TIER (Pick 2)
			{
				Id = 4, Tier = "B", Name = "Warbot", Role = "25 DMG",
				Cost = 18, HP = 200, Damage = 25, Range = 6.2, AttackCooldown = 1.0, Speed = 13,
				Scale = Vector3.new(2.7, 4.0, 2.0), WeaponType = "Hammer", SoundType = "Lunge",
				Color = Color3.fromRGB(90, 115, 145), Accent = Color3.fromRGB(255, 165, 45),
			},
			{
				Id = 5, Tier = "B", Name = "Plasma Robot", Role = "25 DMG (3rd Hit: 55)",
				Cost = 22, HP = 120, Damage = 25, ThirdHitDamage = 55, Range = 25, AttackCooldown = 1.25, Speed = 12.5,
				Scale = Vector3.new(2.3, 3.8, 1.7), WeaponType = "Railgun", SoundType = "Laser",
				Color = Color3.fromRGB(75, 105, 140), Accent = Color3.fromRGB(90, 240, 255),
			},
			{
				Id = 6, Tier = "B", Name = "Shield Bot", Role = "30 Shield Pulse (Non-Stacking)",
				Cost = 25, HP = 100, Damage = 8, Range = 28, AttackCooldown = 1.4, Speed = 12.5,
				ShieldAmount = 30, ShieldInterval = 5.0, ShieldRadius = 28,
				Scale = Vector3.new(2.1, 3.5, 1.7), WeaponType = "Orb", SoundType = "Heal",
				Color = Color3.fromRGB(105, 140, 165), Accent = Color3.fromRGB(80, 220, 255),
			},
			-- A TIER (Pick 2)
			{
				Id = 7, Tier = "A", Name = "Destroyer Mech", Role = "45 DMG (Heavy Mech)",
				Cost = 40, HP = 400, Damage = 45, Range = 6.8, AttackCooldown = 1.05, Speed = 11.5,
				Scale = Vector3.new(3.5, 4.8, 2.6), WeaponType = "Hammer", SoundType = "Lunge",
				Color = Color3.fromRGB(55, 75, 100), Accent = Color3.fromRGB(255, 150, 35),
			},
			{
				Id = 8, Tier = "A", Name = "Heavy Artillery Bot", Role = "65 DMG (42 RNG • 4.0s CD)",
				Cost = 50, HP = 250, Damage = 65, Range = 42, AttackCooldown = 4.0, Speed = 11,
				Scale = Vector3.new(3.0, 4.4, 2.4), WeaponType = "Railgun", SoundType = "Explosion",
				Color = Color3.fromRGB(80, 100, 125), Accent = Color3.fromRGB(255, 195, 60),
			},
			{
				Id = 9, Tier = "A", Name = "Cybernetic Commander", Role = "35 DMG + 20% ATK SPD Aura",
				Cost = 55, HP = 300, Damage = 35, AttackSpeedAura = 0.20, AuraRadius = 24, Range = 26, AttackCooldown = 1.0, Speed = 12.5,
				Scale = Vector3.new(3.1, 4.5, 2.3), WeaponType = "Railgun", SoundType = "Laser",
				Color = Color3.fromRGB(95, 125, 160), Accent = Color3.fromRGB(0, 245, 255),
			},
		},
		Spells = {
			Q = { Name = "Ion Cannon Barrage", Desc = "Fires an orbital satellite beam (115 AoE DMG)", Cooldown = 60 },
			E = { Name = "Nanobot Overdrive", Desc = "Repairs allied machines +75 HP & boosts speed", Cooldown = 60 },
		},
	},
}

return FactionConfig
