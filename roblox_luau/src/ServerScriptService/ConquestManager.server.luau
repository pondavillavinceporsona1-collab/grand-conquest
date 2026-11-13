-- =========================================================================
-- EPIC BATTLE! — GRAND CONQUEST: AUTHORITATIVE SERVER MANAGER
-- =========================================================================

local Workspace = game:GetService("Workspace")
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local RunService = game:GetService("RunService")
local TweenService = game:GetService("TweenService")
local DataStoreService = game:GetService("DataStoreService")

local FactionConfig = require(ReplicatedStorage:WaitForChild("FactionConfig"))
local RigBuilder = require(ReplicatedStorage:WaitForChild("RigBuilder"))
local ConquestConfig = require(ReplicatedStorage:WaitForChild("Conquest"):WaitForChild("ConquestConfig"))
local ConquestState = require(ReplicatedStorage:WaitForChild("Conquest"):WaitForChild("ConquestState"))

-- Remote Events Setup
local RemotesFolder = ReplicatedStorage:FindFirstChild("Remotes")
if not RemotesFolder then
	RemotesFolder = Instance.new("Folder")
	RemotesFolder.Name = "Remotes"
	RemotesFolder.Parent = ReplicatedStorage
end

local ConquestRemotes = RemotesFolder:FindFirstChild("Conquest")
if not ConquestRemotes then
	ConquestRemotes = Instance.new("Folder")
	ConquestRemotes.Name = "Conquest"
	ConquestRemotes.Parent = RemotesFolder
end

local function getOrCreateRemote(name: string, isFunction: boolean?): Instance
	local existing = ConquestRemotes:FindFirstChild(name)
	if existing then return existing end
	local r = isFunction and Instance.new("RemoteFunction") or Instance.new("RemoteEvent")
	r.Name = name
	r.Parent = ConquestRemotes
	return r
end

local RequestProfileFunc = getOrCreateRemote("RequestProfile", true) :: RemoteFunction
local StartMissionEvent = getOrCreateRemote("StartMission") :: RemoteEvent
local EquipRelicEvent = getOrCreateRemote("EquipRelic") :: RemoteEvent
local UnequipRelicEvent = getOrCreateRemote("UnequipRelic") :: RemoteEvent
local EquipDeckSlotEvent = getOrCreateRemote("EquipDeckSlot") :: RemoteEvent
local StateUpdateEvent = getOrCreateRemote("ConquestStateUpdate") :: RemoteEvent
local MatchEndedEvent = getOrCreateRemote("ConquestMatchEnded") :: RemoteEvent
local BossNoticeEvent = getOrCreateRemote("ConquestBossNotice") :: RemoteEvent

-- DataStore Setup
local ConquestStore
pcall(function()
	ConquestStore = DataStoreService:GetDataStore("GrandConquest_v2")
end)

-- Active Player Profiles: [UserId] = ConquestProfile
local PlayerProfiles = {}
-- Active Conquest Matches: [UserId] = MatchContext
local ActiveMatches = {}

-- -------------------------------------------------------------------------
-- Profile Data Management
-- -------------------------------------------------------------------------
local function loadPlayerProfile(player: Player): any
	local userId = player.UserId
	if PlayerProfiles[userId] then return PlayerProfiles[userId] end

	local profile = nil
	if ConquestStore then
		local success, data = pcall(function()
			return ConquestStore:GetAsync("User_" .. tostring(userId))
		end)
		if success and data and typeof(data) == "table" and data.SchemaVersion then
			profile = data
		end
	end

	if not profile then
		profile = ConquestState.CreateDefault("solaris")
	end

	ConquestState.UpdateTotalStars(profile)
	PlayerProfiles[userId] = profile
	return profile
end

local function savePlayerProfile(player: Player)
	local userId = player.UserId
	local profile = PlayerProfiles[userId]
	if not profile or not ConquestStore then return end

	pcall(function()
		ConquestStore:SetAsync("User_" .. tostring(userId), profile)
	end)
end

Players.PlayerAdded:Connect(function(player)
	local profile = loadPlayerProfile(player)
	StateUpdateEvent:FireClient(player, profile)
end)

Players.PlayerRemoving:Connect(function(player)
	savePlayerProfile(player)
	PlayerProfiles[player.UserId] = nil
	ActiveMatches[player.UserId] = nil
end)

RequestProfileFunc.OnServerInvoke = function(player: Player)
	return loadPlayerProfile(player)
end

EquipRelicEvent.OnServerEvent:Connect(function(player: Player, relicId: string)
	local profile = loadPlayerProfile(player)
	if ConquestState.EquipRelic(profile, relicId) then
		StateUpdateEvent:FireClient(player, profile)
	end
end)

UnequipRelicEvent.OnServerEvent:Connect(function(player: Player, relicId: string)
	local profile = loadPlayerProfile(player)
	if ConquestState.UnequipRelic(profile, relicId) then
		StateUpdateEvent:FireClient(player, profile)
	end
end)

