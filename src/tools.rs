use std::f64::consts::FRAC_1_PI;

use eframe::egui::{self, Rgba, Ui, Painter};
use egui_plot::PlotUi;

use crate::{plot::Plot, vec::Vec2};

#[derive(Default)]
pub struct PointSelection<const N: usize>
where
    [Vec2; N]: Default,
{
    points: [Vec2; N],
    current_size: usize,
}

impl<const N: usize> PointSelection<N>
where
    [Vec2; N]: Default,
{
    pub fn poll(&mut self, plot_ui: &PlotUi) {
        let resp = plot_ui.response();
        let alt_pressed = plot_ui.ctx().input(|i| i.modifiers.alt);

        // if left clicked + alt key & point in plot add point
        if resp.clicked_by(egui::PointerButton::Primary) && alt_pressed && resp.hovered() {
            if let Some(point) = plot_ui.pointer_coordinate() {
                self.add_point(Vec2([point.x, point.y]));
            }
        }
    }
    // add new points till size N then wrap back around
    fn add_point(&mut self, pos: Vec2) {
        let size = &mut self.current_size;
        if *size == N {
            self.points[0] = pos;
            *size = 1;
        } else {
            self.points[*size] = pos;
            *size += 1;
        }
    }
}

#[derive(Default)]
pub enum Tools {
    MeasureDistance {
        selection: PointSelection<2>,
    },
    MeasureAngle {
        selection: PointSelection<3>,
    },
    #[default]
    None,
}

impl Tools {
    pub fn draw(&mut self, plot_ui: &mut PlotUi) {
        match self {
            Self::MeasureDistance { selection } => {
                selection.poll(plot_ui);
                Self::draw_measure(plot_ui, selection);
            }
            Self::MeasureAngle { selection } => {
                selection.poll(plot_ui);
                Self::draw_angle_measure(plot_ui, selection);
            }
            Self::None => {}
        }
    }
    pub fn draw_defered(&self, ui: &Ui, resp: &egui_plot::PlotResponse<()>) {
        match self {
            Self::MeasureDistance { selection } => Self::draw_measure_defered(ui, selection, resp),
            Self::MeasureAngle { selection } => Self::draw_angle_defered(ui, selection, resp),
            Self::None => {}
        }
    }

    fn draw_measure_defered(
        ui: &Ui,
        selection: &PointSelection<2>,
        resp: &egui_plot::PlotResponse<()>,
    ) {
        let painter = ui.ctx().layer_painter(egui::layers::LayerId::new(
            egui::layers::Order::Tooltip,
            egui::Id::new("measure_tool"),
        ));

        let draw_text = |ui: &Ui, p1: Vec2, p2: Vec2| {
            let diff = p2 - p1;

            let text_pos = (p1 + p2) * 0.5;

            let dir = diff.normalised();
            let inv_gradient = Vec2([dir.y(), dir.x()]) * (dir.x() * dir.y()).signum();

            let text_offset = inv_gradient * 15.;

            let rotation = f64::atan(dir.y() / dir.x());

            let points = resp.transform.position_from_point(&text_pos.0.into());
            Self::draw_text_centred_with_rotation(ui,points + text_offset.into(), rotation as f32, &painter, &format!("{:.3}", diff.mag()));

        };

        match selection.current_size {
            0 => {}
            1 => {
                // draw line to pointer if pointer is within plot
                if resp.response.hovered() {
                    if let Some(point) = resp.response.hover_pos() {
                        let point = resp.transform.value_from_position(point);
                        draw_text(ui, selection.points[0], Vec2([point.x, point.y]));
                    }
                }
            }
            2 => {
                draw_text(ui, selection.points[0], selection.points[1]);
            }
            _ => unreachable!(),
        }
    }

    fn draw_text_centred_with_rotation(ui: &Ui, offset: egui::Pos2, rotation: f32, painter: &Painter, text: &str) {
        // note coordinate space is logical pixel coordinates
        // i.e. (0,0) is top left
        // this also means that counterclockwise rotation is actually clockwise
        // rotation with respect to logical pixel coordinates
        // TextShape stores this clockwise rotation
        // Hence we negate rotation to work with logical pixel coordinates
        // and what TextShape stores
        let rotation = -rotation;
        let text: egui::widget_text::WidgetText = text.into();
        let galley = text.into_galley(ui, None, 100.0, egui::style::FontSelection::Default).galley;
        let centre_offset = (galley.rect.max - galley.rect.min) * 0.5;
        let (cos, sin) = (rotation.cos(), rotation.sin());
        let centre_offset = egui::Vec2::new(cos*centre_offset.x - sin*centre_offset.y, sin*centre_offset.x + cos*centre_offset.y);
        let mut text = egui::epaint::TextShape::new([offset.x - centre_offset.x, offset.y - centre_offset.y].into(), galley);
        text.angle = rotation;
        painter.add(egui::Shape::Text(text));
    }

