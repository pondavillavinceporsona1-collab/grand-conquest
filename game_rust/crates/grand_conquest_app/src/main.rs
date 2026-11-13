mod app;
mod assets;
mod audio;
mod theme;
mod ui_armory;
mod ui_battle;
mod ui_codex;
mod ui_honors;
mod ui_map;
mod ui_menu;
mod ui_skirmish;
mod ui_vault;

use std::path::PathBuf;

use crate::app::GrandConquestApp;

fn main() -> eframe::Result<()> {
    let mut args = std::env::args().skip(1);
    let mut qa_dir: Option<PathBuf> = None;
    while let Some(arg) = args.next() {
        if arg == "--qa-screenshots" {
            if let Some(dir) = args.next() {
                qa_dir = Some(PathBuf::from(dir));
            }
        }
    }

    let icon = load_window_icon();
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Grand Conquest")
        .with_inner_size([1380.0, 840.0])
        .with_min_inner_size([1080.0, 680.0]);
    if let Some(ic) = icon {
        viewport = viewport.with_icon(std::sync::Arc::new(ic));
    }

    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Grand Conquest",
        options,
        Box::new(move |cc| Ok(Box::new(GrandConquestApp::new(cc, qa_dir)))),
    )
}

fn load_window_icon() -> Option<egui::IconData> {
    let bytes = include_bytes!("../../../assets/art/crests/crest_solaris.png");
    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    let (width, height) = img.dimensions();
    Some(egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    })
}
