use eframe::egui::Ui;

use super::{
    super::{Action, RobotState},
    field_labels::FieldLabel,
};

pub struct ActionBuilderMenu {
    field_labels: [FieldLabel; 3],
    pub field_inputs: [String; 3],
    status: &'static str,
    pub action: Action,
}

impl ActionBuilderMenu {
    pub fn new() -> ActionBuilderMenu {
        ActionBuilderMenu {
            field_labels: [FieldLabel::X, FieldLabel::Y, FieldLabel::Heading],
            field_inputs: Default::default(),
            status: "",
            action: Action::STARTAT,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, state: &mut RobotState) {
        ui.horizontal(|ui| {
            ui.menu_button("Select Action", |ui| {
                if ui.button("Start At").clicked() {
                    ui.close_menu();
                    self.action = Action::STARTAT;
                    self.field_labels[0] = FieldLabel::X;
                    self.field_labels[1] = FieldLabel::Y;
                    self.field_labels[2] = FieldLabel::Heading;
                }
                if ui.button("Move By").clicked() {
                    ui.close_menu();
                    self.action = Action::MOVEREL;
                    self.field_labels[0] = FieldLabel::Distance;
                    self.field_labels[1] = FieldLabel::None;
                    self.field_labels[2] = FieldLabel::None;
                }
                if ui.button("Move By (Abs) ").clicked() {
                    ui.close_menu();
                    self.action = Action::MOVERELABS;
                    self.field_labels[0] = FieldLabel::Distance;
                    self.field_labels[1] = FieldLabel::None;
                    self.field_labels[2] = FieldLabel::None;
                }
                if ui.button("Move To").clicked() {
                    ui.close_menu();
                    self.action = Action::MOVETO;
                    self.field_labels[0] = FieldLabel::X;
                    self.field_labels[1] = FieldLabel::Y;
                    self.field_labels[2] = FieldLabel::None;
                }
                if ui.button("Turn By").clicked() {
                    ui.close_menu();
                    self.action = Action::TURNREL;
                    self.field_labels[0] = FieldLabel::Angle;
                    self.field_labels[1] = FieldLabel::None;
                    self.field_labels[2] = FieldLabel::None;
                }
                if ui.button("Turn By (Abs)").clicked() {
                    ui.close_menu();
                    self.action = Action::TURNRELABS;
                    self.field_labels[0] = FieldLabel::Angle;
                    self.field_labels[1] = FieldLabel::None;
                    self.field_labels[2] = FieldLabel::None;
                }
                if ui.button("Turn To").clicked() {
                    ui.close_menu();
                    self.action = Action::TURNTO;
                    self.field_labels[0] = FieldLabel::Angle;
                    self.field_labels[1] = FieldLabel::None;
                    self.field_labels[2] = FieldLabel::None;
                }
            });

            ui.label(format!("Action: {}", self.action.name()));
        });

        let mut draw_field = |ui: &mut Ui, num: usize| {
            if self.field_labels[num].enabled() {
                ui.horizontal(|ui| {
                    ui.label(self.field_labels[num].label());
                    ui.text_edit_singleline(&mut self.field_inputs[num]);
                });
            }
        };

        for i in 0..3 {
            draw_field(ui, i);
        }

        ui.horizontal(|ui| {
            if ui.button("Add Action").clicked() {
                match Action::try_from(&*self) {
                    Ok(action) => {
                        self.field_inputs = Default::default();
                        self.status = "";
                        match state.try_action(action) {
                            Ok(()) => (),
                            Err(err) => self.status = err.message(),
                        }
                    }
                    Err(err) => self.status = err.message(),
                };
            }

            ui.label(self.status);
        });
    }
}
