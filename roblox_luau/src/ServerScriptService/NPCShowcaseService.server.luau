local Workspace = game:GetService("Workspace")
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Debris = game:GetService("Debris")

local FactionConfig = require(ReplicatedStorage:WaitForChild("FactionConfig"))

local LOBBY_SPAWN_POS = Vector3.new(0, 11.5, -135)
local SHOWCASE_SPAWN_POS = Vector3.new(0, 303, 1390)

-- 1. Setup Lobby to Showcase Teleport Pad
local function setupLobbyPad()
	local lobby = Workspace:WaitForChild("Lobby", 5)
	if not lobby then return end
	local portalFolder = lobby:WaitForChild("LobbyToShowcasePortal", 5)
	if not portalFolder then return end
	local pad = portalFolder:WaitForChild("ShowcasePad", 5)
	if not pad then return end

	pad.Touched:Connect(function(hit)
		local char = hit.Parent
		local hrp = char and char:FindFirstChild("HumanoidRootPart")
		if hrp and not hrp:GetAttribute("TeleportCooldown") then
			hrp:SetAttribute("TeleportCooldown", true)
			hrp.CFrame = CFrame.new(SHOWCASE_SPAWN_POS) + Vector3.new(0, 3, 0)
			task.delay(1.5, function()
				if hrp then hrp:SetAttribute("TeleportCooldown", nil) end
			end)
		end
	end)
end

-- 2. Setup Showcase to Lobby Return Pad
local function setupReturnPad()
	local room = Workspace:WaitForChild("NPCShowcaseRoom", 5)
	if not room then return end
	local portals = room:WaitForChild("Portals", 5)
	if not portals then return end
	local pad = portals:WaitForChild("ReturnToLobbyPad", 5)
	if not pad then return end

	pad.Touched:Connect(function(hit)
		local char = hit.Parent
		local hrp = char and char:FindFirstChild("HumanoidRootPart")
		if hrp and not hrp:GetAttribute("TeleportCooldown") then
			hrp:SetAttribute("TeleportCooldown", true)
			hrp.CFrame = CFrame.new(LOBBY_SPAWN_POS) + Vector3.new(0, 3, 0)
			task.delay(1.5, function()
				if hrp then hrp:SetAttribute("TeleportCooldown", nil) end
			end)
		end
	end)
end

-- 3. Connect All Proximity Prompts in Showcase Room
local function setupPrompts()
	local room = Workspace:WaitForChild("NPCShowcaseRoom", 5)
	if not room then return end

	for _, desc in ipairs(room:GetDescendants()) do
		if desc:IsA("ProximityPrompt") then
			desc.Triggered:Connect(function(player)
				local ped = desc.Parent
				if not ped then return end

				-- Determine unit sound from billboard or name
				local bb = ped:FindFirstChild("InfoBillboard")
				local frame = bb and bb:FindFirstChildWhichIsA("Frame")
				local roleLbl = frame and frame:FindFirstChild("TextLabel")
				local uName = desc.ObjectText or "Unit"

				-- Find sound in FactionConfig
				local sfxType = "Sword"
				for fName, fData in pairs(FactionConfig.Factions) do
					for _, u in ipairs(fData.Units) do
						if u.Name == uName then
							sfxType = u.SoundType or "Sword"
							break
						end
					end
				end

				local soundId = FactionConfig.SFX[sfxType]
				if soundId then
					local snd = Instance.new("Sound")
					snd.SoundId = soundId
					snd.Volume = 0.95
					snd.Parent = ped
					snd:Play()
					Debris:AddItem(snd, 3)
				end

				-- Floating Action Text Popup
				local pop = Instance.new("BillboardGui")
				pop.Size = UDim2.new(4.5, 0, 1.6, 0)
				pop.StudsOffset = Vector3.new(0, 7.8, 0)
				pop.AlwaysOnTop = true
				pop.Adornee = ped
				pop.Parent = ped

				local popLbl = Instance.new("TextLabel")
				popLbl.Size = UDim2.new(1, 0, 1, 0)
				popLbl.BackgroundTransparency = 1
				local actionText = (sfxType == "Heal" and "💚 +HEAL RESTORED!") 
					or (sfxType == "Bow" and "🏹 ARROW FIRED!") 
					or (sfxType == "Rifle" and "💥 GUNFIRE BURST!") 
					or (sfxType == "Laser" and "⚡ PLASMA BEAM!") 
					or (sfxType == "Explosion" and "💣 ARCANE BLAST!") 
					or "⚔️ ATTACK STRIKE!"
				popLbl.Text = actionText
				popLbl.TextColor3 = (sfxType == "Heal" and Color3.fromRGB(80, 255, 120)) or Color3.fromRGB(255, 230, 80)
				popLbl.TextScaled = true
				popLbl.Font = Enum.Font.GothamBold
				popLbl.Parent = pop

				Debris:AddItem(pop, 1.8)
			end)
		end
	end
end

task.spawn(setupLobbyPad)
task.spawn(setupReturnPad)
task.spawn(setupPrompts)
