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

#[derive(Default)]
pub struct RobotState {
    actions: Vec<Action>,
    pos: Vec2,
    heading: f64,
}

impl From<Vec<Action>> for RobotState {
    fn from(actions: Vec<Action>) -> Self {
        let mut ret = RobotState::default();

        for action in actions {
            let _ = ret.try_action(action);
        }

        ret
    }
}

impl RobotState {
    // TODO:
    // 1: figure out how code should be structured
    // to reduce code duplication but preserve needed information
    // (change current duplication once we figure out what is needed)
    pub fn render(&mut self, ui: &mut PlotUi) {
        if self.actions.is_empty() {
            return;
        }

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

    pub fn try_action(&mut self, action: Action) -> Result<(), ActionCreationError> {
        match self.is_valid_next(&action) {
            Ok(()) => {
                self.do_action(&action);
                self.add_action(action);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    // Need better implementation
    pub fn remove_last(&mut self) {
        self.actions.pop();
        self.pos = Default::default();
        self.heading = Default::default();
        for action in &self.actions {
            action.modify_position(&mut self.pos, &mut self.heading);
        }
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
                *self.pos.mut_x() += self.heading.cos() * rel;
                *self.pos.mut_y() += self.heading.sin() * rel;
            }
            Action::MoveTo { pos: new_pos } => {
                let del_x = new_pos.x() - self.pos.x();
                let del_y = new_pos.y() - self.pos.y();
                self.heading = del_y.atan2(del_x);
                self.pos = *new_pos;
            }
            Action::TurnRel { angle } | Action::TurnRelAbs { angle } => {
                self.heading += angle;
            }
            Action::TurnTo {
                heading: new_heading,
            } => {
                self.heading = *new_heading;
            }
        }
    }

    fn is_valid_next(&self, action: &Action) -> Result<(), ActionCreationError> {
        if self.actions.is_empty() && !matches!(action, &Action::StartAt { pos: _, heading: _ }) {
            return Err(ActionCreationError::NoStart);
        }

        if !self.actions.is_empty() && matches!(action, &Action::StartAt { pos: _, heading: _ }) {
            return Err(ActionCreationError::ExtraStart);
        }

        match action {
            Action::StartAt { pos, heading: _ } | Action::MoveTo { pos } => {
                if !(pos.x().abs() < 1.8288 && pos.y().abs() < 1.8288) {
                    return Err(ActionCreationError::OutOfBounds);
                }
            }
            Action::MoveRel { rel } | Action::MoveRelAbs { rel } => {
                let mut pos = self.pos;
                *pos.mut_x() += self.heading.cos() * rel;
                *pos.mut_y() += self.heading.sin() * rel;

                if !(pos.x().abs() < 1.8288 && pos.y().abs() < 1.8288) {
                    return Err(ActionCreationError::OutOfBounds);
                }
            }
            Action::TurnRel { angle: _ } | Action::TurnRelAbs { angle: _ } | Action::TurnTo { heading: _ } => (),
        };

        Ok(())
    }
}
