use crate::{plot::Plot, vec::Vec2};
use eframe::egui::Rgba;
use egui_plot::PlotUi;
use std::f64::{consts::PI, NAN};

pub struct Actions {
    actions: Vec<Action>,
    valid: Result<(), ActionError>,
}

impl Actions {
    pub fn from_actions(actions: Vec<Action>) -> Self {
        let mut ret = Self {
            actions,
            valid: Ok(()),
        };
        ret.validate();
        ret
    }
    pub fn validate(&mut self) {
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

            action.modify_position(&mut pos, &mut heading);
            // check bot within bounds after each movement
            if pos.x().abs() >= 1.8288 || pos.y().abs() >= 1.8288 {
                return self.valid = Err(ActionError::OutOfBounds);
            }
        }

        self.valid = Ok(());
    }
    // TODO:
    // 1: figure out how code should be structured
    // to reduce code duplication but preserve needed information
    // (change current duplication once we figure out what is needed)
    pub fn render(&mut self, ui: &mut PlotUi) {
        if self.actions.is_empty() {
            return;
        }

        self.validate();

        if self.valid.is_err() {
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

        Plot::draw_lines(ui, &path, Rgba::BLACK);
        Plot::draw_points(ui, &[*start_pos], Rgba::RED);
        Plot::draw_points(ui, &path[1..], Rgba::GREEN);
    }
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }
    pub fn is_valid(&self) -> bool {
        self.valid.is_ok()
    }
}

pub enum ActionError {
    NoStartingPos,
    OutOfBounds,
    LateStart,
}

pub enum Action {
    StartAt { pos: Vec2, heading: f64 },
    MoveRel { rel: f64 },
    MoveRelAbs { rel: f64 },
    MoveTo { pos: Vec2 },
}

impl Action {
    pub const STARTAT: Action = Self::StartAt {
        pos: Vec2::NONE,
        heading: NAN,
    };
    pub const MOVEREL: Action = Self::MoveRel { rel: NAN };
    pub const MOVERELABS: Action = Self::MoveRelAbs { rel: NAN };
    pub const MOVETO: Action = Self::MoveTo { pos: Vec2::NONE };

    pub const fn name(&self) -> &str {
        match self {
            Self::StartAt { .. } => "Start At",
            Self::MoveRel { .. } | Self::MoveRelAbs { .. } => "Move",
            Self::MoveTo { .. } => "Move To",
        }
    }
    pub fn value(&self) -> String {
        match self {
            Self::StartAt { pos, heading } => {
                format!("({}m, {}m) @ {} deg", pos.x(), pos.y(), heading * 180. / PI)
            }
            Self::MoveRel { rel } | Self::MoveRelAbs { rel } => {
                format!("{rel}m")
            }
            Self::MoveTo { pos } => {
                format!("({}m, {}m)", pos.x(), pos.y())
            }
        }
    }
    pub const fn modifiers(&self) -> &str {
        match self {
            Self::StartAt { .. } | Self::MoveTo { .. } => "Absolute",
            Self::MoveRel { .. } => "Relative",
            Self::MoveRelAbs { .. } => "Relative (precomputed)",
        }
    }
    pub const fn description(&self) -> &str {
        match self {
            Self::StartAt { .. } => "Sets the robots current position to the specified",
            Self::MoveRel { .. } => "Moves the robot forwards or backwards by the amount specified, where a negative number will make the robot go backwards. This is a relative command meaning it will generate the target point based on the current position given by odometry. Currently this will move the robot in a straight line.",
            Self::MoveRelAbs { .. } => "Moves the robot forwards or backwards by the amount specified, where a negative number will make the robot go backwards. This is a precomputed relative command (not to be confused with relative command) meaning it will generate the target point based on where the robot should be *in theory* meaning it does not need odometry to calculate the target point and instead can calculate it ahead of time. You probably don't want to use this after using MoveRel. Currently this will move the robot in a straight line.",
            Self::MoveTo { .. } => "Moves the robot to the specified target position. Currently this will move the robot in a straight line.",
        }
    }
    pub fn modify_position(&self, pos: &mut Vec2, heading: &mut f64) {
        match self {
            Self::StartAt {
                pos: start_pos,
                heading: start_heading,
            } => {
                *pos = *start_pos;
                *heading = *start_heading;
            }
            Self::MoveRel { rel } | Self::MoveRelAbs { rel } => {
                *pos.mut_x() -= heading.sin() * rel;
                *pos.mut_y() += heading.cos() * rel;
            }
            Self::MoveTo { pos: new_pos } => {
                *pos = *new_pos;
                // calculate new heading
                todo!()
            }
        }
    }
}
