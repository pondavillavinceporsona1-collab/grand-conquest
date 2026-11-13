local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local UserInputService = game:GetService("UserInputService")
local TweenService = game:GetService("TweenService")
local SoundService = game:GetService("SoundService")
local RunService = game:GetService("RunService")

local player = Players.LocalPlayer
local playerGui = player:WaitForChild("PlayerGui")

local FactionConfig = require(ReplicatedStorage:WaitForChild("FactionConfig"))
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

local isAdmin = RunService:IsStudio() or (game.CreatorType == Enum.CreatorType.User and player.UserId == game.CreatorId) or player.UserId == 194993165
local adminPillBtn = nil
local adminModal = nil
local topBar = nil
local bottomDeck = nil

for _, old in ipairs(playerGui:GetChildren()) do
	if old.Name == "BigBattleHUD" then
		old:Destroy()
	end
end

local screenGui = Instance.new("ScreenGui")
screenGui.Name = "BigBattleHUD"
screenGui.ResetOnSpawn = false
screenGui.IgnoreGuiInset = false
screenGui.DisplayOrder = 100
screenGui.ZIndexBehavior = Enum.ZIndexBehavior.Sibling
screenGui.Parent = playerGui

local uiClickSound = Instance.new("Sound")
uiClickSound.SoundId = FactionConfig.SFX.Click
uiClickSound.Volume = 0.42
uiClickSound.Parent = SoundService

local function playUIClick(pitch)
	uiClickSound.PlaybackSpeed = pitch or 1.0
	uiClickSound:Play()
end

local localUpgradeSound = Instance.new("Sound")
localUpgradeSound.Volume = 0.48
localUpgradeSound.Parent = SoundService

local function playLocalSFX(soundType, pitch)
	local soundId = FactionConfig.SFX[soundType]
	if not soundId then return end
	localUpgradeSound.SoundId = soundId
	localUpgradeSound.PlaybackSpeed = pitch or 1.15
	localUpgradeSound:Play()
end

local function addCorner(parent, radius)
	local c = Instance.new("UICorner")
	c.CornerRadius = UDim.new(0, radius or 8)
	c.Parent = parent
	return c
end

local function addStroke(parent, color, thickness, transparency)
	local s = Instance.new("UIStroke")
	s.Color = color or Color3.fromRGB(90, 110, 145)
	s.Thickness = thickness or 1.5
	s.Transparency = transparency or 0
	s.ApplyStrokeMode = Enum.ApplyStrokeMode.Border
	s.Parent = parent
	return s
end

local function addGradient(parent, c1, c2, rot)
	local g = Instance.new("UIGradient")
	g.Color = ColorSequence.new(c1, c2)
	g.Rotation = rot or 90
	g.Parent = parent
	return g
end

local function addPadding(parent, top, bottom, left, right)
	local p = Instance.new("UIPadding")
	p.PaddingTop = UDim.new(0, top or 4)
	p.PaddingBottom = UDim.new(0, bottom or 4)
	p.PaddingLeft = UDim.new(0, left or 6)
	p.PaddingRight = UDim.new(0, right or 6)
	p.Parent = parent
	return p
end

local function addTextConstraint(label, minSize, maxSize)
	label.TextScaled = true
	local tc = Instance.new("UITextSizeConstraint")
	tc.MinTextSize = minSize or 8
	tc.MaxTextSize = maxSize or 16
	tc.Parent = label
	return tc
end

local TIER_COLORS = {
	C = Color3.fromRGB(85, 225, 135),
	B = Color3.fromRGB(80, 185, 255),
	A = Color3.fromRGB(255, 200, 65),
}

local TIER_BG_COLORS = {
	C = Color3.fromRGB(18, 36, 30),
	B = Color3.fromRGB(18, 30, 48),
	A = Color3.fromRGB(40, 32, 18),
}

local mySelectedFaction = "Kingdom"
local myLoadout = { 1, 2, 4, 5, 7, 8 } -- 2 C-tier, 2 B-tier, 2 A-tier (6 cards!)
local botSelectedFaction = "Republic"
local botSelectedDifficulty = "Normal"
local isInMatch = false
local myArenaOriginX = 0
local myArenaOriginZ = 0
local myArenaSlotId = 1
local myArenaName = "👑 SOLARIS EMPIRE ARENA"

-- FANTASY WAR DECK UI (LEFT-SIDE CREST & 6-CARD OVERVIEW)
local openLobbyBtn = Instance.new("TextButton")
openLobbyBtn.Name = "OpenLobbyBtn"
openLobbyBtn.AnchorPoint = Vector2.new(0, 0.5)
openLobbyBtn.Size = UDim2.new(0, 214, 0, 276)
openLobbyBtn.Position = UDim2.new(0, 16, 0.5, 0)
openLobbyBtn.BackgroundColor3 = Color3.fromRGB(15, 12, 19)
openLobbyBtn.BackgroundTransparency = 0.12
openLobbyBtn.Text = ""
openLobbyBtn.AutoButtonColor = false
openLobbyBtn.Visible = true
openLobbyBtn.Parent = screenGui
addCorner(openLobbyBtn, 12)
local sideDeckStroke = addStroke(openLobbyBtn, Color3.fromRGB(225, 185, 75), 1.8)
addGradient(openLobbyBtn, Color3.fromRGB(28, 22, 34), Color3.fromRGB(12, 10, 16), 90)

local sideDeckConstraint = Instance.new("UISizeConstraint")
sideDeckConstraint.MaxSize = Vector2.new(230, 296)
sideDeckConstraint.MinSize = Vector2.new(185, 238)
sideDeckConstraint.Parent = openLobbyBtn

-- 1. Fantasy Header Banner
local sideHeader = Instance.new("Frame")
sideHeader.Name = "SideHeader"
sideHeader.Size = UDim2.new(1, 0, 0, 32)
sideHeader.Position = UDim2.new(0, 0, 0, 0)
sideHeader.BackgroundColor3 = Color3.fromRGB(36, 22, 28)
sideHeader.BackgroundTransparency = 0.1
sideHeader.Parent = openLobbyBtn
addCorner(sideHeader, 11)

local sideTitleLbl = Instance.new("TextLabel")
sideTitleLbl.Size = UDim2.new(0.70, 0, 1, 0)
sideTitleLbl.Position = UDim2.new(0, 10, 0, 0)
sideTitleLbl.BackgroundTransparency = 1
sideTitleLbl.Text = "⚜️ WAR DECK"
sideTitleLbl.TextColor3 = Color3.fromRGB(255, 225, 100)
sideTitleLbl.Font = Enum.Font.Fondamento
sideTitleLbl.TextXAlignment = Enum.TextXAlignment.Left
sideTitleLbl.Parent = sideHeader
addTextConstraint(sideTitleLbl, 12, 16)

local sideHotkeyPill = Instance.new("TextLabel")
sideHotkeyPill.AnchorPoint = Vector2.new(1, 0.5)
sideHotkeyPill.Size = UDim2.new(0, 42, 0, 20)
sideHotkeyPill.Position = UDim2.new(1, -8, 0.5, 0)
sideHotkeyPill.BackgroundColor3 = Color3.fromRGB(24, 16, 12)
sideHotkeyPill.Text = "[ M ]"
sideHotkeyPill.TextColor3 = Color3.fromRGB(255, 215, 110)
sideHotkeyPill.Font = Enum.Font.GothamBold
sideHotkeyPill.Parent = sideHeader
addCorner(sideHotkeyPill, 5)
addStroke(sideHotkeyPill, Color3.fromRGB(180, 140, 50), 1.0)
addTextConstraint(sideHotkeyPill, 8, 11)

local sideGoldDivider = Instance.new("Frame")
sideGoldDivider.Size = UDim2.new(1, -16, 0, 1)
sideGoldDivider.Position = UDim2.new(0, 8, 1, -1)
sideGoldDivider.BackgroundColor3 = Color3.fromRGB(205, 165, 65)
sideGoldDivider.BorderSizePixel = 0
sideGoldDivider.Parent = sideHeader

-- 2. Active Faction Banner
local sideFactionBanner = Instance.new("Frame")
sideFactionBanner.Name = "FactionBanner"
sideFactionBanner.Size = UDim2.new(1, -14, 0, 26)
sideFactionBanner.Position = UDim2.new(0, 7, 0, 36)
sideFactionBanner.BackgroundColor3 = Color3.fromRGB(22, 18, 28)
sideFactionBanner.Parent = openLobbyBtn
addCorner(sideFactionBanner, 6)
local sideFactionStroke = addStroke(sideFactionBanner, Color3.fromRGB(215, 175, 70), 1.2)

local sideFactionText = Instance.new("TextLabel")
sideFactionText.Size = UDim2.new(1, -10, 1, 0)
sideFactionText.Position = UDim2.new(0, 5, 0, 0)
sideFactionText.BackgroundTransparency = 1
sideFactionText.Text = "👑 KINGDOM LEGION"
sideFactionText.TextColor3 = Color3.new(1, 1, 1)
sideFactionText.Font = Enum.Font.GothamBold
sideFactionText.Parent = sideFactionBanner
addTextConstraint(sideFactionText, 9, 12)

-- 3. Equipped 6-Card Mini Grid (3 rows of 2 cards: 2 C, 2 B, 2 A)
local sideCardsHolder = Instance.new("Frame")
sideCardsHolder.Name = "SideCardsHolder"
sideCardsHolder.Size = UDim2.new(1, -14, 0, 160)
sideCardsHolder.Position = UDim2.new(0, 7, 0, 68)
sideCardsHolder.BackgroundTransparency = 1
sideCardsHolder.Parent = openLobbyBtn

local sideCardWidgets = {}
for slot = 1, 6 do
	local col = (slot - 1) % 2
	local row = math.floor((slot - 1) / 2)
	local tierKey = (slot <= 2) and "C" or ((slot <= 4) and "B" or "A")
	local tColor = TIER_COLORS[tierKey] or Color3.fromRGB(200, 200, 200)

	local cardF = Instance.new("Frame")
	cardF.Name = "Slot_" .. slot
	cardF.Size = UDim2.new(0.485, 0, 0.305, 0)
	cardF.Position = UDim2.new(col * 0.515, 0, row * 0.345, 0)
	cardF.BackgroundColor3 = Color3.fromRGB(18, 14, 24)
	cardF.Parent = sideCardsHolder
	addCorner(cardF, 6)
	local cStroke = addStroke(cardF, tColor, 1.2, 0.3)

	local tierPill = Instance.new("TextLabel")
	tierPill.Size = UDim2.new(0.38, 0, 0.38, 0)
	tierPill.Position = UDim2.new(0, 3, 0.06, 0)
	tierPill.BackgroundColor3 = tColor
	tierPill.Text = tierKey
	tierPill.TextColor3 = Color3.fromRGB(12, 10, 16)
	tierPill.Font = Enum.Font.GothamBlack
	tierPill.Parent = cardF
	addCorner(tierPill, 4)
	addTextConstraint(tierPill, 7, 9)

	local costLbl = Instance.new("TextLabel")
	costLbl.Size = UDim2.new(0.56, 0, 0.38, 0)
	costLbl.Position = UDim2.new(0.42, 0, 0.06, 0)
	costLbl.BackgroundTransparency = 1
	costLbl.Text = "🪙 30G"
	costLbl.TextColor3 = Color3.fromRGB(255, 215, 75)
	costLbl.Font = Enum.Font.GothamBold
	costLbl.TextXAlignment = Enum.TextXAlignment.Right
	costLbl.Parent = cardF
	addTextConstraint(costLbl, 7, 9)

	local nameLbl = Instance.new("TextLabel")
	nameLbl.Size = UDim2.new(1, -6, 0.48, 0)
	nameLbl.Position = UDim2.new(0, 3, 0.48, 0)
	nameLbl.BackgroundTransparency = 1
	nameLbl.Text = "Troop"
	nameLbl.TextColor3 = Color3.fromRGB(240, 245, 255)
	nameLbl.Font = Enum.Font.GothamBold
	nameLbl.TextXAlignment = Enum.TextXAlignment.Left
	nameLbl.Parent = cardF
	addTextConstraint(nameLbl, 7, 10)

	sideCardWidgets[slot] = {
		Frame = cardF,
		Stroke = cStroke,
		TierPill = tierPill,
		CostLbl = costLbl,
		NameLbl = nameLbl,
	}
end

-- 4. Action Button at Bottom
local sideManageBtn = Instance.new("TextButton")
sideManageBtn.Name = "SideManageBtn"
sideManageBtn.Size = UDim2.new(1, -14, 0, 30)
sideManageBtn.Position = UDim2.new(0, 7, 1, -34)
sideManageBtn.BackgroundColor3 = Color3.fromRGB(44, 28, 22)
sideManageBtn.Text = ""
sideManageBtn.AutoButtonColor = false
sideManageBtn.Parent = openLobbyBtn
addCorner(sideManageBtn, 7)
addStroke(sideManageBtn, Color3.fromRGB(225, 185, 75), 1.4)
addGradient(sideManageBtn, Color3.fromRGB(56, 36, 26), Color3.fromRGB(34, 20, 16), 90)

local sideManageText = Instance.new("TextLabel")
sideManageText.Size = UDim2.new(1, 0, 1, 0)
sideManageText.BackgroundTransparency = 1
sideManageText.Text = "📜 MANAGE DECK [M]"
sideManageText.TextColor3 = Color3.fromRGB(255, 225, 110)
sideManageText.Font = Enum.Font.GothamBold
sideManageText.Parent = sideManageBtn
addTextConstraint(sideManageText, 8, 12)

local updateSideWarDeckUI = function()
	local fData = FactionConfig.Factions[mySelectedFaction]
	if not fData then return end

	sideFactionText.Text = (fData.Badge or "") .. " " .. string.upper(fData.DisplayName or mySelectedFaction)
	sideFactionBanner.BackgroundColor3 = fData.MainColor
	sideFactionStroke.Color = fData.AccentColor
	sideDeckStroke.Color = fData.AccentColor

	for slot = 1, 6 do
		local uIdx = myLoadout[slot]
		local uDef = fData.Units[uIdx]
		local w = sideCardWidgets[slot]
		if uDef and w then
			local tierKey = uDef.Tier or ((slot <= 2) and "C" or ((slot <= 4) and "B" or "A"))
			local tColor = TIER_COLORS[tierKey] or Color3.fromRGB(200, 200, 200)
			w.TierPill.Text = tierKey
			w.TierPill.BackgroundColor3 = tColor
			w.CostLbl.Text = "🪙 " .. tostring(uDef.Cost) .. "G"
			w.NameLbl.Text = uDef.Name
			w.Stroke.Color = tColor
		end
	end
end

updateSideWarDeckUI()

openLobbyBtn.MouseEnter:Connect(function()
	TweenService:Create(sideDeckStroke, TweenInfo.new(0.2), { Color = Color3.fromRGB(255, 230, 120), Thickness = 2.4 }):Play()
end)
openLobbyBtn.MouseLeave:Connect(function()
	local fData = FactionConfig.Factions[mySelectedFaction]
	local accColor = fData and fData.AccentColor or Color3.fromRGB(225, 185, 75)
	TweenService:Create(sideDeckStroke, TweenInfo.new(0.2), { Color = accColor, Thickness = 1.8 }):Play()
end)

