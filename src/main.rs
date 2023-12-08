use std::f64::consts::FRAC_PI_2;

use eframe::egui;
use egui::{Context, TextureHandle, TextureOptions};
use egui_plot::{Line, Plot, PlotPoints, PlotUi};

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
    path: Vec<[f64; 2]>,
    img: TextureHandle,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            actions: vec![Action::StartAt {
                pos: [0.0, -1.7],
                heading: -FRAC_PI_2,
            },
            Action::MoveRel { rel: 1.0, },
            Action::MoveRelAbs { rel: 1.0 },
            Action::MoveTo { pos: [0.0, 0.0], }],
            img: Self::load_field_image(&cc.egui_ctx),
            path: vec![[0.0, -1.7], [1.0, -1.7], [1.0, -0.7]],
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

    fn draw_menu(ctx: &Context, height: f32) {
        egui::TopBottomPanel::top("menu")
            .exact_height(height)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        ui.button("Save Path As (TODO)").clicked();
                    });
                    ui.menu_button("Units", |ui| {
                        ui.checkbox(&mut true, "metric (TODO)");
                        ui.checkbox(&mut true, "degree (TODO)");
                    });
                    ui.menu_button("Tools", |ui| {
                        ui.button("Measure").clicked();
                        ui.button("Angle (relative)").clicked();
                        ui.button("Angle (absolute)").clicked();
                    });
                });
            });
    }

    fn draw_panel(&self, ctx: &Context, (max_axis, min_len): (usize, f32)) {
        let create_row = |ui: &mut egui::Ui, act: &Action| {
            ui.label(act.action_name());
            ui.label(act.action_value());
            ui.label(act.action_modifiers());
            ui.end_row();
        };

        let table = |ui: &mut _| {
            egui::Grid::new("test_grid")
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    ui.heading("Action");
                    ui.heading("Action Data");
                    ui.heading("Action Type");
                    // ensure button in on the right hand side
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

    pub fn draw_path(&self, plot_ui: &mut PlotUi) {
        let points = PlotPoints::new(self.path.clone());
        plot_ui.line(Line::new(points).width(5.0));
    }

    pub fn draw_plot(&self, ctx: &Context) {
        let plot = Plot::new("plot")
            .view_aspect(1.0)
            .auto_bounds_x()
            .auto_bounds_y();

        let img = egui_plot::PlotImage::new(
            &self.img,
            egui_plot::PlotPoint::new(0.0, 0.0),
            // 12 ft (width/length of field) to m
            [3.6576; 2],
        );
        egui::CentralPanel::default().show(ctx, |ui| {
            plot.show(ui, |plot_ui| {
                plot_ui.image(img);
                self.draw_path(plot_ui);
            });
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        // top menu is fixed size of 30px tall
        Self::draw_menu(ctx, 30.);

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
