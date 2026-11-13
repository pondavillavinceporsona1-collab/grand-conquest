// =========================================================================
// EPIC BATTLE! — GRAND CONQUEST: 2000x1200 INTERACTIVE WORLD MAP ENGINE
// Imperial War Table Aesthetic & Chapter Navigator
// =========================================================================

window.ConquestMapEngine = class ConquestMapEngine {
  constructor(canvasEl, onNodeSelected) {
    this.canvas = canvasEl;
    this.ctx = canvasEl.getContext("2d");
    this.onNodeSelected = onNodeSelected;

    this.worldW = 2000;
    this.worldH = 1200;

    // Start centered on Chapter 1 (Solaris Empire)
    this.camX = 100;
    this.camY = 450;
    this.zoom = 1.0;
    this.targetZoom = 1.0;

    this.isDragging = false;
    this.dragStartX = 0;
    this.dragStartY = 0;
    this.hoveredMission = null;

    this.dashOffset = 0;
    this.pulsePhase = 0;

    this.initEvents();
  }

  initEvents() {
    this.canvas.addEventListener("mousedown", (e) => {
      this.isDragging = true;
      this.dragStartX = e.clientX;
      this.dragStartY = e.clientY;
    });

    window.addEventListener("mouseup", () => {
      this.isDragging = false;
    });

    window.addEventListener("mousemove", (e) => {
      const rect = this.canvas.getBoundingClientRect();
      if (this.isDragging) {
        const dx = (e.clientX - this.dragStartX) / this.zoom;
        const dy = (e.clientY - this.dragStartY) / this.zoom;
        this.camX -= dx;
        this.camY -= dy;
        this.dragStartX = e.clientX;
        this.dragStartY = e.clientY;
        this.clampCamera();
      }

      // Check pin hover
      const mx = e.clientX - rect.left;
      const my = e.clientY - rect.top;
      const worldPos = this.screenToWorld(mx, my);
      this.checkPinHover(worldPos.x, worldPos.y);
    });

    this.canvas.addEventListener("wheel", (e) => {
      e.preventDefault();
      const zoomFactor = e.deltaY < 0 ? 1.12 : 0.88;
      this.zoom = Math.max(0.65, Math.min(1.85, this.zoom * zoomFactor));
      this.clampCamera();
    }, { passive: false });

    this.canvas.addEventListener("click", () => {
      if (this.hoveredMission) {
        const save = window.loadConquestSave();
        const unlocked = window.isMissionUnlocked(this.hoveredMission.id, save);
        if (unlocked && this.onNodeSelected) {
          this.onNodeSelected(this.hoveredMission);
        }
      }
    });
  }

  screenToWorld(sx, sy) {
    return {
      x: this.camX + sx / this.zoom,
      y: this.camY + sy / this.zoom
    };
  }

  clampCamera() {
    const viewW = this.canvas.width / this.zoom;
    const viewH = this.canvas.height / this.zoom;
    this.camX = Math.max(0, Math.min(this.worldW - viewW, this.camX));
    this.camY = Math.max(0, Math.min(this.worldH - viewH, this.camY));
  }

  focusChapter(chapterNum) {
    const chapterCenters = {
      1: { x: 500, y: 720 },  // Solaris Empire
      2: { x: 1450, y: 720 }, // Aegis Coalition
      3: { x: 500, y: 1020 }, // Hive Mind
      4: { x: 1450, y: 1020 },// Hellfire Legion
      5: { x: 500, y: 280 },  // Abyssal Cult
      6: { x: 1450, y: 280 }  // Mechanica Dominion
    };
    const target = chapterCenters[chapterNum] || chapterCenters[1];
    const viewW = this.canvas.width / this.zoom;
    const viewH = this.canvas.height / this.zoom;
    this.camX = target.x - viewW / 2;
    this.camY = target.y - viewH / 2;
    this.clampCamera();
  }

  checkPinHover(wx, wy) {
    if (!window.CONQUEST_MISSIONS) return;
    let found = null;
    for (const m of window.CONQUEST_MISSIONS) {
      const [px, py] = m.pinCoord;
      const dist = Math.hypot(wx - px, wy - py);
      if (dist <= 32) {
        found = m;
        break;
      }
    }
    this.hoveredMission = found;
    this.canvas.style.cursor = found ? "pointer" : "grab";
  }

  render(saveState, dt = 0.016) {
    const ctx = this.ctx;
    const W = this.canvas.width;
    const H = this.canvas.height;

    this.dashOffset -= dt * 24;
    this.pulsePhase += dt * 3.2;

    ctx.save();
    ctx.clearRect(0, 0, W, H);
    ctx.scale(this.zoom, this.zoom);
    ctx.translate(-this.camX, -this.camY);

    // 1. Draw World Background & Continental Regions
    this.drawContinents(ctx);

    // 2. Draw Connection Road Splines
    this.drawRoadConnections(ctx, saveState);

    // 3. Draw 24 Mission Node Pins
    this.drawMissionPins(ctx, saveState);

    // 4. Draw Atmospheric Fog-of-War over Locked Regions
    this.drawFogOfWar(ctx, saveState);

    ctx.restore();
  }

  drawContinents(ctx) {
    // Deep obsidian war-table parchment background
    ctx.fillStyle = "#07090e";
    ctx.fillRect(0, 0, this.worldW, this.worldH);

    // Elegant tactical grid in antique brass
    ctx.strokeStyle = "rgba(212, 175, 55, 0.04)";
    ctx.lineWidth = 1;
    for (let x = 0; x <= this.worldW; x += 100) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, this.worldH);
      ctx.stroke();
    }
    for (let y = 0; y <= this.worldH; y += 100) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(this.worldW, y);
      ctx.stroke();
    }

    // 6 Continental Territories (Warm, muted, imperial colors)
    const zones = [
      { num: "V", name: "CHAPTER V: ABYSSAL VOID", bounds: [50, 50, 900, 480], grad: ["#2d0e4e", "#0a0e17"], border: "rgba(168, 85, 247, 0.3)" },
      { num: "VI", name: "CHAPTER VI: SECTOR ZERO (MECHANICA)", bounds: [1050, 50, 900, 480], grad: ["#134e4a", "#0a0e17"], border: "rgba(20, 184, 166, 0.3)" },
      { num: "I", name: "CHAPTER I: SOLARIS EMPIRE", bounds: [50, 580, 900, 260], grad: ["#664200", "#0a0e17"], border: "rgba(245, 158, 11, 0.3)" },
      { num: "II", name: "CHAPTER II: AEGIS COALITION", bounds: [1050, 580, 900, 260], grad: ["#1e3a8a", "#0a0e17"], border: "rgba(59, 130, 246, 0.3)" },
      { num: "III", name: "CHAPTER III: HIVE CHITIN WASTES", bounds: [50, 880, 900, 270], grad: ["#274e13", "#0a0e17"], border: "rgba(132, 204, 22, 0.3)" },
      { num: "IV", name: "CHAPTER IV: HELLFIRE CRUCIBLE", bounds: [1050, 880, 900, 270], grad: ["#5c1010", "#0a0e17"], border: "rgba(239, 68, 68, 0.3)" }
    ];

    zones.forEach((z) => {
      const [zx, zy, zw, zh] = z.bounds;
      const g = ctx.createLinearGradient(zx, zy, zx + zw, zy + zh);
      g.addColorStop(0, z.grad[0]);
      g.addColorStop(1, z.grad[1]);

      ctx.save();
      ctx.fillStyle = g;
      ctx.globalAlpha = 0.55;
      ctx.beginPath();
      ctx.roundRect(zx, zy, zw, zh, 24);
      ctx.fill();

      // Subtle hairline border
      ctx.strokeStyle = z.border;
      ctx.lineWidth = 1.5;
      ctx.stroke();

      // Imperial Province Watermark Header
      ctx.fillStyle = "rgba(245, 215, 127, 0.25)";
      ctx.font = "900 16px 'Cinzel', serif, sans-serif";
      ctx.fillText(z.name, zx + 24, zy + 36);

      // Roman Numeral Monogram Background
      ctx.fillStyle = "rgba(255, 255, 255, 0.03)";
      ctx.font = "900 120px 'Cinzel', serif, sans-serif";
      ctx.textAlign = "right";
      ctx.fillText(z.num, zx + zw - 30, zy + zh - 30);

      ctx.restore();
    });
  }

  drawRoadConnections(ctx, saveState) {
    if (!window.CONQUEST_MISSIONS) return;

    window.CONQUEST_MISSIONS.forEach((m) => {
      const [tx, ty] = m.pinCoord;
      const isTargetUnlocked = window.isMissionUnlocked(m.id, saveState);
      const isTargetCompleted = saveState.missions[m.id] && saveState.missions[m.id].stars > 0;

      (m.prerequisites || []).forEach((pId) => {
        const src = window.getMissionById(pId);
        if (!src) return;
        const [sx, sy] = src.pinCoord;

        ctx.save();
        ctx.beginPath();
        const midX = (sx + tx) / 2;
        const midY = (sy + ty) / 2 - 25;
        ctx.moveTo(sx, sy);
        ctx.quadraticCurveTo(midX, midY, tx, ty);

        if (isTargetCompleted) {
          ctx.strokeStyle = "#d4af37"; // Solid antique gold
          ctx.lineWidth = 3.0;
          ctx.stroke();
        } else if (isTargetUnlocked) {
          ctx.strokeStyle = "rgba(245, 215, 127, 0.75)"; // Dashed active expedition route
          ctx.lineWidth = 2.5;
          ctx.setLineDash([8, 8]);
          ctx.lineDashOffset = this.dashOffset;
          ctx.stroke();
        } else {
          ctx.strokeStyle = "rgba(71, 85, 105, 0.25)"; // Faint locked road
          ctx.lineWidth = 1.5;
          ctx.setLineDash([4, 6]);
          ctx.stroke();
        }
        ctx.restore();
      });
    });
  }

  drawMissionPins(ctx, saveState) {
    if (!window.CONQUEST_MISSIONS) return;

    window.CONQUEST_MISSIONS.forEach((m) => {
      const [px, py] = m.pinCoord;
      const isUnlocked = window.isMissionUnlocked(m.id, saveState);
      const record = saveState.missions[m.id];
      const stars = record ? record.stars : 0;
      const isHovered = this.hoveredMission && this.hoveredMission.id === m.id;

      ctx.save();
      ctx.translate(px, py);

      // Pulse ring for available unplayed nodes
      if (isUnlocked && stars === 0) {
        const pulseScale = 1.0 + Math.sin(this.pulsePhase) * 0.12;
        ctx.save();
        ctx.scale(pulseScale, pulseScale);
        ctx.strokeStyle = "#d4af37";
        ctx.lineWidth = 2.0;
        ctx.beginPath();
        ctx.arc(0, 0, 32, 0, Math.PI * 2);
        ctx.stroke();
        ctx.restore();
      }

      // Pin Body (Obsidian circle with brass trim)
      ctx.beginPath();
      ctx.arc(0, 0, 24, 0, Math.PI * 2);
      ctx.fillStyle = isUnlocked ? "#0d121c" : "#07090e";
      ctx.fill();

      ctx.lineWidth = stars === 3 ? 3.0 : 2.0;
      ctx.strokeStyle = !isUnlocked
        ? "rgba(71, 85, 105, 0.4)"
        : stars === 3
        ? "#f5d77f"
        : stars > 0
        ? "#d4af37"
        : isHovered
        ? "#ffffff"
        : "#d4af37";
      ctx.stroke();

      // Archetype Icon
      const icon =
        m.archetype === "titan_slayer"
          ? "👑"
          : m.archetype === "hold_the_line"
          ? "🛡️"
          : m.archetype === "blitzkrieg"
          ? "⚡"
          : m.archetype === "mutator"
          ? "☣️"
          : "⚔️";

      ctx.font = "16px sans-serif";
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      ctx.fillText(isUnlocked ? icon : "🔒", 0, 0);

      // Star Badges below pin
      if (isUnlocked) {
        ctx.font = "11px 'JetBrains Mono', monospace";
        ctx.textAlign = "center";
        const starStr = (stars >= 1 ? "★" : "☆") + (stars >= 2 ? "★" : "☆") + (stars >= 3 ? "★" : "☆");
        ctx.fillStyle = stars > 0 ? "#f5d77f" : "#475569";
        ctx.fillText(starStr, 0, 36);
      }

      // Territory Title Pill
      ctx.fillStyle = "rgba(13, 18, 28, 0.94)";
      ctx.strokeStyle = isHovered ? "#d4af37" : "rgba(212, 175, 55, 0.25)";
      ctx.lineWidth = 1.0;
      const nameText = m.name;
      ctx.font = isHovered ? "bold 11px Inter, sans-serif" : "10px Inter, sans-serif";
      const textW = ctx.measureText(nameText).width;
      ctx.beginPath();
      ctx.roundRect(-textW / 2 - 8, 44, textW + 16, 20, 6);
      ctx.fill();
      ctx.stroke();

      ctx.fillStyle = isUnlocked ? "#f8fafc" : "#64748b";
      ctx.fillText(nameText, 0, 58);

      ctx.restore();
    });
  }

  drawFogOfWar(ctx, saveState) {
    // If Chapter 1 is not beaten, dark mist covers upper half
    const ch1Complete = saveState.missions["M1_4_BOSS"] && saveState.missions["M1_4_BOSS"].stars > 0;
    if (!ch1Complete) {
      ctx.save();
      ctx.fillStyle = "rgba(7, 9, 14, 0.72)";
      ctx.fillRect(0, 0, this.worldW, 560);
      ctx.fillStyle = "rgba(212, 175, 55, 0.65)";
      ctx.font = "900 18px 'Cinzel', serif, sans-serif";
      ctx.textAlign = "center";
      ctx.fillText("TERRA INCOGNITA — LIBERATE SOLARIS EMPIRE TO ADVANCE CAMPAIGN", this.worldW / 2, 290);
      ctx.restore();
    }
  }
};
