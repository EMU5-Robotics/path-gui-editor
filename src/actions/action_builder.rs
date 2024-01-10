use std::num::ParseFloatError;

use crate::vec::Vec2;

use super::{Action, Actions};


pub enum ActionCreationError {
    FieldError,
}

impl From<ParseFloatError> for ActionCreationError {
    fn from(value: ParseFloatError) -> Self {
        let _ = value;
        ActionCreationError::FieldError
    }
}

pub struct ActionBuilder {
    pub action: Option<Action>,
}

impl ActionBuilder {
    pub fn new() -> ActionBuilder {
        ActionBuilder {
            action: None,
        }
    }

    pub fn try_add_action(&mut self, inputs: &[String; 3], actions: &mut Actions) 
    -> Result<(), ActionCreationError>  {
        if let Some(action) = &self.action {
            let new_action = match action {
                Action::StartAt { pos: _, heading: _ } => {
                    Action::StartAt { pos: Vec2([
                        inputs[0].trim().parse()?,
                        inputs[1].trim().parse()?]), 
                    heading: inputs[2].trim().parse()? }
                },

                Action::MoveRel { rel: _ } => Action::MoveRel { 
                    rel: inputs[0].trim().parse()?,
                },

                Action::MoveRelAbs { rel: _ } => Action::MoveRelAbs { 
                    rel: inputs[0].trim().parse()?,
                },

                Action::MoveTo { pos: _ } => Action::MoveTo { 
                    pos:  Vec2([inputs[0].trim().parse()?, 
                                inputs[1].trim().parse()?]),
                },

            };

            actions.add(new_action);
        };

        Ok(())
    }
}