use crate::{actions::Action, vec::Vec2};

use eframe::egui;
use egui::{containers::Window, widgets::Label, Context};

use super::Actions;

pub struct ActionBuilder {
    parameters: [String; 6],
    action: Option<Action>,
    pub open: bool,
}

impl ActionBuilder {
    pub fn new() -> ActionBuilder {
        ActionBuilder {
            parameters: Default::default(),
            action: None,
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
                ui.label(format!("Action: {}", match &self.action {
                    Some(action) => action.name(),
                    None => "",
                }));
                let f1 = !self.parameters[0].is_empty();
                let f2 = !self.parameters[2].is_empty();
                let f3 = !self.parameters[4].is_empty();

                ui.horizontal(|ui| {
                    ui.add_enabled_ui(f1, |ui| {
                        ui.label(self.parameters[0].as_str());
                        ui.text_edit_singleline(&mut self.parameters[1]);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_enabled_ui(f2, |ui| {
                        ui.label(self.parameters[2].as_str());
                        ui.text_edit_singleline(&mut self.parameters[3]);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_enabled_ui(f3, |ui| {
                        ui.label(self.parameters[4].as_str());
                        ui.text_edit_singleline(&mut self.parameters[5]);
                    });
                });
                


                ui.menu_button("Select Action", |ui| {
                    if ui.button("Start At").clicked() {
                        ui.close_menu();
                        self.action = Some(Action::STARTAT);
                        self.parameters[0] = String::from("Select x:");
                        self.parameters[2] = String::from("Select y:");
                        self.parameters[4] = String::from("Select heading (Rad):");

                    }
                    if ui.button("Step By").clicked() {
                        ui.close_menu();
                        self.action = Some(Action::MOVEREL);
                        self.parameters[0] = String::from("Select distance:");
                        self.parameters[2] = String::from("");
                        self.parameters[4] = String::from("");
                        
                    }
                    if ui.button("Move By").clicked() {
                        ui.close_menu();
                        self.action = Some(Action::MOVERELABS);
                        self.parameters[0] = String::from("Select distance:");
                        self.parameters[2] = String::from("");
                        self.parameters[4] = String::from("");

                    }
                    if ui.button("Move To").clicked() {
                        ui.close_menu();
                        self.action = Some(Action::MOVETO);
                        self.parameters[0] = String::from("Select x:");
                        self.parameters[2] = String::from("Select y:");
                        self.parameters[4] = String::from("");
                    }
                });

                if ui.button("Add Action (Submit)").clicked() {
                    ui.close_menu();

                    if let Some(action) = &self.action {
                        let new_action = match action {
                            Action::StartAt { pos: _, heading: _ }=> Action::StartAt {
                                pos: Vec2([self.parameters[1].trim().parse().unwrap(), 
                                            self.parameters[3].trim().parse().unwrap()]),
                                heading: self.parameters[5].trim().parse().unwrap(),
                            },
                            Action::MoveRel { rel: _ } => Action::MoveRel { 
                                rel: self.parameters[1].trim().parse().unwrap(),
                            },
                            Action::MoveRelAbs { rel: _ } => Action::MoveRelAbs { 
                                rel: self.parameters[1].trim().parse().unwrap(),
                            },
                            Action::MoveTo { pos: _ }  => Action::MoveTo { 
                                pos:  Vec2([self.parameters[1].trim().parse().unwrap(), 
                                            self.parameters[3].trim().parse().unwrap()]),
                            },
                        };
                        actions.actions.push(new_action);
                    }
                    
                    self.parameters = Default::default();
                    self.action = None;
                }
            });
    }
}