-- ============================================================
-- GRAND LOBBY + 6-CARD DECK BUILDER (2 C + 2 B + 2 A)
-- ============================================================
local lobbyBackdrop = Instance.new("Frame")
lobbyBackdrop.Name = "LobbyBackdrop"
lobbyBackdrop.Size = UDim2.fromScale(1, 1)
lobbyBackdrop.BackgroundColor3 = Color3.fromRGB(6, 9, 16)
lobbyBackdrop.BackgroundTransparency = 0.28
lobbyBackdrop.BorderSizePixel = 0
lobbyBackdrop.Visible = false
lobbyBackdrop.Parent = screenGui

local lobbyModal = Instance.new("Frame")
lobbyModal.Name = "LobbyModal"
lobbyModal.AnchorPoint = Vector2.new(0.5, 0.5)
lobbyModal.Size = UDim2.fromScale(0.92, 0.88)
lobbyModal.Position = UDim2.fromScale(0.5, 0.5)
lobbyModal.BackgroundColor3 = Color3.fromRGB(15, 12, 19)
lobbyModal.Parent = lobbyBackdrop
addCorner(lobbyModal, 16)
local lobbyModalStroke = addStroke(lobbyModal, Color3.fromRGB(225, 185, 75), 2.2)
addGradient(lobbyModal, Color3.fromRGB(28, 22, 34), Color3.fromRGB(12, 10, 16), 90)

local modalSizeConstraint = Instance.new("UISizeConstraint")
modalSizeConstraint.MaxSize = Vector2.new(1080, 580)
modalSizeConstraint.MinSize = Vector2.new(680, 390)
modalSizeConstraint.Parent = lobbyModal

-- HEADER BAR (FANTASY GRIMOIRE TITLE)
local modalHeader = Instance.new("Frame")
modalHeader.Size = UDim2.new(1, -28, 0.095, 0)
modalHeader.Position = UDim2.new(0, 14, 0.015, 0)
modalHeader.BackgroundColor3 = Color3.fromRGB(32, 22, 28)
modalHeader.Parent = lobbyModal
addCorner(modalHeader, 10)
addStroke(modalHeader, Color3.fromRGB(205, 160, 60), 1.4)

local modalTitle = Instance.new("TextLabel")
modalTitle.Size = UDim2.new(0.74, 0, 0.54, 0)
modalTitle.Position = UDim2.new(0, 14, 0.06, 0)
modalTitle.BackgroundTransparency = 1
modalTitle.Text = "⚜️ WAR COUNCIL • 6-CARD GRIMOIRE DECK BUILDER ⚜️"
modalTitle.TextColor3 = Color3.fromRGB(255, 225, 100)
modalTitle.Font = Enum.Font.Fondamento
modalTitle.TextXAlignment = Enum.TextXAlignment.Left
modalTitle.Parent = modalHeader
addTextConstraint(modalTitle, 11, 18)

local modalSub = Instance.new("TextLabel")
modalSub.Size = UDim2.new(0.74, 0, 0.36, 0)
modalSub.Position = UDim2.new(0, 14, 0.58, 0)
modalSub.BackgroundTransparency = 1
modalSub.Text = "📜 Inscribe 6 Champions: 2 C-Vanguard (Starters) • 2 B-Veterans (100G) • 2 A-Mythics (250G)"
modalSub.TextColor3 = Color3.fromRGB(225, 205, 175)
modalSub.Font = Enum.Font.Antique
modalSub.TextXAlignment = Enum.TextXAlignment.Left
modalSub.Parent = modalHeader
addTextConstraint(modalSub, 9, 12)

local explore3DBtn = Instance.new("TextButton")
explore3DBtn.AnchorPoint = Vector2.new(1, 0.5)
explore3DBtn.Size = UDim2.new(0.21, 0, 0.72, 0)
explore3DBtn.Position = UDim2.new(1, -10, 0.5, 0)
explore3DBtn.BackgroundColor3 = Color3.fromRGB(44, 28, 24)
explore3DBtn.Text = "🛡️ Walk in 3D Lobby [M]"
explore3DBtn.TextColor3 = Color3.fromRGB(255, 225, 150)
explore3DBtn.Font = Enum.Font.GothamBold
explore3DBtn.Parent = modalHeader
addCorner(explore3DBtn, 8)
addStroke(explore3DBtn, Color3.fromRGB(205, 160, 65), 1.5)
addPadding(explore3DBtn, 4, 4, 8, 8)
addTextConstraint(explore3DBtn, 9, 13)

explore3DBtn.MouseButton1Click:Connect(function()
	playUIClick(1.0)
	lobbyBackdrop.Visible = false
	openLobbyBtn.Visible = not isInMatch
end)

local function openWarDeckModal()
	playUIClick(1.05)
	lobbyBackdrop.Visible = true
	openLobbyBtn.Visible = false
end

openLobbyBtn.MouseButton1Click:Connect(openWarDeckModal)
if sideManageBtn then
	sideManageBtn.MouseButton1Click:Connect(openWarDeckModal)
end

-- STEP 1: FACTION SELECTOR TABS (6 Factions)
local lobbyFactionRow = Instance.new("Frame")
lobbyFactionRow.Size = UDim2.new(1, -28, 0.11, 0)
lobbyFactionRow.Position = UDim2.new(0, 14, 0.122, 0)
lobbyFactionRow.BackgroundTransparency = 1
lobbyFactionRow.Parent = lobbyModal

local lobbyFactionCards = {}
local rebuildDeckPickerUI
local numFactions = #FactionConfig.Order
local fColWidth = 1 / numFactions

for idx, fName in ipairs(FactionConfig.Order) do
	local fData = FactionConfig.Factions[fName]
	local card = Instance.new("TextButton")
	card.Size = UDim2.new(fColWidth, -5, 1, 0)
	card.Position = UDim2.new((idx - 1) * fColWidth, 0, 0, 0)
	card.BackgroundColor3 = fData.MainColor
	card.Text = ""
	card.Parent = lobbyFactionRow
	addCorner(card, 9)
	local st = addStroke(card, fData.AccentColor, 1.8)

	local fTitle = Instance.new("TextLabel")
	fTitle.Size = UDim2.new(1, -10, 0.52, 0)
	fTitle.Position = UDim2.new(0, 5, 0.06, 0)
	fTitle.BackgroundTransparency = 1
	fTitle.Text = (fData.Badge or "") .. " " .. string.upper(fData.DisplayName or fName)
	fTitle.TextColor3 = Color3.new(1, 1, 1)
	fTitle.Font = Enum.Font.GothamBlack
	fTitle.Parent = card
	addTextConstraint(fTitle, 10, 15)

	local fSub = Instance.new("TextLabel")
	fSub.Size = UDim2.new(1, -10, 0.36, 0)
	fSub.Position = UDim2.new(0, 5, 0.58, 0)
	fSub.BackgroundTransparency = 1
	fSub.Text = fData.Subtitle
	fSub.TextColor3 = Color3.fromRGB(230, 240, 255)
	fSub.Font = Enum.Font.GothamMedium
	fSub.Parent = card
	addTextConstraint(fSub, 8, 11)

	lobbyFactionCards[fName] = { Button = card, Stroke = st }

	card.MouseButton1Click:Connect(function()
		playUIClick(1.1)
		mySelectedFaction = fName
		myLoadout = { 1, 2, 4, 5, 7, 8 }
		LobbyActionEvent:FireServer("SelectFaction", fName)
		if rebuildDeckPickerUI then rebuildDeckPickerUI() end
	end)
end

-- STEP 2: 3 GROUPED TIER CONTAINERS (C-Tier, B-Tier, A-Tier)
local deckSection = Instance.new("Frame")
deckSection.Size = UDim2.new(1, -28, 0.375, 0)
deckSection.Position = UDim2.new(0, 14, 0.244, 0)
deckSection.BackgroundTransparency = 1
deckSection.Parent = lobbyModal

local function isUnitInLoadout(idx)
	for s = 1, 6 do
		if myLoadout[s] == idx then
			return true, s
		end
	end
	return false, nil
end

rebuildDeckPickerUI = function()
	for _, ch in ipairs(deckSection:GetChildren()) do
		ch:Destroy()
	end
	local fData = FactionConfig.Factions[mySelectedFaction]
	if not fData then return end

	local tierGroups = {
		{ Tier = "C", Title = "⚔️ CHAPTER I: VANGUARD (C-TIER  •  PICK 2)", Indices = { 1, 2, 3 } },
		{ Tier = "B", Title = "🛡️ CHAPTER II: VETERANS (B-TIER  •  100G UNLOCK)", Indices = { 4, 5, 6 } },
		{ Tier = "A", Title = "👑 CHAPTER III: MYTHIC ELITES (A-TIER  •  250G)", Indices = { 7, 8, 9 } },
	}

	for gIdx, gInfo in ipairs(tierGroups) do
		local tColor = TIER_COLORS[gInfo.Tier]
		local groupBox = Instance.new("Frame")
		groupBox.Size = UDim2.new(0.328, 0, 1, 0)
		groupBox.Position = UDim2.new((gIdx - 1) * 0.336, 0, 0, 0)
		groupBox.BackgroundColor3 = TIER_BG_COLORS[gInfo.Tier]
		groupBox.Parent = deckSection
		addCorner(groupBox, 11)
		addStroke(groupBox, tColor, 1.5, 0.25)

		local gHeader = Instance.new("TextLabel")
		gHeader.Size = UDim2.new(1, -14, 0.14, 0)
		gHeader.Position = UDim2.new(0, 7, 0.02, 0)
		gHeader.BackgroundTransparency = 1
		gHeader.Text = gInfo.Title
		gHeader.TextColor3 = tColor
		gHeader.Font = Enum.Font.GothamBlack
		gHeader.TextXAlignment = Enum.TextXAlignment.Left
		gHeader.Parent = groupBox
		addTextConstraint(gHeader, 9, 12)

		local cardsHolder = Instance.new("Frame")
		cardsHolder.Size = UDim2.new(1, -12, 0.81, 0)
		cardsHolder.Position = UDim2.new(0, 6, 0.17, 0)
		cardsHolder.BackgroundTransparency = 1
		cardsHolder.Parent = groupBox

		for cPos, unitIdx in ipairs(gInfo.Indices) do
			local uDef = fData.Units[unitIdx]
			if uDef then
				local isSelected, slotNum = isUnitInLoadout(unitIdx)

				local uBtn = Instance.new("TextButton")
				uBtn.Size = UDim2.new(0.32, 0, 1, 0)
				uBtn.Position = UDim2.new((cPos - 1) * 0.34, 0, 0, 0)
				uBtn.BackgroundColor3 = isSelected and fData.MainColor or Color3.fromRGB(20, 25, 38)
				uBtn.Text = ""
				uBtn.Parent = cardsHolder
				addCorner(uBtn, 9)
				addStroke(uBtn, isSelected and tColor or Color3.fromRGB(58, 70, 92), isSelected and 2.5 or 1.2)

				local slotBadge = Instance.new("TextLabel")
				slotBadge.Size = UDim2.new(1, -8, 0.16, 0)
				slotBadge.Position = UDim2.new(0, 4, 0.04, 0)
				slotBadge.BackgroundColor3 = isSelected and tColor or Color3.fromRGB(12, 16, 24)
				slotBadge.Text = isSelected and string.format("⚜️ EQUIPPED [%d]", slotNum) or "📜 INSCRIBE"
				slotBadge.TextColor3 = isSelected and Color3.fromRGB(12, 16, 24) or Color3.fromRGB(145, 165, 195)
				slotBadge.Font = Enum.Font.GothamBlack
				slotBadge.Parent = uBtn
				addCorner(slotBadge, 5)
				addPadding(slotBadge, 1, 1, 3, 3)
				addTextConstraint(slotBadge, 8, 11)

				local nameLbl = Instance.new("TextLabel")
				nameLbl.Size = UDim2.new(1, -8, 0.22, 0)
				nameLbl.Position = UDim2.new(0, 4, 0.22, 0)
				nameLbl.BackgroundTransparency = 1
				nameLbl.Text = uDef.Name
				nameLbl.TextColor3 = Color3.new(1, 1, 1)
				nameLbl.Font = Enum.Font.GothamBlack
				nameLbl.Parent = uBtn
				addTextConstraint(nameLbl, 9, 14)

				local hpPill = Instance.new("TextLabel")
				hpPill.Size = UDim2.new(1, -8, 0.16, 0)
				hpPill.Position = UDim2.new(0, 4, 0.45, 0)
				hpPill.BackgroundColor3 = Color3.fromRGB(15, 22, 32)
				hpPill.Text = "❤️ " .. tostring(uDef.HP) .. " HP"
				hpPill.TextColor3 = Color3.fromRGB(130, 245, 165)
				hpPill.Font = Enum.Font.GothamBold
				hpPill.Parent = uBtn
				addCorner(hpPill, 4)
				addTextConstraint(hpPill, 8, 11)

				local roleLbl = Instance.new("TextLabel")
				roleLbl.Size = UDim2.new(1, -8, 0.18, 0)
				roleLbl.Position = UDim2.new(0, 4, 0.62, 0)
				roleLbl.BackgroundTransparency = 1
				roleLbl.Text = "⚔️ " .. tostring(uDef.Role)
				roleLbl.TextColor3 = Color3.fromRGB(220, 232, 255)
				roleLbl.TextWrapped = true
				roleLbl.Font = Enum.Font.GothamBold
				roleLbl.Parent = uBtn
				addTextConstraint(roleLbl, 7, 10)

				local costLbl = Instance.new("TextLabel")
				costLbl.Size = UDim2.new(1, -8, 0.16, 0)
				costLbl.Position = UDim2.new(0, 4, 0.81, 0)
				costLbl.BackgroundColor3 = Color3.fromRGB(12, 16, 24)
				costLbl.Text = "🪙 " .. tostring(uDef.Cost) .. " GOLD"
				costLbl.TextColor3 = Color3.fromRGB(255, 220, 75)
				costLbl.Font = Enum.Font.GothamBlack
				costLbl.Parent = uBtn
				addCorner(costLbl, 5)
				addTextConstraint(costLbl, 8, 11)

				uBtn.MouseButton1Click:Connect(function()
					playUIClick(1.12)
					if uDef.Tier == "C" then
						if myLoadout[1] ~= unitIdx and myLoadout[2] ~= unitIdx then
							myLoadout[1] = myLoadout[2]
							myLoadout[2] = unitIdx
						end
					elseif uDef.Tier == "B" then
						if myLoadout[3] ~= unitIdx and myLoadout[4] ~= unitIdx then
							myLoadout[3] = myLoadout[4]
							myLoadout[4] = unitIdx
						end
					elseif uDef.Tier == "A" then
						if myLoadout[5] ~= unitIdx and myLoadout[6] ~= unitIdx then
							myLoadout[5] = myLoadout[6]
							myLoadout[6] = unitIdx
						end
					end
					LobbyActionEvent:FireServer("SetLoadout", myLoadout)
					rebuildDeckPickerUI()
				end)
			end
		end
	end
	if updateSideWarDeckUI then
		updateSideWarDeckUI()
	end
end
rebuildDeckPickerUI()

-- STEP 3: 1v1 VS PLAYER DUEL PANEL (FANTASY ARENA - FULL WIDTH)
local lastReceivedState = nil

local pvpPanel = Instance.new("Frame")
pvpPanel.Size = UDim2.new(1, -28, 0.35, 0)
pvpPanel.Position = UDim2.new(0, 14, 0.632, 0)
pvpPanel.BackgroundColor3 = Color3.fromRGB(18, 16, 26)
pvpPanel.Parent = lobbyModal
addCorner(pvpPanel, 12)
addStroke(pvpPanel, Color3.fromRGB(195, 155, 65), 1.6)

