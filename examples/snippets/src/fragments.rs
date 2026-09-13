#![allow(dead_code, unused_variables, unused_must_use)]

use eframe::egui;

// skills/egui-profiler/SKILL.md "Step 2: Idle CPU", verbatim.
fn repaint_causes(ui: &mut egui::Ui) {
    let causes = ui.ctx().repaint_causes();
    if !causes.is_empty() {
        eprintln!("repaint causes: {causes:?}");
    }
}

struct Table;

impl Table {
    // skills/egui-profiler/SKILL.md "Option B", verbatim.
    fn table_ui(&mut self, ui: &mut egui::Ui) {
        profiling::function_scope!();
        // ...
    }
}

// skills/egui-profiler/SKILL.md "Option B" (puffin), verbatim.
fn puffin_main() {
    #[cfg(feature = "profile-with-puffin")]
    let _puffin_server = {
        puffin::set_scopes_on(true);
        puffin_http::Server::new(&format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT)).ok()
    };
}

// skills/egui-ui-design/SKILL.md "1.2 Type scale", verbatim.
fn text_styles(ctx: &egui::Context) {
    use egui::{FontFamily, FontId, TextStyle};

    ctx.all_styles_mut(|style| {
        style.text_styles = [
            (TextStyle::Heading, FontId::new(22.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(15.0, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(15.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace)),
            (TextStyle::Small, FontId::new(11.0, FontFamily::Proportional)),
        ]
        .into();
    });
}

// Inline API names from skills/egui and skills/eframe-project.
fn api_names(ui: &mut egui::Ui, raw_input: egui::RawInput, rect: egui::Rect, items: &[(u64, String)]) {
    let mut open = true;
    let mut expanded = true;
    egui::Panel::left("left").resizable(true).default_size(200.0).min_size(120.0).max_size(400.0).show(ui, |ui| {});
    egui::Panel::right("right").size_range(100.0..=300.0).exact_size(150.0).show(ui, |ui| {});
    egui::Panel::bottom("bottom").show_collapsible(ui, &mut open, |ui| {});
    egui::Panel::show_switched(
        ui,
        &mut expanded,
        egui::Panel::left("collapsed"),
        egui::Panel::left("expanded"),
        |ui, is_expanded| {},
    );
    egui::CentralPanel::default().show(ui, |ui| {
        egui::Frame::NONE.show(ui, |ui| {});
        egui::CornerRadius::same(4);
        for (id, name) in items {
            ui.push_id(id, |ui| {
                egui::CollapsingHeader::new(name).show(ui, |ui| {});
            });
            egui::ComboBox::from_id_salt(id).show_ui(ui, |ui| {});
            egui::ScrollArea::vertical().id_salt(id).show(ui, |ui| {});
        }
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Close").clicked() {
                    ui.close();
                }
            });
        });
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {});
        egui::Sides::new().show(ui, |ui| {}, |ui| {});
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {});
        egui::ScrollArea::vertical().show_rows(ui, 18.0, 1000, |ui, range| {
            for row in range {
                ui.label(row.to_string());
            }
        });
        let mut response = ui.allocate_response(egui::vec2(10.0, 10.0), egui::Sense::click());
        ui.style().interact(&response);
        response.mark_changed();
        let _ = (ui.visuals().error_fg_color, ui.visuals().warn_fg_color, ui.visuals().hyperlink_color);
        let mut text = String::new();
        egui::TextEdit::singleline(&mut text).id_source("alias").show(ui);
    });
    egui::Window::new("Settings")
        .id(egui::Id::new("settings_window"))
        .show(ui.ctx(), |ui| {});
    let ctx = ui.ctx().clone();
    if ctx.input(|i| i.viewport().close_requested()) {
        ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
    }
    ctx.style_mut_of(egui::Theme::Dark, |style| {});
    ctx.set_theme(egui::ThemePreference::System);
    egui::gui_zoom::zoom_in(&ctx);
    ctx.request_repaint_after(std::time::Duration::from_secs(1));
    egui::Context::default().run_ui(raw_input, |ui| {});
}

struct Saving;

impl eframe::App for Saving {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        frame.wgpu_render_state();
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn persist_egui_memory(&self) -> bool {
        false
    }
}

fn native_options() -> eframe::Result {
    let options = eframe::NativeOptions {
        persist_window: true,
        persistence_path: None,
        viewport: egui::ViewportBuilder::default().with_app_id("my_app"),
        ..Default::default()
    };
    eframe::run_ui_native("my_tool", options, move |ui, _frame| {
        ui.label("tool");
    })
}
