use eframe::egui::Ui;

use super::{action_builder::{ActionBuilder, ActionCreationError}, Actions, Action};

pub struct BuilderMenu {
    field_labels: [FieldLabel; 3],
    field_inputs: [String; 3],
    status: &'static str,
    builder: ActionBuilder,
}

enum FieldLabel {
    X,
    Y,
    HEADING,
    DISTANCE,
    NONE,
}

impl FieldLabel {
    fn label(&self) -> &'static str {
        match self {
            FieldLabel::X => "Select x:",
            FieldLabel::Y => "Select y:",
            FieldLabel::HEADING => "Select Heading (rad):",
            FieldLabel::DISTANCE => "Select distance:",
            FieldLabel::NONE => "",
        }
    }

    fn enabled(&self) -> bool {
        match self {
            FieldLabel::NONE => false,
            _ => true,
        }
    }
}

impl Default for FieldLabel {
    fn default() -> Self {
        FieldLabel::NONE
    }
}

impl BuilderMenu {
    pub fn new() -> BuilderMenu {
        BuilderMenu {
            field_labels: Default::default(), 
            field_inputs: Default::default(),
            status: "",
            builder: ActionBuilder::new(),
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, actions: &mut Actions) {

        let mut draw_field = |ui: &mut Ui, num: usize| {
            ui.horizontal(|ui| {
                ui.add_enabled_ui(self.field_labels[num].enabled(), |ui| {
                    ui.label(self.field_labels[num].label());
                    ui.text_edit_singleline(&mut self.field_inputs[num]);
                })
            })
        };

        ui.label(format!("Action: {}", match &self.builder.action {
            Some(action) => action.name(),
            None => "",
        }));

        for i in 0..3 {
            draw_field(ui, i);
        }
        ui.horizontal(|ui| {
            ui.menu_button("Select Action", |ui| {
                if ui.button("Start At").clicked() {
                    ui.close_menu();
                    self.builder.action = Some(Action::STARTAT);
                    self.field_labels[0] = FieldLabel::X;
                    self.field_labels[1] = FieldLabel::Y;
                    self.field_labels[2] = FieldLabel::HEADING;
                }
                if ui.button("Step By").clicked() {
                    ui.close_menu();
                    self.builder.action = Some(Action::MOVEREL);
                    self.field_labels[0] = FieldLabel::DISTANCE;
                    self.field_labels[1] = FieldLabel::NONE;
                    self.field_labels[2] = FieldLabel::NONE;
                }
                if ui.button("Move By").clicked() {
                    ui.close_menu();
                    self.builder.action = Some(Action::MOVERELABS);
                    self.field_labels[0] = FieldLabel::DISTANCE;
                    self.field_labels[1] = FieldLabel::NONE;
                    self.field_labels[2] = FieldLabel::NONE;
                }
                if ui.button("Move To").clicked() {
                    ui.close_menu();
                    self.builder.action = Some(Action::MOVETO);
                    self.field_labels[0] = FieldLabel::X;
                    self.field_labels[1] = FieldLabel::Y;
                    self.field_labels[2] = FieldLabel::NONE;
                }
            });

            if ui.button("Add Action").clicked() {
                match self.builder.try_add_action(&self.field_inputs, actions) {
                    Ok(_) => {
                        self.field_inputs = Default::default();
                        self.status = "";
                    },
                    Err(err) => match err {
                        ActionCreationError::FieldError => {
                           self.status = "Error in fields";
                        },
                    },
                };
            }

            ui.label(self.status);
        });
        

    }
}