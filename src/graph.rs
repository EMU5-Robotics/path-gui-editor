use communication::plot;
use eframe::egui;
use egui::{containers::Window, Context, Ui};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Manager {
    graphs: HashMap<String, Graph>,
}

impl Manager {
    fn add_point(&mut self, name: String, point: [f64; 2]) {
        let graph = self.graphs.entry(name.clone()).or_insert(Graph::new(name));

        // clear graph is robot is restarted (point is earlier in time then last point)
        if let Some(last_point) = graph.points.last() {
            if last_point[0] > point[0] {
                log::warn!("Detected point with time before last point. Robot has probably been restarted: resetting plot.");
                graph.points.clear();
            }
        }
        let last = graph.points.last().copied();
        graph.points.push(point);
        if let Some(last) = last {
            if (last[0] - point[0]).abs() > 0.1 {
                log::warn!("Detected time skip ({:.1}s to {:.1}s)", last[0], point[0]);
            }
        }
    }
    pub fn add_buffers(&mut self, buffers: Vec<(String, communication::plot::Buffer)>) {
        for (buffer_name, buffer) in buffers {
            match buffer {
                plot::Buffer::Scalar(v) => {
                    for (time, scalar) in v {
                        self.add_point(buffer_name.clone(), [time.as_secs_f64(), scalar]);
                    }
                }
                plot::Buffer::Vec2(v) => {
                    for (time, [x, y]) in v {
                        self.add_point(format!("{buffer_name}_x"), [time.as_secs_f64(), x]);
                        self.add_point(format!("{buffer_name}_y"), [time.as_secs_f64(), y]);
                    }
                }
                plot::Buffer::Vec3(v) => {
                    for (time, [x, y, z]) in v {
                        self.add_point(format!("{buffer_name}_x"), [time.as_secs_f64(), x]);
                        self.add_point(format!("{buffer_name}_y"), [time.as_secs_f64(), y]);
                        self.add_point(format!("{buffer_name}_z"), [time.as_secs_f64(), z]);
                    }
                }
            }
        }
    }
    pub fn draw_menu(&mut self, ui: &mut Ui) {
        for (name, graph) in &mut self.graphs {
            if ui.button(name).clicked() {
                graph.enabled = true;
                break;
            }
        }
    }
    pub fn draw_graphs(&mut self, ctx: &Context) {
        for graph in self.graphs.values_mut() {
            graph.draw(ctx);
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    enabled: bool,
    name: String,
    points: Vec<[f64; 2]>,
}

impl Graph {
    pub fn new(name: String) -> Self {
        Self {
            enabled: false,
            name,
            points: Vec::new(),
        }
    }
    pub fn draw(&mut self, ctx: &Context) {
        Window::new(format!("graph: {}", self.name))
            .resizable(true)
            .vscroll(true)
            .open(&mut self.enabled)
            .show(ctx, |ui| {
                let line = egui_plot::Line::new(self.points.clone());
                egui_plot::Plot::new(self.name.clone())
                    .view_aspect(2.0)
                    .show(ui, |plot_ui| plot_ui.line(line));
            });
    }
}
