use crate::{plot::Plot, vec::Vec2};
use eframe::egui::Rgba;
use egui_plot::PlotUi;

mod action;
mod action_builder;

use self::action::ActionCreationError;
pub use self::{
    action::Action,
    action_builder::{ActionBuilderMenu, ActionBuilderWindow},
};

pub struct RobotState {
    actions: Vec<Action>,
    pos: Vec2,
    heading: f64,
}

impl From<Vec<Action>> for RobotState {
    fn from(actions: Vec<Action>) -> Self {
        let mut ret = RobotState {
            actions: Default::default(),
            pos: Default::default(),
            heading: Default::default(),
        };
        for action in actions {
            let _ = ret.try_action(action);
        }
        ret
    }
}

impl RobotState {
    /* pub fn validate(&mut self) {
        // an empty path is still a valid path
        if self.actions.is_empty() {
            return self.valid = Ok(());
        }

        let mut actions = self.actions.iter();

        let mut pos: Vec2;
        let mut heading: f64;

        // ensure StartAt is called before first movement
        // TODO:
        // 1: expand to allow for non movement
        // actions before StartAt?
        // 2: Take into consideration bounding box
        // and heading
        let Some(Action::StartAt {
            pos: start_pos,
            heading: start_heading,
        }) = actions.next()
        else {
            return self.valid = Err(ActionError::NoStartingPos);
        };

        // check bot within bounds
        if start_pos.x().abs() >= 1.8288 || start_pos.y().abs() >= 1.8288 {
            return self.valid = Err(ActionError::OutOfBounds);
        }
        pos = *start_pos;
        heading = *start_heading;

        for action in actions {
            // ensure StartAt is not called twice
            if let Action::StartAt { .. } = action {
                return self.valid = Err(ActionError::LateStart);
            }

            self.try_action(*action);
            // check bot within bounds after each movement
            if pos.x().abs() >= 1.8288 || pos.y().abs() >= 1.8288 {
                return self.valid = Err(ActionError::OutOfBounds);
            }
        }

        self.valid = Ok(());
    } */

    // TODO:
    // 1: figure out how code should be structured
    // to reduce code duplication but preserve needed information
    // (change current duplication once we figure out what is needed)
    pub fn render(&mut self, ui: &mut PlotUi) {
        if self.actions.is_empty() {
            return;
        }
        /* 
        self.validate();

        if self.valid.is_err() {
            return;
        } */

        let mut actions = self.actions.iter();

        let mut pos: Vec2;
        let mut heading: f64;

        let Some(Action::StartAt {
            pos: start_pos,
            heading: start_heading,
        }) = actions.next()
        else {
            unreachable!()
        };

        pos = *start_pos;
        heading = *start_heading;

        let mut path = vec![*start_pos];

        for action in actions {
            let idx = path.len();
            let before_pos = path[idx - 1];

            action.modify_position(&mut pos, &mut heading);

            // don't draw new point if it's within 1cm
            if (before_pos - pos).mag() > 0.01 {
                path.push(pos);
            }
        }

        Plot::draw_lines(ui, path.clone(), Rgba::BLACK);
        Plot::draw_points(ui, vec![*start_pos], Rgba::RED);
        Plot::draw_points(ui, path[1..].to_vec(), Rgba::GREEN);
    }

    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    /* pub fn is_valid(&self) -> bool {
        self.valid.is_ok()
    } */

    pub fn try_action(&mut self, action: Action) -> Result<(), ActionCreationError> {
        match self.is_valid_next(&action) {
            Ok(_) => Ok({
                self.do_action(&action);
                self.add_action(action);
            }),
            Err(err) => Err(err),
        }
    }
    
    fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn remove_last(&mut self) {
        self.actions.pop();
    }

    fn do_action(&mut self, action: &Action) {
        match action {
            Action::StartAt {
                pos: start_pos,
                heading: start_heading,
            } => {
                self.pos = *start_pos;
                self.heading = *start_heading;
            }
            Action::MoveRel { rel } | Action::MoveRelAbs { rel } => {
                *self.pos.mut_x() -= self.heading.sin() * rel;
                *self.pos.mut_y() += self.heading.cos() * rel;
            }
            Action::MoveTo { pos: new_pos } => {
                self.heading = (self.pos.y()/self.pos.x()).atan();
                self.pos = *new_pos;
            }
        }
    }

    fn is_valid_next(&self, action: &Action) -> Result<(), ActionCreationError> {
        if self.actions.is_empty() && !matches!(action, &Action::StartAt { pos: _, heading: _ }) {
            return Err(ActionCreationError::NoStart);
        }

        if !self.actions.is_empty() && matches!(action, &Action::StartAt { pos: _, heading: _}) {
            return Err(ActionCreationError::ExtraStart);
        }

        match action {
            Action::StartAt { pos, heading: _ } => {
                if pos.x().abs() >= 1.8288 || pos.y().abs() >= 1.8288 {
                    return Err(ActionCreationError::OutOfBounds);
                }
            },
            Action::MoveRel { rel } | Action::MoveRelAbs { rel } => {
                let mut pos = self.pos.clone();
                *pos.mut_x() -= self.heading.sin() * rel;
                *pos.mut_y() += self.heading.cos() * rel;

                if pos.x().abs() >= 1.8288 || pos.y().abs() >= 1.8288 {
                    return Err(ActionCreationError::OutOfBounds);
                }
            },
            Action::MoveTo { pos } => {
                if pos.x().abs() >= 1.8288 || pos.y().abs() >= 1.8288 {
                    return Err(ActionCreationError::OutOfBounds);
                }
            },
        };

        Ok(())
    }
}