local pvpTitle = Instance.new("TextLabel")
pvpTitle.Size = UDim2.new(1, -20, 0.14, 0)
pvpTitle.Position = UDim2.new(0, 10, 0.04, 0)
pvpTitle.BackgroundTransparency = 1
pvpTitle.Text = "🛡️ GRAND ARENA (1v1 PVP DUEL)"
pvpTitle.TextColor3 = Color3.fromRGB(140, 210, 255)
pvpTitle.Font = Enum.Font.Fondamento
pvpTitle.TextXAlignment = Enum.TextXAlignment.Left
pvpTitle.Parent = pvpPanel
addTextConstraint(pvpTitle, 10, 14)

local playerScroll = Instance.new("ScrollingFrame")
playerScroll.Size = UDim2.new(1, -20, 0.48, 0)
playerScroll.Position = UDim2.new(0, 10, 0.20, 0)
playerScroll.BackgroundColor3 = Color3.fromRGB(12, 16, 26)
playerScroll.BorderSizePixel = 0
playerScroll.ScrollBarThickness = 5
playerScroll.AutomaticCanvasSize = Enum.AutomaticSize.Y
playerScroll.CanvasSize = UDim2.new(0, 0, 0, 0)
playerScroll.Parent = pvpPanel
addCorner(playerScroll, 8)
addPadding(playerScroll, 6, 6, 6, 6)

local listLayout = Instance.new("UIListLayout")
listLayout.Padding = UDim.new(0, 6)
listLayout.Parent = playerScroll

local joinQueueBtn = Instance.new("TextButton")
joinQueueBtn.Size = UDim2.new(1, -20, 0.24, 0)
joinQueueBtn.Position = UDim2.new(0, 10, 0.71, 0)
joinQueueBtn.BackgroundColor3 = Color3.fromRGB(18, 34, 58)
joinQueueBtn.Text = "⚔️ ENTER ARENA MATCHMAKING"
joinQueueBtn.TextColor3 = Color3.fromRGB(215, 235, 255)
joinQueueBtn.Font = Enum.Font.GothamBlack
joinQueueBtn.Parent = pvpPanel
addCorner(joinQueueBtn, 10)
addStroke(joinQueueBtn, Color3.fromRGB(100, 190, 255), 2.0)
addTextConstraint(joinQueueBtn, 12, 17)

joinQueueBtn.MouseButton1Click:Connect(function()
	playUIClick(1.18)
	LobbyActionEvent:FireServer("JoinPvPQueue")
end)

local challengePopup = Instance.new("Frame")
challengePopup.AnchorPoint = Vector2.new(0.5, 0)
challengePopup.Size = UDim2.new(0, 360, 0, 130)
challengePopup.Position = UDim2.new(0.5, 0, 0, 90)
challengePopup.BackgroundColor3 = Color3.fromRGB(22, 18, 36)
challengePopup.Visible = false
challengePopup.Parent = screenGui
addCorner(challengePopup, 12)
addStroke(challengePopup, Color3.fromRGB(255, 210, 85), 2.2)

-- ============================================================
-- BOT DIFFICULTY SELECTION MODAL (STEP ON BOT PLATFORM TO OPEN)
-- ============================================================
local botModalBackdrop = Instance.new("Frame")
botModalBackdrop.Name = "BotModalBackdrop"
botModalBackdrop.Size = UDim2.new(1, 0, 1, 0)
botModalBackdrop.BackgroundColor3 = Color3.fromRGB(6, 6, 12)
botModalBackdrop.BackgroundTransparency = 0.35
botModalBackdrop.Visible = false
botModalBackdrop.ZIndex = 50
botModalBackdrop.Parent = screenGui

local botModal = Instance.new("Frame")
botModal.Name = "BotDifficultyModal"
botModal.AnchorPoint = Vector2.new(0.5, 0.5)
botModal.Size = UDim2.new(0.72, 0, 0.76, 0)
botModal.Position = UDim2.new(0.5, 0, 0.5, 0)
botModal.BackgroundColor3 = Color3.fromRGB(18, 15, 26)
botModal.ZIndex = 51
botModal.Parent = botModalBackdrop
addCorner(botModal, 14)
addStroke(botModal, Color3.fromRGB(235, 185, 70), 2.4)

local botModalConstraint = Instance.new("UISizeConstraint")
botModalConstraint.MaxSize = Vector2.new(880, 580)
botModalConstraint.MinSize = Vector2.new(580, 420)
botModalConstraint.Parent = botModal

-- Header
local botModalTitle = Instance.new("TextLabel")
botModalTitle.Size = UDim2.new(1, -60, 0, 32)
botModalTitle.Position = UDim2.new(0, 20, 0, 12)
botModalTitle.BackgroundTransparency = 1
botModalTitle.Text = "⚔️ CRUCIBLE BOT TRIAL — SELECT FACTION & DIFFICULTY"
botModalTitle.TextColor3 = Color3.fromRGB(255, 220, 100)
botModalTitle.Font = Enum.Font.Fondamento
botModalTitle.TextXAlignment = Enum.TextXAlignment.Left
botModalTitle.ZIndex = 52
botModalTitle.Parent = botModal
addTextConstraint(botModalTitle, 16, 24)

-- Close Button
local botModalCloseBtn = Instance.new("TextButton")
botModalCloseBtn.Size = UDim2.new(0, 32, 0, 32)
botModalCloseBtn.Position = UDim2.new(1, -44, 0, 12)
botModalCloseBtn.BackgroundColor3 = Color3.fromRGB(45, 22, 28)
botModalCloseBtn.Text = "✕"
botModalCloseBtn.TextColor3 = Color3.fromRGB(255, 140, 150)
botModalCloseBtn.Font = Enum.Font.GothamBold
botModalCloseBtn.ZIndex = 53
botModalCloseBtn.Parent = botModal
addCorner(botModalCloseBtn, 8)
addStroke(botModalCloseBtn, Color3.fromRGB(200, 70, 80), 1.5)
botModalCloseBtn.MouseButton1Click:Connect(function()
	playUIClick(1.0)
	botModalBackdrop.Visible = false
end)

-- Subtitle Notice
local botModalSub = Instance.new("TextLabel")
botModalSub.Size = UDim2.new(1, -40, 0, 28)
botModalSub.Position = UDim2.new(0, 20, 0, 44)
botModalSub.BackgroundTransparency = 1
botModalSub.Text = "Choose the enemy faction you wish to battle against, then pick your challenge level.\n⚠️ PRACTICE TRIAL: Bot matches do NOT grant Ranked Wins (Duel players in PvP to earn Wins)."
botModalSub.TextColor3 = Color3.fromRGB(205, 215, 235)
botModalSub.Font = Enum.Font.GothamMedium
botModalSub.TextXAlignment = Enum.TextXAlignment.Left
botModalSub.ZIndex = 52
botModalSub.Parent = botModal
addTextConstraint(botModalSub, 9, 13)

-- STEP 1: Opponent Bot Faction Selection
local botFactionTitle = Instance.new("TextLabel")
botFactionTitle.Size = UDim2.new(1, -40, 0, 18)
botFactionTitle.Position = UDim2.new(0, 20, 0, 76)
botFactionTitle.BackgroundTransparency = 1
botFactionTitle.Text = "STEP 1: CHOOSE OPPONENT BOT FACTION"
botFactionTitle.TextColor3 = Color3.fromRGB(255, 215, 110)
botFactionTitle.Font = Enum.Font.GothamBold
botFactionTitle.TextXAlignment = Enum.TextXAlignment.Left
botFactionTitle.ZIndex = 52
botFactionTitle.Parent = botModal
addTextConstraint(botFactionTitle, 10, 13)

local botFactionRow = Instance.new("Frame")
botFactionRow.Size = UDim2.new(1, -40, 0, 36)
botFactionRow.Position = UDim2.new(0, 20, 0, 96)
botFactionRow.BackgroundTransparency = 1
botFactionRow.ZIndex = 52
botFactionRow.Parent = botModal

local botFactionBtns = {}
local botSelectedFaction = "Random"

local function refreshBotFactionUI()
	for fKey, item in pairs(botFactionBtns) do
		local sel = (fKey == botSelectedFaction)
		item.Stroke.Thickness = sel and 2.4 or 1.0
		item.Stroke.Color = sel and Color3.fromRGB(255, 230, 120) or item.DefaultAccent
		item.Button.BackgroundTransparency = sel and 0 or 0.45
	end
end

local allBotFactions = {}
for _, fName in ipairs(FactionConfig.Order) do
	local fData = FactionConfig.Factions[fName]
	table.insert(allBotFactions, {
		Key = fName,
		Name = (fData.Badge or "") .. " " .. (fData.DisplayName or fName),
		Color = fData.MainColor,
		Accent = fData.AccentColor
	})
end
table.insert(allBotFactions, {
	Key = "Random",
	Name = "🎲 Random Faction",
	Color = Color3.fromRGB(34, 26, 52),
	Accent = Color3.fromRGB(160, 125, 240)
})

local fBtnWidth = 1 / #allBotFactions
for idx, fInfo in ipairs(allBotFactions) do
	local bBtn = Instance.new("TextButton")
	bBtn.Size = UDim2.new(fBtnWidth, -4, 1, 0)
	bBtn.Position = UDim2.new((idx - 1) * fBtnWidth, 0, 0, 0)
	bBtn.BackgroundColor3 = fInfo.Color
	bBtn.Text = fInfo.Name
	bBtn.TextColor3 = Color3.new(1, 1, 1)
	bBtn.Font = Enum.Font.GothamBold
	bBtn.ZIndex = 53
	bBtn.Parent = botFactionRow
	addCorner(bBtn, 7)
	addPadding(bBtn, 2, 2, 2, 2)
	addTextConstraint(bBtn, 8, 12)
	local st = addStroke(bBtn, fInfo.Accent, 1.0)
	botFactionBtns[fInfo.Key] = { Button = bBtn, Stroke = st, DefaultAccent = fInfo.Accent }

	bBtn.MouseButton1Click:Connect(function()
		playUIClick(1.06)
		botSelectedFaction = fInfo.Key
		refreshBotFactionUI()
	end)
end
refreshBotFactionUI()

-- STEP 2: Difficulty Selection & Immediate Launch
local diffTitle = Instance.new("TextLabel")
diffTitle.Size = UDim2.new(1, -40, 0, 18)
diffTitle.Position = UDim2.new(0, 20, 0, 138)
diffTitle.BackgroundTransparency = 1
diffTitle.Text = "STEP 2: CHOOSE DIFFICULTY (CLICK TO COMMENCE BATTLE)"
diffTitle.TextColor3 = Color3.fromRGB(255, 215, 110)
diffTitle.Font = Enum.Font.GothamBold
diffTitle.TextXAlignment = Enum.TextXAlignment.Left
diffTitle.ZIndex = 52
diffTitle.Parent = botModal
addTextConstraint(diffTitle, 10, 13)

-- 4 Difficulty Cards Grid
local diffGrid = Instance.new("Frame")
diffGrid.Size = UDim2.new(1, -40, 1, -210)
diffGrid.Position = UDim2.new(0, 20, 0, 158)
diffGrid.BackgroundTransparency = 1
diffGrid.ZIndex = 52
diffGrid.Parent = botModal

local gridLayout = Instance.new("UIGridLayout")
gridLayout.CellSize = UDim2.new(0.485, 0, 0.46, 0)
gridLayout.CellPadding = UDim2.new(0.03, 0, 0.08, 0)
gridLayout.Parent = diffGrid

local diffCardDefs = {
	{
		Key = "Easy",
		Title = "🟢 EASY (NOVICE)",
		Desc = "Slower AI army progression & -15% Gold income.\nTier 2 on Wave 6. Perfect for deck & combo testing.",
		Color = Color3.fromRGB(18, 48, 30),
		Accent = Color3.fromRGB(80, 220, 130),
		TextColor = Color3.fromRGB(150, 255, 180),
	},
	{
		Key = "Normal",
		Title = "🟡 NORMAL (COMMANDER)",
		Desc = "Standard balanced AI forces & 1.0x Gold income.\nUnlocks Tier 2 on Wave 4, Tier 3 on Wave 8.",
		Color = Color3.fromRGB(48, 42, 18),
		Accent = Color3.fromRGB(245, 195, 65),
		TextColor = Color3.fromRGB(255, 230, 140),
	},
	{
		Key = "Hard",
		Title = "🟠 HARD (VETERAN)",
		Desc = "Aggressive AI with +20% Gold, starts with 2 Vanguards.\nFast upgrades (Tier 2 on Wave 3, Tier 3 on Wave 6).",
		Color = Color3.fromRGB(56, 32, 16),
		Accent = Color3.fromRGB(255, 145, 55),
		TextColor = Color3.fromRGB(255, 195, 130),
	},
	{
		Key = "Extreme",
		Title = "🔴 EXTREME (OVERLORD)",
		Desc = "Relentless AI with +45% Gold, starts with Vanguard + Veteran.\nTier 2 on Wave 2, Tier 3 on Wave 4. Maximum challenge!",
		Color = Color3.fromRGB(58, 16, 26),
		Accent = Color3.fromRGB(255, 75, 95),
		TextColor = Color3.fromRGB(255, 150, 165),
	},
}

for _, dInfo in ipairs(diffCardDefs) do
	local card = Instance.new("TextButton")
	card.BackgroundColor3 = dInfo.Color
	card.Text = ""
	card.AutoButtonColor = true
	card.ZIndex = 53
	card.Parent = diffGrid
	addCorner(card, 10)
	local cStroke = addStroke(card, dInfo.Accent, 1.8)

	local cTitle = Instance.new("TextLabel")
	cTitle.Size = UDim2.new(1, -16, 0, 24)
	cTitle.Position = UDim2.new(0, 10, 0, 6)
	cTitle.BackgroundTransparency = 1
	cTitle.Text = dInfo.Title
	cTitle.TextColor3 = dInfo.TextColor
	cTitle.Font = Enum.Font.GothamBlack
	cTitle.TextXAlignment = Enum.TextXAlignment.Left
	cTitle.ZIndex = 54
	cTitle.Parent = card
	addTextConstraint(cTitle, 11, 15)

	local cDesc = Instance.new("TextLabel")
	cDesc.Size = UDim2.new(1, -16, 1, -34)
	cDesc.Position = UDim2.new(0, 10, 0, 30)
	cDesc.BackgroundTransparency = 1
	cDesc.Text = dInfo.Desc
	cDesc.TextColor3 = Color3.fromRGB(220, 225, 235)
	cDesc.Font = Enum.Font.GothamMedium
	cDesc.TextXAlignment = Enum.TextXAlignment.Left
	cDesc.TextYAlignment = Enum.TextYAlignment.Top
	cDesc.TextWrapped = true
	cDesc.ZIndex = 54
	cDesc.Parent = card
	addTextConstraint(cDesc, 9, 12)

	card.MouseEnter:Connect(function()
		cStroke.Thickness = 2.6
	end)
	card.MouseLeave:Connect(function()
		cStroke.Thickness = 1.8
	end)

	card.MouseButton1Click:Connect(function()
		playUIClick(1.2)
		botModalBackdrop.Visible = false
		if lastReceivedState and lastReceivedState.ArenaActive and not lastReceivedState.InMatch then
			bannerLabel.Text = "⚔️ An arena battle is already in progress! Please wait or spectate!"
			bannerLabel.TextColor3 = Color3.fromRGB(255, 160, 80)
			bannerLabel.TextTransparency = 0
			task.delay(2.8, function()
				if bannerLabel.Text:find("arena battle is already in progress") then
					TweenService:Create(bannerLabel, TweenInfo.new(0.5), { TextTransparency = 1 }):Play()
				end
			end)
			return
		end
		-- Determine chosen enemy faction (specific faction or nil for random)
		local enemyFaction = (botSelectedFaction ~= "Random" and botSelectedFaction) or nil
		LobbyActionEvent:FireServer("StartBotMatch", mySelectedFaction, enemyFaction, dInfo.Key)
	end)
