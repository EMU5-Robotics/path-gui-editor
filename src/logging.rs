use crate::egui::Label;
use communication::SimpleLog;
use eframe::egui;
use egui::{containers::Window, Context};
use std::time::SystemTime;

#[derive(Debug, Default)]
pub struct Logging {
    pub window: bool,
    logs: Vec<SimpleLog>,
}

impl Logging {
    pub fn add_logs(&mut self, logs: Vec<SimpleLog>) {
        self.logs.extend(logs);
    }
    pub fn draw(&mut self, ctx: &Context) {
        let create_row = |ui: &mut egui::Ui, log: &SimpleLog| {
            ui.add(Label::new(log.level.to_string()).wrap(true));
            ui.add(Label::new(log.msg.clone()).wrap(true));
            ui.add(Label::new(Self::format_timestamp(log.timestamp)).wrap(true));
            ui.end_row();
        };

        Window::new("Logs")
            .resizable(true)
            .open(&mut self.window)
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
