use communication::{packets::*, ClientListener};
use eframe::egui;
use egui::Context;

mod graph;
mod help;
mod logging;
mod pid;
mod plot;
mod robot;
mod tools;
mod vec;

use help::Help;
use logging::Logging;
use pid::Pid;
use plot::Plot;
use tools::{PointSelection, Tools};

fn main() {
    env_logger::init();
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
    logging: Logging,
    graphing: graph::Manager,
    listener: ClientListener,
    pid: Pid,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let listener = ClientListener::new(
            "127.0.0.1:8733".parse().unwrap(),
            ClientInfo::new(format!("{}", gethostname::gethostname().to_string_lossy())),
        );
        cc.egui_ctx.set_pixels_per_point(1.5);
        Self {
            help: Help::default(),
            plot: Plot::new(&cc.egui_ctx),
            logging: Logging::default(),
            graphing: graph::Manager::default(),
            listener,
            pid: Pid::default(),
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
                    ui.menu_button("Communication", |ui| {
                        if ui.button("logs").clicked() {
                            self.logging.window = true;
                        }
                        if ui.button("pid").clicked() {
                            self.pid.window = true;
                        }
                    });
                    ui.menu_button("Graphs", |ui| {
                        self.graphing.draw_menu(ui);
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
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        // draw help
        self.help.draw(ctx);

        let pkts = self.listener.get_packets();

        let mut logs = Vec::new();
        let mut point_buffers = Vec::new();
        for pkt in pkts {
            match pkt {
                ToClient::Log(l) => logs.push(l),
                ToClient::PointBuffer(plt_name, subplt_name, buffer) => {
                    point_buffers.push((plt_name, subplt_name, buffer));
                }
                ToClient::Odometry(name, dim, pos, rot) => {
                    self.plot.set_odom(name, dim, pos, rot);
                }
                _ => {}
            }
        }

        if let Some(val) = self.pid.draw(ctx) {
            //self.comms.send_packet(ToRobot::Pid(val));
        }

        // draw logs
        self.logging.add_logs(logs);
        self.logging.draw(ctx);

        // draw graphs
        self.graphing.add_buffers(point_buffers);
        self.graphing.draw_graphs(ctx);

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

        // draw plot with the field and path and tools on it
        self.plot.draw(ctx);
    }
}
