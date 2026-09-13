#![allow(dead_code)]

use std::sync::mpsc;

// Glue: stands in for real async work such as an HTTP request.
async fn fetch_text() -> String {
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    "fetched".to_owned()
}

pub struct FetchApp {
    result_tx: mpsc::Sender<String>,
    result_rx: mpsc::Receiver<String>,
    result: Option<String>,
}

impl Default for FetchApp {
    fn default() -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        Self { result_tx, result_rx, result: None }
    }
}

impl eframe::App for FetchApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // skills/egui/SKILL.md "Async work with tokio", verbatim.
        if let Ok(text) = self.result_rx.try_recv() {
            self.result = Some(text);
        }
        egui::CentralPanel::default().show(ui, |ui| {
            if ui.button("Fetch").clicked() {
                let tx = self.result_tx.clone();
                let ctx = ui.ctx().clone();
                tokio::spawn(async move {
                    let text = fetch_text().await;
                    let _ = tx.send(text);
                    ctx.request_repaint();
                });
            }
            if let Some(text) = &self.result {
                ui.label(text);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use egui_kittest::{Harness, kittest::Queryable};

    fn click_fetch() -> Harness<'static, super::FetchApp> {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(800.0, 600.0))
            .build_eframe(|_cc| super::FetchApp::default());
        harness.run();
        harness.get_by_label("Fetch").click();
        harness.run();
        harness
    }

    #[test]
    fn fetch_result_arrives() {
        // skills/egui/SKILL.md "Async work with tokio", verbatim.
        let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
        let _guard = rt.enter(); // lets tokio::spawn find the runtime

        let mut harness = click_fetch();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while harness.query_by_label("fetched").is_none() {
            assert!(std::time::Instant::now() < deadline, "result never arrived");
            std::thread::sleep(std::time::Duration::from_millis(10));
            harness.step();
        }
    }

    // The skill's claim: without an entered runtime, tokio::spawn panics.
    #[test]
    #[should_panic(expected = "no reactor running")]
    fn spawn_without_runtime_panics() {
        click_fetch();
    }
}