end
-- Return to Lobby button at bottom
local botModalBackBtn = Instance.new("TextButton")
botModalBackBtn.Size = UDim2.new(1, -40, 0, 32)
botModalBackBtn.Position = UDim2.new(0, 20, 1, -44)
botModalBackBtn.BackgroundColor3 = Color3.fromRGB(25, 22, 35)
botModalBackBtn.Text = "← RETURN TO LOBBY"
botModalBackBtn.TextColor3 = Color3.fromRGB(180, 190, 215)
botModalBackBtn.Font = Enum.Font.GothamBold
botModalBackBtn.ZIndex = 53
botModalBackBtn.Parent = botModal
addCorner(botModalBackBtn, 8)
addStroke(botModalBackBtn, Color3.fromRGB(75, 70, 95), 1.2)
addTextConstraint(botModalBackBtn, 10, 13)

botModalBackBtn.MouseButton1Click:Connect(function()
	playUIClick(1.0)
	botModalBackdrop.Visible = false
end)

local lastBotModalOpenTime = 0

local function openBotDifficultyModal()
	-- If modal is ALREADY visible, DO NOT re-trigger or play sound repeatedly!
	if botModalBackdrop and botModalBackdrop.Visible then
		return
	end
	local now = os.clock()
	if (now - lastBotModalOpenTime) < 2.0 then
		-- Debounce cooldown to prevent rapid multi-limb triggers
		return
	end
	lastBotModalOpenTime = now

	if lobbyBackdrop then lobbyBackdrop.Visible = false end
	if botModalBackdrop then
		botModalBackdrop.Visible = true
		playUIClick(1.15)
	end
end

-- Step Detection on Lobby BotStepPad (with debounce)
task.spawn(function()
	local lobby = workspace:WaitForChild("Lobby", 10)
	if not lobby then return end
	local botPad = lobby:WaitForChild("BotStepPad", 10)
	if botPad then
		botPad.Touched:Connect(function(hit)
			local char = Players.LocalPlayer.Character
			if char and hit:IsDescendantOf(char) then
				if botModalBackdrop and not botModalBackdrop.Visible and (not lastReceivedState or not lastReceivedState.InMatch) then
					openBotDifficultyModal()
				end
			end
		end)
	end
end)

local challengeText = Instance.new("TextLabel")
challengeText.Size = UDim2.new(1, -20, 0, 60)
challengeText.Position = UDim2.new(0, 10, 0, 10)
challengeText.BackgroundTransparency = 1
challengeText.Text = "⚔️ 1v1 DUEL CHALLENGE!"
challengeText.TextColor3 = Color3.new(1, 1, 1)
challengeText.TextWrapped = true
challengeText.Font = Enum.Font.GothamBlack
challengeText.TextSize = 14
challengeText.Parent = challengePopup

local pendingChallengerId = nil

local acceptDuelBtn = Instance.new("TextButton")
acceptDuelBtn.Size = UDim2.new(0.46, 0, 0, 38)
acceptDuelBtn.Position = UDim2.new(0.04, 0, 1, -46)
acceptDuelBtn.BackgroundColor3 = Color3.fromRGB(45, 175, 90)
acceptDuelBtn.Text = "ACCEPT 1v1"
acceptDuelBtn.TextColor3 = Color3.new(1, 1, 1)
acceptDuelBtn.Font = Enum.Font.GothamBlack
acceptDuelBtn.TextSize = 13
acceptDuelBtn.Parent = challengePopup
addCorner(acceptDuelBtn, 8)

local declineDuelBtn = Instance.new("TextButton")
declineDuelBtn.Size = UDim2.new(0.44, 0, 0, 38)
declineDuelBtn.Position = UDim2.new(0.52, 0, 1, -46)
declineDuelBtn.BackgroundColor3 = Color3.fromRGB(175, 50, 65)
declineDuelBtn.Text = "DECLINE"
declineDuelBtn.TextColor3 = Color3.new(1, 1, 1)
declineDuelBtn.Font = Enum.Font.GothamBlack
declineDuelBtn.TextSize = 13
declineDuelBtn.Parent = challengePopup
addCorner(declineDuelBtn, 8)

acceptDuelBtn.MouseButton1Click:Connect(function()
	playUIClick(1.2)
	challengePopup.Visible = false
	if pendingChallengerId then
		LobbyActionEvent:FireServer("AcceptChallenge", pendingChallengerId)
	end
end)

declineDuelBtn.MouseButton1Click:Connect(function()
	playUIClick(0.9)
	challengePopup.Visible = false
end)

ChallengeEvent.OnClientEvent:Connect(function(evtType, challengerId, challengerName, challengerFaction)
	if evtType == "IncomingChallenge" then
		pendingChallengerId = challengerId
		local cfData = FactionConfig.Factions[challengerFaction]
		local cfDisplay = (cfData and cfData.DisplayName) or challengerFaction
		challengeText.Text = string.format("⚔️ %s (%s) challenged you to a 1v1 Duel!", challengerName, cfDisplay)
		challengePopup.Visible = true
		playUIClick(1.25)
	end
end)

LobbyActionEvent.OnClientEvent:Connect(function(evtType, arg1, arg2)
	if evtType == "OpenLobbyUI" then
		lobbyBackdrop.Visible = true
		openLobbyBtn.Visible = false
	elseif evtType == "OpenBotDifficultyModal" then
		openBotDifficultyModal()
	end
end)

-- ============================================================
-- MATCH END RESULT MODAL (WINNER DISPLAY, +1 WIN, AUTO-RETURN)
-- ============================================================
local resultBackdrop = Instance.new("Frame")
resultBackdrop.Name = "ResultBackdrop"
resultBackdrop.Size = UDim2.new(1, 0, 1, 0)
resultBackdrop.BackgroundColor3 = Color3.fromRGB(6, 8, 16)
resultBackdrop.BackgroundTransparency = 0.28
resultBackdrop.Visible = false
resultBackdrop.ZIndex = 60
resultBackdrop.Parent = screenGui

local resultModal = Instance.new("Frame")
resultModal.Name = "ResultModal"
resultModal.AnchorPoint = Vector2.new(0.5, 0.5)
resultModal.Size = UDim2.new(0.48, 0, 0.60, 0)
resultModal.Position = UDim2.new(0.5, 0, 0.5, 0)
resultModal.BackgroundColor3 = Color3.fromRGB(16, 20, 32)
resultModal.ZIndex = 61
resultModal.Parent = resultBackdrop
addCorner(resultModal, 16)
local resultStroke = addStroke(resultModal, Color3.fromRGB(255, 215, 80), 2.8)

local resultSizeConstraint = Instance.new("UISizeConstraint")
resultSizeConstraint.MaxSize = Vector2.new(620, 480)
resultSizeConstraint.MinSize = Vector2.new(420, 340)
resultSizeConstraint.Parent = resultModal

local resultHeader = Instance.new("TextLabel")
resultHeader.Size = UDim2.new(1, -24, 0.17, 0)
resultHeader.Position = UDim2.new(0, 12, 0.04, 0)
resultHeader.BackgroundTransparency = 1
resultHeader.Text = "🏆 VICTORY!"
resultHeader.TextColor3 = Color3.fromRGB(255, 215, 80)
resultHeader.Font = Enum.Font.GothamBlack
resultHeader.ZIndex = 62
resultHeader.Parent = resultModal
addTextConstraint(resultHeader, 20, 32)

local resultSub = Instance.new("TextLabel")
resultSub.Size = UDim2.new(1, -24, 0.10, 0)
resultSub.Position = UDim2.new(0, 12, 0.21, 0)
resultSub.BackgroundTransparency = 1
resultSub.Text = "Commander destroyed the Enemy Citadel!"
resultSub.TextColor3 = Color3.fromRGB(220, 230, 248)
resultSub.Font = Enum.Font.GothamBold
resultSub.ZIndex = 62
resultSub.Parent = resultModal
addTextConstraint(resultSub, 10, 15)

local resultCard = Instance.new("Frame")
resultCard.Size = UDim2.new(1, -28, 0.44, 0)
resultCard.Position = UDim2.new(0, 14, 0.33, 0)
resultCard.BackgroundColor3 = Color3.fromRGB(22, 28, 44)
resultCard.ZIndex = 62
resultCard.Parent = resultModal
addCorner(resultCard, 12)
addStroke(resultCard, Color3.fromRGB(55, 70, 95), 1.2)

local winBanner = Instance.new("TextLabel")
winBanner.Size = UDim2.new(1, -20, 0.32, 0)
winBanner.Position = UDim2.new(0, 10, 0.08, 0)
winBanner.BackgroundColor3 = Color3.fromRGB(35, 55, 75)
winBanner.Text = "⭐ +1 WIN EARNED! ⭐"
winBanner.TextColor3 = Color3.fromRGB(255, 220, 80)
winBanner.Font = Enum.Font.GothamBlack
winBanner.ZIndex = 63
winBanner.Parent = resultCard
addCorner(winBanner, 8)
addTextConstraint(winBanner, 11, 16)

local statsDetail = Instance.new("TextLabel")
statsDetail.Size = UDim2.new(1, -20, 0.50, 0)
statsDetail.Position = UDim2.new(0, 10, 0.44, 0)
statsDetail.BackgroundTransparency = 1
statsDetail.Text = "Winner: Kingdom\nWaves Survived: 8 Waves • 1v1 vs Bot\n🏆 Total Wins: 1"
statsDetail.TextColor3 = Color3.fromRGB(200, 215, 235)
statsDetail.Font = Enum.Font.GothamMedium
statsDetail.ZIndex = 63
statsDetail.Parent = resultCard
addTextConstraint(statsDetail, 10, 14)

local returnLobbyBtn = Instance.new("TextButton")
returnLobbyBtn.Size = UDim2.new(1, -28, 0.14, 0)
returnLobbyBtn.Position = UDim2.new(0, 14, 0.81, 0)
returnLobbyBtn.BackgroundColor3 = Color3.fromRGB(45, 135, 85)
returnLobbyBtn.Text = "RETURNING TO LOBBY IN 5s... (CLICK TO RETURN NOW)"
returnLobbyBtn.TextColor3 = Color3.new(1, 1, 1)
returnLobbyBtn.Font = Enum.Font.GothamBlack
returnLobbyBtn.ZIndex = 62
returnLobbyBtn.Parent = resultModal
addCorner(returnLobbyBtn, 10)
addStroke(returnLobbyBtn, Color3.fromRGB(110, 245, 145), 2.0)
addTextConstraint(returnLobbyBtn, 10, 14)

local returnCountdownThread = nil

MatchResultEvent.OnClientEvent:Connect(function(data)
	if not data or not data.IsParticipant then return end
	if returnCountdownThread then
		task.cancel(returnCountdownThread)
		returnCountdownThread = nil
	end

	if topBar then topBar.Visible = false end
	if bottomDeck then bottomDeck.Visible = false end
	if cameraDock then cameraDock.Visible = false end
	if setCameraMode then setCameraMode(false) end

	local isWin = data.IsWinner
	resultHeader.Text = isWin and "🏆 VICTORY!" or (data.IsParticipant and "💀 DEFEAT!" or "⚔️ MATCH RESULT")
	resultHeader.TextColor3 = isWin and Color3.fromRGB(255, 215, 80) or (data.IsParticipant and Color3.fromRGB(255, 85, 105) or Color3.fromRGB(100, 205, 255))
	resultStroke.Color = isWin and Color3.fromRGB(255, 215, 80) or (data.IsParticipant and Color3.fromRGB(255, 85, 105) or Color3.fromRGB(100, 205, 255))

	resultSub.Text = string.format("%s (%s) has destroyed the %s Citadel!", data.WinnerName, data.WinnerFaction, data.LoserFaction)

	if isWin then
		if data.MatchMode == "PvP" then
			winBanner.Text = "⭐ +1 RANKED WIN EARNED! ⭐"
			winBanner.TextColor3 = Color3.fromRGB(255, 220, 80)
			winBanner.BackgroundColor3 = Color3.fromRGB(45, 65, 30)
			statsDetail.Text = string.format(
				"Winner: %s %s (%s)\nWaves Survived: %d Waves  •  1v1 PvP Duel\n🏆 YOUR RANKED WINS: %d",
				data.WinnerBadge or "🏆", data.WinnerName, data.WinnerFaction,
				data.WaveNumber or 0,
				data.PlayerWins or 0
			)
		else
			winBanner.Text = "⚔️ BOT TRIAL CONCLUDED (PRACTICE MATCH) ⚔️"
			winBanner.TextColor3 = Color3.fromRGB(200, 225, 255)
			winBanner.BackgroundColor3 = Color3.fromRGB(35, 45, 65)
			statsDetail.Text = string.format(
				"Winner: %s %s (%s)\nWaves Survived: %d Waves  •  Crucible Bot (%s)\n🏆 Ranked Wins: %d (Duel players to earn wins!)",
				data.WinnerBadge or "🏆", data.WinnerName, data.WinnerFaction,
				data.WaveNumber or 0,
				data.BotDifficulty or "Practice",
				data.PlayerWins or 0
			)
		end
	else
		winBanner.Text = data.IsParticipant and "BETTER LUCK NEXT ROUND!" or "MATCH CONCLUDED"
		winBanner.TextColor3 = Color3.fromRGB(220, 230, 245)
		winBanner.BackgroundColor3 = Color3.fromRGB(30, 36, 50)
		statsDetail.Text = string.format(
			"Winner: %s %s (%s)\nWaves Survived: %d Waves  •  %s\n🏆 Your Total Wins: %d",
			data.WinnerBadge or "🏆", data.WinnerName, data.WinnerFaction,
			data.WaveNumber or 0,
			data.MatchMode == "PvP" and "1v1 Player Duel" or "1v1 vs Bot Commander",
			data.PlayerWins or 0
		)
	end

	resultBackdrop.Visible = true

	local remaining = data.AutoReturnDelay or 5
	returnCountdownThread = task.spawn(function()
		while remaining > 0 do
			returnLobbyBtn.Text = string.format("RETURNING TO LOBBY IN %ds... (CLICK TO RETURN NOW)", remaining)
			task.wait(1.0)
			remaining -= 1
		end
		resultBackdrop.Visible = false
		lobbyBackdrop.Visible = false
		openLobbyBtn.Visible = true
	end)
end)

returnLobbyBtn.MouseButton1Click:Connect(function()
	playUIClick(1.1)
	if returnCountdownThread then
		task.cancel(returnCountdownThread)
		returnCountdownThread = nil
	end
	resultBackdrop.Visible = false
	lobbyBackdrop.Visible = false
	openLobbyBtn.Visible = true	LobbyActionEvent:FireServer("LeaveToLobby")
end)

