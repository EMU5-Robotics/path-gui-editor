use crate::{robot_state::*, tools::Tools, vec::Vec2};
use eframe::egui::{self, Context, Rgba, TextureHandle, TextureOptions};
use egui_plot::{Line, PlotPoints, PlotUi, Points};

use std::f64::consts::FRAC_PI_2;

pub struct Plot {
    img: TextureHandle,
    pub actions: RobotState,
    pub action_builder_window: ActionBuilderWindow,
    tools: Tools,
}

impl Plot {
    pub fn new(ctx: &Context) -> Self {
        Self {
            img: Self::load_field_image(ctx),
            actions: RobotState::from(vec![
                Action::StartAt {
                    pos: Vec2([0.0, -1.7]),
                    heading: -FRAC_PI_2,
                },
                Action::MoveRelAbs { rel: 0.2 },
                Action::MoveRel { rel: 1. },
            ]),
            action_builder_window: ActionBuilderWindow::new(),
            tools: Default::default(),
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
    pub fn draw(&mut self, ctx: &Context) {
        let plot = egui_plot::Plot::new("plot")
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
            let plot_resp = plot.show(ui, |plot_ui| {
                plot_ui.image(img);
                self.actions.render(plot_ui);

                self.tools.draw(plot_ui);
            });

            self.tools.draw_defered(ui, plot_resp);
        });
    }
    pub fn set_tools(&mut self, tools: Tools) {
        self.tools = tools;
    }
    pub fn draw_points(ui: &mut PlotUi, points: Vec<Vec2>, color: Rgba) {
        let plotpoints = PlotPoints::new(points.iter().map(|v| v.0.into()).collect());
        let points = Points::new(plotpoints).filled(true).color(color).radius(4.);
        ui.points(points);
    }

    pub fn draw_lines(ui: &mut PlotUi, points: Vec<Vec2>, color: Rgba) {
        let plotpoints = PlotPoints::new(points.iter().map(|v| v.0.into()).collect());
        let points = Line::new(plotpoints).color(color).width(2.);
        ui.line(points);
    }
}
