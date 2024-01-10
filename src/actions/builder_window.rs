use eframe::egui::{Context, Window};

use super::{Actions, builder_menu::BuilderMenu};


pub struct BuilderWindow {
    menu: BuilderMenu,
    pub open: bool,
}

impl BuilderWindow {
    pub fn new() -> BuilderWindow {
        BuilderWindow {
            menu: BuilderMenu::new(),
            open: false,
        }
    }

    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn draw(&mut self, ctx: &Context, actions: &mut Actions) {
        Window::new("Add Action")
            .resizable(true)
            .open(&mut self.open)
            .show(ctx, |ui| {
                self.menu.draw(ui, actions);
            });
    }

    
}