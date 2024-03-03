use communication::plot;
use eframe::egui;
use egui::{containers::Window, Context, Ui};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Manager {
    graphs: HashMap<String, Graph>,
}

impl Manager {
    fn add_point(&mut self, plot_name: &str, subplot_name: &str, point: [f64; 2]) {
        match self.graphs.get_mut(plot_name) {
            Some(graph) => graph.add_point(subplot_name, point),
            None => {
                self.graphs
                    .insert(plot_name.to_owned(), Graph::new(plot_name.to_owned()));
            }
        }
    }
    pub fn add_buffers(&mut self, buffers: Vec<((String, String), communication::plot::Buffer)>) {
        for ((plot_name, subplot_name), buffer) in buffers {
            let subplot_names = if subplot_name == plot_name {
                ["x".into(), "y".into(), "z".into()]
            } else {
                [
                    format!("{subplot_name} (x)"),
                    format!("{subplot_name} (y)"),
                    format!("{subplot_name} (z)"),
                ]
            };
            match buffer {
                plot::Buffer::Scalar(v) => {
                    for (time, scalar) in v {
                        self.add_point(&plot_name, &subplot_name, [time.as_secs_f64(), scalar]);
                    }
                }
                plot::Buffer::Vec2(v) => {
                    for (time, [x, y]) in v {
                        self.add_point(&plot_name, &subplot_names[0], [time.as_secs_f64(), x]);
                        self.add_point(&plot_name, &subplot_names[1], [time.as_secs_f64(), y]);
                    }
                }
                plot::Buffer::Vec3(v) => {
                    for (time, [x, y, z]) in v {
                        self.add_point(&plot_name, &subplot_names[0], [time.as_secs_f64(), x]);
                        self.add_point(&plot_name, &subplot_names[1], [time.as_secs_f64(), y]);
                        self.add_point(&plot_name, &subplot_names[2], [time.as_secs_f64(), z]);
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
    subplots: HashMap<String, Vec<[f64; 2]>>,
}

impl Graph {
    pub fn new(name: String) -> Self {
        Self {
            enabled: false,
            name,
            subplots: HashMap::new(),
        }
    }
    pub fn add_point(&mut self, subplot_name: &str, point: [f64; 2]) {
        match self.subplots.get_mut(subplot_name) {
            Some(vec) => {
                assert!(!vec.is_empty());
                if let Some(last) = vec.last() {
                    if last[0] > point[0] {
                        // clear graph is robot is restarted (point is earlier in time then last point)
                        log::warn!("Detected point with time before last point. Robot has probably been restarted: resetting plot.");
                        return {
                            self.subplots.clear();
                            self.add_point(subplot_name, point);
                        };
                    } else if point[0] - last[0] > 0.1 {
                        log::warn!("Detected time skip ({:.1}s to {:.1}s)", last[0], point[0]);
                    }
                    vec.push(point);
                }
            }
            None => {
                self.subplots.insert(subplot_name.to_owned(), vec![point]);
            }
        }
    }
    pub fn draw(&mut self, ctx: &Context) {
        Window::new(format!("graph: {}", self.name))
            .resizable(true)
            .vscroll(true)
            .open(&mut self.enabled)
            .show(ctx, |ui| {
                let mut lines = Vec::new();
                let mut legend = self.subplots.len() != 1;
                for (name, subplot_points) in &self.subplots {
                    legend |= !name.is_empty();
                    lines.push(egui_plot::Line::new(subplot_points.clone()).name(name));
                }
                let mut plot = egui_plot::Plot::new(self.name.clone()).view_aspect(2.0);
                if legend {
                    plot = plot.legend(egui_plot::Legend::default());
                }
                plot.show(ui, |plot_ui| {
                    for line in lines {
                        plot_ui.line(line);
                    }
                });
            });
    }
}