-- ============================================================
-- IN-MATCH TACTICAL HUD (TOP BAR + 6-CARD COMMAND DECK)
-- ============================================================
topBar = Instance.new("Frame")
topBar.Name = "TopBar"
topBar.AnchorPoint = Vector2.new(0.5, 0)
topBar.Size = UDim2.new(0.56, 0, 0, 50)
topBar.Position = UDim2.new(0.5, 0, 0, 8)
topBar.BackgroundColor3 = Color3.fromRGB(12, 16, 24)
topBar.BackgroundTransparency = 0.22
topBar.Visible = false
topBar.Parent = screenGui
addCorner(topBar, 9)
local topStroke = addStroke(topBar, Color3.fromRGB(60, 85, 120), 1.0)

local topBarSizeConstraint = Instance.new("UISizeConstraint")
topBarSizeConstraint.MaxSize = Vector2.new(780, 52)
topBarSizeConstraint.MinSize = Vector2.new(480, 46)
topBarSizeConstraint.Parent = topBar

local factionBtnContainer = Instance.new("Frame")
factionBtnContainer.Size = UDim2.new(0.33, 0, 0.76, 0)
factionBtnContainer.Position = UDim2.new(0.008, 0, 0.12, 0)
factionBtnContainer.BackgroundTransparency = 1
factionBtnContainer.Visible = false
factionBtnContainer.Parent = topBar

local factionButtons = {}
for idx, fName in ipairs(FactionConfig.Order) do
	local fData = FactionConfig.Factions[fName]
	local btn = Instance.new("TextButton")
	btn.Name = fName
	btn.Size = UDim2.new(fColWidth, -3, 1, 0)
	btn.Position = UDim2.new((idx - 1) * fColWidth, 0, 0, 0)
	btn.BackgroundColor3 = fData.MainColor
	btn.Text = (fData.Badge or "") .. "\n" .. (fData.DisplayName or fName)
	btn.TextColor3 = Color3.new(1, 1, 1)
	btn.Font = Enum.Font.GothamBold
	btn.Parent = factionBtnContainer
	addCorner(btn, 7)
	addPadding(btn, 2, 2, 2, 2)
	addTextConstraint(btn, 8, 11)
	local st = addStroke(btn, fData.AccentColor, 1.4)
	factionButtons[fName] = { Button = btn, Stroke = st }

	btn.MouseButton1Click:Connect(function()
		playUIClick(1.1)
		SwitchFactionEvent:FireServer("Player", fName)
	end)
end

local centerStatus = Instance.new("Frame")
centerStatus.Size = UDim2.new(0.78, 0, 0.88, 0)
centerStatus.Position = UDim2.new(0.02, 0, 0.06, 0)
centerStatus.BackgroundTransparency = 1
centerStatus.Parent = topBar

local waveTitle = Instance.new("TextLabel")
waveTitle.Size = UDim2.new(1, 0, 0.28, 0)
waveTitle.BackgroundTransparency = 1
waveTitle.Text = "WAVE 0  •  NEXT: 15s  (+20G)"
waveTitle.TextColor3 = Color3.fromRGB(245, 230, 160)
waveTitle.Font = Enum.Font.GothamBold
waveTitle.Parent = centerStatus
addTextConstraint(waveTitle, 8, 12)

local hpRow = Instance.new("Frame")
hpRow.Size = UDim2.new(1, 0, 0.38, 0)
hpRow.Position = UDim2.new(0, 0, 0.30, 0)
hpRow.BackgroundColor3 = Color3.fromRGB(10, 13, 18)
hpRow.Parent = centerStatus
addCorner(hpRow, 4)
addStroke(hpRow, Color3.fromRGB(45, 58, 80), 1.0)

local allyHPFill = Instance.new("Frame")
allyHPFill.Size = UDim2.new(0.495, 0, 1, 0)
allyHPFill.BackgroundColor3 = Color3.fromRGB(45, 215, 135)
allyHPFill.Parent = hpRow
addCorner(allyHPFill, 3)

local enemyHPFill = Instance.new("Frame")
enemyHPFill.Size = UDim2.new(0.495, 0, 1, 0)
enemyHPFill.Position = UDim2.new(0.505, 0, 0, 0)
enemyHPFill.BackgroundColor3 = Color3.fromRGB(235, 65, 85)
enemyHPFill.Parent = hpRow
addCorner(enemyHPFill, 3)

local hpVsText = Instance.new("TextLabel")
hpVsText.Size = UDim2.new(1, 0, 1, 0)
hpVsText.BackgroundTransparency = 1
hpVsText.Text = "3000 HP          CITADELS          3000 HP"
hpVsText.TextColor3 = Color3.new(1, 1, 1)
hpVsText.TextStrokeTransparency = 0.35
hpVsText.Font = Enum.Font.GothamBold
hpVsText.Parent = hpRow
addTextConstraint(hpVsText, 8, 11)

local turretRow = Instance.new("TextLabel")
turretRow.Size = UDim2.new(1, 0, 0.28, 0)
turretRow.Position = UDim2.new(0, 0, 0.70, 0)
turretRow.BackgroundTransparency = 1
turretRow.Text = ""
turretRow.Visible = false
turretRow.TextColor3 = Color3.fromRGB(165, 195, 225)
turretRow.Font = Enum.Font.GothamMedium
turretRow.Parent = centerStatus
addTextConstraint(turretRow, 7, 10)

local rightControls = Instance.new("Frame")
rightControls.Size = UDim2.new(0.18, 0, 0.80, 0)
rightControls.Position = UDim2.new(0.81, 0, 0.10, 0)
rightControls.BackgroundTransparency = 1
rightControls.Parent = topBar

local lobbyReturnBtn = Instance.new("TextButton")
lobbyReturnBtn.Size = UDim2.new(0.62, 0, 1, 0)
lobbyReturnBtn.Position = UDim2.new(0, 0, 0, 0)
lobbyReturnBtn.BackgroundColor3 = Color3.fromRGB(22, 32, 50)
lobbyReturnBtn.Text = "[M] LOBBY"
lobbyReturnBtn.TextColor3 = Color3.fromRGB(220, 235, 255)
lobbyReturnBtn.Font = Enum.Font.GothamBold
lobbyReturnBtn.Parent = rightControls
addCorner(lobbyReturnBtn, 6)
addPadding(lobbyReturnBtn, 2, 2, 2, 2)
addTextConstraint(lobbyReturnBtn, 7, 10)
addStroke(lobbyReturnBtn, Color3.fromRGB(70, 105, 155), 1.0)

lobbyReturnBtn.MouseButton1Click:Connect(function()
	playUIClick(1.0)
	LobbyActionEvent:FireServer("LeaveToLobby")
end)

local restartBtn = Instance.new("TextButton")
restartBtn.Size = UDim2.new(0.32, 0, 1, 0)
restartBtn.Position = UDim2.new(0.68, 0, 0, 0)
restartBtn.BackgroundColor3 = Color3.fromRGB(28, 36, 52)
restartBtn.Text = "↻"
restartBtn.TextColor3 = Color3.fromRGB(255, 215, 110)
restartBtn.Font = Enum.Font.GothamBlack
restartBtn.Parent = rightControls
addCorner(restartBtn, 6)
addPadding(restartBtn, 2, 2, 2, 2)
addTextConstraint(restartBtn, 8, 12)
addStroke(restartBtn, Color3.fromRGB(80, 105, 145), 1.0)
restartBtn.MouseButton1Click:Connect(function()
	playUIClick(0.95)
	SwitchFactionEvent:FireServer("Restart")
end)

-- ============================================================
-- TACTICAL BATTLEFIELD CAMERA SYSTEM (DYNAMIC PLAYER FOLLOW)
-- ============================================================
local camera = workspace.CurrentCamera
local isTacticalCam = true
local isInspecting = false
local inspectTargetX = -50
local currentCamX = -50
local currentCamY = 32
local currentCamZ = 38
local currentZoom = 38
local targetZoom = 38
local isRightMouseDown = false
local lastMousePos = Vector2.zero

local function getPlayerPosition()
	local char = player.Character
	if char and char.PrimaryPart then
		return char.PrimaryPart.Position
	end
	return nil
end

local cameraDock = Instance.new("Frame")
cameraDock.Name = "CameraDock"
cameraDock.AnchorPoint = Vector2.new(0.5, 0)
cameraDock.Size = UDim2.new(0, 490, 0, 26)
cameraDock.Position = UDim2.new(0.5, 0, 0, 62)
cameraDock.BackgroundColor3 = Color3.fromRGB(12, 16, 24)
cameraDock.BackgroundTransparency = 0.30
cameraDock.Visible = false
cameraDock.Parent = screenGui
addCorner(cameraDock, 6)
addStroke(cameraDock, Color3.fromRGB(50, 75, 110), 1.0)

local camList = Instance.new("UIListLayout")
camList.FillDirection = Enum.FillDirection.Horizontal
camList.HorizontalAlignment = Enum.HorizontalAlignment.Center
camList.VerticalAlignment = Enum.VerticalAlignment.Center
camList.Padding = UDim.new(0, 5)
camList.Parent = cameraDock

local function makeCamBtn(name, text, w, bgColor, textColor, strokeColor)
	local b = Instance.new("TextButton")
	b.Name = name
	b.Size = UDim2.new(0, w, 0, 20)
	b.BackgroundColor3 = bgColor
	b.Text = text
	b.TextColor3 = textColor or Color3.fromRGB(230, 240, 255)
	b.Font = Enum.Font.GothamBold
	b.TextSize = 10
	b.Parent = cameraDock
	addCorner(b, 4)
	addStroke(b, strokeColor or Color3.fromRGB(70, 95, 135), 0.8)
	return b
end

local camModeBtn = makeCamBtn("CamModeBtn", "🎥 TACTICAL [V]", 110, Color3.fromRGB(22, 45, 75), Color3.fromRGB(130, 215, 255), Color3.fromRGB(60, 130, 220))
local camPlayerBtn = makeCamBtn("CamPlayerBtn", "👤 PLAYER [F]", 85, Color3.fromRGB(20, 42, 40), Color3.fromRGB(110, 240, 180), Color3.fromRGB(45, 120, 85))
local camClashBtn = makeCamBtn("CamClashBtn", "⚔️ CLASH [X]", 80, Color3.fromRGB(54, 38, 18), Color3.fromRGB(255, 220, 110), Color3.fromRGB(180, 135, 45))
local camAllyBtn = makeCamBtn("CamAllyBtn", "🏰 ALLY [Z]", 75, Color3.fromRGB(20, 32, 50), Color3.fromRGB(160, 210, 255), Color3.fromRGB(50, 80, 120))
local camEnemyBtn = makeCamBtn("CamEnemyBtn", "🎯 ENEMY [C]", 75, Color3.fromRGB(48, 20, 30), Color3.fromRGB(255, 145, 165), Color3.fromRGB(150, 50, 70))

local camHint = Instance.new("TextLabel")
camHint.Size = UDim2.new(0, 48, 0, 20)
camHint.BackgroundTransparency = 1
camHint.Text = "WASD"
camHint.TextColor3 = Color3.fromRGB(140, 165, 195)
camHint.Font = Enum.Font.GothamMedium
camHint.TextSize = 9
camHint.Parent = cameraDock

local function focusClash()
	isInspecting = true
	local folderName = (myArenaSlotId and myArenaSlotId > 1) and ("ArenaUnits_" .. tostring(myArenaSlotId)) or "Units"
	local uFolder = workspace:FindFirstChild(folderName) or workspace:FindFirstChild("Units")
	if not uFolder then
		inspectTargetX = myArenaOriginX
		return
	end
	local sumX, count = 0, 0
	for _, m in ipairs(uFolder:GetChildren()) do
		if m:IsA("Model") and m.PrimaryPart and math.abs(m.PrimaryPart.Position.Z - myArenaOriginZ) <= 55 and math.abs(m.PrimaryPart.Position.X - myArenaOriginX) <= 140 then
			sumX += m.PrimaryPart.Position.X
			count += 1
		end
	end
	if count > 0 then
		inspectTargetX = math.clamp(sumX / count, myArenaOriginX - 100, myArenaOriginX + 100)
	else
		inspectTargetX = myArenaOriginX
	end
end

local function setCameraMode(tactical)
	isTacticalCam = tactical
	local char = player.Character
	local hum = char and char:FindFirstChildOfClass("Humanoid")
	if isTacticalCam then
		camera.CameraType = Enum.CameraType.Scriptable
		camModeBtn.Text = "🎥 TACTICAL [V]"
		camModeBtn.TextColor3 = Color3.fromRGB(130, 215, 255)
		camModeBtn.BackgroundColor3 = Color3.fromRGB(22, 45, 75)
		isInspecting = false
		if hum then
			hum.WalkSpeed = 16
		end
	else
		camera.CameraType = Enum.CameraType.Custom
		camModeBtn.Text = "👤 3RD PERSON [V]"
		camModeBtn.TextColor3 = Color3.fromRGB(220, 220, 220)
		camModeBtn.BackgroundColor3 = Color3.fromRGB(35, 38, 48)
		if hum then
			camera.CameraSubject = hum
			hum.WalkSpeed = 16
		end
	end
end

player.CharacterAdded:Connect(function(newChar)
	local hum = newChar:WaitForChild("Humanoid", 6)
	if hum then
		hum.WalkSpeed = 16
		if not isTacticalCam then
			camera.CameraSubject = hum
			camera.CameraType = Enum.CameraType.Custom
		end
	end
end)

camModeBtn.MouseButton1Click:Connect(function()
	playUIClick(1.05)
	setCameraMode(not isTacticalCam)
end)

camPlayerBtn.MouseButton1Click:Connect(function()
	playUIClick(1.05)
	if not isTacticalCam then setCameraMode(true) end
	isInspecting = false
end)

camAllyBtn.MouseButton1Click:Connect(function()
	playUIClick(1.05)
	if not isTacticalCam then setCameraMode(true) end
	isInspecting = true
	inspectTargetX = myArenaOriginX - 75
end)

camClashBtn.MouseButton1Click:Connect(function()
	playUIClick(1.1)
	if not isTacticalCam then setCameraMode(true) end
	focusClash()
end)

camEnemyBtn.MouseButton1Click:Connect(function()
	playUIClick(1.05)
	if not isTacticalCam then setCameraMode(true) end
	isInspecting = true
	inspectTargetX = myArenaOriginX + 75
end)

