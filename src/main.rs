use std::f64::consts::FRAC_PI_2;

use eframe::egui;
use egui::{containers::Window, widgets::Label, Context, TextureHandle, TextureOptions};
use egui_plot::Plot;

mod actions;
use actions::Action;

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
    actions: Vec<Action>,
    valid_actions: bool,
    img: TextureHandle,
    help_actions: bool,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            actions: vec![
                Action::StartAt {
                    pos: [0.0, -1.7],
                    heading: -FRAC_PI_2,
                },
                Action::MoveRelAbs { rel: 0.2 },
                Action::MoveRel { rel: 1. },
            ],
            valid_actions: true,
            img: Self::load_field_image(&cc.egui_ctx),
            help_actions: false,
        }
    }

    fn load_field_image(ctx: &Context) -> TextureHandle {
        // load ColorImage according to https://docs.rs/epaint/0.24.1/epaint/image/struct.ColorImage.html
        let data = image::io::Reader::open("res/field.jpg")
            .unwrap()
            .decode()
            .unwrap();

        let img = egui::ColorImage::from_rgba_unmultiplied(
            [data.width() as _, data.height() as _],
            data.to_rgba8().as_flat_samples().as_slice(),
        );

        ctx.load_texture("field", img, TextureOptions::default())
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
                        ui.button("Measure (TODO)").clicked();
                        ui.button("Angle (relative) (TODO)").clicked();
                        ui.button("Angle (absolute) (TODO)").clicked();
                    });
                    ui.menu_button("Help", |ui| {
                        if ui.button("Actions").clicked() {
                            self.help_actions = true;
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
                        ui.heading(if self.valid_actions { "✅" } else { "⚠" });
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.button("Add Action").clicked();
                    });
                    ui.end_row();
                    for action in &self.actions {
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

    pub fn draw_plot(&mut self, ctx: &Context) {
        let plot = Plot::new("plot")
            .view_aspect(1.0)
            .auto_bounds_x()
            .auto_bounds_y();

        let img = egui_plot::PlotImage::new(
            &self.img,
            egui_plot::PlotPoint::new(0., 0.),
            // 12 ft (width/length of field) to m
            [3.6576; 2],
        );
        egui::CentralPanel::default().show(ctx, |ui| {
            plot.show(ui, |plot_ui| {
                plot_ui.image(img);
                // draw path if it is valid else prevent future drawing
                // TODO:
                // 1: add validation on path construction
                // to reset self.valid_actions
                if self.valid_actions {
                    self.valid_actions = Action::render(&self.actions, plot_ui).is_ok();
                }
            });
        });
    }
    pub fn draw_help_actions(&mut self, ctx: &Context) {
        let create_row = |ui: &mut egui::Ui, act: &Action| {
            ui.add(Label::new(act.name()).wrap(true));
            ui.add(Label::new(act.modifiers()).wrap(true));
            ui.add(Label::new(act.description()).wrap(true));
            ui.end_row();
        };

        Window::new("Action Help")
            .resizable(true)
            .open(&mut self.help_actions)
            .show(ctx, |ui| {
                egui::Grid::new("action help")
                    .striped(true)
                    .num_columns(5)
                    .show(ui, |ui| {
                        ui.heading("Action");
                        ui.heading("Action Type");
                        ui.heading("Action Description");
                        // ensure button in on the right hand side
                        ui.end_row();
                        for action in &[
                            Action::STARTAT,
                            Action::MOVEREL,
                            Action::MOVERELABS,
                            Action::MOVETO,
                        ] {
                            create_row(ui, action);
                        }
                    });
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        // draw help
        self.draw_help_actions(ctx);

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

        // draw plot with the field and path on it
        self.draw_plot(ctx);
    }
}
