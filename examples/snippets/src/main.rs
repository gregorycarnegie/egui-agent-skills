// Hide the console window in Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("My App")
            .with_inner_size([960.0, 640.0])
            .with_min_inner_size([480.0, 320.0]),
        ..Default::default()
    };
    eframe::run_native(
        "my_app",
        options,
        Box::new(|cc| Ok(Box::new(app::MyApp::new(cc)))),
    )
}

// Everything above is skills/eframe-project/SKILL.md, verbatim.
mod fragments;
#[cfg(test)]
mod kittest_snips;
mod persist;
mod viewports;
