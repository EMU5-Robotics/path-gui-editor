use crate::egui::Label;
use communication::{client::Client, SimpleLog, ToClient};
use eframe::egui;
use egui::{containers::Window, Context};
use std::time::SystemTime;
use std::time::{Duration, Instant};

pub struct Comms {
    addr: String,
    client: Option<Client>,
    pub log_window: bool,
    logs: Vec<SimpleLog>,
    last_checked: Instant,
}

impl Comms {
    pub fn new(addr: &str) -> Self {
        let mut a = Self {
            addr: addr.to_string(),
            client: None,
            log_window: false,
            logs: Vec::new(),
            last_checked: Instant::now(),
        };
        a.poll_logs();
        a
    }
    pub fn poll_logs(&mut self) {
        // ensure we don't poll too often
        if self.last_checked.elapsed() < Duration::from_millis(250) {
            return;
        }

        self.last_checked = Instant::now();

        // try to establish a connection to the robot if it doesn't already exist
        if self.client.is_none() {
            match Client::new(&self.addr) {
                Ok(c) => {
                    println!("connection established");
                    self.client = Some(c);
                }
                Err(_) => return,
            }
        }

        let Some(ref mut client) = self.client else {
            unreachable!()
        };

        // try read for new data
        let Ok(new_pkts) = client.receive_data() else {
            return;
        };

        // extract logs
        for pkt in new_pkts {
            if let ToClient::Log(l) = pkt {
                self.logs.push(l);
            }
        }
    }
    pub fn draw(&mut self, ctx: &Context) {
        self.poll_logs();
        self.draw_logs(ctx);
    }
    fn draw_logs(&mut self, ctx: &Context) {
        let create_row = |ui: &mut egui::Ui, log: &SimpleLog| {
            ui.add(Label::new(log.level.to_string()).wrap(true));
            ui.add(Label::new(log.msg.to_owned()).wrap(true));
            ui.add(Label::new(Self::format_timestamp(log.timestamp)).wrap(true));
            ui.end_row();
        };

        Window::new("Logs")
            .resizable(true)
            .open(&mut self.log_window)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("action help")
                        .striped(true)
                        .num_columns(3)
                        .show(ui, |ui| {
                            ui.heading("Level");
                            ui.heading("Message");
                            ui.heading("Time");
                            ui.end_row();
                            for log in &self.logs {
                                create_row(ui, log);
                            }
                        });
                });
            });
    }
    fn format_timestamp(t: SystemTime) -> String {
        <SystemTime as Into<time::OffsetDateTime>>::into(t)
            .format(time::macros::format_description!(
                "[hour]:[minute]:[second]"
            ))
            .unwrap()
    }
}