RunService.RenderStepped:Connect(function(dt)
	if isInMatch and isTacticalCam then
		local char = player.Character
		local root = char and (char:FindFirstChild("HumanoidRootPart") or char.PrimaryPart)
		local hum = char and char:FindFirstChildOfClass("Humanoid")

		-- If player moves their character via WASD/controller/arrow keys, automatically return to following player!
		if hum and hum.MoveDirection.Magnitude > 0.05 then
			if isInspecting then
				isInspecting = false
			end
		end

		local rootPos = root and root.Position or Vector3.new(myArenaOriginX - 96, 4.5, myArenaOriginZ + 18)
		local targetX, targetY, targetZ

		if isInspecting then
			targetX = inspectTargetX
			targetY = 4.5 + currentZoom * 0.78
			targetZ = myArenaOriginZ + 38
		else
			-- Smoothly track the player character along the lane!
			local localZ = rootPos.Z - myArenaOriginZ
			targetX = math.clamp(rootPos.X, myArenaOriginX - 105, myArenaOriginX + 105)
			targetY = 4.5 + currentZoom * 0.78
			targetZ = myArenaOriginZ + math.clamp(localZ * 0.35 + currentZoom * 0.88, 26, 54)
		end

		-- Keyboard manual pan keys (Arrow Left / Arrow Right)
		if UserInputService:IsKeyDown(Enum.KeyCode.Left) then
			if not isInspecting then
				isInspecting = true
				inspectTargetX = currentCamX
			end
			inspectTargetX = math.clamp(inspectTargetX - 55 * dt, myArenaOriginX - 105, myArenaOriginX + 105)
		end
		if UserInputService:IsKeyDown(Enum.KeyCode.Right) then
			if not isInspecting then
				isInspecting = true
				inspectTargetX = currentCamX
			end
			inspectTargetX = math.clamp(inspectTargetX + 55 * dt, myArenaOriginX - 105, myArenaOriginX + 105)
		end

		-- Frame-rate independent smooth exponential damping (eliminates dt jitter & micro-stutters)
		local lerpFactor = 1 - math.exp(-14 * dt)
		currentCamX = currentCamX + (targetX - currentCamX) * lerpFactor
		currentCamY = currentCamY + (targetY - currentCamY) * lerpFactor
		currentCamZ = currentCamZ + (targetZ - currentCamZ) * lerpFactor
		currentZoom = currentZoom + (targetZoom - currentZoom) * lerpFactor

		-- Rock-solid tactical perspective:
		-- LookTarget X matches camPos X exactly -> ZERO yaw wobble when moving with D/A!
		-- LookTarget Y is locked to battlefield ground level (4.5) -> ZERO footstep bobbing!
		-- LookTarget Z is locked to the arena center line (0) -> Steady, clear overview!
		local camPos = Vector3.new(currentCamX, currentCamY, currentCamZ)
		local lookTarget = Vector3.new(currentCamX, 4.5, myArenaOriginZ)

		camera.CameraType = Enum.CameraType.Scriptable
		camera.CFrame = CFrame.lookAt(camPos, lookTarget)
	end
end)

UserInputService.InputChanged:Connect(function(input, processed)
	if not isInMatch or not isTacticalCam then return end
	if input.UserInputType == Enum.UserInputType.MouseWheel then
		targetZoom = math.clamp(targetZoom - input.Position.Z * 4.5, 18, 55)
	elseif input.UserInputType == Enum.UserInputType.MouseMovement and isRightMouseDown then
		isInspecting = true
		local deltaX = input.Position.X - lastMousePos.X
		inspectTargetX = math.clamp((inspectTargetX or currentCamX) - deltaX * 0.22, myArenaOriginX - 105, myArenaOriginX + 105)
		lastMousePos = Vector2.new(input.Position.X, input.Position.Y)
	end
end)

UserInputService.InputBegan:Connect(function(input, processed)
	if not isInMatch then return end
	if input.UserInputType == Enum.UserInputType.MouseButton2 then
		isRightMouseDown = true
		lastMousePos = Vector2.new(input.Position.X, input.Position.Y)
		inspectTargetX = currentCamX
	end
end)

UserInputService.InputEnded:Connect(function(input, processed)
	if input.UserInputType == Enum.UserInputType.MouseButton2 then
		isRightMouseDown = false
	end
end)

local bannerLabel = Instance.new("TextLabel")
bannerLabel.AnchorPoint = Vector2.new(0.5, 0)
bannerLabel.Size = UDim2.new(0.70, 0, 0, 34)
bannerLabel.Position = UDim2.new(0.5, 0, 0.135, 4)
bannerLabel.BackgroundTransparency = 1
bannerLabel.Text = ""
bannerLabel.TextColor3 = Color3.fromRGB(255, 225, 90)
bannerLabel.TextStrokeTransparency = 0.1
bannerLabel.Font = Enum.Font.GothamBlack
bannerLabel.Parent = screenGui
addTextConstraint(bannerLabel, 12, 20)

BannerNoticeEvent.OnClientEvent:Connect(function(msg, color, localSoundType)
	if localSoundType then
		playLocalSFX(localSoundType, 1.15)
	end
	bannerLabel.Text = msg
	bannerLabel.TextColor3 = color or Color3.fromRGB(255, 225, 90)
	bannerLabel.TextTransparency = 0
	task.delay(2.8, function()
		if bannerLabel.Text == msg then
			TweenService:Create(bannerLabel, TweenInfo.new(0.5), { TextTransparency = 1 }):Play()
		end
	end)
end)

-- ============================================================
-- BOTTOM COMMAND DECK: Treasury + 6 Troop Cards ([1]..[6]) + Upgrades ([7]/[8]) + Hero Skills ([Q]/[E])
-- ============================================================
bottomDeck = Instance.new("Frame")
bottomDeck.Name = "BottomDeck"
bottomDeck.AnchorPoint = Vector2.new(0.5, 1)
bottomDeck.Size = UDim2.new(0.86, 0, 0, 80)
bottomDeck.Position = UDim2.new(0.5, 0, 1, -8)
bottomDeck.BackgroundColor3 = Color3.fromRGB(11, 14, 22)
bottomDeck.BackgroundTransparency = 0.20
bottomDeck.Visible = false
bottomDeck.Parent = screenGui
addCorner(bottomDeck, 10)
local deckStroke = addStroke(bottomDeck, Color3.fromRGB(60, 80, 115), 1.0)

local bottomDeckConstraint = Instance.new("UISizeConstraint")
bottomDeckConstraint.MaxSize = Vector2.new(1200, 84)
bottomDeckConstraint.MinSize = Vector2.new(660, 74)
bottomDeckConstraint.Parent = bottomDeck

-- 1) TREASURY PILL
local goldBox = Instance.new("Frame")
goldBox.Size = UDim2.new(0.082, 0, 0.86, 0)
goldBox.Position = UDim2.new(0.006, 0, 0.07, 0)
goldBox.BackgroundColor3 = Color3.fromRGB(16, 20, 30)
goldBox.Parent = bottomDeck
addCorner(goldBox, 7)
addStroke(goldBox, Color3.fromRGB(205, 165, 55), 1.0)

local goldHeader = Instance.new("TextLabel")
goldHeader.Visible = false
goldHeader.Parent = goldBox

local goldTitle = Instance.new("TextLabel")
goldTitle.Size = UDim2.new(1, -4, 0.52, 0)
goldTitle.Position = UDim2.new(0, 2, 0.06, 0)
goldTitle.BackgroundTransparency = 1
goldTitle.Text = "45G"
goldTitle.TextColor3 = Color3.fromRGB(255, 220, 80)
goldTitle.Font = Enum.Font.GothamBlack
goldTitle.Parent = goldBox
addTextConstraint(goldTitle, 11, 19)

local incomeSub = Instance.new("TextLabel")
incomeSub.Size = UDim2.new(1, -4, 0.36, 0)
incomeSub.Position = UDim2.new(0, 2, 0.58, 0)
incomeSub.BackgroundTransparency = 1
incomeSub.Text = "+30/w"
incomeSub.TextColor3 = Color3.fromRGB(105, 235, 155)
incomeSub.Font = Enum.Font.GothamBold
incomeSub.Parent = goldBox
addTextConstraint(incomeSub, 7, 10)

-- 2) 6 TROOP CARDS
local unitCards = {}
for i = 1, 6 do
	local card = Instance.new("TextButton")
	card.Name = "UnitCard_" .. i
	card.Size = UDim2.new(0.091, 0, 0.86, 0)
	card.Position = UDim2.new(0.094 + (i - 1) * 0.094, 0, 0.07, 0)
	card.BackgroundColor3 = Color3.fromRGB(18, 23, 34)
	card.Text = ""
	card.AutoButtonColor = true
	card.Parent = bottomDeck
	addCorner(card, 7)
	local stroke = addStroke(card, Color3.fromRGB(65, 85, 120), 1.0)

	local keyBadge = Instance.new("TextLabel")
	keyBadge.Size = UDim2.new(0.48, 0, 0.22, 0)
	keyBadge.Position = UDim2.new(0, 3, 0.04, 0)
	keyBadge.BackgroundTransparency = 1
	keyBadge.Text = "[" .. i .. "] C"
	keyBadge.TextColor3 = Color3.fromRGB(255, 225, 100)
	keyBadge.Font = Enum.Font.GothamBold
	keyBadge.Parent = card
	addTextConstraint(keyBadge, 7, 10)

	local countBadge = Instance.new("TextLabel")
	countBadge.Size = UDim2.new(0.46, 0, 0.22, 0)
	countBadge.Position = UDim2.new(0.52, -3, 0.04, 0)
	countBadge.BackgroundTransparency = 1
	countBadge.Text = "x0"
	countBadge.TextColor3 = Color3.fromRGB(110, 245, 155)
	countBadge.Font = Enum.Font.GothamBlack
	countBadge.Parent = card
	addTextConstraint(countBadge, 7, 10)

	local uName = Instance.new("TextLabel")
	uName.Size = UDim2.new(1, -6, 0.30, 0)
	uName.Position = UDim2.new(0, 3, 0.26, 0)
	uName.BackgroundTransparency = 1
	uName.Text = "Unit Name"
	uName.TextColor3 = Color3.new(1, 1, 1)
	uName.Font = Enum.Font.GothamBold
	uName.Parent = card
	addTextConstraint(uName, 8, 12)

	local uRole = Instance.new("TextLabel")
	uRole.Size = UDim2.new(1, -6, 0.18, 0)
	uRole.Position = UDim2.new(0, 3, 0.57, 0)
	uRole.BackgroundTransparency = 1
	uRole.Text = "Tank"
	uRole.TextColor3 = Color3.fromRGB(165, 185, 215)
	uRole.Font = Enum.Font.GothamMedium
	uRole.Parent = card
	addTextConstraint(uRole, 7, 9)

	local uStats = Instance.new("TextLabel")
	uStats.Visible = false
	uStats.Parent = card

	local costBar = Instance.new("Frame")
	costBar.Size = UDim2.new(1, -6, 0.19, 0)
	costBar.Position = UDim2.new(0, 3, 0.77, 0)
	costBar.BackgroundColor3 = Color3.fromRGB(12, 16, 24)
	costBar.Parent = card
	addCorner(costBar, 4)

	local uCost = Instance.new("TextLabel")
	uCost.Size = UDim2.new(1, -2, 1, 0)
	uCost.Position = UDim2.new(0, 1, 0, 0)
	uCost.BackgroundTransparency = 1
	uCost.Text = "6G"
	uCost.TextColor3 = Color3.fromRGB(255, 215, 80)
	uCost.Font = Enum.Font.GothamBold
	uCost.Parent = costBar
	addTextConstraint(uCost, 7, 10)

	local lockOverlay = Instance.new("Frame")
	lockOverlay.Name = "LockOverlay"
	lockOverlay.Size = UDim2.fromScale(1, 1)
	lockOverlay.BackgroundColor3 = Color3.fromRGB(8, 11, 18)
	lockOverlay.BackgroundTransparency = 0.15
	lockOverlay.Visible = false
	lockOverlay.ZIndex = 5
	lockOverlay.Parent = card
	addCorner(lockOverlay, 7)

	local lockIcon = Instance.new("TextLabel")
	lockIcon.Size = UDim2.new(1, -4, 0.52, 0)
	lockIcon.Position = UDim2.new(0, 2, 0.10, 0)
	lockIcon.BackgroundTransparency = 1
	lockIcon.Text = "🔒 LOCKED"
	lockIcon.TextColor3 = Color3.fromRGB(255, 135, 110)
	lockIcon.Font = Enum.Font.GothamBold
	lockIcon.ZIndex = 6
	lockIcon.Parent = lockOverlay
	addTextConstraint(lockIcon, 8, 11)

	local lockSub = Instance.new("TextLabel")
	lockSub.Size = UDim2.new(1, -4, 0.32, 0)
	lockSub.Position = UDim2.new(0, 2, 0.60, 0)
	lockSub.BackgroundTransparency = 1
	lockSub.Text = "[8] 100G"
	lockSub.TextColor3 = Color3.fromRGB(255, 215, 95)
	lockSub.Font = Enum.Font.GothamMedium
	lockSub.ZIndex = 6
	lockSub.Parent = lockOverlay
	addTextConstraint(lockSub, 7, 9)

	card.MouseButton1Click:Connect(function()
		playUIClick(1.08)
		BuyUnitEvent:FireServer(i)
	end)

	unitCards[i] = {
		Button = card,
		Stroke = stroke,
		KeyBadge = keyBadge,
		NameLabel = uName,
		RoleLabel = uRole,
		StatsLabel = uStats,
		CostLabel = uCost,
		CountBadge = countBadge,
		LockOverlay = lockOverlay,
		LockIcon = lockIcon,
		LockSub = lockSub,
	}
end

-- 3) UPGRADES
local incomeBtn = Instance.new("TextButton")
incomeBtn.Size = UDim2.new(0.108, 0, 0.40, 0)
incomeBtn.Position = UDim2.new(0.662, 0, 0.07, 0)
incomeBtn.BackgroundColor3 = Color3.fromRGB(24, 52, 38)
incomeBtn.Text = "[7] +INCOME (100G)"
incomeBtn.TextColor3 = Color3.fromRGB(210, 255, 200)
incomeBtn.Font = Enum.Font.GothamBold
incomeBtn.Parent = bottomDeck
addCorner(incomeBtn, 6)
addPadding(incomeBtn, 2, 2, 2, 2)
addTextConstraint(incomeBtn, 7, 10)
addStroke(incomeBtn, Color3.fromRGB(75, 165, 105), 1.0)
incomeBtn.MouseButton1Click:Connect(function()
	playUIClick(1.12)
	UpgradeStatEvent:FireServer("Income")
end)

local tierUnlockBtn = Instance.new("TextButton")
tierUnlockBtn.Size = UDim2.new(0.108, 0, 0.40, 0)
tierUnlockBtn.Position = UDim2.new(0.662, 0, 0.53, 0)
tierUnlockBtn.BackgroundColor3 = Color3.fromRGB(26, 48, 80)
tierUnlockBtn.Text = "[8] UNLOCK B (100G)"
tierUnlockBtn.TextColor3 = Color3.fromRGB(195, 235, 255)
tierUnlockBtn.Font = Enum.Font.GothamBold
tierUnlockBtn.Parent = bottomDeck
addCorner(tierUnlockBtn, 6)
addPadding(tierUnlockBtn, 2, 2, 2, 2)
addTextConstraint(tierUnlockBtn, 7, 10)
local tierUnlockStroke = addStroke(tierUnlockBtn, Color3.fromRGB(70, 145, 215), 1.0)
tierUnlockBtn.MouseButton1Click:Connect(function()
	playUIClick(1.15)
	UpgradeStatEvent:FireServer("TierUnlock")
end)

