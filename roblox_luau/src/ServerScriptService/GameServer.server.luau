local Workspace = game:GetService("Workspace")
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local RunService = game:GetService("RunService")
local TweenService = game:GetService("TweenService")
local Debris = game:GetService("Debris")
local ProximityPromptService = game:GetService("ProximityPromptService")

local FactionConfig = require(ReplicatedStorage:WaitForChild("FactionConfig"))
local RigBuilder = require(ReplicatedStorage:WaitForChild("RigBuilder"))
local Remotes = ReplicatedStorage:WaitForChild("Remotes")

local BuyUnitEvent = Remotes:WaitForChild("BuyUnit")
local CastSpellEvent = Remotes:WaitForChild("CastSpell")
local UpgradeStatEvent = Remotes:WaitForChild("UpgradeStat")
local SwitchFactionEvent = Remotes:WaitForChild("SwitchFaction")
local StateUpdateEvent = Remotes:WaitForChild("StateUpdate")
local BannerNoticeEvent = Remotes:WaitForChild("BannerNotice")
local LobbyActionEvent = Remotes:WaitForChild("LobbyAction")
local ChallengeEvent = Remotes:WaitForChild("ChallengeEvent")
local MatchResultEvent = Remotes:WaitForChild("MatchResult")
local AdminCommandEvent = Remotes:WaitForChild("AdminCommand")

local DataStoreService = game:GetService("DataStoreService")
local WinsDataStore
local FactionDataStore
local GlobalWinsStore
pcall(function()
	WinsDataStore = DataStoreService:GetDataStore("PlayerWins_v1")
	FactionDataStore = DataStoreService:GetDataStore("PlayerFaction_v1")
	GlobalWinsStore = DataStoreService:GetOrderedDataStore("GlobalLeaderboard_Wins_v1")
end)

local PlayerWins = {}
local PlayerSelectedFaction = {}
local PlayerLoadouts = {}
local updateLobbyLeaderboard
local broadcastState

-- Persistent Leaderboard Registry: retains all commanders who have joined or won, even when they leave!
-- [UserId] = { UserId = id, Name = name, DisplayName = displayName, Faction = faction, Wins = wins, Online = bool }
local PersistentLeaderboard = {}

local function recordLeaderboardEntry(userId, name, displayName, faction, wins, isOnline)
	if not userId or userId <= 0 then return end
	local existing = PersistentLeaderboard[userId]
	if existing then
		existing.Name = name or existing.Name
		existing.DisplayName = displayName or existing.DisplayName
		if faction and faction ~= "" and FactionConfig.Factions[faction] then
			existing.Faction = faction
		end
		if wins and typeof(wins) == "number" then
			existing.Wins = math.max(existing.Wins or 0, wins)
		end
		if isOnline ~= nil then
			existing.Online = isOnline
		end
	else
		PersistentLeaderboard[userId] = {
			UserId = userId,
			Name = name or "Commander",
			DisplayName = displayName or name or "Commander",
			Faction = (faction and FactionConfig.Factions[faction]) and faction or "Kingdom",
			Wins = (typeof(wins) == "number") and wins or 0,
			Online = (isOnline == true),
		}
	end
end

-- Helper to safely save a player's selected faction to DataStore
local function savePlayerFaction(player, factionName)
	if not player or not player.UserId or player.UserId <= 0 then return end
	if not factionName or not FactionConfig.Factions[factionName] then return end
	PlayerSelectedFaction[player] = factionName
	if PersistentLeaderboard[player.UserId] then
		PersistentLeaderboard[player.UserId].Faction = factionName
	end
	task.spawn(function()
		pcall(function()
			if FactionDataStore then
				FactionDataStore:SetAsync("Faction_" .. tostring(player.UserId), factionName)
			end
		end)
	end)
end

local function setupPlayerStats(plr)
	local ls = plr:FindFirstChild("leaderstats")
	if not ls then
		ls = Instance.new("Folder")
		ls.Name = "leaderstats"
		ls.Parent = plr
	end

	local winsVal = ls:FindFirstChild("Wins")
	if not winsVal then
		winsVal = Instance.new("IntValue")
		winsVal.Name = "Wins"
		winsVal.Value = 0
		winsVal.Parent = ls
	end

	PlayerWins[plr] = 0

	-- Immediately register into PersistentLeaderboard as online
	recordLeaderboardEntry(
		plr.UserId,
		plr.Name,
		plr.DisplayName,
		PlayerSelectedFaction[plr] or "Kingdom",
		0,
		true
	)

	-- Asynchronously load saved wins from DataStore
	task.spawn(function()
		pcall(function()
			if WinsDataStore then
				local savedWins = WinsDataStore:GetAsync("Wins_" .. plr.UserId)
				if savedWins and typeof(savedWins) == "number" then
					PlayerWins[plr] = savedWins
					winsVal.Value = savedWins
					recordLeaderboardEntry(plr.UserId, plr.Name, plr.DisplayName, PlayerSelectedFaction[plr] or "Kingdom", savedWins, true)
					if updateLobbyLeaderboard then
						updateLobbyLeaderboard()
					end
				end
			end
		end)
	end)

	-- Asynchronously load saved faction from DataStore
	task.spawn(function()
		pcall(function()
			if FactionDataStore then
				local savedFaction = FactionDataStore:GetAsync("Faction_" .. tostring(plr.UserId))
				if savedFaction and FactionConfig.Factions[savedFaction] then
					PlayerSelectedFaction[plr] = savedFaction
					if PersistentLeaderboard[plr.UserId] then
						PersistentLeaderboard[plr.UserId].Faction = savedFaction
					end
					if broadcastState then
						broadcastState()
					end
				end
			end
		end)
	end)
end

local Battlefield = Workspace:WaitForChild("Battlefield")
local UnitsFolder = Workspace:WaitForChild("Units")
local EffectsFolder = Workspace:WaitForChild("Effects")
local OrbsFolder = Workspace:WaitForChild("Orbs")

local function sanitizeMapNoNeon()
	local Lighting = game:GetService("Lighting")
	local bloom = Lighting:FindFirstChildOfClass("BloomEffect") or Lighting:FindFirstChild("Bloom")
	if bloom then
		bloom.Enabled = false
		bloom.Intensity = 0
	end
	local atm = Lighting:FindFirstChildOfClass("Atmosphere")
	if atm then
		atm.Density = 0.12
		atm.Haze = 0
		atm.Glare = 0
	end
	Lighting.FogEnd = 100000
	Lighting.Brightness = 1.8

	for _, inst in ipairs(Workspace:GetDescendants()) do
		if inst:IsA("BasePart") then
			if inst.Material == Enum.Material.Neon then
				inst.Material = Enum.Material.SmoothPlastic
			end
			inst.Reflectance = 0
			inst.TopSurface = Enum.SurfaceType.Smooth
			inst.BottomSurface = Enum.SurfaceType.Smooth
			inst.LeftSurface = Enum.SurfaceType.Smooth
			inst.RightSurface = Enum.SurfaceType.Smooth
			inst.FrontSurface = Enum.SurfaceType.Smooth
			inst.BackSurface = Enum.SurfaceType.Smooth
		elseif inst:IsA("PointLight") or inst:IsA("SpotLight") or inst:IsA("SurfaceLight") then
			local room = Workspace:FindFirstChild("NPCShowcaseRoom")
			if not (room and inst:IsDescendantOf(room)) then
				inst.Enabled = false
				inst.Brightness = 0
			end
		end
	end

	local sp = Workspace:FindFirstChild("SpawnLocation")
	if sp and sp:IsA("BasePart") then
		sp.Material = Enum.Material.Slate
		sp.Color = Color3.fromRGB(38, 44, 58)
		sp.Transparency = 0
	end

	local lobby = Workspace:FindFirstChild("Lobby")
	if lobby then
		for _, name in ipairs({ "TrimNorth", "TrimSouth", "TrimEast", "TrimWest" }) do
			local p = lobby:FindFirstChild(name)
			if p and p:IsA("BasePart") then
				p.Material = Enum.Material.Slate
				p.Color = Color3.fromRGB(40, 46, 60)
			end
		end
		for _, name in ipairs({ "RunwayCenter", "RunwayCross", "SpawnRing", "CommandHoloTop" }) do
			local p = lobby:FindFirstChild(name)
			if p and p:IsA("BasePart") then
				p.Material = Enum.Material.Slate
				p.Color = Color3.fromRGB(38, 44, 58)
			end
		end
		for _, name in ipairs({ "GlassRailSouth", "GlassRailEast", "GlassRailWest" }) do
			local p = lobby:FindFirstChild(name)
			if p and p:IsA("BasePart") then
				p.Material = Enum.Material.Slate
				p.Color = Color3.fromRGB(36, 42, 54)
				p.Transparency = 0.25
			end
		end
		local padStyles = {
			BotStepPad = Color3.fromRGB(42, 95, 68),
			DuelPadWest = Color3.fromRGB(45, 78, 118),
			DuelPadEast = Color3.fromRGB(115, 48, 62),
		}
		for name, col in pairs(padStyles) do
			local p = lobby:FindFirstChild(name)
			if p and p:IsA("BasePart") then
				p.Material = Enum.Material.Slate
				p.Color = col
			end
		end
		local showcasePortal = lobby:FindFirstChild("LobbyToShowcasePortal")
		if showcasePortal then
			local pg = showcasePortal:FindFirstChild("PortalGlow")
			if pg and pg:IsA("BasePart") then
				pg.Material = Enum.Material.Slate
				pg.Color = Color3.fromRGB(40, 46, 60)
			end
			local spad = showcasePortal:FindFirstChild("ShowcasePad")
			if spad and spad:IsA("BasePart") then
				spad.Material = Enum.Material.Slate
				spad.Color = Color3.fromRGB(48, 68, 95)
			end
		end
		local lbMon = lobby:FindFirstChild("LeaderboardMonument")
		if lbMon then
			for _, name in ipairs({ "PillarNeonL", "PillarNeonR", "PedestalTrim", "TrophyOrb" }) do
				local p = lbMon:FindFirstChild(name)
				if p and p:IsA("BasePart") then
					p.Material = Enum.Material.Metal
					p.Color = Color3.fromRGB(135, 110, 52)
				end
			end
		end
		local shrines = lobby:FindFirstChild("Shrines")
		local shrineColors = {
			Shrine_Kingdom = Color3.fromRGB(55, 85, 140),
			Shrine_Republic = Color3.fromRGB(135, 115, 50),
			Shrine_Alien = Color3.fromRGB(48, 110, 65),
			Shrine_Demon = Color3.fromRGB(125, 45, 45),
			Shrine_Void = Color3.fromRGB(95, 55, 130),
			Shrine_Mechanica = Color3.fromRGB(50, 105, 130),
		}
		if shrines then
			for _, shrine in ipairs(shrines:GetChildren()) do
				local mutedCol = shrineColors[shrine.Name] or Color3.fromRGB(60, 70, 85)
				for _, partName in ipairs({ "PillarOrb", "PillarNeon", "PedestalTrim" }) do
					local p = shrine:FindFirstChild(partName)
					if p and p:IsA("BasePart") then
						p.Material = Enum.Material.Slate
						p.Color = mutedCol
					end
				end
				local fKey = string.gsub(shrine.Name, "Shrine_", "")
				local fData = FactionConfig.Factions[fKey]
				if fData then
					local plaque = shrine:FindFirstChild("PedestalPlaque")
					if plaque then
						local seenTag = false
						for _, ch in ipairs(plaque:GetChildren()) do
							if ch.Name == "PlaqueTag" then
								if not seenTag then
									seenTag = true
									local keptLbl = false
									for _, sub in ipairs(ch:GetChildren()) do
										if sub:IsA("TextLabel") then
											if not keptLbl then
												keptLbl = true
												sub.Text = (fData.Badge or "⚔️") .. " " .. string.upper(fData.DisplayName)
												sub.Position = UDim2.new(0, 0, 0.15, 0)
												sub.Size = UDim2.new(1, 0, 0.7, 0)
											else
												sub:Destroy()
											end
										end
									end
								else
									ch:Destroy()
								end
							end
						end
					end
					for _, d in ipairs(shrine:GetDescendants()) do
						if d:IsA("ProximityPrompt") then
							d.ObjectText = fData.DisplayName
						end
					end
				end
			end
		end
	end

	local lobbyGlassNorth = lobby and lobby:FindFirstChild("GlassRailNorth")
	if lobbyGlassNorth and lobbyGlassNorth:IsA("BasePart") then
		lobbyGlassNorth.Material = Enum.Material.Slate
		lobbyGlassNorth.Color = Color3.fromRGB(36, 42, 54)
		lobbyGlassNorth.Transparency = 0.25
	end
end

sanitizeMapNoNeon()

local MAX_BASE_HP = 3000
local MAX_TURRET_HP = 1500
local TURRET_DAMAGE = 40
local TURRET_COOLDOWN = 1.0
local TURRET_RANGE = 56
local WAVE_INTERVAL = 15 -- 15 seconds per wave

-- (PlayerSelectedFaction & PlayerLoadouts declared at top of module)

local function getDefaultLoadout()
	return { 1, 2, 4, 5, 7, 8 }
end

local function validateLoadout(factionName, proposed)
	local fData = FactionConfig.Factions[factionName]
	if not fData or type(proposed) ~= "table" or #proposed ~= 6 then
		return getDefaultLoadout()
	end
	local u1, u2 = fData.Units[proposed[1]], fData.Units[proposed[2]]
	local u3, u4 = fData.Units[proposed[3]], fData.Units[proposed[4]]
	local u5, u6 = fData.Units[proposed[5]], fData.Units[proposed[6]]
	if u1 and u2 and u3 and u4 and u5 and u6
		and u1.Tier == "C" and u2.Tier == "C" and proposed[1] ~= proposed[2]
		and u3.Tier == "B" and u4.Tier == "B" and proposed[3] ~= proposed[4]
		and u5.Tier == "A" and u6.Tier == "A" and proposed[5] ~= proposed[6] then
		return { proposed[1], proposed[2], proposed[3], proposed[4], proposed[5], proposed[6] }
	end
	return getDefaultLoadout()
end

-- TierUnlock: 1 = C-Tier only, 2 = B-Tier unlocked (costs 150G), 3 = A-Tier unlocked (costs 300G)
local FACTION_ARENA_DEFS = {
	[1] = { Faction = "Kingdom", ArenaName = "👑 SOLARIS EMPIRE ARENA", OriginX = 0, OriginZ = 0 },
	[2] = { Faction = "Republic", ArenaName = "🦅 AEGIS COALITION ARENA", OriginX = 285, OriginZ = -65 },
	[3] = { Faction = "Alien", ArenaName = "👽 HIVE MIND ARENA", OriginX = 285, OriginZ = -205 },
	[4] = { Faction = "Demon", ArenaName = "🔥 HELLFIRE LEGION ARENA", OriginX = 0, OriginZ = -270 },
	[5] = { Faction = "Void", ArenaName = "🌌 ABYSSAL VOID ARENA", OriginX = -285, OriginZ = -205 },
	[6] = { Faction = "Mechanica", ArenaName = "⚙️ MECHANICA CORPS ARENA", OriginX = -285, OriginZ = -65 },
}
local ARENA_Z_SPACING = 145
local INITIAL_ARENA_COUNT = 6
local MAX_ARENA_COUNT = 12
local BattlefieldTemplate = Battlefield:Clone()
local ArenaSlots = {}
local CurrentArena = nil
local CurrentArenaOriginX = 0
local CurrentArenaOriginZ = 0
local CurrentArenaUnitsFolder = UnitsFolder