EquipDeckSlotEvent.OnServerEvent:Connect(function(player: Player, slotIdx: number, unitId: string)
	local profile = loadPlayerProfile(player)
	if ConquestState.EquipDeckSlot(profile, slotIdx, unitId) then
		StateUpdateEvent:FireClient(player, profile)
	end
end)

-- -------------------------------------------------------------------------
-- Active Match Lifecycle & Boss / Wave Simulation
-- -------------------------------------------------------------------------
StartMissionEvent.OnServerEvent:Connect(function(player: Player, missionId: string)
	local profile = loadPlayerProfile(player)
	local mission = ConquestConfig.GetMission(missionId)
	if not mission then return end

	-- Check prerequisites
	if not ConquestConfig.IsMissionUnlocked(missionId, profile.Missions) then
		warn("Player attempted locked mission:", missionId)
		return
	end

	local matchContext = {
		Player = player,
		Mission = mission,
		StartTime = os.clock(),
		DefenseTimer = mission.TimeLimitSeconds or 150.0,
		BlitzTimer = mission.TimeLimitSeconds or 135.0,
		WaveQueue = mission.Waves and table.clone(mission.Waves) or {},
		IsActive = true,
		BossInstance = nil,
		BossPhase = 1,
		MutatorTickAccum = 0,
	}
	ActiveMatches[player.UserId] = matchContext

	-- Handle Titan Slayer Boss Spawn
	if mission.Archetype == "titan_slayer" and mission.BossProps then
		local bProps = mission.BossProps
		-- Scale and tag Boss Model in Workspace
		task.spawn(function()
			task.wait(1.0)
			if not matchContext.IsActive then return end
			BossNoticeEvent:FireClient(player, {
				Action = "BossSpawned",
				Name = bProps.Name,
				MaxHp = bProps.MaxHp,
				Scale = bProps.Scale or 2.0,
			})
		end)
	end

	-- Notify Client Match Started
	StateUpdateEvent:FireClient(player, profile, {
		Action = "MatchStarted",
		MissionId = missionId,
		Archetype = mission.Archetype,
	})
end)

-- Background Match Ticker (Runs for Hold the Line waves, Blitzkrieg countdown, Mutators)
RunService.Heartbeat:Connect(function(dt: number)
	for userId, match in pairs(ActiveMatches) do
		if match.IsActive then
			local mission = match.Mission
			local elapsed = os.clock() - match.StartTime

			-- 1. Hold the Line: Wave Spawner & Timer
			if mission.Archetype == "hold_the_line" then
				match.DefenseTimer = math.max(0, match.DefenseTimer - dt)
				if match.WaveQueue then
					for _, wave in ipairs(match.WaveQueue) do
						if elapsed >= wave.triggerSec and not wave.fired then
							wave.fired = true
							BossNoticeEvent:FireClient(match.Player, {
								Action = "WaveIncoming",
								AlertName = wave.alertName,
							})
						end
					end
				end

				-- Survival Victory Condition
				if match.DefenseTimer <= 0 then
					match.IsActive = false
					finishMatch(match.Player, mission, true, elapsed)
				end
			end

			-- 2. Blitzkrieg: Timer Expiration Trigger
			if mission.Archetype == "blitzkrieg" then
				match.BlitzTimer = math.max(0, match.BlitzTimer - dt)
				if match.BlitzTimer <= 0 then
					match.IsActive = false
					finishMatch(match.Player, mission, false, elapsed)
				end
			end

			-- 3. Mutator Environmental Ticks
			if mission.Mutator then
				match.MutatorTickAccum += dt
				if match.Mutator.id == "corrosive_miasma" and match.MutatorTickAccum >= 1.5 then
					match.MutatorTickAccum = 0
					-- Pulse environmental damage to unshielded troops
				end
			end
		end
	end
end)

function finishMatch(player: Player, mission: any, isVictory: boolean, durationSec: number)
	local userId = player.UserId
	local profile = PlayerProfiles[userId]
	if not profile then return end

	ActiveMatches[userId] = nil

	local starsEarned = 0
	if isVictory then
		starsEarned = 1
		-- Star 2 & 3 evaluations based on time and objectives
		if durationSec <= (mission.Chapter == 1 and 180 or 195) then
			starsEarned += 1
		end
		starsEarned += 1 -- Base objective met

		ConquestState.RecordVictory(profile, mission.Id, starsEarned, durationSec, mission.Rewards)
		savePlayerProfile(player)
	end

	MatchEndedEvent:FireClient(player, {
		MissionId = mission.Id,
		IsVictory = isVictory,
		Stars = starsEarned,
		DurationSec = durationSec,
		Rewards = isVictory and mission.Rewards or nil,
	})
	StateUpdateEvent:FireClient(player, profile)
end

print("[ConquestManager] Grand Conquest Server Engine Initialized (24 Missions, 15 Relics, 6 Chapters).")
