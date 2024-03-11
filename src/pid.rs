use std::error::Error;

use eframe::egui;
use egui::{containers::Window, Context};

pub struct Pid {
    pub window: bool,
    kp: String,
    ki: String,
    kd: String,
    err: Option<Box<dyn Error>>,
}

impl Default for Pid {
    fn default() -> Self {
        Self {
            window: false,
            kp: String::new(),
            ki: String::new(),
            kd: String::new(),
            err: None,
        }
    }
}

impl Pid {
    pub fn draw(&mut self, ctx: &Context) -> Option<(f64, f64, f64)> {
        let mut ret = None;
        let mut open = self.window;
        Window::new("UI Help")
            .resizable(true)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("kp");
                    ui.text_edit_singleline(&mut self.kp);
                });
                ui.horizontal(|ui| {
                    ui.label("ki");
                    ui.text_edit_singleline(&mut self.ki);
                });
                ui.horizontal(|ui| {
                    ui.label("kd");
                    ui.text_edit_singleline(&mut self.kd);
                });
                if ui.button("Submit").clicked() {
                    match self.parse() {
                        Ok(v) => {
                            ret = Some(v);
                            self.err = None
                        }
                        Err(e) => self.err = Some(e),
                    }
                }
                if let Some(ref e) = self.err {
                    ui.label(format!("{e}"));
                }
            });
        self.window = open;
        ret
    }
    fn parse(&self) -> Result<(f64, f64, f64), Box<dyn Error>> {
        Ok((self.kp.parse()?, self.ki.parse()?, self.kd.parse()?))
    }
}