-- 4) HERO COMMANDER SKILLS
local spellButtons = {}
for idx, key in ipairs({"Q", "E"}) do
	local sBtn = Instance.new("TextButton")
	sBtn.Size = UDim2.new(0.106, 0, 0.86, 0)
	sBtn.Position = UDim2.new(0.776 + (idx - 1) * 0.111, 0, 0.07, 0)
	sBtn.BackgroundColor3 = Color3.fromRGB(36, 28, 56)
	sBtn.Text = ""
	sBtn.Parent = bottomDeck
	addCorner(sBtn, 7)
	local st = addStroke(sBtn, Color3.fromRGB(180, 140, 70), 1.0)

	local kLbl = Instance.new("TextLabel")
	kLbl.Size = UDim2.new(1, -4, 0.20, 0)
	kLbl.Position = UDim2.new(0, 2, 0.04, 0)
	kLbl.BackgroundTransparency = 1
	kLbl.Text = "[" .. key .. "] SKILL"
	kLbl.TextColor3 = Color3.fromRGB(255, 225, 95)
	kLbl.Font = Enum.Font.GothamBold
	kLbl.Parent = sBtn
	addTextConstraint(kLbl, 7, 9)

	local nameLbl = Instance.new("TextLabel")
	nameLbl.Size = UDim2.new(1, -4, 0.34, 0)
	nameLbl.Position = UDim2.new(0, 2, 0.26, 0)
	nameLbl.BackgroundTransparency = 1
	nameLbl.Text = "Spell Name"
	nameLbl.TextColor3 = Color3.new(1, 1, 1)
	nameLbl.Font = Enum.Font.GothamBold
	nameLbl.Parent = sBtn
	addTextConstraint(nameLbl, 8, 11)

	local descLbl = Instance.new("TextLabel")
	descLbl.Visible = false
	descLbl.Parent = sBtn

	local cdBar = Instance.new("Frame")
	cdBar.Size = UDim2.new(1, -6, 0.28, 0)
	cdBar.Position = UDim2.new(0, 3, 0.65, 0)
	cdBar.BackgroundColor3 = Color3.fromRGB(12, 16, 24)
	cdBar.Parent = sBtn
	addCorner(cdBar, 4)

	local cdLbl = Instance.new("TextLabel")
	cdLbl.Size = UDim2.new(1, 0, 1, 0)
	cdLbl.BackgroundTransparency = 1
	cdLbl.Text = "READY"
	cdLbl.TextColor3 = Color3.fromRGB(110, 255, 145)
	cdLbl.Font = Enum.Font.GothamBold
	cdLbl.Parent = cdBar
	addTextConstraint(cdLbl, 7, 10)

	sBtn.MouseButton1Click:Connect(function()
		playUIClick(1.2)
		CastSpellEvent:FireServer(key)
	end)

	spellButtons[key] = {
		Button = sBtn,
		Stroke = st,
		NameLabel = nameLbl,
		DescLabel = descLbl,
		CDLabel = cdLbl,
	}
end

UserInputService.InputBegan:Connect(function(input, processed)
	if processed then return end
	if input.KeyCode == Enum.KeyCode.F2 and isAdmin then
		playUIClick(1.1)
		if adminModal then
			adminModal.Visible = not adminModal.Visible
		end
		return
	end
	if input.KeyCode == Enum.KeyCode.M then
		if not isInMatch then
			playUIClick(1.0)
			lobbyBackdrop.Visible = not lobbyBackdrop.Visible
			openLobbyBtn.Visible = not lobbyBackdrop.Visible
		end
	elseif isInMatch then
		if input.KeyCode == Enum.KeyCode.One then
			playUIClick(1.08)
			BuyUnitEvent:FireServer(1)
		elseif input.KeyCode == Enum.KeyCode.Two then
			playUIClick(1.08)
			BuyUnitEvent:FireServer(2)
		elseif input.KeyCode == Enum.KeyCode.Three then
			playUIClick(1.08)
			BuyUnitEvent:FireServer(3)
		elseif input.KeyCode == Enum.KeyCode.Four then
			playUIClick(1.08)
			BuyUnitEvent:FireServer(4)
		elseif input.KeyCode == Enum.KeyCode.Five then
			playUIClick(1.08)
			BuyUnitEvent:FireServer(5)
		elseif input.KeyCode == Enum.KeyCode.Six then
			playUIClick(1.08)
			BuyUnitEvent:FireServer(6)
		elseif input.KeyCode == Enum.KeyCode.Seven then
			playUIClick(1.12)
			UpgradeStatEvent:FireServer("Income")
		elseif input.KeyCode == Enum.KeyCode.Eight then
			playUIClick(1.15)
			UpgradeStatEvent:FireServer("TierUnlock")
		elseif input.KeyCode == Enum.KeyCode.Q then
			playUIClick(1.2)
			CastSpellEvent:FireServer("Q")
		elseif input.KeyCode == Enum.KeyCode.E then
			playUIClick(1.2)
			CastSpellEvent:FireServer("E")
		elseif input.KeyCode == Enum.KeyCode.V then
			playUIClick(1.05)
			setCameraMode(not isTacticalCam)
		elseif input.KeyCode == Enum.KeyCode.F then
			playUIClick(1.05)
			if not isTacticalCam then setCameraMode(true) end
			isInspecting = false
		elseif input.KeyCode == Enum.KeyCode.Z then
			playUIClick(1.05)
			if not isTacticalCam then setCameraMode(true) end
			isInspecting = true
			inspectTargetX = myArenaOriginX - 75
		elseif input.KeyCode == Enum.KeyCode.X then
			playUIClick(1.1)
			if not isTacticalCam then setCameraMode(true) end
			focusClash()
		elseif input.KeyCode == Enum.KeyCode.C then
			playUIClick(1.05)
			if not isTacticalCam then setCameraMode(true) end
			isInspecting = true
			inspectTargetX = myArenaOriginX + 75
		end
	end
end)

local function formatTurretList(list)
	return ""
end

local function rebuildLobbyPlayersUI(lobbyPlayers)
	for _, child in ipairs(playerScroll:GetChildren()) do
		if child:IsA("Frame") then
			child:Destroy()
		end
	end
	if not lobbyPlayers then return end

	local otherCount = 0
	for _, pInfo in ipairs(lobbyPlayers) do
		local row = Instance.new("Frame")
		row.Size = UDim2.new(1, -6, 0, 38)
		row.BackgroundColor3 = Color3.fromRGB(22, 28, 42)
		row.Parent = playerScroll
		addCorner(row, 7)

		local fData = FactionConfig.Factions[pInfo.Faction] or FactionConfig.Factions.Kingdom
		local nameLbl = Instance.new("TextLabel")
		nameLbl.Size = UDim2.new(0.62, -10, 1, 0)
		nameLbl.Position = UDim2.new(0, 10, 0, 0)
		nameLbl.BackgroundTransparency = 1
		local winStr = (pInfo.Wins and pInfo.Wins > 0) and string.format("  •  🏆 %dW", pInfo.Wins) or ""
		nameLbl.Text = string.format("%s %s (%s)%s", fData.Badge or "", pInfo.DisplayName, fData.DisplayName or pInfo.Faction, winStr)
		nameLbl.TextColor3 = Color3.new(1, 1, 1)
		nameLbl.Font = Enum.Font.GothamBold
		nameLbl.TextXAlignment = Enum.TextXAlignment.Left
		nameLbl.Parent = row
		addTextConstraint(nameLbl, 10, 13)

		if pInfo.UserId == player.UserId then
			local statusStr = pInfo.InMatch and "IN MATCH" or (pInfo.QueuedPvP and "QUEUED 1v1" or "LOBBY")
			local youBadge = Instance.new("TextLabel")
			youBadge.Size = UDim2.new(0.32, 0, 0, 26)
			youBadge.Position = UDim2.new(0.65, 0, 0.5, -13)
			youBadge.BackgroundColor3 = pInfo.QueuedPvP and Color3.fromRGB(140, 40, 75) or Color3.fromRGB(34, 48, 72)
			youBadge.Text = "YOU (" .. statusStr .. ")"
			youBadge.TextColor3 = pInfo.QueuedPvP and Color3.fromRGB(255, 195, 215) or Color3.fromRGB(145, 215, 255)
			youBadge.Font = Enum.Font.GothamBlack
			youBadge.Parent = row
			addCorner(youBadge, 6)
			addTextConstraint(youBadge, 8, 11)

			if joinQueueBtn then
				if pInfo.QueuedPvP then
					joinQueueBtn.Text = "⏳ IN 1v1 QUEUE (CLICK TO LEAVE)"
					joinQueueBtn.BackgroundColor3 = Color3.fromRGB(145, 85, 30)
				else
					joinQueueBtn.Text = "⚔️ JOIN 1v1 PVP QUEUE"
					joinQueueBtn.BackgroundColor3 = Color3.fromRGB(195, 45, 85)
				end
			end
		else
			otherCount += 1
			local chalBtn = Instance.new("TextButton")
			chalBtn.Size = UDim2.new(0.32, 0, 0, 28)
			chalBtn.Position = UDim2.new(0.65, 0, 0.5, -14)
			if pInfo.InMatch then
				chalBtn.BackgroundColor3 = Color3.fromRGB(48, 56, 72)
				chalBtn.Text = pInfo.ArenaName or string.format("⚔️ ARENA #%d", pInfo.ArenaSlotId or 1)
				chalBtn.TextColor3 = Color3.fromRGB(175, 195, 225)
			else
				chalBtn.BackgroundColor3 = Color3.fromRGB(195, 45, 75)
				chalBtn.Text = "⚔️ CHALLENGE 1v1"
				chalBtn.TextColor3 = Color3.new(1, 1, 1)
			end
			chalBtn.Font = Enum.Font.GothamBlack
			chalBtn.Parent = row
			addCorner(chalBtn, 6)
			addTextConstraint(chalBtn, 8, 11)

			chalBtn.MouseButton1Click:Connect(function()
				playUIClick(1.18)
				LobbyActionEvent:FireServer("ChallengePlayer", pInfo.UserId)
			end)
		end
	end

	if otherCount == 0 then
		local note = Instance.new("Frame")
		note.Size = UDim2.new(1, -6, 0, 44)
		note.BackgroundColor3 = Color3.fromRGB(18, 22, 34)
		note.Parent = playerScroll
		addCorner(note, 7)

		local noteTxt = Instance.new("TextLabel")
		noteTxt.Size = UDim2.new(1, -16, 1, 0)
		noteTxt.Position = UDim2.new(0, 8, 0, 0)
		noteTxt.BackgroundTransparency = 1
		noteTxt.Text = "Invite a friend to 1v1, or launch a 1v1 vs Bot Commander on the left!"
		noteTxt.TextColor3 = Color3.fromRGB(155, 175, 205)
		noteTxt.TextWrapped = true
		noteTxt.Font = Enum.Font.GothamMedium
		noteTxt.Parent = note
		addTextConstraint(noteTxt, 9, 12)
	end
end

