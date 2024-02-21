use communication::path::Action;
use eframe::egui;
use egui::{containers::Window, widgets::Label, Context};

use crate::robot_state::ActionGuiReq;

#[derive(Default)]
pub struct Help {
    pub actions: bool,
    pub ui: bool,
    pub about: bool,
}

impl Help {
    pub fn draw(&mut self, ctx: &Context) {
        self.draw_action_help(ctx);
        self.draw_ui_help(ctx);
        self.draw_about(ctx);
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
            .vscroll(true)
            .open(&mut self.actions)
            .show(ctx, |ui| {
                egui::Grid::new("action help")
                    .striped(true)
                    .num_columns(3)
                    .show(ui, |ui| {
                        ui.heading("Action");
                        ui.heading("Action Type");
                        ui.heading("Action Description");
                        ui.end_row();
                        for action in &[
                            Action::START_AT,
                            Action::MOVE_REL,
                            Action::MOVE_REL_ABS,
                            Action::MOVE_TO,
                            Action::TURN_REL,
                            Action::TURN_REL_ABS,
                            Action::TURN_TO,
                        ] {
                            create_row(ui, action);
                        }
                    });
                // allow blank space
                ui.allocate_space(ui.available_size());
            });
    }

    fn draw_ui_help(&mut self, ctx: &Context) {
        Window::new("UI Help")
            .resizable(true)
            .open(&mut self.ui)
            .show(ctx, |ui| {
                ui.label("TODO");
            });
    }

    fn draw_about(&mut self, ctx: &Context) {
        Window::new("About")
            .resizable(true)
            .open(&mut self.about)
            .show(ctx, |ui| {
                ui.heading("About");
                ui.label("TODO");
            });
    }
}
