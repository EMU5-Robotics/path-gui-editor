use eframe::egui;
use egui::Context;

mod actions;
mod help;
mod plot;
mod tools;
mod vec;

use actions::Action;
use help::Help;
use plot::Plot;
use tools::{PointSelection, Tools};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Path Editor",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .unwrap();
}

struct App {
    plot: Plot,
    help: Help,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            help: Help::default(),
            plot: Plot::new(&cc.egui_ctx),
        }
    }

    fn draw_menu(&mut self, ctx: &Context, height: f32) {
        egui::TopBottomPanel::top("menu")
            .exact_height(height)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        ui.button("Save Path As (TODO)").clicked();
                    });
                    ui.menu_button("Units", |ui| {
                        ui.checkbox(&mut true, "Use metric (TODO)");
                        ui.checkbox(&mut true, "Use degrees (TODO)");
                    });
                    ui.menu_button("Tools", |ui| {
                        if ui.button("None").clicked() {
                            self.plot.set_tools(Tools::None);
                        } else if ui.button("Measure Distance").clicked() {
                            self.plot.set_tools(Tools::MeasureDistance {
                                selection: PointSelection::default(),
                            });
                        } else if ui.button("Measure Angle").clicked() {
                            self.plot.set_tools(Tools::MeasureAngle {
                                selection: PointSelection::default(),
                            });
                        }
                    });
                    ui.menu_button("Help", |ui| {
                        if ui.button("Actions").clicked() {
                            self.help.actions = true;
                        }
                        if ui.button("Ui (TODO)").clicked() {
                            self.help.ui = true;
                        }
                        if ui.button("About (TODO)").clicked() {
                            self.help.about = true;
                        }
                    })
                });
            });
    }

    fn draw_panel(&self, ctx: &Context, (max_axis, min_len): (usize, f32)) {
        let create_row = |ui: &mut egui::Ui, act: &Action| {
            ui.label(act.name());
            ui.label(act.value());
            ui.label(act.modifiers());
            ui.end_row();
        };

        let table = |ui: &mut _| {
            egui::Grid::new("actions")
                .striped(true)
                .num_columns(5)
                .show(ui, |ui| {
                    ui.heading("Action");
                    ui.heading("Action Data");
                    ui.heading("Action Type");
                    // ensure button in on the right hand side
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        ui.heading(if self.plot.actions.is_valid() {
                            "✅"
                        } else {
                            "⚠"
                        });
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.button("Add Action").clicked();
                        ui.button("Remove Action").clicked();
                    });
                    ui.end_row();
                    for action in self.plot.actions.actions() {
                        create_row(ui, action);
                    }
                });
        };

        if max_axis == 0 {
            egui::SidePanel::left("actions")
                .exact_width(min_len)
                .show(ctx, |ui| {
                    table(ui);
                });
        } else {
            egui::TopBottomPanel::bottom("actions")
                .exact_height(min_len)
                .show(ctx, |ui| {
                    table(ui);
                });
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        // draw help
        self.help.draw(ctx);

        // top menu is fixed size of 30px tall
        self.draw_menu(ctx, 30.);

        // calculate sizing for left/bottom panel
        // which is the remaining size from having the
        // plot be the largest it can be with a 1:1 aspect ratio
        let win_size = ctx.available_rect();
        let win_size = win_size.max - win_size.min;
        let (max_axis, panel_size) = if win_size.x > win_size.y {
            (0, win_size.x - win_size.y)
        } else {
            (1, win_size.y - win_size.x)
        };

        // draw panel which has the table of robot actions on it
        self.draw_panel(ctx, (max_axis, panel_size));

        // draw plot with the field and path and tools on it
        self.plot.draw(ctx);
    }
}