StateUpdateEvent.OnClientEvent:Connect(function(state)
	lastReceivedState = state
	local wasInMatch = isInMatch
	isInMatch = state.InMatch == true
	myArenaOriginX = state.ArenaOriginX or 0
	myArenaOriginZ = state.ArenaOriginZ or 0
	myArenaSlotId = state.ArenaSlotId or 1
	myArenaName = state.ArenaName or ("ARENA #" .. tostring(myArenaSlotId))

	if launchBotBtn then
		launchBotBtn.Text = "⚔️ LAUNCH 1v1 VS BOT"
		launchBotBtn.BackgroundColor3 = Color3.fromRGB(38, 170, 85)
	end

	if isInMatch ~= wasInMatch then
		topBar.Visible = isInMatch
		bottomDeck.Visible = isInMatch
		cameraDock.Visible = isInMatch
		if isInMatch then
			lobbyBackdrop.Visible = false
			openLobbyBtn.Visible = false
			setCameraMode(true)
			isInspecting = false
			local pPos = getPlayerPosition()
			currentCamX = pPos and pPos.X or (myArenaOriginX - 50)
			currentCamY = pPos and (pPos.Y + 28) or 32
			currentCamZ = myArenaOriginZ + 38
			currentZoom = 38
			targetZoom = 38
		else
			lobbyBackdrop.Visible = false
			openLobbyBtn.Visible = true
			setCameraMode(false)
		end
		if adminPillBtn then
			adminPillBtn.Position = isInMatch and UDim2.new(1, -14, 0, 94) or UDim2.new(1, -14, 0, 10)
		end
	end

	local prevFaction = mySelectedFaction
	mySelectedFaction = state.SelectedLobbyFaction or mySelectedFaction
	if state.LobbyLoadout and #state.LobbyLoadout == 6 then
		myLoadout = state.LobbyLoadout
	end
	if prevFaction ~= mySelectedFaction then
		rebuildDeckPickerUI()
	elseif updateSideWarDeckUI then
		updateSideWarDeckUI()
	end

	for fName, info in pairs(lobbyFactionCards) do
		local sel = (fName == mySelectedFaction)
		info.Stroke.Thickness = sel and 3.0 or 1.2
		info.Button.BackgroundTransparency = sel and 0 or 0.42
	end
	if FactionConfig.Factions[mySelectedFaction] then
		lobbyModalStroke.Color = FactionConfig.Factions[mySelectedFaction].AccentColor
	end

	rebuildLobbyPlayersUI(state.LobbyPlayers)

	for fName, info in pairs(factionButtons) do
		local isSelected = (fName == state.PlayerFaction)
		info.Stroke.Thickness = isSelected and 2.6 or 1
		info.Button.BackgroundTransparency = isSelected and 0 or 0.42
	end

	waveTitle.Text = string.format("%s  •  WAVE %d  •  %ds  (+%dG)  •  VS %s", myArenaName, state.WaveNumber, state.WaveTimer, state.WaveIncome or 30, state.OpponentName or "BOT")
	hpVsText.Text = string.format("%d HP          CITADELS          %d HP", state.PlayerBaseHP, state.EnemyBaseHP)
	allyHPFill.Size = UDim2.new(0.495 * math.clamp(state.PlayerBaseHP / state.MaxBaseHP, 0, 1), 0, 1, 0)
	enemyHPFill.Size = UDim2.new(0.495 * math.clamp(state.EnemyBaseHP / state.MaxBaseHP, 0, 1), 0, 1, 0)

	turretRow.Text = string.format("ALLY %s    │    ENEMY %s", formatTurretList(state.PlayerTurrets), formatTurretList(state.EnemyTurrets))

	goldTitle.Text = tostring(state.Gold) .. "G"
	incomeSub.Text = string.format("+%d/w", state.WaveIncome or 30)

	local incomeLevel = state.IncomeLevel or 1
	if incomeLevel >= 5 or (state.IncomeCost or 0) <= 0 then
		incomeBtn.Text = "[7] MAX INCOME (LV5)"
		incomeBtn.BackgroundColor3 = Color3.fromRGB(24, 65, 42)
	else
		incomeBtn.Text = string.format("[7] +INCOME LV%d (%dG)", incomeLevel, state.IncomeCost)
		incomeBtn.BackgroundColor3 = (state.Gold >= state.IncomeCost) and Color3.fromRGB(24, 52, 38) or Color3.fromRGB(16, 28, 22)
	end

	-- Tier Unlock Button state (1 -> Unlock B-Tier 150G; 2 -> Unlock A-Tier 300G; 3 -> Max Unlocked)
	local tierUnlock = state.TierUnlock or 1
	local tierCost = state.TierUnlockCost or 100
	if tierUnlock == 1 then
		tierUnlockBtn.Text = string.format("[8] UNLOCK B (%dG)", tierCost)
		tierUnlockBtn.BackgroundColor3 = (state.Gold >= tierCost) and Color3.fromRGB(26, 56, 95) or Color3.fromRGB(18, 28, 45)
		tierUnlockStroke.Color = TIER_COLORS.B
	elseif tierUnlock == 2 then
		tierUnlockBtn.Text = string.format("[8] UNLOCK A (%dG)", tierCost)
		tierUnlockBtn.BackgroundColor3 = (state.Gold >= tierCost) and Color3.fromRGB(110, 80, 20) or Color3.fromRGB(38, 30, 16)
		tierUnlockStroke.Color = TIER_COLORS.A
	else
		tierUnlockBtn.Text = "[8] MAX TIERS ✓"
		tierUnlockBtn.BackgroundColor3 = Color3.fromRGB(24, 65, 42)
		tierUnlockStroke.Color = TIER_COLORS.C
	end

	local fData = FactionConfig.Factions[state.PlayerFaction]
	local activeLoadout = (state.PlayerLoadout and #state.PlayerLoadout == 6) and state.PlayerLoadout or myLoadout
	if fData then
		topStroke.Color = fData.AccentColor
		deckStroke.Color = fData.AccentColor

		for slotIdx = 1, 6 do
			local unitId = activeLoadout[slotIdx] or slotIdx
			local uDef = fData.Units[unitId]
			local card = unitCards[slotIdx]
			if uDef and card then
				local owned = state.PlayerLineup[slotIdx] or 0
				local dynCost = FactionConfig.GetDynamicCost(uDef, owned)
				local bucketSize = FactionConfig.GetPriceBucketSize and FactionConfig.GetPriceBucketSize(uDef) or (uDef.PriceBucketSize or 2)
				local inBucket = owned % bucketSize

				local isUnlocked = true
				if uDef.Tier == "B" and tierUnlock < 2 then
					isUnlocked = false
					card.LockOverlay.Visible = true
					card.LockIcon.Text = "🔒 B-TIER"
					card.LockSub.Text = "[8] 100G"
				elseif uDef.Tier == "A" and tierUnlock < 3 then
					isUnlocked = false
					card.LockOverlay.Visible = true
					card.LockIcon.Text = "🔒 A-TIER"
					card.LockSub.Text = (tierUnlock < 2) and "Need B first" or "[8] 250G"
				else
					card.LockOverlay.Visible = false
				end

				card.KeyBadge.Text = string.format("[%d] %s", slotIdx, uDef.Tier)
				card.KeyBadge.TextColor3 = TIER_COLORS[uDef.Tier] or Color3.new(1, 1, 1)
				card.NameLabel.Text = uDef.Name
				card.RoleLabel.Text = string.format("%s · %d HP", uDef.Role, uDef.HP)
				card.CostLabel.Text = string.format("%dG (%d/%d)", dynCost, inBucket, bucketSize)
				card.CountBadge.Text = string.format("x%d", owned)

				local canAfford = isUnlocked and (state.Gold >= dynCost)
				card.Button.BackgroundColor3 = canAfford and Color3.fromRGB(26, 34, 48) or Color3.fromRGB(16, 20, 28)
				card.Stroke.Color = canAfford and (TIER_COLORS[uDef.Tier] or fData.AccentColor) or Color3.fromRGB(45, 55, 72)
			end
		end

		for _, key in ipairs({"Q", "E"}) do
			local sDef = fData.Spells[key]
			local sUI = spellButtons[key]
			if sDef and sUI then
				sUI.NameLabel.Text = sDef.Name
				local cd = state.SpellCooldowns[key] or 0
				if cd > 0 then
					sUI.CDLabel.Text = tostring(cd) .. "s CD"
					sUI.CDLabel.TextColor3 = Color3.fromRGB(255, 120, 120)
					sUI.Button.BackgroundColor3 = Color3.fromRGB(24, 26, 36)
				else
					sUI.CDLabel.Text = "READY"
					sUI.CDLabel.TextColor3 = Color3.fromRGB(110, 255, 145)
					sUI.Button.BackgroundColor3 = Color3.fromRGB(36, 28, 56)
				end
			end
		end
	end
end)

-- ============================================================
-- ADMIN / DEVELOPER CONSOLE UI & SHORTCUT BUTTON
-- ============================================================
if isAdmin then
	adminPillBtn = Instance.new("TextButton")
	adminPillBtn.Name = "AdminPillBtn"
	adminPillBtn.AnchorPoint = Vector2.new(1, 0)
	adminPillBtn.Size = UDim2.new(0, 92, 0, 28)
	adminPillBtn.Position = UDim2.new(1, -12, 0, 10)
	adminPillBtn.BackgroundColor3 = Color3.fromRGB(18, 16, 26)
	adminPillBtn.BackgroundTransparency = 0.25
	adminPillBtn.Text = "DEV [F2]"
	adminPillBtn.TextColor3 = Color3.fromRGB(240, 215, 120)
	adminPillBtn.Font = Enum.Font.GothamBold
	adminPillBtn.Visible = true
	adminPillBtn.Parent = screenGui
	addCorner(adminPillBtn, 6)
	addStroke(adminPillBtn, Color3.fromRGB(140, 120, 190), 1.0)
	addTextConstraint(adminPillBtn, 8, 11)

	adminModal = Instance.new("Frame")
	adminModal.Name = "AdminModal"
	adminModal.AnchorPoint = Vector2.new(0.5, 0.5)
	adminModal.Size = UDim2.new(0, 440, 0, 390)
	adminModal.Position = UDim2.new(0.5, 0, 0.5, 0)
	adminModal.BackgroundColor3 = Color3.fromRGB(15, 18, 28)
	adminModal.BackgroundTransparency = 0.02
	adminModal.Visible = false
	adminModal.ZIndex = 60
	adminModal.Parent = screenGui
	addCorner(adminModal, 14)
	addStroke(adminModal, Color3.fromRGB(255, 215, 0), 2.2)
	addGradient(adminModal, Color3.fromRGB(22, 26, 42), Color3.fromRGB(12, 14, 22), 90)

	adminPillBtn.MouseButton1Click:Connect(function()
		playUIClick(1.05)
		adminModal.Visible = not adminModal.Visible
	end)

	local adminHeader = Instance.new("Frame")
	adminHeader.Size = UDim2.new(1, 0, 0, 48)
	adminHeader.BackgroundTransparency = 1
	adminHeader.Parent = adminModal

	local headerTitle = Instance.new("TextLabel")
	headerTitle.Size = UDim2.new(1, -50, 0, 24)
	headerTitle.Position = UDim2.new(0, 16, 0, 6)
	headerTitle.BackgroundTransparency = 1
	headerTitle.Text = "⚡ DEVELOPER & ADMIN CONSOLE"
	headerTitle.TextColor3 = Color3.fromRGB(255, 225, 95)
	headerTitle.Font = Enum.Font.GothamBlack
	headerTitle.TextXAlignment = Enum.TextXAlignment.Left
	headerTitle.Parent = adminHeader
	addTextConstraint(headerTitle, 12, 16)

	local headerSub = Instance.new("TextLabel")
	headerSub.Size = UDim2.new(1, -50, 0, 16)
	headerSub.Position = UDim2.new(0, 16, 0, 28)
	headerSub.BackgroundTransparency = 1
	headerSub.Text = "Authorized: " .. (player.DisplayName or "Owner") .. " • Hotkey: [F2]"
	headerSub.TextColor3 = Color3.fromRGB(160, 180, 215)
	headerSub.Font = Enum.Font.GothamBold
	headerSub.TextXAlignment = Enum.TextXAlignment.Left
	headerSub.Parent = adminHeader
	addTextConstraint(headerSub, 8, 11)

	local closeAdminBtn = Instance.new("TextButton")
	closeAdminBtn.AnchorPoint = Vector2.new(1, 0)
	closeAdminBtn.Size = UDim2.new(0, 32, 0, 32)
	closeAdminBtn.Position = UDim2.new(1, -10, 0, 8)
	closeAdminBtn.BackgroundColor3 = Color3.fromRGB(50, 25, 30)
	closeAdminBtn.Text = "✕"
	closeAdminBtn.TextColor3 = Color3.fromRGB(255, 120, 120)
	closeAdminBtn.Font = Enum.Font.GothamBold
	closeAdminBtn.Parent = adminHeader
	addCorner(closeAdminBtn, 8)
	addStroke(closeAdminBtn, Color3.fromRGB(220, 70, 70), 1.2)
	closeAdminBtn.MouseButton1Click:Connect(function()
		playUIClick(0.9)
		adminModal.Visible = false
	end)

	local statusLbl = Instance.new("TextLabel")
	statusLbl.Size = UDim2.new(1, -32, 0, 24)
	statusLbl.Position = UDim2.new(0, 16, 1, -32)
	statusLbl.BackgroundTransparency = 1
	statusLbl.Text = "Ready. In chat: /gold 1000, /tier 3, /wave, /win"
	statusLbl.TextColor3 = Color3.fromRGB(130, 205, 255)
	statusLbl.Font = Enum.Font.Gotham
	statusLbl.TextXAlignment = Enum.TextXAlignment.Center
	statusLbl.Parent = adminModal
	addTextConstraint(statusLbl, 8, 11)

	local function setStatus(txt, isGood)
		statusLbl.Text = txt
		statusLbl.TextColor3 = isGood and Color3.fromRGB(120, 255, 150) or Color3.fromRGB(255, 140, 120)
	end

	local content = Instance.new("Frame")
	content.Size = UDim2.new(1, -32, 1, -88)
	content.Position = UDim2.new(0, 16, 0, 52)
	content.BackgroundTransparency = 1
	content.Parent = adminModal

	local function makeSectionTitle(yPos, text)
		local lbl = Instance.new("TextLabel")
		lbl.Size = UDim2.new(1, 0, 0, 18)
		lbl.Position = UDim2.new(0, 0, 0, yPos)
		lbl.BackgroundTransparency = 1
		lbl.Text = text
		lbl.TextColor3 = Color3.fromRGB(255, 215, 95)
		lbl.Font = Enum.Font.GothamBlack
		lbl.TextXAlignment = Enum.TextXAlignment.Left
		lbl.Parent = content
		addTextConstraint(lbl, 9, 12)
		return lbl
	end

	makeSectionTitle(0, "💰 INSTANT GOLD (IN-MATCH)")

	local goldGrid = Instance.new("Frame")
	goldGrid.Size = UDim2.new(1, 0, 0, 36)
	goldGrid.Position = UDim2.new(0, 0, 0, 22)
	goldGrid.BackgroundTransparency = 1
	goldGrid.Parent = content

	local goldAmounts = { 500, 1000, 5000, 50000 }
	for i, amt in ipairs(goldAmounts) do
		local gBtn = Instance.new("TextButton")
		gBtn.Size = UDim2.new(0.23, 0, 1, 0)
		gBtn.Position = UDim2.new((i - 1) * 0.256, 0, 0, 0)
		gBtn.BackgroundColor3 = Color3.fromRGB(36, 42, 28)
		gBtn.Text = string.format("+%s G", amt >= 1000 and (amt / 1000 .. "k") or tostring(amt))
		gBtn.TextColor3 = Color3.fromRGB(255, 230, 110)
		gBtn.Font = Enum.Font.GothamBlack
		gBtn.Parent = goldGrid
		addCorner(gBtn, 8)
		addStroke(gBtn, Color3.fromRGB(255, 200, 60), 1.2)
		addTextConstraint(gBtn, 9, 13)

		gBtn.MouseButton1Click:Connect(function()
			playUIClick(1.1)
			AdminCommandEvent:FireServer("addgold", amt)
			setStatus("⚡ Requested +" .. amt .. " Gold!", true)
		end)
	end

	local customRow = Instance.new("Frame")
	customRow.Size = UDim2.new(1, 0, 0, 32)
	customRow.Position = UDim2.new(0, 0, 0, 64)
	customRow.BackgroundTransparency = 1
	customRow.Parent = content

	local customInput = Instance.new("TextBox")
	customInput.Size = UDim2.new(0.68, 0, 1, 0)
	customInput.Position = UDim2.new(0, 0, 0, 0)
	customInput.BackgroundColor3 = Color3.fromRGB(18, 22, 34)
	customInput.PlaceholderText = "Type custom amount (e.g. 2500)..."
	customInput.PlaceholderColor3 = Color3.fromRGB(110, 125, 155)
	customInput.Text = ""
	customInput.TextColor3 = Color3.fromRGB(255, 255, 255)
	customInput.Font = Enum.Font.GothamMedium
	customInput.ClearTextOnFocus = false
	customInput.Parent = customRow
	addCorner(customInput, 8)
	addStroke(customInput, Color3.fromRGB(80, 110, 160), 1)
	addPadding(customInput, 2, 2, 8, 8)
	addTextConstraint(customInput, 9, 12)

	local customBtn = Instance.new("TextButton")
	customBtn.Size = UDim2.new(0.30, 0, 1, 0)
	customBtn.Position = UDim2.new(0.70, 0, 0, 0)
	customBtn.BackgroundColor3 = Color3.fromRGB(40, 75, 45)
	customBtn.Text = "+ ADD GOLD"
	customBtn.TextColor3 = Color3.fromRGB(130, 255, 150)
	customBtn.Font = Enum.Font.GothamBlack
	customBtn.Parent = customRow
	addCorner(customBtn, 8)
	addStroke(customBtn, Color3.fromRGB(80, 210, 110), 1.2)
	addTextConstraint(customBtn, 9, 12)

	customBtn.MouseButton1Click:Connect(function()
		local val = tonumber(customInput.Text)
		if val and val > 0 then
			playUIClick(1.1)
			AdminCommandEvent:FireServer("addgold", val)
			setStatus("⚡ Requested +" .. val .. " Gold!", true)
			customInput.Text = ""
		else
			setStatus("⚠️ Please enter a valid number!", false)
		end
	end)

	makeSectionTitle(104, "⚔️ COMBAT & MATCH CONTROLS")

	local matchBtnContainer = Instance.new("Frame")
	matchBtnContainer.Size = UDim2.new(1, 0, 0, 120)
	matchBtnContainer.Position = UDim2.new(0, 0, 0, 126)
	matchBtnContainer.BackgroundTransparency = 1
	matchBtnContainer.Parent = content

	local matchActions = {
		{ name = "UNLOCK ALL TIERS", cmd = "unlocktiers", arg = 3, col = Color3.fromRGB(255, 205, 75), bg = Color3.fromRGB(42, 38, 22), border = Color3.fromRGB(220, 175, 40) },
		{ name = "NEXT WAVE NOW", cmd = "nextwave", arg = nil, col = Color3.fromRGB(120, 225, 255), bg = Color3.fromRGB(22, 40, 52), border = Color3.fromRGB(70, 170, 220) },
		{ name = "HEAL CITADEL 100%", cmd = "healbase", arg = nil, col = Color3.fromRGB(125, 255, 145), bg = Color3.fromRGB(22, 45, 30), border = Color3.fromRGB(60, 195, 90) },
		{ name = "WIPE ENEMY UNITS", cmd = "wipeenemies", arg = nil, col = Color3.fromRGB(255, 150, 95), bg = Color3.fromRGB(50, 30, 24), border = Color3.fromRGB(220, 105, 60) },
		{ name = "🏆 INSTANT VICTORY", cmd = "winmatch", arg = nil, col = Color3.fromRGB(255, 235, 120), bg = Color3.fromRGB(50, 42, 20), border = Color3.fromRGB(240, 200, 50) },
	}

	for i, act in ipairs(matchActions) do
		local btn = Instance.new("TextButton")
		if i <= 4 then
			local row = math.floor((i - 1) / 2)
			local col = (i - 1) % 2
			btn.Size = UDim2.new(0.485, 0, 0, 32)
			btn.Position = UDim2.new(col * 0.515, 0, 0, row * 38)
		else
			btn.Size = UDim2.new(1, 0, 0, 34)
			btn.Position = UDim2.new(0, 0, 0, 76)
		end
		btn.BackgroundColor3 = act.bg
		btn.Text = act.name
		btn.TextColor3 = act.col
		btn.Font = Enum.Font.GothamBlack
		btn.Parent = matchBtnContainer
		addCorner(btn, 8)
		addStroke(btn, act.border, 1.2)
		addTextConstraint(btn, 8, 12)

		btn.MouseButton1Click:Connect(function()
			playUIClick(1.05)
			AdminCommandEvent:FireServer(act.cmd, act.arg)
			setStatus("⚡ Executed: " .. act.name, true)
		end)
	end
end
