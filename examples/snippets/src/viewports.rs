#![allow(dead_code)]

#[derive(Default)]
pub struct SettingsApp {
    show_settings: bool,
}

impl eframe::App for SettingsApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.checkbox(&mut self.show_settings, "Show settings");
        });

        // skills/egui/SKILL.md "Windows and closing", verbatim.
        if self.show_settings {
            ui.ctx().show_viewport_immediate(
                egui::ViewportId::from_hash_of("settings"),
                egui::ViewportBuilder::default()
                    .with_title("Settings")
                    .with_inner_size([320.0, 240.0]),
                |ui, _class| {
                    egui::CentralPanel::default().show(ui, |ui| {
                        ui.label("Settings go here");
                    });
                    if ui.ctx().input(|i| i.viewport().close_requested()) {
                        self.show_settings = false;
                    }
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use egui_kittest::{Harness, kittest::Queryable};

    // kittest has no OS windows, so the viewport is embedded as an egui::Window.
    #[test]
    fn settings_viewport_opens_embedded() {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(800.0, 600.0))
            .build_eframe(|_cc| super::SettingsApp::default());
        harness.run();
        assert!(harness.query_by_label("Settings go here").is_none());

        harness.get_by_label("Show settings").click();
        harness.run();
        harness.get_by_label("Settings go here");
    }
}
