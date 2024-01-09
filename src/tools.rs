use std::f64::consts::FRAC_1_PI;

use eframe::egui::{self, Rgba, Ui};
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

            let wt: egui::widget_text::WidgetText = format!("{:.3}m", diff.mag()).into();
            let galley = wt
                .into_galley(ui, None, 100.0, egui::style::FontSelection::Default)
                .galley;
            let text_pos = (p1 + p2) / 2.0;

            let dir = diff.normalised();
            let inv_gradient = Vec2([dir.y(), dir.x()]) * (dir.x() * dir.y()).signum();

            // TODO: calculate proper offset (take into consideration rotation around top left)
            let text_offset = inv_gradient * 20.;

            let rotation = f64::atan(dir.y() / dir.x());

            let points = resp.transform.position_from_point(&text_pos.0.into());
            let mut shape = egui::epaint::TextShape::new(
                [
                    points.x + text_offset.x() as f32,
                    points.y + text_offset.y() as f32,
                ]
                .into(),
                galley,
            );
            shape.angle = -rotation as f32;
            painter.add(egui::Shape::Text(shape));
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
            let circle_centre = resp.transform.position_from_point(&p2.0.into());

            let (v1, v2) = ((p1 - p2).normalised(), (p3 - p2).normalised());
            let angle_offset = (v1 + v2) * 0.5 * 50.;
            let small_angle = v1.dot(&v2).acos().abs() * 180.0 * FRAC_1_PI;
            let large_angle = 360.0 - small_angle;

            let small_angle_text: egui::widget_text::WidgetText =
                format!("{small_angle:.3}°").into();
            let small_angle_text = small_angle_text
                .into_galley(ui, None, 100.0, egui::style::FontSelection::Default)
                .galley;
            let small_angle_text = egui::epaint::TextShape::new(
                [
                    circle_centre.x + angle_offset.x() as f32,
                    circle_centre.y - angle_offset.y() as f32,
                ]
                .into(),
                small_angle_text,
            ); // since +y is down (pixel coordinates)
            let large_angle_text: egui::widget_text::WidgetText =
                format!("{large_angle:.3}°").into();
            let large_angle_text = large_angle_text
                .into_galley(ui, None, 100.0, egui::style::FontSelection::Default)
                .galley;
            let large_angle_text = egui::epaint::TextShape::new(
                [
                    circle_centre.x - angle_offset.x() as f32,
                    circle_centre.y + angle_offset.y() as f32,
                ]
                .into(),
                large_angle_text,
            );

            let circle = egui::epaint::CircleShape::stroke(
                circle_centre,
                10.,
                egui::epaint::Stroke::new(3., egui::epaint::Color32::WHITE),
            );
            painter.add(egui::Shape::Circle(circle));
            painter.add(egui::Shape::Text(small_angle_text));
            painter.add(egui::Shape::Text(large_angle_text));
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
