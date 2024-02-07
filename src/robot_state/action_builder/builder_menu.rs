use eframe::egui::{self, Slider, Ui};

use crate::robot_state::action::{
    units::{AngleUnit, LengthUnit},
    ActionGuiReq,
};

use super::{
    super::{Action, RobotState},
    field_labels::FieldLabel,
};

pub struct ActionBuilderMenu {
    pub field_inputs: [String; 3],
    status: &'static str,
    pub action: Action,
    pub length_unit: LengthUnit,
    pub angle_unit: AngleUnit,
    pub velocity: f64,
}

impl Default for ActionBuilderMenu {
    fn default() -> ActionBuilderMenu {
        ActionBuilderMenu {
            field_inputs: Default::default(),
            status: Default::default(),
            action: Action::START_AT,
            length_unit: Default::default(),
            angle_unit: AngleUnit::Degree,
            velocity: 100.,
        }
    }
}

impl ActionBuilderMenu {
    pub fn draw(&mut self, ui: &mut Ui, state: &mut RobotState) {
        let draw_values = |ui: &mut Ui,
                           action_ref: &mut Action,
                           actions: &[Action],
                           field_inputs: &mut [String; 3]| {
            for action in actions {
                if ui
                    .selectable_value(action_ref, action.clone(), action.name())
                    .clicked()
                {
                    ui.close_menu();
                    *field_inputs = Default::default();
                }
            }
        };
        let get_field_label = |action: &Action, num: usize| match (action, num) {
            (
                Action::TurnRel { angle: _ }
                | Action::TurnRelAbs { angle: _ }
                | Action::TurnTo { heading: _ },
                0,
            ) => FieldLabel::Angle,
            (Action::MoveRel { rel: _ } | Action::MoveRelAbs { rel: _ }, 0) => FieldLabel::Distance,
            (Action::StartAt { pos: _, heading: _ } | Action::MoveTo { pos: _ }, 0) => {
                FieldLabel::X
            }
            (Action::StartAt { pos: _, heading: _ } | Action::MoveTo { pos: _ }, 1) => {
                FieldLabel::Y
            }
            (Action::StartAt { pos: _, heading: _ }, 2) => FieldLabel::Heading,
            _ => FieldLabel::None,
        };

        let draw_field =
            |ui: &mut Ui, action: &Action, num: usize, field_inputs: &mut [String; 3]| {
                let field = get_field_label(action, num);
                if field.enabled() {
                    ui.horizontal(|ui| {
                        ui.label(field.label());
                        ui.text_edit_singleline(&mut field_inputs[num]);
                    });
                } else {
                    ui.add_enabled(false, |ui: &mut Ui| {
                        ui.text_edit_singleline(&mut field_inputs[num])
                    });
                }
            };

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.set_width(200.);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Action: ");

                        ui.menu_button(
                            self.action.name().to_owned()
                                + &" ".repeat(15 - self.action.name().len())
                                + "⏷",
                            |ui| {
                                draw_values(
                                    ui,
                                    &mut self.action,
                                    &[Action::START_AT],
                                    &mut self.field_inputs,
                                );

                                ui.menu_button("Move", |ui| {
                                    draw_values(
                                        ui,
                                        &mut self.action,
                                        &[Action::MOVE_REL, Action::MOVE_REL_ABS, Action::MOVE_TO],
                                        &mut self.field_inputs,
                                    );
                                });

                                ui.menu_button("Turn", |ui| {
                                    draw_values(
                                        ui,
                                        &mut self.action,
                                        &[Action::TURN_REL, Action::TURN_REL_ABS, Action::TURN_TO],
                                        &mut self.field_inputs,
                                    );
                                });
                            },
                        );
                    });

                    ui.separator();

                    for i in 0..3 {
                        draw_field(ui, &self.action, i, &mut self.field_inputs)
                    }

                    ui.separator();

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
                })
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.set_width(158.);
                    ui.label("Set Units");
                    egui::ComboBox::new("Length Unit Selection", "")
                        .selected_text(self.length_unit.to_string())
                        .show_ui(ui, |ui| {
                            for unit in [LengthUnit::Metre, LengthUnit::Inch, LengthUnit::Foot] {
                                ui.selectable_value(&mut self.length_unit, unit, unit.to_string());
                            }
                        });

                    egui::ComboBox::new("Angle Unit Selection", "")
                        .selected_text(self.angle_unit.to_string())
                        .show_ui(ui, |ui| {
                            for unit in [AngleUnit::Degree, AngleUnit::Radian] {
                                ui.selectable_value(&mut self.angle_unit, unit, unit.to_string());
                            }
                        });
                });

                ui.group(|ui| {
                    ui.set_width(158.);
                    ui.label("Set Speed");
                    ui.add(
                        Slider::new(&mut self.velocity, 0.0..=100.0)
                            .clamp_to_range(true)
                            .smart_aim(true)
                            .suffix("%"),
                    );
                });
            });
        });
    }
}
