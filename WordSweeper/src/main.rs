mod game;
mod gui;

use eframe::egui;
use gui::WordSweeperApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "WordSweeper",
        options,
        Box::new(|_cc| Ok(Box::new(WordSweeperApp::new()))),
    )
}