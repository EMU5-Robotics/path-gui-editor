use egui_plot::{Line, PlotPoints, PlotUi, Points};
use std::f64::{consts::PI, NAN};
use crate::vec::Vec2;

pub enum ActionError {
    NoStartingPos,
    OutOfBounds,
    LateStart,
}

pub enum Action {
    StartAt { pos: [f64; 2], heading: f64 },
    MoveRel { rel: f64 },
    MoveRelAbs { rel: f64 },
    MoveTo { pos: [f64; 2] },
}

impl Action {
    pub const STARTAT: Action = Self::StartAt {
        pos: [NAN, NAN],
        heading: NAN,
    };
    pub const MOVEREL: Action = Self::MoveRel { rel: NAN };
    pub const MOVERELABS: Action = Self::MoveRelAbs { rel: NAN };
    pub const MOVETO: Action = Self::MoveTo { pos: [NAN, NAN] };

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
                format!("({}m, {}m) @ {} deg", pos[0], pos[1], heading * 180. / PI)
            }
            Self::MoveRel { rel } | Self::MoveRelAbs { rel } => {
                format!("{rel}m")
            }
            Self::MoveTo { pos } => {
                format!("({}m, {}m)", pos[0], pos[1])
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
    pub fn modify_position(&self, pos: &mut [f64; 2], heading: &mut f64) {
        match self {
            Self::StartAt {
                pos: start_pos,
                heading: start_heading,
            } => {
                *pos = *start_pos;
                *heading = *start_heading;
            }
            Self::MoveRel { rel } | Self::MoveRelAbs { rel } => {
                pos[0] -= heading.sin() * rel;
                pos[1] += heading.cos() * rel;
            }
            Self::MoveTo { pos: new_pos } => {
                *pos = *new_pos;
                // calculate new heading
                todo!()
            }
        }
    }
    pub fn validate(actions: &[Self]) -> Result<(), ActionError> {
        // an empty path is still a valid path
        if actions.is_empty() {
            return Ok(());
        }

        let mut actions = actions.iter();

        let mut pos: [f64; 2];
        let mut heading: f64;

        // ensure StartAt is called before first movement
        // TODO:
        // 1: expand to allow for non movement
        // actions before StartAt?
        // 2: Take into consideration bounding box
        // and heading
        let Some(Self::StartAt {
            pos: start_pos,
            heading: start_heading,
        }) = actions.next()
        else {
            return Err(ActionError::NoStartingPos);
        };

        // check bot within bounds
        if start_pos[0].abs() >= 1.8288 || start_pos[1].abs() >= 1.8288 {
            return Err(ActionError::OutOfBounds);
        }
        pos = *start_pos;
        heading = *start_heading;

        for action in actions {
            // ensure StartAt is not called twice
            if let Self::StartAt { .. } = action {
                return Err(ActionError::LateStart);
            }

            action.modify_position(&mut pos, &mut heading);
            // check bot within bounds after each movement
            if pos[0].abs() >= 1.8288 || pos[1].abs() >= 1.8288 {
                return Err(ActionError::OutOfBounds);
            }
        }

        Ok(())
    }
    // TODO:
    // 1: figure out how code should be structured
    // to reduce code duplication but preserve needed information
    // (change current duplication once we figure out what is needed)
    pub fn render(actions: &[Self], ui: &mut PlotUi) -> Result<(), ActionError> {
        if actions.is_empty() {
            return Ok(());
        }

        Self::validate(actions)?;

        let mut actions = actions.iter();

        let mut pos: [f64; 2];
        let mut heading: f64;

        let Some(Self::StartAt {
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
            if (Vec2(before_pos) - Vec2(pos)).mag() > 0.01 {
                path.push(pos);
            }
        }

        // draw lines
        ui.line(
            Line::new(PlotPoints::new(path.clone()))
                .color(eframe::egui::Rgba::BLACK)
                .width(2.),
        );

        // draw points
        ui.points(
            Points::new(PlotPoints::new(vec![*start_pos]))
                .filled(true)
                .radius(4.)
                .color(eframe::egui::Rgba::RED)
                .highlight(true)
                .name("Start"),
        );
        ui.points(
            Points::new(PlotPoints::new(path[1..].to_vec()))
                .filled(true)
                .radius(4.)
                .color(eframe::egui::Rgba::GREEN),
        );

        Ok(())
    }
}
