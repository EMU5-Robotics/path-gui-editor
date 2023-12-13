use eframe::egui;
use egui::{containers::Window, widgets::Label, Context};

use crate::actions::Action;

#[derive(Default)]
pub struct Help {
    pub actions: bool,
    pub ui: bool,
}

impl Help {
    pub fn draw(&mut self, ctx: &Context) {
        self.draw_action_help(ctx);
        self.draw_ui_help(ctx);
    }
    fn draw_action_help(&mut self, ctx: &Context) {
        let create_row = |ui: &mut egui::Ui, act: &Action| {
            ui.add(Label::new(act.name()).wrap(true));
            ui.add(Label::new(act.modifiers()).wrap(true));
            ui.add(Label::new(act.description()).wrap(true));
            ui.end_row();
        };

        Window::new("Action Help")
            .resizable(true)
            .open(&mut self.actions)
            .show(ctx, |ui| {
                egui::Grid::new("action help")
                    .striped(true)
                    .num_columns(5)
                    .show(ui, |ui| {
                        ui.heading("Action");
                        ui.heading("Action Type");
                        ui.heading("Action Description");
                        // ensure button in on the right hand side
                        ui.end_row();
                        for action in &[
                            Action::STARTAT,
                            Action::MOVEREL,
                            Action::MOVERELABS,
                            Action::MOVETO,
                        ] {
                            create_row(ui, action);
                        }
                    });
            });
    }
    fn draw_ui_help(&mut self, ctx: &Context) {
        Window::new("UI Help").resizable(true).open(&mut self.ui).show(ctx, |ui| {
            ui.label("TODO");
        });
    }
}
