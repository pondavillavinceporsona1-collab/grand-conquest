-- =========================================================================
-- EPIC BATTLE! — GRAND CONQUEST: STATE ENGINE & PROFILE MANAGER
-- =========================================================================

local ReplicatedStorage = game:GetService("ReplicatedStorage")
local ConquestConfig = require(ReplicatedStorage:WaitForChild("Conquest"):WaitForChild("ConquestConfig"))

local ConquestState = {}

local CHECKSUM_SALT = "GEO_TERRA_LUAU_SALT_v2"
local MAX_RELIC_POWER = 3

export type MissionRecord = {
	Stars: number,
	BestTimeSeconds: number,
	CompletedAtIso: string,
}

export type ConquestProfile = {
	SchemaVersion: number,
	PlayerCommanderFaction: string,
	AccountGoldVault: number,
	TotalConquestStars: number,
	Missions: { [string]: MissionRecord },
	UnlockedBlueprintIds: { string },
	EquippedDeckUnitIds: { string },
	EquippedRelicIds: { string },
	UnlockedRelicIds: { string },
	Checksum: string?,
}

function ConquestState.CreateDefault(starterFaction: string?): ConquestProfile
	local faction = starterFaction or "solaris"
	local defaultStarters = {
		solaris = { "sol_1", "sol_2", "sol_3" },
		aegis = { "aeg_1", "aeg_2", "aeg_3" },
		hive = { "hiv_1", "hiv_2", "hiv_3" },
		hellfire = { "hel_1", "hel_2", "hel_3" },
		void = { "voi_1", "voi_2", "voi_3" },
		mechanica = { "mec_1", "mec_2", "mec_3" },
	}
	local starters = defaultStarters[faction] or defaultStarters.solaris

	local profile: ConquestProfile = {
		SchemaVersion = 2,
		PlayerCommanderFaction = faction,
		AccountGoldVault = 0,
		TotalConquestStars = 0,
		Missions = {},
		UnlockedBlueprintIds = { starters[1], starters[2], starters[3] },
		EquippedDeckUnitIds = { starters[1], starters[2], starters[3], starters[1], starters[2], starters[3] },
		EquippedRelicIds = {},
		UnlockedRelicIds = {},
	}

	ConquestState.UpdateTotalStars(profile)
	return profile
end

function ConquestState.UpdateTotalStars(profile: ConquestProfile): number
	local total = 0
	if profile.Missions then
		for _, rec in pairs(profile.Missions) do
			if rec and typeof(rec.Stars) == "number" then
				total += rec.Stars
			end
		end
	end
	profile.TotalConquestStars = total

	-- Check auto-unlock of Relics based on Star thresholds
	for _, relic in pairs(ConquestConfig.Relics) do
		if total >= relic.StarsReq and not table.find(profile.UnlockedRelicIds, relic.Id) then
			table.insert(profile.UnlockedRelicIds, relic.Id)
		end
	end

	return total
end

function ConquestState.GetUsedRelicPower(profile: ConquestProfile): number
	local used = 0
	for _, rId in ipairs(profile.EquippedRelicIds or {}) do
		local rDef = ConquestConfig.GetRelic(rId)
		if rDef then
			used += (rDef.RPCost or 1)
		end
	end
	return used
end

function ConquestState.CanEquipRelic(profile: ConquestProfile, relicId: string): (boolean, string?)
	local rDef = ConquestConfig.GetRelic(relicId)
	if not rDef then
		return false, "Invalid relic ID"
	end
	if not table.find(profile.UnlockedRelicIds, relicId) and profile.TotalConquestStars < rDef.StarsReq then
		return false, "Relic is locked (requires " .. tostring(rDef.StarsReq) .. " stars)"
	end
	if table.find(profile.EquippedRelicIds, relicId) then
		return false, "Relic already equipped"
	end
	local currentUsed = ConquestState.GetUsedRelicPower(profile)
	if currentUsed + (rDef.RPCost or 1) > MAX_RELIC_POWER then
		return false, "Exceeds Relic Power capacity (Max " .. tostring(MAX_RELIC_POWER) .. " RP)"
	end
	return true, nil
end

function ConquestState.EquipRelic(profile: ConquestProfile, relicId: string): boolean
	local ok, _ = ConquestState.CanEquipRelic(profile, relicId)
	if ok then
		table.insert(profile.EquippedRelicIds, relicId)
		return true
	end
	return false
end

function ConquestState.UnequipRelic(profile: ConquestProfile, relicId: string): boolean
	local idx = table.find(profile.EquippedRelicIds, relicId)
	if idx then
		table.remove(profile.EquippedRelicIds, idx)
		return true
	end
	return false
end

function ConquestState.UnlockBlueprint(profile: ConquestProfile, unitId: string): boolean
	if not table.find(profile.UnlockedBlueprintIds, unitId) then
		table.insert(profile.UnlockedBlueprintIds, unitId)
		return true
	end
	return false
end

function ConquestState.EquipDeckSlot(profile: ConquestProfile, slotIdx: number, unitId: string): boolean
	if slotIdx < 1 or slotIdx > 6 then return false end
	if not table.find(profile.UnlockedBlueprintIds, unitId) then return false end
	profile.EquippedDeckUnitIds[slotIdx] = unitId
	return true
end

function ConquestState.RecordVictory(profile: ConquestProfile, missionId: string, starsEarned: number, durationSec: number, rewards: any): boolean
	local mission = ConquestConfig.GetMission(missionId)
	if not mission then return false end

	local existing = profile.Missions[missionId]
	if not existing or starsEarned > existing.Stars then
		profile.Missions[missionId] = {
			Stars = math.clamp(starsEarned, 1, 3),
			BestTimeSeconds = durationSec,
			CompletedAtIso = os.date("!%Y-%m-%dT%H:%M:%SZ"),
		}
	end

	-- Add Vault Bounty
	if rewards and rewards.goldBounty then
		profile.AccountGoldVault = (profile.AccountGoldVault or 0) + rewards.goldBounty
	end

	-- Unlock Blueprints
	if rewards and rewards.blueprints then
		for _, bId in ipairs(rewards.blueprints) do
			ConquestState.UnlockBlueprint(profile, bId)
		end
	end

	ConquestState.UpdateTotalStars(profile)
	return true
end

return ConquestState
