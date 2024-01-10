use eframe::egui::{Context, Window};

use super::{super::RobotState, ActionBuilderMenu};

pub struct ActionBuilderWindow {
    menu: ActionBuilderMenu,
    pub open: bool,
}

impl ActionBuilderWindow {
    pub fn new() -> ActionBuilderWindow {
        ActionBuilderWindow {
            menu: ActionBuilderMenu::new(),
            open: false,
        }
    }

    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn draw(&mut self, ctx: &Context, actions: &mut RobotState) {
        Window::new("Add Action")
            .resizable(true)
            .open(&mut self.open)
            .show(ctx, |ui| {
                self.menu.draw(ui, actions);
            });
    }
}
