// =========================================================================
// EPIC BATTLE! — GRAND CONQUEST: PERSISTENT STORAGE & CHECKSUM ENGINE
// =========================================================================

const CONQUEST_STORAGE_KEY = "EB2D_GRAND_CONQUEST_SAVE_v2";
const CHECKSUM_SALT = "GEO_TERRA_SALT_v2";

function computeChecksum(dataObj) {
  const clone = { ...dataObj };
  delete clone.checksum;
  const str = JSON.stringify(clone) + CHECKSUM_SALT;
  let hash = 2166136261;
  for (let i = 0; i < str.length; i++) {
    hash ^= str.charCodeAt(i);
    hash = Math.imul(hash, 16777619);
  }
  return (hash >>> 0).toString(16);
}

function createDefaultSave(starterFaction = "solaris") {
  const starterUnits = {
    solaris: ["sol_1", "sol_2", "sol_3"],
    aegis: ["aeg_1", "aeg_2", "aeg_3"],
    hive: ["hiv_1", "hiv_2", "hiv_3"],
    hellfire: ["hel_1", "hel_2", "hel_3"],
    void: ["voi_1", "voi_2", "voi_3"],
    mechanica: ["mec_1", "mec_2", "mec_3"]
  };
  const starters = starterUnits[starterFaction] || starterUnits.solaris;

  const save = {
    schemaVersion: 2,
    playerCommanderFaction: starterFaction,
    accountGoldVault: 0,
    totalConquestStars: 0,
    missions: {},
    unlockedBlueprintIds: [...starters],
    equippedDeckUnitIds: [starters[0], starters[1], starters[2], starters[0], starters[1], starters[2]],
    equippedRelicIds: [],
    unlockedRelicIds: [],
    unlockedTrophies: [],
    activeFeatures: { tier2: false, tier3: false }
  };
  save.checksum = computeChecksum(save);
  return save;
}

window.loadConquestSave = function () {
  try {
    const raw = localStorage.getItem(CONQUEST_STORAGE_KEY);
    if (!raw) {
      const def = createDefaultSave("solaris");
      window.saveConquestProgress(def);
      return def;
    }
    const parsed = JSON.parse(raw);
    // Anti-tamper verification
    const expected = computeChecksum(parsed);
    if (parsed.checksum !== expected) {
      console.warn("Conquest save checksum mismatch. Regenerating valid hash.");
      parsed.checksum = expected;
      window.saveConquestProgress(parsed);
    }
    // Ensure all data structures exist
    if (!parsed.missions) parsed.missions = {};
    if (!parsed.unlockedBlueprintIds) parsed.unlockedBlueprintIds = ["sol_1", "sol_2", "sol_3"];
    if (!parsed.equippedDeckUnitIds) parsed.equippedDeckUnitIds = ["sol_1", "sol_2", "sol_3", "sol_1", "sol_2", "sol_3"];
    if (!parsed.equippedRelicIds) parsed.equippedRelicIds = [];
    if (!parsed.unlockedRelicIds) parsed.unlockedRelicIds = [];
    if (!parsed.activeFeatures) parsed.activeFeatures = { tier2: false, tier3: false };

    window.recalculateTotalStars(parsed);
    return parsed;
  } catch (err) {
    console.error("Error loading conquest save, resetting to default:", err);
    const def = createDefaultSave("solaris");
    window.saveConquestProgress(def);
    return def;
  }
};

window.saveConquestProgress = function (saveState) {
  window.recalculateTotalStars(saveState);
  window.checkRelicUnlocks(saveState);
  saveState.checksum = computeChecksum(saveState);
  try {
    localStorage.setItem(CONQUEST_STORAGE_KEY, JSON.stringify(saveState));
  } catch (err) {
    console.error("Failed to write conquest save to localStorage:", err);
  }
};

window.recalculateTotalStars = function (saveState) {
  let total = 0;
  Object.values(saveState.missions || {}).forEach((rec) => {
    if (rec && typeof rec.stars === "number") {
      total += rec.stars;
    }
  });
  saveState.totalConquestStars = total;
  return total;
};

window.checkRelicUnlocks = function (saveState) {
  const stars = saveState.totalConquestStars || 0;
  if (!window.CONQUEST_RELICS) return;
  window.CONQUEST_RELICS.forEach((relic) => {
    if (stars >= relic.starsReq && !saveState.unlockedRelicIds.includes(relic.id)) {
      saveState.unlockedRelicIds.push(relic.id);
      console.log(`Unlocked new War Relic: ${relic.name}!`);
    }
  });
};

window.isMissionUnlocked = function (missionId, saveState) {
  const mission = window.getMissionById(missionId);
  if (!mission) return false;
  if (!mission.prerequisites || mission.prerequisites.length === 0) return true;
  // All prerequisites must have at least 1 star
  return mission.prerequisites.every((prereqId) => {
    const rec = saveState.missions[prereqId];
    return rec && rec.stars > 0;
  });
};

window.getUsedRelicPower = function (saveState) {
  let rp = 0;
  (saveState.equippedRelicIds || []).forEach((rId) => {
    const rDef = window.getRelicDef(rId);
    if (rDef) rp += rDef.rpCost;
  });
  return rp;
};
