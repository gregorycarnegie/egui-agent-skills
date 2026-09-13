#![allow(dead_code)]

// skills/egui/SKILL.md "Saving state", verbatim.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // old saves still load after a field is added
pub struct MyApp {
    name: String,
    #[serde(skip)] // exists only while the app runs
    worker: Option<std::sync::mpsc::Receiver<String>>,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }
}

impl eframe::App for MyApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.text_edit_singleline(&mut self.name);
        });
    }
}
