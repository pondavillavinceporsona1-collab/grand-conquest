const fs = require('fs');
const path = require('path');

const EB2D_SRC = path.join(__dirname, '..', 'epic_battle_2d', 'src');
const ROBLOX_SRC = path.join(__dirname, '..', 'epic_battle', 'src');

// Read conquest_relics.js
let relicsCode = fs.readFileSync(path.join(EB2D_SRC, 'conquest_relics.js'), 'utf-8');
// Mock window
const mockWindow = {};
eval(relicsCode.replace(/window\./g, 'mockWindow.'));
const relics = mockWindow.CONQUEST_RELICS || [];

// Read conquest_missions.js
let missionsCode = fs.readFileSync(path.join(EB2D_SRC, 'conquest_missions.js'), 'utf-8');
eval(missionsCode.replace(/window\./g, 'mockWindow.'));
const missions = mockWindow.CONQUEST_MISSIONS || [];

console.log(`Loaded ${relics.length} relics and ${missions.length} missions.`);

function toLuauValue(val, indent = 0) {
  const pad = '  '.repeat(indent);
  if (val === null || val === undefined) return 'nil';
  if (typeof val === 'boolean') return val ? 'true' : 'false';
  if (typeof val === 'number') return val.toString();
  if (typeof val === 'string') return JSON.stringify(val);
  if (Array.isArray(val)) {
    if (val.length === 0) return '{}';
    // If simple array of primitives
    if (val.every(x => typeof x === 'number' || typeof x === 'string')) {
      return '{ ' + val.map(x => toLuauValue(x)).join(', ') + ' }';
    }
    const lines = val.map(x => `${pad}  ${toLuauValue(x, indent + 1)},`);
    return `{\n${lines.join('\n')}\n${pad}}`;
  }
  if (typeof val === 'object') {
    const keys = Object.keys(val);
    if (keys.length === 0) return '{}';
    const lines = keys.map(k => {
      const luauKey = /^[a-zA-Z_][a-zA-Z0-9_]*$/.test(k) ? k : `[${JSON.stringify(k)}]`;
      return `${pad}  ${luauKey} = ${toLuauValue(val[k], indent + 1)},`;
    });
    return `{\n${lines.join('\n')}\n${pad}}`;
  }
  return 'nil';
}

// 1. Generate ConquestConfig.luau
let configLuau = `-- =========================================================================
-- EPIC BATTLE! — GRAND CONQUEST CONFIGURATION & MISSION REGISTRY
-- Production Luau Port: 6 Chapters, 24 Missions, 15 War Relics
-- =========================================================================

local ConquestConfig = {}

-- -------------------------------------------------------------------------
-- 15 WAR RELICS CATALOG & COSTS
-- -------------------------------------------------------------------------
ConquestConfig.Relics = {
`;

relics.forEach(r => {
  const cleanRelic = {
    Id: r.id,
    Name: r.name,
    StarsReq: r.starsReq,
    RPCost: r.rpCost,
    Icon: r.icon,
    Description: r.desc
  };
  configLuau += `  ["${r.id}"] = {\n`;
  configLuau += `    Id = "${r.id}",\n`;
  configLuau += `    Name = ${JSON.stringify(r.name)},\n`;
  configLuau += `    StarsReq = ${r.starsReq},\n`;
  configLuau += `    RPCost = ${r.rpCost},\n`;
  configLuau += `    Icon = ${JSON.stringify(r.icon)},\n`;
  configLuau += `    Description = ${JSON.stringify(r.desc)},\n`;
  configLuau += `  },\n`;
});
configLuau += `}\n\n`;

configLuau += `-- -------------------------------------------------------------------------
-- 24 CAMPAIGN MISSIONS (CHAPTERS 1 - 6)
-- -------------------------------------------------------------------------
ConquestConfig.Missions = {
`;

missions.forEach(m => {
  configLuau += `  ["${m.id}"] = {\n`;
  configLuau += `    Id = "${m.id}",\n`;
  configLuau += `    Chapter = ${m.chapter},\n`;
  configLuau += `    Name = ${JSON.stringify(m.name)},\n`;
  configLuau += `    TerritoryName = ${JSON.stringify(m.territoryName)},\n`;
  configLuau += `    PinCoord = Vector2.new(${m.pinCoord[0]}, ${m.pinCoord[1]}),\n`;
  configLuau += `    Prerequisites = ${toLuauValue(m.prerequisites)},\n`;
  configLuau += `    Archetype = "${m.archetype}",\n`;
  configLuau += `    EnemyFaction = "${m.enemyFaction}",\n`;
  configLuau += `    AIDifficulty = "${m.aiDifficulty}",\n`;
  configLuau += `    StartingGold = ${m.startingGold},\n`;
  configLuau += `    EnemyDeck = ${toLuauValue(m.enemyDeck)},\n`;
  configLuau += `    Briefing = ${JSON.stringify(m.briefing)},\n`;
  if (m.timeLimitSeconds) {
    configLuau += `    TimeLimitSeconds = ${m.timeLimitSeconds},\n`;
  }
  if (m.mutator) {
    configLuau += `    Mutator = ${toLuauValue(m.mutator, 2)},\n`;
  }
  if (m.bossProps) {
    configLuau += `    BossProps = ${toLuauValue(m.bossProps, 2)},\n`;
  }
  if (m.waves) {
    configLuau += `    Waves = ${toLuauValue(m.waves, 2)},\n`;
  }
  configLuau += `    StarCriteria = ${toLuauValue(m.starCriteria, 2)},\n`;
  configLuau += `    Rewards = ${toLuauValue(m.rewards, 2)},\n`;
  configLuau += `  },\n`;
});
configLuau += `}\n\n`;

configLuau += `
-- Quick Access Helpers
function ConquestConfig.GetMission(missionId: string)
  return ConquestConfig.Missions[missionId]
end

function ConquestConfig.GetRelic(relicId: string)
  return ConquestConfig.Relics[relicId]
end

function ConquestConfig.GetMissionsForChapter(chapterNum: number)
  local list = {}
  for _, m in pairs(ConquestConfig.Missions) do
    if m.Chapter == chapterNum then
      table.insert(list, m)
    end
  end
  table.sort(list, function(a, b) return a.Id < b.Id end)
  return list
end

function ConquestConfig.IsMissionUnlocked(missionId: string, completedMissions: { [string]: any })
  local m = ConquestConfig.Missions[missionId]
  if not m then return false end
  if not m.Prerequisites or #m.Prerequisites == 0 then return true end
  for _, reqId in ipairs(m.Prerequisites) do
    local record = completedMissions[reqId]
    if not record or (record.Stars or 0) <= 0 then
      return false
    end
  end
  return true
end

return ConquestConfig
`;

const configOutPath = path.join(ROBLOX_SRC, 'ReplicatedStorage', 'Conquest', 'ConquestConfig.luau');
fs.writeFileSync(configOutPath, configLuau, 'utf-8');
console.log(`Wrote ${configOutPath} (${(fs.statSync(configOutPath).size / 1024).toFixed(1)} KB)`);