    fn draw_angle_defered(
        ui: &Ui,
        selection: &PointSelection<3>,
        resp: &egui_plot::PlotResponse<()>,
    ) {
        let painter = ui.ctx().layer_painter(egui::layers::LayerId::new(
            egui::layers::Order::Tooltip,
            egui::Id::new("measure_tool"),
        ));

        let draw_circle = |ui: &Ui, p1: Vec2, p2: Vec2, p3: Vec2| {
            let (v1, v2) = ((p1 - p2).normalised(), (p3 - p2).normalised());

            let mut angle_offset = (v1 + v2).normalised() * 25.0;
            // convert angle_offset into logical pixel coordinates
            *angle_offset.mut_y() = -angle_offset.y();
            let angle_offset: egui::Vec2 = angle_offset.into();

            let small_angle = v1.dot(&v2).acos().abs() * 180.0 * FRAC_1_PI;
            let large_angle = 360.0 - small_angle;

            let circle_centre = resp.transform.position_from_point(&p2.0.into());
            Self::draw_text_centred_with_rotation(ui, circle_centre + angle_offset, 0., &painter, &format!("{small_angle:.2}"));
            Self::draw_text_centred_with_rotation(ui, circle_centre - angle_offset, 0., &painter, &format!("{large_angle:.2}"));
        };

        match selection.current_size {
            0 | 1 => {}
            2 => {
                // draw line to pointer if pointer is within plot
                if resp.response.hovered() {
                    if let Some(point) = resp.response.hover_pos() {
                        let point = resp.transform.value_from_position(point);
                        draw_circle(
                            ui,
                            selection.points[0],
                            selection.points[1],
                            Vec2([point.x, point.y]),
                        );
                    }
                }
            }
            3 => {
                draw_circle(
                    ui,
                    selection.points[0],
                    selection.points[1],
                    selection.points[2],
                );
            }
            _ => unreachable!(),
        }
    }

    fn draw_measure(ui: &mut PlotUi, selection: &PointSelection<2>) {
        match selection.current_size {
            0 => {}
            1 => {
                let first_point = selection.points[0];

                // draw line to pointer if pointer is within plot
                if ui.response().hovered() {
                    if let Some(point) = ui.pointer_coordinate() {
                        Plot::draw_lines(
                            ui,
                            &[first_point, [point.x, point.y].into()],
                            Rgba::BLACK,
                        );
                    }
                }

                Plot::draw_points(ui, &[first_point], Rgba::BLUE);
            }
            2 => {
                Plot::draw_lines(ui, &selection.points, Rgba::BLACK);
                Plot::draw_points(ui, &selection.points, Rgba::BLUE);
            }
            _ => unreachable!(),
        }
    }

    fn draw_angle_measure(ui: &mut PlotUi, selection: &PointSelection<3>) {
        match selection.current_size {
            0 => {}
            1 => {
                let first_point = selection.points[0];

                // draw line to pointer if pointer is within plot
                if ui.response().hovered() {
                    if let Some(point) = ui.pointer_coordinate() {
                        Plot::draw_lines(
                            ui,
                            &[first_point, [point.x, point.y].into()],
                            Rgba::BLACK,
                        );
                    }
                }

                Plot::draw_points(ui, &[first_point], Rgba::BLUE);
            }
            2 => {
                let first_point = selection.points[0];
                let second_point = selection.points[1];

                Plot::draw_lines(ui, &[first_point, second_point], Rgba::BLACK);

                // draw line to pointer if pointer is within plot
                if ui.response().hovered() {
                    if let Some(point) = ui.pointer_coordinate() {
                        Plot::draw_lines(
                            ui,
                            &[second_point, [point.x, point.y].into()],
                            Rgba::BLACK,
                        );
                    }
                }
                Plot::draw_points(ui, &[first_point], Rgba::BLUE);
            }
            3 => {
                Plot::draw_lines(ui, &selection.points, Rgba::BLACK);
                Plot::draw_points(ui, &[selection.points[0], selection.points[2]], Rgba::BLUE);
            }
            _ => unreachable!(),
        }
    }
}