local function makeEmptyMatch()
	return {
		Active = false,
		Mode = "Bot",
		BotDifficulty = "Normal",
		WestPlayer = nil,
		EastPlayer = nil,
		WaveNumber = 0,
		WaveTimer = WAVE_INTERVAL,
		GameOver = false,
		Sides = {
			Player = {
				Faction = "Kingdom",
				Loadout = { 1, 2, 4, 5, 7, 8 },
				BaseHP = MAX_BASE_HP,
				Gold = 45,
				IncomeLevel = 1,
				TierUnlock = 1,
				Lineup = { 1, 0, 0, 0, 0, 0 },
				SpellCooldowns = { Q = 0, E = 0 },
			},
			Enemy = {
				Faction = "Republic",
				Loadout = { 1, 2, 4, 5, 7, 8 },
				BaseHP = MAX_BASE_HP,
				Gold = 45,
				IncomeLevel = 1,
				TierUnlock = 1,
				Lineup = { 1, 0, 0, 0, 0, 0 },
				SpellCooldowns = { Q = 0, E = 0 },
			},
		},
	}
end

local Match = makeEmptyMatch()
local ActiveUnits = {}
local ActiveTurrets = {}
local PendingWaveSpawns = {}

local function setArenaContext(arena)
	if not arena then return end
	CurrentArena = arena
	CurrentArenaOriginX = arena.OriginX or 0
	CurrentArenaOriginZ = arena.OriginZ or 0
	Battlefield = arena.Battlefield
	CurrentArenaUnitsFolder = arena.UnitsFolder
	Match = arena.Match
	ActiveUnits = arena.ActiveUnits
	ActiveTurrets = arena.ActiveTurrets
	PendingWaveSpawns = arena.PendingWaveSpawns
end

local function getPlayerArena(player)
	if not player then return nil end
	for _, slot in ipairs(ArenaSlots) do
		if slot.Match.Active and (slot.Match.WestPlayer == player or slot.Match.EastPlayer == player) then
			return slot
		end
	end
	return nil
end

local lastSoundTime = {}
local function playSoundAt(soundType, pos, volume, pitch)
	local now = os.clock()
	if lastSoundTime[soundType] and (now - lastSoundTime[soundType] < 0.08) then
		return
	end
	lastSoundTime[soundType] = now

	local soundId = FactionConfig.SFX[soundType] or FactionConfig.SFX.Sword
	local part = Instance.new("Part")
	part.Size = Vector3.new(0.2, 0.2, 0.2)
	part.Position = pos or Vector3.new(0, 8, 0)
	part.Anchored = true
	part.CanCollide = false
	part.Transparency = 1
	part.Parent = EffectsFolder

	local snd = Instance.new("Sound")
	snd.SoundId = soundId
	snd.Volume = volume or 0.42
	snd.PlaybackSpeed = (pitch or 1.0) + (math.random(-6, 6) * 0.01)
	snd.RollOffMaxDistance = 95
	snd.RollOffMinDistance = 12
	snd.Parent = part
	snd:Play()
	Debris:AddItem(part, 2.0)
end

-- Base income: 20 Gold per wave (+20 Gold per upgrade). Upgrade price starts at 100 Gold and increases by +40% per upgrade!
local MAX_INCOME_LEVEL = 5

local function getSideWaveIncome(sideKey)
	local s = Match.Sides[sideKey]
	return 30 * math.min(s.IncomeLevel or 1, MAX_INCOME_LEVEL)
end

local function getSideIncomeUpgradeCost(sideKey)
	local s = Match.Sides[sideKey]
	if (s.IncomeLevel or 1) >= MAX_INCOME_LEVEL then
		return 0
	end
	return 100 + ((s.IncomeLevel or 1) - 1) * 20
end

-- Tier Unlock Cost: 1 -> 2 (Unlock B-Tier) = 150 Gold; 2 -> 3 (Unlock A-Tier) = 300 Gold; 3 = Max (0)
local function getSideTierUnlockCost(sideKey)
	local s = Match.Sides[sideKey]
	if s.TierUnlock == 1 then
		return 100
	elseif s.TierUnlock == 2 then
		return 250
	else
		return 0
	end
end

local function getPlayerSide(player)
	local arena = getPlayerArena(player)
	if arena and arena.Match.EastPlayer == player then
		return "Enemy"
	end
	if Match.Active and Match.EastPlayer == player then
		return "Enemy"
	end
	return "Player"
end

local function teleportPlayerTo(player, targetPos, lookAtPos)
	local char = player and player.Character
	local root = char and char:FindFirstChild("HumanoidRootPart")
	if root then
		root.CFrame = CFrame.lookAt(targetPos, Vector3.new(lookAtPos.X, targetPos.Y, lookAtPos.Z))
	end
end

local function aimTurretModel(t, lookPos)
	if not t.Head or not t.Head.Parent then return end
	local hPos = t.Head.Position
	local flatTarget = Vector3.new(lookPos.X, hPos.Y, lookPos.Z)
	if (flatTarget - hPos).Magnitude < 0.5 then return end
	local cf = CFrame.lookAt(hPos, flatTarget)
	t.Head.CFrame = t.Head.CFrame:Lerp(cf, 0.35)
	if t.BarrelL then t.BarrelL.CFrame = t.Head.CFrame * CFrame.new(-1.3, 0, -4.0) end
	if t.BarrelR then t.BarrelR.CFrame = t.Head.CFrame * CFrame.new(1.3, 0, -4.0) end
	if t.Muzzle then t.Muzzle.CFrame = t.Head.CFrame * CFrame.new(0, 0, -6.5) end
end

local function updateTurretVisual(t)
	if not t.Head or not t.Head.Parent then return end
	local hud = t.Head:FindFirstChild("BaseHUD")
	if hud then
		hud.AlwaysOnTop = false
		hud.MaxDistance = 72
		if hud:FindFirstChild("Title") then
			hud.Title.Text = ""
			hud.Title.Visible = false
		end
		local ratio = math.clamp(t.HP / t.MaxHP, 0, 1)
		hud.BarBg.BarFill.Size = UDim2.new(ratio, 0, 1, 0)
		hud.BarBg.HPText.Text = t.Dead and "DESTROYED" or string.format("%d / %d HP", math.max(0, math.floor(t.HP)), t.MaxHP)
	end
	for _, p in ipairs(t.Model:GetChildren()) do
		if p:IsA("BasePart") and p.Name ~= "Pedestal" then
			p.Transparency = t.Dead and 0.72 or 0
		end
	end
end

local TURRET_CONFIGS = {
	{ Name = "Turret 1", ModelName = "Turret1", HP = 1500, Damage = 50, Range = 36, Bounty = 100 },
	{ Name = "Turret 2", ModelName = "Turret2", HP = 2500, Damage = 100, Range = 28, Bounty = 200 },
}

local function setupTurretsForBase(side, factionName)
	local baseModel = Battlefield:FindFirstChild(side == "Player" and "PlayerBase" or "EnemyBase")
	if not baseModel then return end
	local fData = FactionConfig.Factions[factionName]
	if not fData then return end

	for _, tConfig in ipairs(TURRET_CONFIGS) do
		local tModel = baseModel:FindFirstChild(tConfig.ModelName)
		if tModel then
			for _, part in ipairs(tModel:GetChildren()) do
				if part:IsA("BasePart") then
					if part.Name == "NeonRing" or part.Name == "Muzzle" then
						part.Material = Enum.Material.Metal
						part.Color = Color3.new(fData.AccentColor.R * 0.55, fData.AccentColor.G * 0.55, fData.AccentColor.B * 0.55)
					elseif part.Name == "Column" or part.Name == "TurretHead" then
						part.Color = fData.MainColor
					end
				end
			end

			local head = tModel:FindFirstChild("TurretHead")
			local tObj = {
				Side = side,
				Name = tConfig.Name,
				ModelName = tConfig.ModelName,
				Model = tModel,
				Head = head,
				BarrelL = tModel:FindFirstChild("BarrelL"),
				BarrelR = tModel:FindFirstChild("BarrelR"),
				Muzzle = tModel:FindFirstChild("Muzzle"),
				Position = head and Vector3.new(head.Position.X, 2.5, head.Position.Z) or Vector3.new(0, 2.5, 0),
				HP = tConfig.HP,
				MaxHP = tConfig.HP,
				Damage = tConfig.Damage,
				Range = tConfig.Range or 32,
				Bounty = tConfig.Bounty or (tConfig.Name == "Turret 2" and 200 or 100),
				LastAttack = 0,
				Dead = false,
			}
			local hud = head and head:FindFirstChild("BaseHUD")
			if hud then
				hud.AlwaysOnTop = false
				hud.MaxDistance = 72
				if hud:FindFirstChild("Title") then
					hud.Title.Text = ""
					hud.Title.Visible = false
				end
			end
			updateTurretVisual(tObj)
			table.insert(ActiveTurrets, tObj)
		end
	end
end

local function updateBaseVisuals(side, factionName, hp)
	local baseModel = Battlefield:FindFirstChild(side == "Player" and "PlayerBase" or "EnemyBase")
	if not baseModel then return end
	local fData = FactionConfig.Factions[factionName]
	if not fData then return end

	for _, part in ipairs(baseModel:GetChildren()) do
		if part:IsA("BasePart") then
			if part.Name == "Crystal" then
				part.Material = Enum.Material.Slate
				part.Transparency = 0
				part.Color = Color3.new(fData.AccentColor.R * 0.55, fData.AccentColor.G * 0.55, fData.AccentColor.B * 0.55)
			elseif part.Name ~= "Platform" then
				part.Color = fData.MainColor
			end
		end
	end

	local core = baseModel:FindFirstChild("Core")
	if core and core:FindFirstChild("BaseHUD") then
		local hud = core.BaseHUD
		hud.AlwaysOnTop = false
		hud.MaxDistance = 78
		hud.Title.Text = (side == "Player" and "WEST: " or "EAST: ") .. string.upper(fData.DisplayName)
		hud.Title.TextColor3 = fData.AccentColor
		local ratio = math.clamp(hp / MAX_BASE_HP, 0, 1)
		hud.BarBg.BarFill.Size = UDim2.new(ratio, 0, 1, 0)
		hud.BarBg.BarFill.BackgroundColor3 = side == "Player" and Color3.fromRGB(65, 220, 120) or Color3.fromRGB(245, 75, 95)
		hud.BarBg.HPText.Text = string.format("%d / %d HP", math.max(0, math.floor(hp)), MAX_BASE_HP)
	end
end

local function getTurretSummary(side)
	local list = {}
	for _, t in ipairs(ActiveTurrets) do
		if t.Side == side then
			table.insert(list, {
				Name = t.Name,
				HP = math.max(0, math.floor(t.HP)),
				MaxHP = t.MaxHP,
				Dead = t.Dead,
				Bounty = t.Bounty or (t.Name == "Turret 2" and 200 or 100),
			})
		end
	end
	return list
end

local function makeInvisibleBarrierPart(parent, name, size, pos)
	local p = parent:FindFirstChild(name)
	if not p then
		p = Instance.new("Part")
		p.Name = name
		p.Parent = parent
	end
	p.Anchored = true
	p.CanCollide = true
	p.CanQuery = false
	p.CanTouch = false
	p.CastShadow = false
	p.Transparency = 1
	p.Material = Enum.Material.SmoothPlastic
	p.Size = size
	p.Position = pos
	return p
end

local function ensureLobbyBarriers()
	local lobby = Workspace:FindFirstChild("Lobby")
	if not lobby then return end
	local bFolder = lobby:FindFirstChild("LobbyBarriers")
	if not bFolder then
		bFolder = Instance.new("Folder")
		bFolder.Name = "LobbyBarriers"
		bFolder.Parent = lobby
	end
	makeInvisibleBarrierPart(bFolder, "BarrierNorth", Vector3.new(180, 48, 2), Vector3.new(0, 32, -199))
	makeInvisibleBarrierPart(bFolder, "BarrierSouth", Vector3.new(180, 48, 2), Vector3.new(0, 32, -71))
	makeInvisibleBarrierPart(bFolder, "BarrierEast", Vector3.new(2, 48, 130), Vector3.new(89, 32, -135))
	makeInvisibleBarrierPart(bFolder, "BarrierWest", Vector3.new(2, 48, 130), Vector3.new(-89, 32, -135))
	makeInvisibleBarrierPart(bFolder, "BarrierCeiling", Vector3.new(180, 2, 130), Vector3.new(0, 56, -135))
end

local function ensureArenaBarriers(bfModel, ox, oz)
	if not bfModel then return end
	local oldSign = bfModel:FindFirstChild("ArenaSignPart")
	if oldSign then
		oldSign:Destroy()
	end
	local bFolder = bfModel:FindFirstChild("ArenaBarriers")
	if not bFolder then
		bFolder = Instance.new("Folder")
		bFolder.Name = "ArenaBarriers"
		bFolder.Parent = bfModel
	end
	makeInvisibleBarrierPart(bFolder, "BarrierNorth", Vector3.new(272, 52, 2), Vector3.new(ox, 26, oz - 39))
	makeInvisibleBarrierPart(bFolder, "BarrierSouth", Vector3.new(272, 52, 2), Vector3.new(ox, 26, oz + 39))
	makeInvisibleBarrierPart(bFolder, "BarrierWest", Vector3.new(2, 52, 80), Vector3.new(ox - 135, 26, oz))
	makeInvisibleBarrierPart(bFolder, "BarrierEast", Vector3.new(2, 52, 80), Vector3.new(ox + 135, 26, oz))
	makeInvisibleBarrierPart(bFolder, "BarrierCeiling", Vector3.new(272, 2, 80), Vector3.new(ox, 52, oz))
end

ensureLobbyBarriers()

local function createArenaSlot(slotId)
	local def = FACTION_ARENA_DEFS[slotId]
	local originX = def and def.OriginX or 0
	local originZ = def and def.OriginZ or ((slotId - 6) * ARENA_Z_SPACING)
	local factionTheme = def and def.Faction or "Kingdom"
	local arenaName = def and def.ArenaName or ("⚔️ ARENA #" .. tostring(slotId))

	local bfModel
	if slotId == 1 then
		bfModel = Workspace:WaitForChild("Battlefield")
	else
		local existing = Workspace:FindFirstChild("Battlefield_" .. tostring(slotId))
		if existing then
			bfModel = existing
		else
			bfModel = BattlefieldTemplate:Clone()
			bfModel.Name = "Battlefield_" .. tostring(slotId)
			for _, inst in ipairs(bfModel:GetDescendants()) do
				if inst:IsA("BasePart") then
					inst.CFrame = inst.CFrame + Vector3.new(originX, 0, originZ)
				end
			end
			bfModel.Parent = Workspace
		end
	end

	local uFolder
	if slotId == 1 then
		uFolder = UnitsFolder
	else
		uFolder = Workspace:FindFirstChild("ArenaUnits_" .. tostring(slotId))
		if not uFolder then
			uFolder = Instance.new("Folder")
			uFolder.Name = "ArenaUnits_" .. tostring(slotId)
			uFolder.Parent = Workspace
		end
	end

	local slot = {
		SlotId = slotId,
		FactionTheme = factionTheme,
		ArenaName = arenaName,
		OriginX = originX,
		OriginZ = originZ,
		Battlefield = bfModel,
		UnitsFolder = uFolder,
		Match = makeEmptyMatch(),
		ActiveUnits = {},
		ActiveTurrets = {},
		PendingWaveSpawns = {},
		SecondAccumulator = 0,
	}
	ArenaSlots[slotId] = slot

	ensureArenaBarriers(bfModel, originX, originZ)
	setArenaContext(slot)
	updateBaseVisuals("Player", factionTheme, MAX_BASE_HP)
	updateBaseVisuals("Enemy", factionTheme, MAX_BASE_HP)
	setupTurretsForBase("Player", factionTheme)
	setupTurretsForBase("Enemy", factionTheme)
	return slot
