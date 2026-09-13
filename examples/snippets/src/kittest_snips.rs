// Glue: the app the kittest snippets assume.
#[derive(Default)]
struct MyApp {
    unsaved: bool,
    playing: bool,
}

impl MyApp {
    fn for_tests(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            if ui.button("Save").clicked() {
                self.unsaved = false;
            }
            if self.playing {
                ui.ctx().request_repaint();
            }
        });
    }
}

// skills/egui-kittest/SKILL.md:84, verbatim.
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn saving_clears_the_unsaved_flag() {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(800.0, 600.0))
        .build_eframe(|cc| MyApp::for_tests(cc));
    harness.run();

    harness.state_mut().unsaved = true;
    harness.get_by_label("Save").click();
    harness.run();

    assert!(!harness.state().unsaved, "Save should clear the unsaved flag");
}

#[test]
fn playback_keeps_repainting() {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(800.0, 600.0))
        .build_eframe(|cc| MyApp::for_tests(cc));
    harness.run();

    // skills/egui-kittest/SKILL.md:159, verbatim.
    harness.state_mut().playing = true;
    harness.step();
    assert!(harness.ctx.has_requested_repaint(), "playback should keep the UI repainting");
}

// SKILL.md:151: run() panics when the UI repaints every frame; run_steps does not.
#[test]
#[should_panic]
fn run_panics_on_continuous_repaint() {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(800.0, 600.0))
        .build_eframe(|cc| MyApp::for_tests(cc));
    harness.state_mut().playing = true;
    harness.run_steps(5);
    harness.run();
}

// Compile-only: SKILL.md:135 and :182 plus every name in the kittest tables.
#[allow(dead_code, unused_must_use)]
fn compile_only() {
    use egui_kittest::kittest::NodeT;

    let mut harness = Harness::builder()
        .with_size(egui::vec2(800.0, 600.0))
        .with_pixels_per_point(1.0)
        .with_theme(egui::Theme::Light)
        .with_max_steps(4)
        .build_eframe(|cc| MyApp::for_tests(cc));
    let pos = egui::pos2(10.0, 10.0);

    harness.hover_at(pos);
    harness.run();
    harness.drag_at(pos);
    harness.run();
    harness.drop_at(pos);
    harness.run();

    harness.fit_contents(); // shrink the window to the content; smaller images
    harness.snapshot("settings_panel");

    harness.get_by_label_contains("Save").click_secondary();
    harness.query_by_label("Delete").is_none();
    harness.query_by_label_contains("Del");
    harness.get_all_by_value("text").count();
    harness.get_by_label("Name").focus();
    harness.get_by_label("Name").type_text("abc");
    harness.get_by_label("Grid").accesskit_node().toggled();
    harness.get_by_label("Name").value();
    harness.key_press(egui::Key::Escape);
    harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::S);
    harness.input_mut().events.push(egui::Event::PointerGone);
    harness.run_steps(3);
    harness.try_run();
    harness.mask(egui::Rect::ZERO);
    harness.take_snapshot_results();

    Harness::new_ui(|ui| {
        ui.label("x");
    })
    .run();
    Harness::new_ui_state(|ui, state: &mut bool| {
        ui.checkbox(state, "Grid");
    }, false)
    .run();
}
