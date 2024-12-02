use std::collections::HashMap;

use crate::{
    robot::Robot,
    //robot_state::{ActionBuilderWindow, RobotState},
    tools::Tools,
    vec::Vec2,
};
//use communication::path::Action;
use eframe::egui::{self, Context, Rgba, TextureHandle, TextureOptions};
use egui_plot::{Line, PlotPoints, PlotUi, Points, Polygon};

pub struct Plot {
    img: TextureHandle,
    //pub actions: RobotState,
    //pub action_builder_window: ActionBuilderWindow,
    tools: Tools,
    robots: HashMap<String, ([f64; 2], [f64; 2], f64)>,
}

impl Plot {
    pub fn new(ctx: &Context) -> Self {
        Self {
            img: Self::load_field_image(ctx),
            /*actions: RobotState::from(vec![
                Action::StartAt {
                    pos: Vec2([0.0, -1.7]).0,
                    heading: 0.,
                },
                Action::MoveRelAbs { rel: 0.2 },
                Action::MoveRel { rel: 1. },
            ]),
            action_builder_window: ActionBuilderWindow::new(),*/
            tools: Tools::default(),
            robots: HashMap::new(),
        }
    }
    fn load_field_image(ctx: &Context) -> TextureHandle {
        // load ColorImage according to https://docs.rs/epaint/0.24.1/epaint/image/struct.ColorImage.html
        let data = image::io::Reader::open("res/field.png")
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

                self.tools.draw(plot_ui);

                for (robot_name, ([width, height], pos, heading)) in self.robots.iter() {
                    // draw name at robot pos
                    plot_ui.text(
                        egui_plot::Text::new((*pos).into(), robot_name).color(egui::Color32::GOLD),
                    );
                    let hwidth = 0.5 * width;
                    let hheight = 0.5 * height;
                    let mut points = [
                        [-hwidth, -hheight],
                        [-hwidth, hheight],
                        [hwidth, hheight],
                        [hwidth, -hheight],
                        [-0.5 * hwidth, 1.1 * hheight],
                        [0.0, 1.2 * hheight],
                        [0.5 * hwidth, 1.1 * hheight],
                    ];

                    let (s, c) = heading.sin_cos();

                    for point in points.iter_mut() {
                        // rotate points
                        *point = [
                            point[0] * c - point[1] * s + pos[0],
                            point[0] * s + point[1] * c + pos[1],
                        ];
                    }

                    let rect = Polygon::new(points[..4].to_vec()).color(egui::Color32::GREEN);
                    let arrow = Line::new(points[4..].to_vec()).color(egui::Color32::GREEN);

                    plot_ui.polygon(rect);
                    plot_ui.line(arrow)
                }
            });

            self.tools.draw_defered(ui, &plot_resp);
        });
    }
    pub fn set_tools(&mut self, tools: Tools) {
        self.tools = tools;
    }
    pub fn draw_points(ui: &mut PlotUi, points: &[Vec2], color: Rgba) {
        let plotpoints = PlotPoints::new(points.iter().map(|v| v.0).collect());
        let points = Points::new(plotpoints).filled(true).color(color).radius(4.);
        ui.points(points);
    }

    pub fn draw_lines(ui: &mut PlotUi, points: &[Vec2], color: Rgba) {
        let plotpoints = PlotPoints::new(points.iter().map(|v| v.0).collect());
        let points = Line::new(plotpoints).color(color).width(2.);
        ui.line(points);
    }
    pub fn set_odom(&mut self, name: String, dim: [f64; 2], pos: [f64; 2], heading: f64) {
        let _ = self.robots.insert(name, (dim, pos, heading));
    }
}
