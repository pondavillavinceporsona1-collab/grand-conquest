# Grand Conquest — Native Rust Edition (`v2.0.0`)

A full, zero-browser, GPU-accelerated native Rust rebuild of **Grand Conquest** (`Epic Battle 2D`), powered by a deterministic pure-Rust simulation engine (`grand_conquest_core`) and an `eframe`/`egui` GPU desktop frontend (`grand_conquest_app`) with an overhauled tactical cartographer UI.

## Workspace Architecture

```
game_rust/
├── Cargo.toml                         # Cargo workspace root
├── assets/
│   ├── pivots.json                    # 6-limb rotation pivots for all 54 units
│   ├── static_all_54/                 # 54 unit portrait PNGs (embedded at compile time)
│   ├── separated_parts/               # 54 × 6 articulated skeletal limbs (324 PNGs)
│   └── fonts/                         # Display (Cinzel), UI (Segoe UI), Mono (Consolas)
└── crates/
    ├── grand_conquest_core/           # Pure deterministic simulation & campaign rules
    │   ├── model.rs                   # Factions, 21 passives, 15 relics, 6 mutators
    │   ├── units.rs                   # 54 units + 3-piece & 6-piece Faction Affinities
    │   ├── relics.rs                  # 15 War Relics catalog
    │   ├── missions.rs                # 24 Campaign missions across 6 Provinces + 6 Bosses
    │   ├── save.rs                    # Persistent JSON + FNV-1a anti-tamper checksum
    │   ├── ai.rs                      # Multi-difficulty Bot Commander AI
    │   ├── sim.rs                     # Combat loop, Turret Rule, Star Auditor
    │   └── tests.rs                   # Automated QA test suite (TC-01..TC-10)
    └── grand_conquest_app/            # Native GPU application (eframe / egui / rodio)
        ├── assets.rs                  # Embedded asset bank + 6-limb rotated GPU mesh renderer
        ├── audio.rs                   # Procedural PCM SFX synthesizer
        ├── theme.rs                   # Tactical Slate & Brushed Brass theme + vector icons
        ├── ui_map.rs                  # 2000x1200 Interactive Geo-Terra War Table & Dossier
        ├── ui_armory.rs               # 6-Slot Formation Deck Builder & Synergy Matrix
        ├── ui_vault.rs                # Relic Vault (15 artifacts, 4 RP capacity)
        ├── ui_codex.rs                # 54-Unit Interactive Battle Codex & Rig Turntable
        ├── ui_skirmish.rs             # Skirmish & Mutator Sandbox Builder
        └── ui_battle.rs               # High-FPS Battlefield Theater, Boss HUD & Debrief
```

## Build & Run

Ensure Rust (`stable` toolchain + MSVC Build Tools on Windows) is installed:

```bash
# Run all deterministic simulation & QA verification tests
cargo test --workspace

# Launch the native desktop game (optimized release build)
cargo run --release -p grand_conquest_app
```

## Controls

| Screen | Controls |
| :--- | :--- |
| **Campaign Atlas** | Drag or `WASD` / `Arrow Keys` to pan, Mouse Wheel to zoom, Click node pin for Briefing Dossier, Double-click to quick-deploy |
| **Battlefield** | `1`–`6` Spawn Unit Slots • `U` Upgrade Income (`+2G/s`) • `T` Unlock Tier • `Q` Commander Offensive Strike • `E` Commander Surge Heal • `Space`/`Esc` Pause • `Tab` Cycle Speed (`1x`/`2x`/`3x`) • `V` Toggle `Skeletal Rig` / `Geometric Shapes` |