end

local function findAvailableArenaSlot(westPlayer, eastPlayer, preferredFaction)
	for _, slot in ipairs(ArenaSlots) do
		if slot.Match.Active and not slot.Match.GameOver then
			if (westPlayer and (slot.Match.WestPlayer == westPlayer or slot.Match.EastPlayer == westPlayer))
				or (eastPlayer and (slot.Match.WestPlayer == eastPlayer or slot.Match.EastPlayer == eastPlayer)) then
				return slot
			end
		end
	end
	local p1Theme = preferredFaction or (westPlayer and PlayerSelectedFaction[westPlayer]) or "Kingdom"
	for _, slot in ipairs(ArenaSlots) do
		if (not slot.Match.Active or slot.Match.GameOver) and slot.FactionTheme == p1Theme then
			return slot
		end
	end
	local p2Theme = eastPlayer and PlayerSelectedFaction[eastPlayer]
	if p2Theme then
		for _, slot in ipairs(ArenaSlots) do
			if (not slot.Match.Active or slot.Match.GameOver) and slot.FactionTheme == p2Theme then
				return slot
			end
		end
	end
	for _, slot in ipairs(ArenaSlots) do
		if not slot.Match.Active or slot.Match.GameOver then
			return slot
		end
	end
	if #ArenaSlots < MAX_ARENA_COUNT then
		return createArenaSlot(#ArenaSlots + 1)
	end
	return nil
end

for slotIdx = 1, INITIAL_ARENA_COUNT do
	createArenaSlot(slotIdx)
end
setArenaContext(ArenaSlots[1])

local PvPQueuePlayer = nil
local LastPadTrigger = {}

local function getLobbyPlayersList()
	local list = {}
	for _, p in ipairs(Players:GetPlayers()) do
		local pArena = getPlayerArena(p)
		table.insert(list, {
			UserId = p.UserId,
			Name = p.Name,
			DisplayName = p.DisplayName,
			Faction = PlayerSelectedFaction[p] or "Kingdom",
			InMatch = (pArena ~= nil),
			ArenaSlotId = pArena and pArena.SlotId or nil,
			ArenaName = pArena and pArena.ArenaName or nil,
			QueuedPvP = (PvPQueuePlayer == p),
			Wins = PlayerWins[p] or 0,
		})
	end
	return list
end

function updateLobbyLeaderboard(lobbyPlayers)
	local lobbyFolder = Workspace:FindFirstChild("Lobby")
	local lbMon = lobbyFolder and lobbyFolder:FindFirstChild("LeaderboardMonument")
	local disp = lbMon and lbMon:FindFirstChild("DisplayScreen")
	local sg = disp and disp:FindFirstChild("LeaderboardGui")
	local mf = sg and sg:FindFirstChild("MainFrame")
	local listHolder = mf and mf:FindFirstChild("ListHolder")
	if not listHolder then return end

	-- Ensure any currently connected players are registered/synced in PersistentLeaderboard
	for _, p in ipairs(Players:GetPlayers()) do
		recordLeaderboardEntry(
			p.UserId,
			p.Name,
			p.DisplayName,
			PlayerSelectedFaction[p] or "Kingdom",
			PlayerWins[p] or 0,
			true
		)
	end

	for _, ch in ipairs(listHolder:GetChildren()) do
		if ch:IsA("Frame") then
			ch:Destroy()
		end
	end

	-- Collect all commanders from PersistentLeaderboard (persists even when players leave!)
	local sorted = {}
	for _, entry in pairs(PersistentLeaderboard) do
		table.insert(sorted, entry)
	end

	-- Sort by Wins descending; secondary sort alphabetical
	table.sort(sorted, function(a, b)
		local wA = a.Wins or 0
		local wB = b.Wins or 0
		if wA ~= wB then
			return wA > wB
		end
		return (a.DisplayName or "") < (b.DisplayName or "")
	end)

	local rankIcons = { "🥇", "🥈", "🥉" }
	local rankColors = {
		Color3.fromRGB(255, 215, 80),
		Color3.fromRGB(215, 225, 240),
		Color3.fromRGB(210, 140, 75),
	}

	for i = 1, math.min(#sorted, 12) do
		local p = sorted[i]
		local isTop3 = (i <= 3)
		local row = Instance.new("Frame")
		row.Name = "Row_" .. tostring(i)
		row.Size = UDim2.new(1, 0, 0, 32)
		row.BackgroundColor3 = isTop3 and Color3.fromRGB(24, 30, 48) or Color3.fromRGB(16, 20, 32)
		row.BorderSizePixel = 0
		row.LayoutOrder = i
		row.Parent = listHolder
		local cr = Instance.new("UICorner")
		cr.CornerRadius = UDim.new(0, 6)
		cr.Parent = row
		if i == 1 then
			local st = Instance.new("UIStroke")
			st.Color = Color3.fromRGB(255, 215, 80)
			st.Thickness = 1.5
			st.Parent = row
		end

		local rankLbl = Instance.new("TextLabel")
		rankLbl.Size = UDim2.new(0.16, 0, 1, 0)
		rankLbl.BackgroundTransparency = 1
		rankLbl.Text = (rankIcons[i] or ("#" .. tostring(i)))
		rankLbl.TextColor3 = rankColors[i] or Color3.fromRGB(180, 195, 220)
		rankLbl.Font = Enum.Font.GothamBlack
		rankLbl.TextScaled = true
		rankLbl.Parent = row

		local nameLbl = Instance.new("TextLabel")
		nameLbl.Size = UDim2.new(0.58, -6, 1, 0)
		nameLbl.Position = UDim2.new(0.16, 4, 0, 0)
		nameLbl.BackgroundTransparency = 1
		local onlineBadge = p.Online and "🟢 " or ""
		nameLbl.Text = string.format("%s%s", onlineBadge, p.DisplayName or p.Name or "Commander")
		nameLbl.TextColor3 = (i == 1) and Color3.fromRGB(255, 230, 120) or (p.Online and Color3.new(1, 1, 1) or Color3.fromRGB(185, 195, 210))
		nameLbl.Font = Enum.Font.GothamBold
		nameLbl.TextXAlignment = Enum.TextXAlignment.Left
		nameLbl.TextScaled = true
		nameLbl.Parent = row

		local winsLbl = Instance.new("TextLabel")
		winsLbl.Size = UDim2.new(0.24, -6, 1, 0)
		winsLbl.Position = UDim2.new(0.75, 0, 0, 0)
		winsLbl.BackgroundTransparency = 1
		winsLbl.Text = tostring(p.Wins or 0) .. " WINS"
		winsLbl.TextColor3 = (p.Wins and p.Wins > 0) and Color3.fromRGB(255, 215, 80) or Color3.fromRGB(150, 170, 195)
		winsLbl.Font = Enum.Font.GothamBlack
		winsLbl.TextScaled = true
		winsLbl.Parent = row
	end

	if #sorted == 0 then
		local emptyRow = Instance.new("Frame")
		emptyRow.Size = UDim2.new(1, 0, 0, 36)
		emptyRow.BackgroundColor3 = Color3.fromRGB(18, 22, 34)
		emptyRow.Parent = listHolder
		local cr = Instance.new("UICorner")
		cr.CornerRadius = UDim.new(0, 6)
		cr.Parent = emptyRow
		local emptyLbl = Instance.new("TextLabel")
		emptyLbl.Size = UDim2.new(1, -12, 1, 0)
		emptyLbl.Position = UDim2.new(0, 6, 0, 0)
		emptyLbl.BackgroundTransparency = 1
		emptyLbl.Text = "Play 1v1 Matches to earn Wins and rank up!"
		emptyLbl.TextColor3 = Color3.fromRGB(160, 180, 210)
		emptyLbl.Font = Enum.Font.GothamMedium
		emptyLbl.TextScaled = true
		emptyLbl.Parent = emptyRow
	end
end

function broadcastState()
	local prevArena = CurrentArena
	local lobbyPlayers = getLobbyPlayersList()
	updateLobbyLeaderboard(lobbyPlayers)
	for _, plr in ipairs(Players:GetPlayers()) do
		local pArena = getPlayerArena(plr)
		local isParticipant = (pArena ~= nil)
		local activeArena = pArena or ArenaSlots[1]
		setArenaContext(activeArena)
		local mySideKey = (isParticipant and Match.EastPlayer == plr) and "Enemy" or "Player"
		local oppSideKey = (mySideKey == "Player") and "Enemy" or "Player"
		local mySide = Match.Sides[mySideKey]
		local oppSide = Match.Sides[oppSideKey]

		StateUpdateEvent:FireClient(plr, {
			InMatch = isParticipant,
			ArenaActive = isParticipant and Match.Active and not Match.GameOver,
			ArenaSlotId = activeArena.SlotId,
			ArenaOriginX = activeArena.OriginX or 0,
			ArenaOriginZ = activeArena.OriginZ or 0,
			ArenaTheme = activeArena.FactionTheme or "Kingdom",
			ArenaName = activeArena.ArenaName or ("ARENA #" .. tostring(activeArena.SlotId)),
			MatchMode = Match.Mode,
			BotDifficulty = Match.BotDifficulty,
			OpponentName = (Match.Mode == "PvP" and Match.EastPlayer) and (mySideKey == "Player" and Match.EastPlayer.DisplayName or (Match.WestPlayer and Match.WestPlayer.DisplayName or "Player")) or ("BOT (" .. Match.BotDifficulty .. ")"),
			SelectedLobbyFaction = PlayerSelectedFaction[plr] or "Kingdom",
			LobbyLoadout = PlayerLoadouts[plr] or getDefaultLoadout(),
			LobbyPlayers = lobbyPlayers,
			PlayerFaction = mySide.Faction,
			EnemyFaction = oppSide.Faction,
			PlayerLoadout = mySide.Loadout,
			PlayerBaseHP = mySide.BaseHP,
			EnemyBaseHP = oppSide.BaseHP,
			MaxBaseHP = MAX_BASE_HP,
			PlayerTurrets = getTurretSummary(mySideKey),
			EnemyTurrets = getTurretSummary(oppSideKey),
			Gold = math.floor(mySide.Gold),
			WaveIncome = getSideWaveIncome(mySideKey),
			IncomeLevel = mySide.IncomeLevel,
			IncomeCost = getSideIncomeUpgradeCost(mySideKey),
			TierUnlock = mySide.TierUnlock,
			TierUnlockCost = getSideTierUnlockCost(mySideKey),
			WaveNumber = Match.WaveNumber,
			WaveTimer = math.ceil(Match.WaveTimer),
			WaveInterval = WAVE_INTERVAL,
			PlayerLineup = mySide.Lineup,
			SpellCooldowns = {
				Q = math.max(0, math.ceil(mySide.SpellCooldowns.Q - os.clock())),
				E = math.max(0, math.ceil(mySide.SpellCooldowns.E - os.clock())),
			},
		})
	end
	if prevArena then
		setArenaContext(prevArena)
	end
end

local FrameBulkParts = {}
local FrameBulkCFrames = {}

local function createVisualEffect(pos, radius, color, duration)
	if #EffectsFolder:GetChildren() > 26 then return end
	local p = Instance.new("Part")
	p.Shape = Enum.PartType.Ball
	p.Anchored = true
	p.CanCollide = false
	p.CanQuery = false
	p.CanTouch = false
	p.Material = Enum.Material.Neon
	p.Color = color
	p.Size = Vector3.new(2, 2, 2)
	p.Position = pos
	p.Transparency = 0.25
	p.TopSurface = Enum.SurfaceType.Smooth
	p.BottomSurface = Enum.SurfaceType.Smooth
	p.Parent = EffectsFolder

	TweenService:Create(p, TweenInfo.new(duration or 0.26, Enum.EasingStyle.Quad, Enum.EasingDirection.Out), {
		Size = Vector3.new(radius * 2, radius * 2, radius * 2),
		Transparency = 1,
	}):Play()
	Debris:AddItem(p, (duration or 0.26) + 0.04)
end

local function createBeamEffect(fromPos, toPos, color, thickness)
	if #EffectsFolder:GetChildren() > 26 then return end
	local dist = (toPos - fromPos).Magnitude
	if dist < 0.5 then return end
	local beam = Instance.new("Part")
	beam.Anchored = true
	beam.CanCollide = false
	beam.CanQuery = false
	beam.CanTouch = false
	beam.Material = Enum.Material.Neon
	beam.Color = color
	beam.Size = Vector3.new(thickness or 0.42, thickness or 0.42, dist)
	beam.CFrame = CFrame.lookAt((fromPos + toPos) * 0.5, toPos)
	beam.Transparency = 0.15
	beam.TopSurface = Enum.SurfaceType.Smooth
	beam.BottomSurface = Enum.SurfaceType.Smooth
	beam.Parent = EffectsFolder
	Debris:AddItem(beam, 0.12)
end

local function buildUnitModel(side, factionName, unitDef, spawnPos, stackMult)
	local mult = stackMult or 1.0
	local rig = RigBuilder.BuildUnitRig(side, factionName, unitDef, spawnPos, CurrentArenaUnitsFolder or UnitsFolder, false)
	local now = os.clock()

	if mult > 1.05 and rig.Head then
		local hud = rig.Head:FindFirstChild("UnitHUD")
		local lbl = hud and hud:FindFirstChildOfClass("TextLabel")
		if lbl then
			lbl.Text = string.format("[%s ★x%.0f] %s", unitDef.Tier, mult, unitDef.Name)
		end
	end

	local scaledHP = math.floor(unitDef.HP * mult + 0.5)
	local scaledDmg = unitDef.Damage * mult

	local unitObj = {
		Side = side,
		Faction = factionName,
		Def = unitDef,
		StackMult = mult,
		Model = rig.Model,
		Body = rig.Body,
		Head = rig.Head,
		LeftLeg = rig.LeftLeg,
		RightLeg = rig.RightLeg,
		LeftArm = rig.LeftArm,
		RightArm = rig.RightArm,
		Weapon = rig.Weapon,
		Ring = rig.Ring,
		HPBarFill = rig.HPBarFill,
		Attachments = rig.Attachments,
		Archetype = rig.Archetype,
		IsHover = rig.IsHover,
		WeldedAttachments = rig.WeldedAttachments,
		IsQuadruped = rig.IsQuadruped,
		ScaleMult = rig.ScaleMult,
		TorsoSize = rig.TorsoSize,
		LimbSize = rig.LimbSize,
		HeadSize = rig.HeadSize,
		HP = scaledHP,
		MaxHP = scaledHP,
		Damage = scaledDmg,
		Position = rig.Position,
		FacingCFrame = rig.FacingCFrame,
		GroundY = rig.GroundY,
		AnimPhase = rig.AnimPhase,
		IsMoving = true,
		LastAttack = 0,
		LastHeal = now,
		LastSummon = now,
		LastDoTTick = now,
		NextScanTime = now + math.random() * 0.12,
		CachedTarget = nil,
		CachedTurret = nil,
		CachedSepVec = Vector3.zero,
		CachedDmgReduction = unitDef.DamageReduction or (unitDef.DamageReductionAura or ((rig.Archetype == "ShieldTank") and 0.18 or 0)),
		CachedAtkSpeedMult = 1.0 + (unitDef.AttackSpeedAura or 0),
		ShotsFired = 0,
		AttacksMade = 0,
		HitCount = 0,
		ShieldHP = 0,
		MaxShieldHP = 0,
		SkipNextShield = false,
		PoisonUntil = 0,
		PoisonDPS = 0,
		BurnUntil = 0,
		BurnDPS = 0,
		SlowUntil = 0,
		SlowFactor = 1.0,
		AttackDebuffFactor = 0,
		DeathImmuneTriggered = false,
		DeathImmuneUntil = 0,
		ReviveTriggered = false,
		EmergencyShieldTriggered = false,
		LastRegen = now,
		SpeedBoostUntil = 0,
		Dead = false,
	}
	table.insert(ActiveUnits, unitObj)
	return unitObj
end

local function updateUnitPose(u, dt, now, lookTarget, outParts, outCFrames)
	RigBuilder.PoseRig(u, dt, now, lookTarget, outParts, outCFrames)
end

local function getUnitAttackDamage(u, targetDist)
	local dmg = u.Damage
	if u.Def.FirstStrikeDamage and (u.AttacksMade or 0) == 0 then
		dmg = u.Def.FirstStrikeDamage * (u.StackMult or 1.0)
	end
	if u.Def.FourthHitDamage then
		local hitCount = (u.HitCount or 0) + 1
		if hitCount % 4 == 0 then
			dmg = u.Def.FourthHitDamage * (u.StackMult or 1.0)
		end
	end
	if u.Def.ThirdHitDamage then
		local hitCount = (u.HitCount or 0) + 1
		if hitCount % 3 == 0 then
			dmg = u.Def.ThirdHitDamage * (u.StackMult or 1.0)
		end
	end
	if u.Def.CloseRangeDamage and u.Def.LongRangeDamage and targetDist then
		local thresh = u.Def.CloseRangeThreshold or 8.5
		if targetDist <= thresh then
			dmg = u.Def.CloseRangeDamage * (u.StackMult or 1.0)
		else
			dmg = u.Def.LongRangeDamage * (u.StackMult or 1.0)
		end
	end
	return dmg * (1 - (u.AttackDebuffFactor or 0))
end

local function updateUnitHPBar(u)
	if u.HPBarFill and u.HPBarFill.Parent then
		u.HPBarFill.Size = UDim2.new(math.clamp(u.HP / u.MaxHP, 0, 1), 0, 1, 0)
		if u.ShieldHP and u.ShieldHP > 0 then
			u.HPBarFill.BackgroundColor3 = Color3.fromRGB(80, 220, 255)
		elseif not u.DeathImmuneTriggered then
			u.HPBarFill.BackgroundColor3 = (u.Side == "Player") and Color3.fromRGB(50, 205, 50) or Color3.fromRGB(255, 60, 60)
		end
	end
end

local function refreshUnitCachedScan(u)
	local bestUnitTarget = nil
	local bestUnitDist = math.huge
	local sepVec = Vector3.zero
	local minSep = (u.Def.Scale and u.Def.Scale.X or 2.2) * 1.45

	local reduction = u.Def.DamageReduction or (u.Def.DamageReductionAura or ((u.Archetype == "ShieldTank") and 0.18 or 0))
	local atkBonus = u.Def.AttackSpeedAura or 0
	local ux, uz = u.Position.X, u.Position.Z

	for i = 1, #ActiveUnits do
		local other = ActiveUnits[i]
		if not other.Dead then
			local dx = other.Position.X - ux
			local dz = other.Position.Z - uz
			local distSq = dx * dx + dz * dz
			if other.Side ~= u.Side then
				if distSq < bestUnitDist then
					bestUnitDist = distSq
					bestUnitTarget = other
				end
			else
				if other ~= u and distSq > 0.0001 and distSq < (minSep * minSep) then
					local dist = math.sqrt(distSq)
					sepVec += Vector3.new(-dx / dist, 0, -dz / dist) * ((minSep - dist) / minSep)
				end
				if other ~= u and distSq <= 576 then
					if other.Def.DamageReductionAura then
						reduction = math.max(reduction, other.Def.DamageReductionAura)
					end
					if u.Def.DamageReductionAura then
						reduction = math.max(reduction, u.Def.DamageReductionAura)
					end
				end
				if other.Def.AttackSpeedAura and distSq <= 576 then
					atkBonus = math.max(atkBonus, other.Def.AttackSpeedAura)
				end
			end
		end
	end

	local bestTurretTarget = nil
	local bestTurretDist = math.huge
	for i = 1, #ActiveTurrets do
		local t = ActiveTurrets[i]
		if not t.Dead and t.Side ~= u.Side then
			local isTurret2 = (t.ModelName == "Turret2" or t.Name == "Turret 2")
			local t1Alive = false
			if isTurret2 then
				for j = 1, #ActiveTurrets do
					local otherT = ActiveTurrets[j]
					if not otherT.Dead and otherT.Side == t.Side and (otherT.ModelName == "Turret1" or otherT.Name == "Turret 1") then
						t1Alive = true
						break
					end
				end
			end

			if not t1Alive then
				local dx = t.Position.X - ux
				local dz = t.Position.Z - uz
				local dSq = dx * dx + dz * dz
				if dSq < bestTurretDist then
					bestTurretDist = dSq
					bestTurretTarget = t
				end
			end
		end
	end

	u.CachedTarget = bestUnitTarget
	u.CachedTurret = bestTurretTarget
	u.CachedSepVec = sepVec
	u.CachedDmgReduction = reduction
	u.CachedAtkSpeedMult = 1 + atkBonus
end

-- Damage unit (NO gold drop on kill!)
local function damageUnit(u, amount, attacker)
	if u.Dead then return false end
	if u.DeathImmuneTriggered then
		return false
	end
	local bonusDrain = (attacker and not attacker.Dead and attacker.Def.FlatDrain) or 0
	local dmgRed = u.CachedDmgReduction or 0
	local finalDmg = math.max(1, (amount + bonusDrain) * (1 - dmgRed))

	-- Shield absorption (Shield Bot mechanic)
	if u.ShieldHP and u.ShieldHP > 0 then
		if finalDmg <= u.ShieldHP then
			u.ShieldHP = u.ShieldHP - finalDmg
			finalDmg = 0
			updateUnitHPBar(u)
			createVisualEffect(u.Position, 2.4, Color3.fromRGB(80, 220, 255), 0.16)
		else
			finalDmg = finalDmg - u.ShieldHP
			u.ShieldHP = 0
			u.SkipNextShield = true -- Shield broken: won't gain shield on the next pulse
			updateUnitHPBar(u)
			createVisualEffect(u.Position, 3.0, Color3.fromRGB(120, 230, 255), 0.22)
		end
	end

	if finalDmg > 0 then
		-- Emergency Shield check (e.g. Plasma Overlord: Gain 40 shield when HP drops to/below 30%)
		if u.Def.EmergencyShieldAmount and not u.EmergencyShieldTriggered then
			local thresholdHP = u.MaxHP * (u.Def.EmergencyShieldThreshold or 0.30)
			if (u.HP - finalDmg) <= thresholdHP then
				u.EmergencyShieldTriggered = true
				local dmgToThreshold = math.max(0, u.HP - thresholdHP)
				local overflowDmg = finalDmg - dmgToThreshold
				u.HP = math.max(0, thresholdHP)
				local shieldAmt = u.Def.EmergencyShieldAmount * (u.StackMult or 1)
				u.ShieldHP = shieldAmt
				u.MaxShieldHP = math.max(u.MaxShieldHP or 0, shieldAmt)
				createVisualEffect(u.Position, 3.8, Color3.fromRGB(100, 240, 255), 0.35)
				playSoundAt("Laser", u.Position, 0.55, 1.35)

				if overflowDmg > 0 then
					if overflowDmg <= u.ShieldHP then
						u.ShieldHP = u.ShieldHP - overflowDmg
					else
						local remAfterShield = overflowDmg - u.ShieldHP
						u.ShieldHP = 0
						u.HP = u.HP - remAfterShield
					end
				end
				finalDmg = 0
				updateUnitHPBar(u)
			end
		end
	end

	if finalDmg > 0 then
		u.HP = u.HP - finalDmg
		updateUnitHPBar(u)
	end

	if attacker and not attacker.Dead then
		if attacker.Def.HealOnHit and attacker.Def.HealOnHit > 0 then
			attacker.HP = math.min(attacker.MaxHP, attacker.HP + (attacker.Def.HealOnHit * (attacker.StackMult or 1)))
			updateUnitHPBar(attacker)
		elseif attacker.Def.Lifesteal and attacker.Def.Lifesteal > 0 then
			local healAmt = attacker.Def.Lifesteal >= 1
				and (attacker.Def.Lifesteal * (attacker.StackMult or 1))
				or (finalDmg * attacker.Def.Lifesteal)
			attacker.HP = math.min(attacker.MaxHP, attacker.HP + healAmt)
			updateUnitHPBar(attacker)
		end
		if bonusDrain > 0 then
			attacker.HP = math.min(attacker.MaxHP, attacker.HP + bonusDrain)
			updateUnitHPBar(attacker)
		end
	end

	if u.HP <= 0 then
		if u.Def.DeathImmunity and u.Def.DeathImmunity > 0 and not u.DeathImmuneTriggered then
			u.DeathImmuneTriggered = true
			u.DeathImmuneUntil = os.clock() + u.Def.DeathImmunity
			u.HP = 1
			updateUnitHPBar(u)
			if u.HPBarFill and u.HPBarFill.Parent then
				u.HPBarFill.BackgroundColor3 = Color3.fromRGB(220, 140, 255)
			end
			createVisualEffect(u.Position, 3.0, Color3.fromRGB(215, 110, 255), 0.35)
			return false
		end
		if u.Def.ReviveHP and u.Def.ReviveHP > 0 and not u.ReviveTriggered then
			u.ReviveTriggered = true
			local restoreAmt = u.Def.ReviveHP * (u.StackMult or 1)
			u.HP = math.min(u.MaxHP, restoreAmt)
			updateUnitHPBar(u)
			if u.HPBarFill and u.HPBarFill.Parent then
				u.HPBarFill.BackgroundColor3 = Color3.fromRGB(140, 255, 75)
			end
			createVisualEffect(u.Position, 3.6, Color3.fromRGB(140, 255, 75), 0.40)
			playSoundAt("Heal", u.Position, 0.65, 0.9)
			return false
		end
		u.Dead = true
		if attacker and not attacker.Dead and attacker.Def.HealOnKill then
			attacker.HP = math.min(attacker.MaxHP, attacker.HP + attacker.Def.HealOnKill)
			updateUnitHPBar(attacker)
			createVisualEffect(attacker.Position, 2.8, Color3.fromRGB(255, 95, 145), 0.22)
		end
		createVisualEffect(u.Position, 3.2, u.Def.Accent, 0.22)
		if u.Model then
			u.Model:Destroy()
		end
		return true
	end
	return false
end

local function applyOnHitEffects(attacker, target, now)
	if target.Dead then return end
	if attacker.Def.PoisonDPS then
		target.PoisonDPS = attacker.Def.PoisonDPS
		target.PoisonUntil = now + (attacker.Def.PoisonDuration or 4.0)
	end
	if attacker.Def.BurnDPS then
		target.BurnDPS = attacker.Def.BurnDPS
		target.BurnUntil = now + (attacker.Def.BurnDuration or 3.0)
	end
	if attacker.Def.SlowFactor then
		target.SlowFactor = attacker.Def.SlowFactor
		target.SlowUntil = now + (attacker.Def.SlowDuration or 3.0)
	end
	if attacker.Def.AttackDebuff then
		target.AttackDebuffFactor = math.max(target.AttackDebuffFactor or 0, attacker.Def.AttackDebuff)
	end
end

local function damageTurret(t, amount)
	if t.Dead then return end
	t.HP = math.max(0, t.HP - amount)
	if t.HP <= 0 then
		t.Dead = true
		playSoundAt("Explosion", t.Position, 0.7, 0.85)
		createVisualEffect(t.Position + Vector3.new(0, 5, 0), 12, Color3.fromRGB(255, 140, 50), 0.45)

		local bounty = t.Bounty or (t.Name == "Turret 2" and 200 or 100)
		local enemySideKey = (t.Side == "Player") and "Enemy" or "Player"
		if Match and Match.Sides and Match.Sides[enemySideKey] then
			Match.Sides[enemySideKey].Gold = (Match.Sides[enemySideKey].Gold or 0) + bounty
		end

		local victimSideName = (t.Side == "Player") and "Ally" or "Enemy"
		local enemySideName = (t.Side == "Player") and "Enemy" or "Ally"
		for _, plr in ipairs({ Match.WestPlayer, Match.EastPlayer }) do
			if plr and plr.Parent == Players then
				BannerNoticeEvent:FireClient(
					plr,
					string.format("💥 %s %s destroyed! %s awarded +%d Gold bounty!", victimSideName, t.Name, enemySideName, bounty),
					Color3.fromRGB(255, 215, 0)
				)
			end
		end
		broadcastState()
	end
	updateTurretVisual(t)
end

-- Queue wave spawns with Elite Squad Merging (caps physical models per slot so large waves never lag!)
local function spawnSideWave(sideKey)
	local s = Match.Sides[sideKey]
	local fData = FactionConfig.Factions[s.Faction]
	if not fData then return end
	local baseX = CurrentArenaOriginX + (sideKey == "Player" and -98 or 98)
	local dir = sideKey == "Player" and 1 or -1

	local sideActiveCount = 0
	for _, u in ipairs(ActiveUnits) do
		if not u.Dead and u.Side == sideKey then
			sideActiveCount += 1
		end
	end

	local spawnIndex = 0
	for slotIdx = 1, 6 do
		local count = s.Lineup[slotIdx] or 0
		local unitId = s.Loadout[slotIdx] or slotIdx
		local unitDef = fData.Units[unitId]
		if unitDef and count > 0 then
			local totalToSpawn = count * (unitDef.SpawnCount or 1)
			local modelsToSpawn = totalToSpawn
			local stackMult = 1.0

			for i = 1, modelsToSpawn do
				spawnIndex += 1
				local row = math.floor((spawnIndex - 1) / 5)
				local col = (spawnIndex - 1) % 5
				local zOffset = (col - 2) * 6.5 + (math.random(-15, 15) * 0.1)
				local xOffset = -dir * (row * 3.5)
				local pos = Vector3.new(baseX + xOffset, 2.5, CurrentArenaOriginZ + zOffset)
				table.insert(PendingWaveSpawns, {
					Side = sideKey,
					Faction = s.Faction,
					Def = unitDef,
					Pos = pos,
					Dir = dir,
					StackMult = stackMult,
				})
			end
		end
	end
end

local function startMatch(mode, westPlayer, eastPlayer, botFaction, botDiff)
	local p1Faction = (westPlayer and PlayerSelectedFaction[westPlayer]) or "Kingdom"
	local p2Faction = (eastPlayer and PlayerSelectedFaction[eastPlayer]) or botFaction or "Republic"

	local targetArena = findAvailableArenaSlot(westPlayer, eastPlayer, p1Faction)
	if not targetArena then
		if westPlayer then
			BannerNoticeEvent:FireClient(westPlayer, "⚔️ All arena instances are currently full! Please wait a moment.", Color3.fromRGB(255, 160, 80))
		end
		return false
	end

	setArenaContext(targetArena)
	targetArena.SecondAccumulator = 0

	for _, u in ipairs(ActiveUnits) do
		if u.Model then u.Model:Destroy() end
	end
	table.clear(ActiveUnits)
	table.clear(PendingWaveSpawns)
	table.clear(ActiveTurrets)
	if CurrentArenaUnitsFolder then
		CurrentArenaUnitsFolder:ClearAllChildren()
	end

	Match.Active = true
	Match.Mode = mode or "Bot"	Match.BotDifficulty = botDiff or "Normal"
	Match.WestPlayer = westPlayer
	Match.EastPlayer = eastPlayer
	Match.WaveNumber = 0
	Match.WaveTimer = WAVE_INTERVAL -- 15 seconds until first wave spawns
	Match.GameOver = false

	local p1Loadout = validateLoadout(p1Faction, westPlayer and PlayerLoadouts[westPlayer])
	local p2Loadout = validateLoadout(p2Faction, eastPlayer and PlayerLoadouts[eastPlayer])

	Match.Sides.Player = {
		Faction = p1Faction,
		Loadout = p1Loadout,
		BaseHP = MAX_BASE_HP,
		Gold = 45,
		IncomeLevel = 1,
		TierUnlock = 1, -- 1 = C-Tier unlocked, must pay 150G for B-Tier (2) and 300G for A-Tier (3)
		Lineup = { 1, 0, 0, 0, 0, 0 },
		SpellCooldowns = { Q = 0, E = 0 },
	}
	local botStartLineup = { 1, 0, 0, 0, 0, 0 }
	local botStartGold = 45
	if botDiff == "Easy" then
		botStartLineup = { 1, 0, 0, 0, 0, 0 }
		botStartGold = 35
	elseif botDiff == "Hard" then
		botStartLineup = { 2, 0, 0, 0, 0, 0 }
		botStartGold = 65
	elseif botDiff == "Extreme" then
		botStartLineup = { 2, 1, 0, 0, 0, 0 }
		botStartGold = 90
	end

	Match.Sides.Enemy = {
		Faction = p2Faction,
		Loadout = p2Loadout,
		BaseHP = MAX_BASE_HP,
		Gold = (Match.Mode == "Bot") and botStartGold or 45,
		IncomeLevel = 1,
		TierUnlock = 1,
		Lineup = (Match.Mode == "Bot") and botStartLineup or { 1, 0, 0, 0, 0, 0 },
		SpellCooldowns = { Q = 0, E = 0 },
	}

	updateBaseVisuals("Player", p1Faction, MAX_BASE_HP)
	updateBaseVisuals("Enemy", p2Faction, MAX_BASE_HP)
	setupTurretsForBase("Player", p1Faction)
	setupTurretsForBase("Enemy", p2Faction)

	if westPlayer then
		teleportPlayerTo(westPlayer, Vector3.new(CurrentArenaOriginX - 96, 4.5, CurrentArenaOriginZ + 24), Vector3.new(CurrentArenaOriginX, 4.5, CurrentArenaOriginZ))
	end
	if eastPlayer then
		teleportPlayerTo(eastPlayer, Vector3.new(CurrentArenaOriginX + 96, 4.5, CurrentArenaOriginZ + 24), Vector3.new(CurrentArenaOriginX, 4.5, CurrentArenaOriginZ))
	end

	playSoundAt("WaveHorn", Vector3.new(CurrentArenaOriginX, 15, CurrentArenaOriginZ), 0.65, 1.05)
	for _, plr in ipairs({ westPlayer, eastPlayer }) do
		if plr and plr.Parent == Players then
			BannerNoticeEvent:FireClient(
				plr,
				string.format("%s — 1v1 BATTLE: %s vs %s! Wave 1 deploys in 15s!", targetArena.ArenaName or ("ARENA #" .. tostring(targetArena.SlotId)), FactionConfig.Factions[p1Faction].DisplayName, FactionConfig.Factions[p2Faction].DisplayName),
				FactionConfig.Factions[p1Faction].AccentColor
			)
		end
	end
	broadcastState()
	return true
end

local function endMatchAndReturnToLobby(arenaSlot)
	local prevArena = CurrentArena
	if arenaSlot then
		setArenaContext(arenaSlot)
	end
	local p1 = Match.WestPlayer
	local p2 = Match.EastPlayer

	Match.Active = false
	Match.GameOver = false
	Match.WestPlayer = nil
	Match.EastPlayer = nil

	for _, u in ipairs(ActiveUnits) do
		if u.Model then u.Model:Destroy() end
	end
	table.clear(ActiveUnits)
	table.clear(PendingWaveSpawns)
	table.clear(ActiveTurrets)
	if CurrentArenaUnitsFolder then
		CurrentArenaUnitsFolder:ClearAllChildren()
	end

	local idleTheme = (CurrentArena and CurrentArena.FactionTheme) or "Kingdom"
	updateBaseVisuals("Player", idleTheme, MAX_BASE_HP)
	updateBaseVisuals("Enemy", idleTheme, MAX_BASE_HP)
	setupTurretsForBase("Player", idleTheme)
	setupTurretsForBase("Enemy", idleTheme)

	if p1 and p1.Parent == Players then
		teleportPlayerTo(p1, Vector3.new(0, 11.5, -135), Vector3.new(0, 11.5, -178))
	end
	if p2 and p2.Parent == Players then
		teleportPlayerTo(p2, Vector3.new(0, 11.5, -135), Vector3.new(0, 11.5, -178))
	end

	broadcastState()
	if prevArena and prevArena ~= arenaSlot then
		setArenaContext(prevArena)
	end
end

local function finishMatch(winningSideKey)
	if Match.GameOver then return end
	Match.GameOver = true
	local finishedArena = CurrentArena

	local losingSideKey = (winningSideKey == "Player") and "Enemy" or "Player"
	local winFactionKey = Match.Sides[winningSideKey].Faction
	local loseFactionKey = Match.Sides[losingSideKey].Faction
	local winFData = FactionConfig.Factions[winFactionKey] or FactionConfig.Factions.Kingdom
	local loseFData = FactionConfig.Factions[loseFactionKey] or FactionConfig.Factions.Republic

	local winnerPlayer = nil
	local winnerName = ""

	if winningSideKey == "Player" then
		winnerPlayer = Match.WestPlayer
		winnerName = winnerPlayer and winnerPlayer.DisplayName or "West Commander"
	else
		if Match.Mode == "PvP" then
			winnerPlayer = Match.EastPlayer
			winnerName = winnerPlayer and winnerPlayer.DisplayName or "East Commander"
		else
			winnerPlayer = nil
			winnerName = "BOT COMMANDER (" .. Match.BotDifficulty .. ")"
		end
	end

	-- Award 1 Win to the winner player ONLY if it was a PvP duel! (Winning against Bot does NOT award Ranked Wins)
	if winnerPlayer and Match.Mode == "PvP" then
		PlayerWins[winnerPlayer] = (PlayerWins[winnerPlayer] or 0) + 1
		local ls = winnerPlayer:FindFirstChild("leaderstats")
		local wVal = ls and ls:FindFirstChild("Wins")
		if wVal then
			wVal.Value = PlayerWins[winnerPlayer]
		end
		recordLeaderboardEntry(
			winnerPlayer.UserId,
			winnerPlayer.Name,
			winnerPlayer.DisplayName,
			PlayerSelectedFaction[winnerPlayer] or "Kingdom",
			PlayerWins[winnerPlayer],
			true
		)
		task.spawn(function()
			pcall(function()
				if WinsDataStore then
					WinsDataStore:SetAsync("Wins_" .. winnerPlayer.UserId, PlayerWins[winnerPlayer])
				end
			end)
			pcall(function()
				if GlobalWinsStore then
					GlobalWinsStore:SetAsync(tostring(winnerPlayer.UserId), PlayerWins[winnerPlayer])
				end
			end)
		end)
	end

	playSoundAt("WaveHorn", Vector3.new(CurrentArenaOriginX, 15, CurrentArenaOriginZ), 0.85, 1.15)

	local loserPlayer = (winningSideKey == "Player") and Match.EastPlayer or Match.WestPlayer

	-- Targeted banner notices so lobby players never receive victory modals or victory banners
	if winnerPlayer and winnerPlayer.Parent == Players then
		if Match.Mode == "PvP" then
			BannerNoticeEvent:FireClient(
				winnerPlayer,
				string.format("🏆 VICTORY! You defeated %s in 1v1 PvP Duel! (+1 RANKED WIN)", loserPlayer and loserPlayer.DisplayName or "Enemy"),
				winFData.AccentColor
			)
		else
			BannerNoticeEvent:FireClient(
				winnerPlayer,
				string.format("⚔️ VICTORY! You defeated the %s Bot Commander! (Practice Match — Duel players in PvP to earn Wins)", Match.BotDifficulty),
				Color3.fromRGB(255, 215, 80)
			)
		end
	end

	if loserPlayer and loserPlayer.Parent == Players then
		BannerNoticeEvent:FireClient(
			loserPlayer,
			string.format("💀 DEFEAT! Your Citadel was destroyed by %s!", winnerName),
			Color3.fromRGB(255, 90, 100)
		)
	end

	-- Broadcast match conclusion to lobby ONLY for PvP duels
	if Match.Mode == "PvP" then
		for _, plr in ipairs(Players:GetPlayers()) do
			if plr ~= winnerPlayer and plr ~= loserPlayer then
				BannerNoticeEvent:FireClient(
					plr,
					string.format("⚔️ Arena Match Concluded: %s (%s) defeated %s!", winnerName, winFData.DisplayName, loseFData.DisplayName),
					Color3.fromRGB(180, 205, 235)
				)
			end
		end
	end

	-- Send result screen modal ONLY to active match participants
	for _, plr in ipairs({ Match.WestPlayer, Match.EastPlayer }) do
		if plr and plr.Parent == Players then
			local isWinner = (winnerPlayer == plr)
			local myWins = PlayerWins[plr] or 0

			MatchResultEvent:FireClient(plr, {
				WinnerName = winnerName,
				WinnerFaction = winFData.DisplayName,
				WinnerBadge = winFData.Badge or "🏆",
				WinnerAccent = winFData.AccentColor,
				LoserFaction = loseFData.DisplayName,
				IsParticipant = true,
				IsWinner = isWinner,
				PlayerWins = myWins,
				MatchMode = Match.Mode,
				BotDifficulty = Match.BotDifficulty,
				WaveNumber = Match.WaveNumber,
				AutoReturnDelay = 5,
			})
		end
	end

	broadcastState()

	-- Automatically return to lobby after 5 seconds!
	task.delay(5.0, function()
		if finishedArena and finishedArena.Match.GameOver then
			endMatchAndReturnToLobby(finishedArena)
		end
	end)
end

local function returnPlayerToLobby(player)
	local pArena = getPlayerArena(player)
	if pArena then
		endMatchAndReturnToLobby(pArena)
	else
		teleportPlayerTo(player, Vector3.new(0, 11.5, -135), Vector3.new(0, 11.5, -178))
		broadcastState()
	end
end

Players.PlayerAdded:Connect(function(plr)
	setupPlayerStats(plr)
	setupAdminPlayer(plr)
	PlayerSelectedFaction[plr] = PlayerSelectedFaction[plr] or "Kingdom"
	PlayerLoadouts[plr] = getDefaultLoadout()
	recordLeaderboardEntry(plr.UserId, plr.Name, plr.DisplayName, PlayerSelectedFaction[plr], PlayerWins[plr] or 0, true)
	plr.CharacterAdded:Connect(function()
		task.wait(0.2)
		local isParticipant = (getPlayerArena(plr) ~= nil)
		if not isParticipant then
			teleportPlayerTo(plr, Vector3.new(0, 11.5, -135), Vector3.new(0, 11.5, -178))
		end
		broadcastState()
	end)
	task.defer(broadcastState)
end)

Players.PlayerRemoving:Connect(function(plr)
	-- Ensure chosen faction is persisted to DataStore upon quit!
	local curFaction = PlayerSelectedFaction[plr]
	if curFaction and FactionDataStore then
		pcall(function()
			FactionDataStore:SetAsync("Faction_" .. tostring(plr.UserId), curFaction)
		end)
	end

	-- Mark player as offline in PersistentLeaderboard, but DO NOT delete their entry!
	if PersistentLeaderboard[plr.UserId] then
		PersistentLeaderboard[plr.UserId].Online = false
	end

	PlayerSelectedFaction[plr] = nil
	PlayerLoadouts[plr] = nil
	if PvPQueuePlayer == plr then
		PvPQueuePlayer = nil
	end
	local pArena = getPlayerArena(plr)
	if pArena then
		setArenaContext(pArena)
		if Match.Mode == "PvP" then
			local remainingSide = (Match.WestPlayer == plr) and "Enemy" or "Player"
			local remainingPlr = (Match.WestPlayer == plr) and Match.EastPlayer or Match.WestPlayer
			if remainingPlr and remainingPlr.Parent == Players then
				finishMatch(remainingSide)
			else
				endMatchAndReturnToLobby(pArena)
			end
		else
			endMatchAndReturnToLobby(pArena)
		end
	end
	broadcastState()
end)

for _, plr in ipairs(Players:GetPlayers()) do
	setupPlayerStats(plr)
	setupAdminPlayer(plr)
	PlayerSelectedFaction[plr] = PlayerSelectedFaction[plr] or "Kingdom"
	PlayerLoadouts[plr] = PlayerLoadouts[plr] or getDefaultLoadout()
	recordLeaderboardEntry(plr.UserId, plr.Name, plr.DisplayName, PlayerSelectedFaction[plr], PlayerWins[plr] or 0, true)
end

-- Background sync for cross-server / persistent cloud leaderboard
task.spawn(function()
	task.wait(3.0)
	while true do
		pcall(function()
			if GlobalWinsStore then
				local pages = GlobalWinsStore:GetSortedAsync(false, 12)
				local entries = pages:GetCurrentPage()
				for _, data in ipairs(entries) do
					local uId = tonumber(data.key)
					local wCount = tonumber(data.value) or 0
					if uId and wCount > 0 then
						local pName = "Commander_" .. uId
						pcall(function()
							pName = Players:GetNameFromUserIdAsync(uId)
						end)
						local existing = PersistentLeaderboard[uId]
						if not existing then
							PersistentLeaderboard[uId] = {
								UserId = uId,
								Name = pName,
								DisplayName = pName,
								Faction = "Kingdom",
								Wins = wCount,
								Online = false,
							}
						else
							existing.Wins = math.max(existing.Wins or 0, wCount)
						end
					end
				end
				if updateLobbyLeaderboard then
					updateLobbyLeaderboard()
				end
			end
		end)
		task.wait(60.0)
	end
end)

ProximityPromptService.PromptTriggered:Connect(function(prompt, player)
	local shrineFaction = prompt:GetAttribute("SelectFaction") or (prompt.Parent and prompt.Parent:GetAttribute("FactionName"))
	if shrineFaction and FactionConfig.Factions[shrineFaction] then
		savePlayerFaction(player, shrineFaction)
		PlayerLoadouts[player] = validateLoadout(shrineFaction, PlayerLoadouts[player])
		BannerNoticeEvent:FireClient(player, "Selected Faction: " .. FactionConfig.Factions[shrineFaction].DisplayName, FactionConfig.Factions[shrineFaction].AccentColor, "Heal")
		LobbyActionEvent:FireClient(player, "OpenLobbyUI", "Bot")
		broadcastState()
		return
	end

	local portalMode = prompt:GetAttribute("PortalMode") or (prompt.Parent and prompt.Parent:GetAttribute("PortalMode"))
	local lobbyPortal = prompt:GetAttribute("LobbyPortal") or (prompt.Parent and prompt.Parent:GetAttribute("LobbyPortal"))
	if portalMode == "Bot" then
		if getPlayerArena(player) then
			return
		end
		LobbyActionEvent:FireClient(player, "OpenBotDifficultyModal")
	elseif portalMode == "PvP" then
		LobbyActionEvent:FireClient(player, "OpenLobbyUI", "PvP")
	elseif lobbyPortal == "OpenUI" then
		LobbyActionEvent:FireClient(player, "OpenLobbyUI", "Bot")
	end
end)

LobbyActionEvent.OnServerEvent:Connect(function(player, action, arg1, arg2, arg3, arg4, arg5, arg6)
	if action == "SelectFaction" and FactionConfig.Factions[arg1] then
		savePlayerFaction(player, arg1)
		PlayerLoadouts[player] = validateLoadout(arg1, PlayerLoadouts[player])
		broadcastState()
	elseif action == "SetLoadout" then
		local fName = PlayerSelectedFaction[player] or "Kingdom"
		local proposed = type(arg1) == "table" and arg1 or {
			tonumber(arg1) or 1,
			tonumber(arg2) or 2,
			tonumber(arg3) or 4,
			tonumber(arg4) or 5,
			tonumber(arg5) or 7,
			tonumber(arg6) or 8,
		}
		PlayerLoadouts[player] = validateLoadout(fName, proposed)
		local pArena = getPlayerArena(player)
		if pArena then
			setArenaContext(pArena)
			local sideKey = getPlayerSide(player)
			Match.Sides[sideKey].Loadout = PlayerLoadouts[player]
		end
		broadcastState()
	elseif action == "StartBotMatch" then
		local myFaction = (arg1 and FactionConfig.Factions[arg1]) and arg1 or (PlayerSelectedFaction[player] or "Kingdom")
		local botFaction = (arg2 and FactionConfig.Factions[arg2]) and arg2 or nil
		if not botFaction then
			local validFactions = FactionConfig.Order or { "Kingdom", "Republic", "Alien", "Demon", "Void", "Mechanica" }
			local candidateList = {}
			for _, f in ipairs(validFactions) do
				if f ~= myFaction then table.insert(candidateList, f) end
			end
			botFaction = (#candidateList > 0) and candidateList[math.random(1, #candidateList)] or "Republic"
		end
		local botDiff = arg3
		if botDiff ~= "Easy" and botDiff ~= "Normal" and botDiff ~= "Hard" and botDiff ~= "Extreme" then
			botDiff = "Normal"
		end
		savePlayerFaction(player, myFaction)
		PlayerLoadouts[player] = validateLoadout(myFaction, PlayerLoadouts[player])
		if PvPQueuePlayer == player then
			PvPQueuePlayer = nil
		end
		startMatch("Bot", player, nil, botFaction, botDiff)
	elseif action == "LeaveToLobby" then
		returnPlayerToLobby(player)
	elseif action == "ChallengePlayer" then
		local targetUserId = tonumber(arg1)
		local targetPlr = targetUserId and Players:GetPlayerByUserId(targetUserId)
		if targetPlr and targetPlr ~= player then
			if getPlayerArena(targetPlr) then
				BannerNoticeEvent:FireClient(player, targetPlr.DisplayName .. " is currently in a match!", Color3.fromRGB(255, 160, 80))
				return
			end
			ChallengeEvent:FireClient(targetPlr, "IncomingChallenge", player.UserId, player.DisplayName, PlayerSelectedFaction[player] or "Kingdom")
			BannerNoticeEvent:FireClient(player, "Sent 1v1 Duel Challenge to " .. targetPlr.DisplayName .. "!", Color3.fromRGB(90, 220, 255))
		end
	elseif action == "AcceptChallenge" then
		if getPlayerArena(player) then
			BannerNoticeEvent:FireClient(player, "⚔️ You are already in an active match!", Color3.fromRGB(255, 160, 80))
			return
		end
		local challengerId = tonumber(arg1)
		local challenger = challengerId and Players:GetPlayerByUserId(challengerId)
		if challenger then
			if getPlayerArena(challenger) then
				BannerNoticeEvent:FireClient(player, challenger.DisplayName .. " is already in another match!", Color3.fromRGB(255, 160, 80))
				return
			end
			if PvPQueuePlayer == challenger or PvPQueuePlayer == player then
				PvPQueuePlayer = nil
			end
			startMatch("PvP", challenger, player, nil, nil)
		end
	elseif action == "JoinPvPQueue" then
		if getPlayerArena(player) then
			BannerNoticeEvent:FireClient(player, "You are currently in an active match!", Color3.fromRGB(255, 160, 80))
			return
		end
		if PvPQueuePlayer == player then
			PvPQueuePlayer = nil
			BannerNoticeEvent:FireClient(player, "Left 1v1 PvP Matchmaking Queue.", Color3.fromRGB(200, 210, 230))
			broadcastState()
		elseif PvPQueuePlayer and PvPQueuePlayer.Parent == Players and PvPQueuePlayer ~= player and not getPlayerArena(PvPQueuePlayer) then
			local opponent = PvPQueuePlayer
			PvPQueuePlayer = nil
			startMatch("PvP", opponent, player, nil, nil)
		else
			PvPQueuePlayer = player
			BannerNoticeEvent:FireAllClients("⚔️ " .. player.DisplayName .. " joined the 1v1 PvP Queue! Step on a Duel Pad or click Join Queue to fight!", Color3.fromRGB(255, 110, 145))
			broadcastState()
		end
	end
end)

-- BuyUnit: Enforces B-Tier (150G) and A-Tier (300G) unlock gate, and queues for next 15s wave!
BuyUnitEvent.OnServerEvent:Connect(function(player, slotIdx)
	local pArena = getPlayerArena(player)
	if not pArena then return end
	setArenaContext(pArena)
	if not Match.Active or Match.GameOver then return end
	if type(slotIdx) ~= "number" or slotIdx < 1 or slotIdx > 6 then return end

	local sideKey = getPlayerSide(player)
	local s = Match.Sides[sideKey]
	local fData = FactionConfig.Factions[s.Faction]
	local unitId = s.Loadout[slotIdx] or slotIdx
	local unitDef = fData and fData.Units[unitId]
	if not unitDef then return end

	-- Check Tier Unlock requirement!
	if unitDef.Tier == "B" and s.TierUnlock < 2 then
		BannerNoticeEvent:FireClient(player, "🔒 B-Tier Troops are LOCKED! Upgrade Tier Unlock [8] (100 Gold) first!", Color3.fromRGB(255, 130, 95))
		return
	elseif unitDef.Tier == "A" and s.TierUnlock < 3 then
		local neededMsg = (s.TierUnlock < 2) and "Unlock B-Tier (100G) then A-Tier (250G) [8] first!" or "Upgrade Tier Unlock [8] (250 Gold) to unlock A-Tier!"
		BannerNoticeEvent:FireClient(player, "🔒 A-Tier Troops are LOCKED! " .. neededMsg, Color3.fromRGB(255, 130, 95))
		return
	end

	local ownedCount = s.Lineup[slotIdx] or 0
	local currentCost = FactionConfig.GetDynamicCost(unitDef, ownedCount)

	if s.Gold >= currentCost then
		s.Gold -= currentCost
		s.Lineup[slotIdx] = ownedCount + 1
		broadcastState()
	end
end)

UpgradeStatEvent.OnServerEvent:Connect(function(player, upgradeType)
	local pArena = getPlayerArena(player)
	if not pArena then return end
	setArenaContext(pArena)
	if not Match.Active or Match.GameOver then return end
	local sideKey = getPlayerSide(player)
	local s = Match.Sides[sideKey]

	if upgradeType == "Income" then
		if s.IncomeLevel >= MAX_INCOME_LEVEL then
			BannerNoticeEvent:FireClient(player, "Gold Income is already at MAX Level (Lv. 5)!", Color3.fromRGB(120, 245, 155))
			return
		end
		local cost = getSideIncomeUpgradeCost(sideKey)
		if s.Gold >= cost then
			s.Gold -= cost
			s.IncomeLevel = math.min(MAX_INCOME_LEVEL, s.IncomeLevel + 1)
			if s.IncomeLevel >= MAX_INCOME_LEVEL then
				BannerNoticeEvent:FireClient(player, string.format("Income Upgraded (Lv. %d MAX)! Now +%d Gold per 15s Wave!", s.IncomeLevel, getSideWaveIncome(sideKey)), Color3.fromRGB(255, 220, 80), "Heal")
			else
				BannerNoticeEvent:FireClient(player, string.format("Income Upgraded (Lv. %d/%d)! Now +%d Gold per 15s Wave! (Next Upgrade: %dG)", s.IncomeLevel, MAX_INCOME_LEVEL, getSideWaveIncome(sideKey), getSideIncomeUpgradeCost(sideKey)), Color3.fromRGB(255, 220, 80), "Heal")
			end
			broadcastState()
		else
			BannerNoticeEvent:FireClient(player, string.format("Need %d Gold to upgrade Gold Income to Lv. %d!", cost, s.IncomeLevel + 1), Color3.fromRGB(255, 130, 95))
		end
	elseif upgradeType == "TierUnlock" or upgradeType == "Tech" then
		if s.TierUnlock >= 3 then
			BannerNoticeEvent:FireClient(player, "All Troop Tiers (C, B & A) are already unlocked!", Color3.fromRGB(120, 245, 155))
			return
		end
		local cost = getSideTierUnlockCost(sideKey)
		if s.Gold >= cost then
			s.Gold -= cost
			s.TierUnlock += 1
			if s.TierUnlock == 2 then
				BannerNoticeEvent:FireClient(player, "🔓 B-TIER TROOPS UNLOCKED! You can now recruit Slots [3] & [4]!", Color3.fromRGB(90, 200, 255), "Spell")
			elseif s.TierUnlock == 3 then
				BannerNoticeEvent:FireClient(player, "🔓 A-TIER ELITE TROOPS UNLOCKED! You can now recruit Slots [5] & [6]!", Color3.fromRGB(255, 215, 75), "Spell")
			end
			broadcastState()
		else
			local tierName = (s.TierUnlock == 1) and "B-Tier" or "A-Tier"
			BannerNoticeEvent:FireClient(player, string.format("Need %d Gold to unlock %s Troops!", cost, tierName), Color3.fromRGB(255, 130, 95))
		end
	end
end)

SwitchFactionEvent.OnServerEvent:Connect(function(player, targetSide, factionName)
	local pArena = getPlayerArena(player)
	if pArena then
		setArenaContext(pArena)
	end
	if targetSide == "Restart" then
		if pArena and Match.Active then
			startMatch(Match.Mode, Match.WestPlayer, Match.EastPlayer, Match.Sides.Enemy.Faction, Match.BotDifficulty)
		end
	elseif targetSide == "Player" and FactionConfig.Factions[factionName] then
		savePlayerFaction(player, factionName)
		PlayerLoadouts[player] = validateLoadout(factionName, PlayerLoadouts[player])
		if pArena and Match.Active then
			startMatch(Match.Mode, Match.WestPlayer, Match.EastPlayer, Match.Sides.Enemy.Faction, Match.BotDifficulty)
		else
			broadcastState()
		end
	elseif targetSide == "Enemy" and FactionConfig.Factions[factionName] then
		if pArena and Match.Active and Match.Mode == "Bot" then
			Match.Sides.Enemy.Faction = factionName
			startMatch("Bot", Match.WestPlayer, nil, factionName, Match.BotDifficulty)
		else
			broadcastState()
		end
	end
end)

CastSpellEvent.OnServerEvent:Connect(function(player, key)
	local pArena = getPlayerArena(player)
	if not pArena then return end
	setArenaContext(pArena)
	if not Match.Active or Match.GameOver then return end
	if key ~= "Q" and key ~= "E" then return end
	local sideKey = getPlayerSide(player)
	local s = Match.Sides[sideKey]
	local now = os.clock()
	if now < (s.SpellCooldowns[key] or 0) then return end

	local fData = FactionConfig.Factions[s.Faction]
	local spellDef = fData and fData.Spells[key]
	if not spellDef then return end

	s.SpellCooldowns[key] = now + (spellDef.Cooldown or 60)
	local char = player.Character
	local root = char and char:FindFirstChild("HumanoidRootPart")
	local dir = sideKey == "Player" and 1 or -1
	local heroPos = root and root.Position or Vector3.new(CurrentArenaOriginX - 30 * dir, 3, CurrentArenaOriginZ)

	if key == "Q" then
		local strikePos = Vector3.new(math.clamp(heroPos.X + 24 * dir, CurrentArenaOriginX - 100, CurrentArenaOriginX + 105), 3, math.clamp(heroPos.Z, CurrentArenaOriginZ - 30, CurrentArenaOriginZ + 30))
		local closestEnemyDist = 65
		for _, u in ipairs(ActiveUnits) do
			if not u.Dead and u.Side ~= sideKey then
				local d = (u.Position - heroPos).Magnitude
				if d < closestEnemyDist then
					closestEnemyDist = d
					strikePos = u.Position
				end
			end
		end

		playSoundAt("Spell", strikePos, 0.75, 0.95)
		playSoundAt("Explosion", strikePos, 0.7, 0.9)
		createBeamEffect(strikePos + Vector3.new(0, 68, 0), strikePos, fData.AccentColor, 6.0)
		createVisualEffect(strikePos, 23, fData.AccentColor, 0.55)

		for _, u in ipairs(ActiveUnits) do
			if not u.Dead and u.Side ~= sideKey then
				if (u.Position - strikePos).Magnitude <= 23 then
					damageUnit(u, 115, nil)
				end
			end
		end
		for _, plr in ipairs({ Match.WestPlayer, Match.EastPlayer }) do
			if plr and plr.Parent == Players then
				BannerNoticeEvent:FireClient(plr, "SPELL CARD: " .. spellDef.Name .. "!", fData.AccentColor)
			end
		end
	elseif key == "E" then
		playSoundAt("Heal", heroPos, 0.75, 1.1)
		createVisualEffect(heroPos, 26, fData.MainColor, 0.55)
		for _, u in ipairs(ActiveUnits) do
			if not u.Dead and u.Side == sideKey then
				u.HP = math.min(u.MaxHP, u.HP + 75)
				u.SpeedBoostUntil = now + 6.0
				updateUnitHPBar(u)
				createVisualEffect(u.Position, 3.5, fData.AccentColor, 0.3)
			end
		end
		for _, plr in ipairs({ Match.WestPlayer, Match.EastPlayer }) do
			if plr and plr.Parent == Players then
				BannerNoticeEvent:FireClient(plr, "SPELL CARD: " .. spellDef.Name .. "!", fData.AccentColor)
			end
		end
	end

	broadcastState()
end)

-- ============================================================
-- ADMIN / DEVELOPER COMMAND SYSTEM
-- ============================================================
local function isPlayerAdmin(plr)
	if not plr then return false end
	if RunService:IsStudio() then return true end
	if game.CreatorType == Enum.CreatorType.User and plr.UserId == game.CreatorId then
		return true
	end
	if plr.UserId == 194993165 then
		return true
	end
	return false
end

local function handleAdminCommand(player, commandStr, arg)
	if not isPlayerAdmin(player) then
		warn("[SECURITY] Unauthorized admin command attempt by " .. tostring(player))
		return
	end

	local pArena = getPlayerArena(player)
	if pArena then
		setArenaContext(pArena)
	end
	local cmd = string.lower(tostring(commandStr or ""))
	local sideKey = getPlayerSide(player)

	if cmd == "addgold" or cmd == "gold" then
		local amount = tonumber(arg) or 1000
		if Match.Active and Match.Sides[sideKey] then
			Match.Sides[sideKey].Gold += amount
			broadcastState()
			BannerNoticeEvent:FireClient(player, string.format("⚡ [ADMIN] +%d Gold Added! (Total: %d)", amount, math.floor(Match.Sides[sideKey].Gold)), Color3.fromRGB(255, 215, 0))
		else
			BannerNoticeEvent:FireClient(player, "⚡ [ADMIN] Gold can only be added during a match!", Color3.fromRGB(255, 120, 100))
		end

	elseif cmd == "unlocktiers" or cmd == "tiers" or cmd == "tier" then
		if Match.Active and Match.Sides[sideKey] then
			local targetTier = tonumber(arg) or 3
			Match.Sides[sideKey].TierUnlock = math.clamp(targetTier, 1, 3)
			broadcastState()
			BannerNoticeEvent:FireClient(player, "⚡ [ADMIN] Tier Unlock set to Level " .. Match.Sides[sideKey].TierUnlock .. " (All Tiers Ready)!", Color3.fromRGB(255, 215, 0))
		else
			BannerNoticeEvent:FireClient(player, "⚡ [ADMIN] Tiers can only be set during a match!", Color3.fromRGB(255, 120, 100))
		end

	elseif cmd == "nextwave" or cmd == "wave" then
		if Match.Active and not Match.GameOver then
			Match.WaveTimer = 0.05
			BannerNoticeEvent:FireClient(player, "⚡ [ADMIN] Triggering Immediate Wave!", Color3.fromRGB(255, 215, 0))
		else
			BannerNoticeEvent:FireClient(player, "⚡ [ADMIN] No active match to advance wave!", Color3.fromRGB(255, 120, 100))
		end

	elseif cmd == "healbase" or cmd == "heal" then
		if Match.Active and Match.Sides[sideKey] then
			Match.Sides[sideKey].BaseHP = MAX_BASE_HP
			broadcastState()
			BannerNoticeEvent:FireClient(player, string.format("⚡ [ADMIN] Citadel HP restored to 100%% (%d)!", MAX_BASE_HP), Color3.fromRGB(80, 255, 120))
		end

	elseif cmd == "wipeenemies" or cmd == "wipe" or cmd == "killbot" then
		if Match.Active and not Match.GameOver then
			local enemySide = (sideKey == "Player") and "Enemy" or "Player"
			local wipedCount = 0
			for _, u in ipairs(ActiveUnits) do
				if u.Side == enemySide and not u.Dead then
					u.Dead = true
					if u.Model then
						u.Model:Destroy()
					end
					wipedCount += 1
				end
			end
			BannerNoticeEvent:FireClient(player, string.format("⚡ [ADMIN] Wiped %d Enemy Units!", wipedCount), Color3.fromRGB(255, 215, 0))
		end

	elseif cmd == "killturret" or cmd == "destroyturret" then
		if Match.Active and not Match.GameOver then
			local argStr = tostring(arg or "1")
			local targetSide = (string.find(argStr, "ally") or string.find(argStr, "player")) and sideKey or ((sideKey == "Player") and "Enemy" or "Player")
			local targetName = string.find(argStr, "2") and "Turret 2" or "Turret 1"
			for _, t in ipairs(ActiveTurrets) do
				if t.Side == targetSide and t.Name == targetName and not t.Dead then
					damageTurret(t, 999999)
					BannerNoticeEvent:FireClient(player, string.format("⚡ [ADMIN] Destroyed %s %s!", targetSide, targetName), Color3.fromRGB(255, 215, 0))
					break
				end
			end
		end

	elseif cmd == "winmatch" or cmd == "win" then
		if Match.Active and not Match.GameOver then
			finishMatch(sideKey)
			BannerNoticeEvent:FireClient(player, "⚡ [ADMIN] Instant Victory Triggered!", Color3.fromRGB(255, 215, 0))
		end
	end
end

AdminCommandEvent.OnServerEvent:Connect(handleAdminCommand)

function setupAdminPlayer(plr)
	plr.Chatted:Connect(function(msg)
		if not isPlayerAdmin(plr) then return end
		if msg:sub(1, 1) == "/" or msg:sub(1, 1) == ";" or msg:sub(1, 1) == "!" then
			local clean = msg:sub(2)
			local parts = string.split(clean, " ")
			local command = parts[1]
			local param = parts[2]
			if command then
				handleAdminCommand(plr, command, param)
			end
		end
	end)
end

local TCS = game:GetService("TextChatService")
task.spawn(function()
	local tcc = TCS:WaitForChild("TextChatCommands", 4)
	if tcc then
		local function bindCommand(cmdName, handlerKey)
			local cmd = tcc:FindFirstChild(cmdName)
			if cmd then
				cmd.Triggered:Connect(function(textSource, rawText)
					local plr = Players:GetPlayerByUserId(textSource.UserId)
					if plr and isPlayerAdmin(plr) then
						local parts = string.split(rawText, " ")
						local arg = parts[2]
						handleAdminCommand(plr, handlerKey, arg)
					end
				end)
			end
		end
		bindCommand("GoldCmd", "gold")
		bindCommand("TierCmd", "tier")
		bindCommand("WaveCmd", "wave")
		bindCommand("WinCmd", "win")
		bindCommand("WipeCmd", "wipe")
		bindCommand("HealCmd", "heal")
	end
end)

local function triggerBurstAttack(u, enemyBaseX)
	if not u.Def.BurstHits or not u.Def.BurstDamage then return end
	if ((u.AttacksMade or 0) % (u.Def.BurstInterval or 3)) ~= 0 then return end

	local burstCount = u.Def.BurstHits or 3
	local burstDmg = (u.Def.BurstDamage or 10) * (u.StackMult or 1)
	local soundType = u.Def.SoundType or "Rifle"
	local accent = u.Def.Accent or Color3.fromRGB(90, 235, 255)

	for shotIdx = 1, burstCount do
		local delayTime = (shotIdx - 1) * 0.08
		task.delay(delayTime, function()
			if u.Dead then return end

			local candidates = {}
			for _, other in ipairs(ActiveUnits) do
				if not other.Dead and other.Side ~= u.Side then
					table.insert(candidates, other)
				end
			end

			if #candidates > 0 then
				local targetUnit = candidates[math.random(1, #candidates)]
				playSoundAt(soundType, u.Position, 0.35, 1.05 + shotIdx * 0.08)
				createBeamEffect(u.Weapon.Position, targetUnit.Position, accent, 0.28)
				createVisualEffect(targetUnit.Position, 2.6, accent, 0.20)
				damageUnit(targetUnit, burstDmg, u)
			elseif u.CachedTurret and not u.CachedTurret.Dead then
				playSoundAt(soundType, u.Position, 0.35, 1.05 + shotIdx * 0.08)
				createBeamEffect(u.Weapon.Position, u.CachedTurret.Position + Vector3.new(0, 6, 0), accent, 0.28)
				damageTurret(u.CachedTurret, burstDmg)
			else
				local targetSideKey = (u.Side == "Player") and "Enemy" or "Player"
				local targetSide = Match.Sides and Match.Sides[targetSideKey]
				if targetSide and (targetSide.BaseHP or 0) > 0 then
					playSoundAt(soundType, u.Position, 0.35, 1.05 + shotIdx * 0.08)
					createBeamEffect(u.Weapon.Position, Vector3.new(enemyBaseX, 8, CurrentArenaOriginZ), accent, 0.28)
					targetSide.BaseHP = math.max(0, targetSide.BaseHP - burstDmg)
					updateBaseVisuals(targetSideKey, targetSide.Faction, targetSide.BaseHP)
					if targetSide.BaseHP <= 0 then
						finishMatch(u.Side)
					end
				end
			end
		end)
	end
end

local lobbyPadCheckTimer = 0
RunService.Heartbeat:Connect(function(dt)
	local nowClock = os.clock()
	lobbyPadCheckTimer += dt
	if lobbyPadCheckTimer >= 0.35 then
		lobbyPadCheckTimer = 0
		local lobbyFolder = Workspace:FindFirstChild("Lobby")
		if lobbyFolder then
			local botPad = lobbyFolder:FindFirstChild("BotStepPad")
			local duelWest = lobbyFolder:FindFirstChild("DuelPadWest")
			local duelEast = lobbyFolder:FindFirstChild("DuelPadEast")

			local westDuelist = nil
			local eastDuelist = nil

			for _, plr in ipairs(Players:GetPlayers()) do
				local pArena = getPlayerArena(plr)
				local root = plr.Character and plr.Character:FindFirstChild("HumanoidRootPart")
				if root then
					local pPos = root.Position
					if pArena then
						local ox = pArena.OriginX or 0
						local oz = pArena.OriginZ or 0
						if math.abs(pPos.X - ox) > 133 or math.abs(pPos.Z - oz) > 37 or pPos.Y < -2 or pPos.Y > 48 then
							local clampedX = math.clamp(pPos.X, ox - 125, ox + 125)
							local clampedZ = math.clamp(pPos.Z, oz - 32, oz + 32)
							teleportPlayerTo(plr, Vector3.new(clampedX, 4.5, clampedZ), Vector3.new(ox, 4.5, oz))
						end
					else
						if pPos.Z < 500 and (math.abs(pPos.X) > 90 or pPos.Z < -200 or pPos.Z > -70 or pPos.Y < 5) then
							teleportPlayerTo(plr, Vector3.new(0, 11.5, -135), Vector3.new(0, 11.5, -178))
						end
						if botPad and (Vector3.new(pPos.X, botPad.Position.Y, pPos.Z) - botPad.Position).Magnitude <= 6.5 and math.abs(pPos.Y - botPad.Position.Y) <= 8 then
							if not LastPadTrigger[plr] or (nowClock - LastPadTrigger[plr] > 3.0) then
								LastPadTrigger[plr] = nowClock
								if PvPQueuePlayer == plr then
									PvPQueuePlayer = nil
								end
								LobbyActionEvent:FireClient(plr, "OpenBotDifficultyModal")
								break
							end
						end
						if duelWest and (Vector3.new(pPos.X, duelWest.Position.Y, pPos.Z) - duelWest.Position).Magnitude <= 5.0 and math.abs(pPos.Y - duelWest.Position.Y) <= 8 then
							westDuelist = plr
						elseif duelEast and (Vector3.new(pPos.X, duelEast.Position.Y, pPos.Z) - duelEast.Position).Magnitude <= 5.0 and math.abs(pPos.Y - duelEast.Position.Y) <= 8 then
							eastDuelist = plr
						end
					end
				end
			end

			if westDuelist and eastDuelist and westDuelist ~= eastDuelist then
				if PvPQueuePlayer == westDuelist or PvPQueuePlayer == eastDuelist then
					PvPQueuePlayer = nil
				end
				startMatch("PvP", westDuelist, eastDuelist, nil, nil)
			elseif westDuelist and PvPQueuePlayer ~= westDuelist then
				if PvPQueuePlayer and PvPQueuePlayer ~= westDuelist and PvPQueuePlayer.Parent == Players and not getPlayerArena(PvPQueuePlayer) then
					local opp = PvPQueuePlayer
					PvPQueuePlayer = nil
					startMatch("PvP", opp, westDuelist, nil, nil)
				else
					PvPQueuePlayer = westDuelist
					BannerNoticeEvent:FireClient(westDuelist, "⚔️ Queued on Duel Pad #1! Waiting for a challenger on Pad #2 or UI Queue...", Color3.fromRGB(90, 200, 255))
					broadcastState()
				end
			elseif eastDuelist and PvPQueuePlayer ~= eastDuelist then
				if PvPQueuePlayer and PvPQueuePlayer ~= eastDuelist and PvPQueuePlayer.Parent == Players and not getPlayerArena(PvPQueuePlayer) then
					local opp = PvPQueuePlayer
					PvPQueuePlayer = nil
					startMatch("PvP", opp, eastDuelist, nil, nil)
				else
					PvPQueuePlayer = eastDuelist
					BannerNoticeEvent:FireClient(eastDuelist, "⚔️ Queued on Duel Pad #2! Waiting for a challenger on Pad #1 or UI Queue...", Color3.fromRGB(255, 110, 145))
					broadcastState()
				end
			end
		end
	end

	local now = os.clock()
	local anyStateChanged = false
	table.clear(FrameBulkParts)
	table.clear(FrameBulkCFrames)

	for _, arena in ipairs(ArenaSlots) do
		if arena.Match.Active and not arena.Match.GameOver then
			setArenaContext(arena)
			arena.SecondAccumulator = (arena.SecondAccumulator or 0) + dt

			if arena.SecondAccumulator >= 1.0 then
				arena.SecondAccumulator -= 1.0
				Match.WaveTimer -= 1

				if Match.WaveTimer <= 0 then
					Match.WaveNumber += 1
					Match.WaveTimer = WAVE_INTERVAL

					local pGold = getSideWaveIncome("Player")
					local eGold = getSideWaveIncome("Enemy")
					Match.Sides.Player.Gold += pGold
					local botIncomeMult = 1.0
					if Match.Mode == "Bot" then
						local bDiff = Match.BotDifficulty or "Normal"
						botIncomeMult = (bDiff == "Easy" and 0.85) or (bDiff == "Hard" and 1.20) or (bDiff == "Extreme" and 1.45) or 1.0
					end
					Match.Sides.Enemy.Gold += math.floor(eGold * botIncomeMult)
					playSoundAt("WaveHorn", Vector3.new(CurrentArenaOriginX, 15, CurrentArenaOriginZ), 0.6, 1.0)

					-- Bot AI buys tier unlocks or troops using its Gold
					if Match.Mode == "Bot" then
						local botSide = Match.Sides.Enemy
						local botFData = FactionConfig.Factions[botSide.Faction]
						local diff = Match.BotDifficulty or "Normal"
						if botFData then
							local t2Wave = (diff == "Easy" and 6) or (diff == "Hard" and 3) or (diff == "Extreme" and 2) or 4
							local t3Wave = (diff == "Easy" and 12) or (diff == "Hard" and 6) or (diff == "Extreme" and 4) or 8
							if botSide.TierUnlock == 1 and Match.WaveNumber >= t2Wave and botSide.Gold >= 100 then
								botSide.Gold -= 100
								botSide.TierUnlock = 2
							elseif botSide.TierUnlock == 2 and Match.WaveNumber >= t3Wave and botSide.Gold >= 250 then
								botSide.Gold -= 250
								botSide.TierUnlock = 3
							end

							local buyAttempts = (diff == "Extreme" and 3) or (diff == "Hard" and 2) or 1
							for _ = 1, buyAttempts do
								local maxAllowedSlot = (botSide.TierUnlock == 1 and 2) or (botSide.TierUnlock == 2 and 4) or 6
								local pickSlot = math.random(1, maxAllowedSlot)
								local uId = botSide.Loadout[pickSlot] or pickSlot
								local uDef = botFData.Units[uId]
								if uDef then
									local owned = botSide.Lineup[pickSlot] or 0
									local cost = FactionConfig.GetDynamicCost(uDef, owned)
									if botSide.Gold >= cost then
										botSide.Gold -= cost
										botSide.Lineup[pickSlot] = owned + 1
									end
								end
							end
						end
					end

					spawnSideWave("Player")
					spawnSideWave("Enemy")
					for _, plr in ipairs({ Match.WestPlayer, Match.EastPlayer }) do
						if plr and plr.Parent == Players then
							local sideG = (plr == Match.EastPlayer) and eGold or pGold
							BannerNoticeEvent:FireClient(
								plr,
								string.format("WAVE %d DEPLOYED!  (+%d Wave Gold)", Match.WaveNumber, sideG),
								Color3.fromRGB(255, 220, 80)
							)
						end
					end
				end

				anyStateChanged = true
			end

			-- Turrets AI (Turret 1: 36 Range / 50 DMG; Turret 2: 28 Range / 100 DMG, activates only when Turret 1 falls)
			for _, t in ipairs(ActiveTurrets) do
				if not t.Dead then
					local isTurret2 = (t.ModelName == "Turret2" or t.Name == "Turret 2")
					local t1Alive = false
					if isTurret2 then
						for _, otherT in ipairs(ActiveTurrets) do
							if not otherT.Dead and otherT.Side == t.Side and (otherT.ModelName == "Turret1" or otherT.Name == "Turret 1") then
								t1Alive = true
								break
							end
						end
					end

					-- Turret 2 stays on standby while Turret 1 is defending!
					if not t1Alive then
						local nearestEnemy = nil
						local tMaxRange = t.Range or 32
						local nearestDist = tMaxRange
						for _, u in ipairs(ActiveUnits) do
							if not u.Dead and u.Side ~= t.Side then
								local d = (u.Position - t.Position).Magnitude
								if d < nearestDist then
									nearestDist = d
									nearestEnemy = u
								end
							end
						end

						if nearestEnemy then
							aimTurretModel(t, nearestEnemy.Position)
							if now - t.LastAttack >= TURRET_COOLDOWN then
								t.LastAttack = now
								local fData = FactionConfig.Factions[Match.Sides[t.Side].Faction]
								local beamColor = fData and fData.AccentColor or Color3.fromRGB(255, 210, 80)
								local muzzlePos = t.Muzzle and t.Muzzle.Position or (t.Position + Vector3.new(0, 7, 0))
								playSoundAt("Turret", muzzlePos, 0.55, 1.0)
								createBeamEffect(muzzlePos, nearestEnemy.Position, beamColor, 0.85)
								createVisualEffect(nearestEnemy.Position, 4.5, beamColor, 0.22)
								damageUnit(nearestEnemy, t.Damage or 50, nil)
							end
						end
					end
				end
			end

			-- Process Staggered Wave Spawns (up to 6 models per frame per arena to prevent wave spawn lag spikes!)
			local spawnedThisFrame = 0
			while #PendingWaveSpawns > 0 and spawnedThisFrame < 6 do
				local item = table.remove(PendingWaveSpawns, 1)
				local u = buildUnitModel(item.Side, item.Faction, item.Def, item.Pos, item.StackMult)
				refreshUnitCachedScan(u)
				spawnedThisFrame += 1
			end

			for i = #ActiveUnits, 1, -1 do
				local u = ActiveUnits[i]
				if u.Dead or not u.Model or not u.Model.Parent then
					table.remove(ActiveUnits, i)
				elseif u.DeathImmuneTriggered and now >= u.DeathImmuneUntil then
					u.HP = 0
					u.Dead = true
					createVisualEffect(u.Position, 3.2, u.Def.Accent, 0.22)
					if u.Model then
						u.Model:Destroy()
					end
					table.remove(ActiveUnits, i)
				else
					-- 1) Damage over Time ticks (Poison & Burning)
					if now - u.LastDoTTick >= 1.0 then
						u.LastDoTTick = now
						if now < u.PoisonUntil and u.PoisonDPS > 0 then
							if damageUnit(u, u.PoisonDPS, nil) then
								continue
							end
						end
						if now < u.BurnUntil and u.BurnDPS > 0 then
							if damageUnit(u, u.BurnDPS, nil) then
								continue
							end
						end
					end

					-- 2) Healer Ability (Priest, High Priest, Medic Bot, Repair Bot)
					if u.Def.HealAmount and u.Def.HealInterval then
						if now - u.LastHeal >= u.Def.HealInterval then
							local healedCount = 0
							local maxTargets = u.Def.HealMaxTargets or 5
							local healAmt = u.Def.HealAmount * (u.StackMult or 1)
							for _, ally in ipairs(ActiveUnits) do
								if not ally.Dead and ally.Side == u.Side and ally.HP < ally.MaxHP then
									if (ally.Position - u.Position).Magnitude <= 28 then
										ally.HP = math.min(ally.MaxHP, ally.HP + healAmt)
										updateUnitHPBar(ally)
										createBeamEffect(u.Weapon.Position, ally.Position, Color3.fromRGB(110, 255, 145), 0.32)
										healedCount += 1
										if healedCount >= maxTargets then
											break
										end
									end
								end
							end
							if healedCount > 0 then
								u.LastHeal = now
								playSoundAt("Heal", u.Position, 0.45, 1.08)
								createVisualEffect(u.Position, 3.0, Color3.fromRGB(110, 255, 145), 0.25)
							end
						end
					end

					-- 2b) Shield Bot Ability (Casts 30 shield to nearby allies within range)
					if u.Def.ShieldAmount and u.Def.ShieldInterval then
						if now - u.LastHeal >= u.Def.ShieldInterval then
							local shieldedCount = 0
							local shieldAmt = u.Def.ShieldAmount * (u.StackMult or 1)
							local radius = u.Def.ShieldRadius or u.Def.Range or 28
							for _, ally in ipairs(ActiveUnits) do
								if not ally.Dead and ally.Side == u.Side then
									if (ally.Position - u.Position).Magnitude <= radius then
										if ally.ShieldHP and ally.ShieldHP > 0 then
											-- Not stackable
										elseif ally.SkipNextShield then
											-- Shield was broken: miss this shield spawn, reset flag
											ally.SkipNextShield = false
										else
											ally.ShieldHP = shieldAmt
											ally.MaxShieldHP = shieldAmt
											updateUnitHPBar(ally)
											createBeamEffect(u.Weapon.Position, ally.Position, Color3.fromRGB(80, 220, 255), 0.35)
											createVisualEffect(ally.Position, 2.6, Color3.fromRGB(80, 220, 255), 0.25)
											shieldedCount += 1
										end
									end
								end
							end
							if shieldedCount > 0 then
								u.LastHeal = now
								playSoundAt("Heal", u.Position, 0.45, 1.25)
								createVisualEffect(u.Position, 3.5, Color3.fromRGB(70, 210, 255), 0.30)
							end
						end
					end

					-- 2c) Self-Regen Ability (Alien Juggernaut: Restores 60 HP every 4.5s)
					if u.Def.RegenHP and u.Def.RegenInterval then
						u.LastRegen = u.LastRegen or now
						if now - u.LastRegen >= u.Def.RegenInterval then
							u.LastRegen = now
							if u.HP < u.MaxHP then
								local regenAmt = u.Def.RegenHP * (u.StackMult or 1)
								u.HP = math.min(u.MaxHP, u.HP + regenAmt)
								updateUnitHPBar(u)
								playSoundAt("Heal", u.Position, 0.40, 1.15)
								createVisualEffect(u.Position, 3.2, Color3.fromRGB(130, 255, 90), 0.28)
							end
						end
					end

					-- 3) Summoner Ability (Hive Commander: Summons 2 Alien Grunts)
					if u.Def.SummonUnitId and u.Def.SummonInterval then
						if now - u.LastSummon >= u.Def.SummonInterval then
							u.LastSummon = now
							if #ActiveUnits < 42 then
								local fData = FactionConfig.Factions[u.Faction]
								local summonDef = fData and fData.Units[u.Def.SummonUnitId]
								if summonDef then
									local dir = u.Side == "Player" and 1 or -1
									playSoundAt("Spell", u.Position, 0.42, 1.15)
									local sPos = Vector3.new(
										math.clamp(u.Position.X + dir * 4, CurrentArenaOriginX - 104, CurrentArenaOriginX + 104),
										2.5,
										math.clamp(u.Position.Z, CurrentArenaOriginZ - 30, CurrentArenaOriginZ + 30)
									)
									table.insert(PendingWaveSpawns, {
										Side = u.Side,
										Faction = u.Faction,
										Def = summonDef,
										Pos = sPos,
										Dir = dir,
										StackMult = (u.Def.SummonCount or 2) * (u.StackMult or 1),
									})
								end
							end
						end
					end

					-- 4) Throttled Target, Turret, Separation & Aura Scan (every 0.15s or if target died)
					if now >= u.NextScanTime or (u.CachedTarget and u.CachedTarget.Dead) or (u.CachedTurret and u.CachedTurret.Dead) then
						u.NextScanTime = now + 0.14 + math.random() * 0.04
						refreshUnitCachedScan(u)
					end

					local bestUnitTarget = u.CachedTarget
					local bestUnitDist = (bestUnitTarget and not bestUnitTarget.Dead) and (bestUnitTarget.Position - u.Position).Magnitude or math.huge

					local bestTurretTarget = u.CachedTurret
					local bestTurretDist = (bestTurretTarget and not bestTurretTarget.Dead)
						and (Vector3.new(bestTurretTarget.Position.X, u.Position.Y, bestTurretTarget.Position.Z) - u.Position).Magnitude
						or math.huge

					local enemyBaseX = CurrentArenaOriginX + (u.Side == "Player" and 107 or -107)
					local localZ = u.Position.Z - CurrentArenaOriginZ
					local enemyBasePos = Vector3.new(enemyBaseX, u.Position.Y, CurrentArenaOriginZ + localZ * 0.35)
					local distToBase = math.abs(u.Position.X - enemyBaseX)
					local range = u.Def.Range or 6
					if u.Def.InitialRange and (u.ShotsFired or 0) < (u.Def.MaxLongRangeShots or 3) then
						range = u.Def.InitialRange
					end
					local atkSpeedMult = u.CachedAtkSpeedMult or 1.0
					local effectiveCooldown = (u.Def.AttackCooldown or 1.0) / atkSpeedMult

					if bestUnitTarget and bestUnitDist <= range then
						u.IsMoving = false
						if now - u.LastAttack >= effectiveCooldown then
							u.LastAttack = now
							local effectiveDmg = getUnitAttackDamage(u, bestUnitDist)
							u.AttacksMade = (u.AttacksMade or 0) + 1
							u.HitCount = (u.HitCount or 0) + 1
							if u.Def.InitialRange and (u.ShotsFired or 0) < (u.Def.MaxLongRangeShots or 3) then
								u.ShotsFired = (u.ShotsFired or 0) + 1
							end
							playSoundAt(u.Def.SoundType or "Sword", u.Position, 0.36, 1.0)
							if range > 8 then
								createBeamEffect(u.Weapon.Position, bestUnitTarget.Position, u.Def.Accent, 0.40)
							end
							if u.Def.FourthHitDamage and ((u.HitCount or 0) % 4 == 0) then
								createVisualEffect(bestUnitTarget.Position, 3.2, Color3.fromRGB(160, 90, 255), 0.25)
							elseif u.Def.ThirdHitDamage and ((u.HitCount or 0) % 3 == 0) then
								createVisualEffect(bestUnitTarget.Position, 3.5, u.Def.Accent or Color3.fromRGB(80, 230, 255), 0.25)
							elseif u.Def.FirstStrikeDamage and u.AttacksMade == 1 then
								createVisualEffect(bestUnitTarget.Position, 4.0, Color3.fromRGB(255, 150, 40), 0.35)
							end
							applyOnHitEffects(u, bestUnitTarget, now)
							damageUnit(bestUnitTarget, effectiveDmg, u)
							triggerBurstAttack(u, enemyBaseX)
						end
						updateUnitPose(u, dt, now, bestUnitTarget.Position, FrameBulkParts, FrameBulkCFrames)

					elseif bestTurretTarget and bestTurretDist <= math.max(range, 7.5) then
						u.IsMoving = false
						local turretAimPos = Vector3.new(bestTurretTarget.Position.X, u.Position.Y, bestTurretTarget.Position.Z)
						if now - u.LastAttack >= effectiveCooldown then
							u.LastAttack = now
							local effectiveDmg = getUnitAttackDamage(u, bestTurretDist)
							u.AttacksMade = (u.AttacksMade or 0) + 1
							u.HitCount = (u.HitCount or 0) + 1
							if u.Def.InitialRange and (u.ShotsFired or 0) < (u.Def.MaxLongRangeShots or 3) then
								u.ShotsFired = (u.ShotsFired or 0) + 1
							end
							playSoundAt(u.Def.SoundType or "Sword", u.Position, 0.36, 1.0)
							createBeamEffect(u.Weapon.Position, bestTurretTarget.Position + Vector3.new(0, 6, 0), u.Def.Accent, 0.42)
							damageTurret(bestTurretTarget, effectiveDmg)
							triggerBurstAttack(u, enemyBaseX)
						end
						updateUnitPose(u, dt, now, turretAimPos, FrameBulkParts, FrameBulkCFrames)

					elseif distToBase <= math.max(range, 8.5) then
						u.IsMoving = false
						if now - u.LastAttack >= effectiveCooldown then
							u.LastAttack = now
							local effectiveDmg = getUnitAttackDamage(u, distToBase)
							u.AttacksMade = (u.AttacksMade or 0) + 1
							u.HitCount = (u.HitCount or 0) + 1
							if u.Def.InitialRange and (u.ShotsFired or 0) < (u.Def.MaxLongRangeShots or 3) then
								u.ShotsFired = (u.ShotsFired or 0) + 1
							end
							playSoundAt(u.Def.SoundType or "Sword", u.Position, 0.4, 0.95)
							createBeamEffect(u.Weapon.Position, Vector3.new(enemyBaseX, 8, CurrentArenaOriginZ), u.Def.Accent, 0.45)
							local targetSideKey = (u.Side == "Player") and "Enemy" or "Player"
							local targetSide = Match.Sides[targetSideKey]
							targetSide.BaseHP = math.max(0, targetSide.BaseHP - effectiveDmg)
							updateBaseVisuals(targetSideKey, targetSide.Faction, targetSide.BaseHP)
							if targetSide.BaseHP <= 0 then
								finishMatch(u.Side)
							end
							triggerBurstAttack(u, enemyBaseX)
						end
						updateUnitPose(u, dt, now, enemyBasePos, FrameBulkParts, FrameBulkCFrames)

					else
						u.IsMoving = true
						local moveTarget = enemyBasePos
						if bestUnitTarget and bestUnitDist < math.max(46, range + 8) then
							moveTarget = bestUnitTarget.Position
						elseif bestTurretTarget and bestTurretDist < math.max(38, range + 6) then
							moveTarget = Vector3.new(bestTurretTarget.Position.X, u.Position.Y, bestTurretTarget.Position.Z)
						end

						local seekVec = Vector3.new(moveTarget.X - u.Position.X, 0, moveTarget.Z - u.Position.Z)
						if seekVec.Magnitude > 0.05 then
							seekVec = seekVec.Unit
						end

						local combinedDir = (seekVec + (u.CachedSepVec or Vector3.zero) * 0.85)
						if combinedDir.Magnitude > 0.05 then
							combinedDir = combinedDir.Unit
							local slowMult = (now < u.SlowUntil) and (u.SlowFactor or 0.6) or 1.0
							local boostMult = (now < u.SpeedBoostUntil) and 1.45 or 1.0
							local speed = (u.Def.Speed or 12) * slowMult * boostMult
							local nextPos = u.Position + combinedDir * (speed * dt)
							nextPos = Vector3.new(math.clamp(nextPos.X, CurrentArenaOriginX - 108, CurrentArenaOriginX + 108), u.GroundY, math.clamp(nextPos.Z, CurrentArenaOriginZ - 33, CurrentArenaOriginZ + 33))
							u.Position = nextPos
							updateUnitPose(u, dt, now, u.Position + combinedDir * 5, FrameBulkParts, FrameBulkCFrames)
						end
					end
				end
			end
		end
	end

	if anyStateChanged then
		broadcastState()
	end

	if #FrameBulkParts > 0 then
		Workspace:BulkMoveTo(FrameBulkParts, FrameBulkCFrames, Enum.BulkMoveMode.FireCFrameChanged)
	end
end)
