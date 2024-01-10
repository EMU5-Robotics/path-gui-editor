use std::f64::NAN;

use crate::vec::Vec2;

use super::ActionBuilderMenu;

mod action_creation_error;
pub use action_creation_error::ActionCreationError;

pub enum Action {
    StartAt { pos: Vec2, heading: f64 },
    MoveRel { rel: f64 },
    MoveRelAbs { rel: f64 },
    MoveTo { pos: Vec2 },
    TurnBy { angle: f64 },
    TurnTo { heading: f64 },
}

impl Action {
    pub const STARTAT: Action = Self::StartAt {
        pos: Vec2::NONE,
        heading: NAN,
    };
    pub const MOVEREL: Action = Self::MoveRel { rel: NAN };
    pub const MOVERELABS: Action = Self::MoveRelAbs { rel: NAN };
    pub const MOVETO: Action = Self::MoveTo { pos: Vec2::NONE };
    pub const TURNBY: Action = Self::TurnBy { angle: NAN };
    pub const TURNTO: Action = Self::TurnTo { heading: NAN };

    pub const fn name(&self) -> &str {
        match self {
            Self::StartAt { .. } => "Start At",
            Self::MoveRel { .. } | Self::MoveRelAbs { .. } => "Move",
            Self::MoveTo { .. } => "Move To",
            Self::TurnBy { .. } => "Turn By",
            Self::TurnTo { .. } => "Turn To",
        }
    }
    pub fn value(&self) -> String {
        match self {
            Self::StartAt { pos, heading } => {
                format!(
                    "({}m, {}m) @ {} deg",
                    pos.x(),
                    pos.y(),
                    heading.to_degrees().round(),
                )
            }
            Self::MoveRel { rel } | Self::MoveRelAbs { rel } => {
                format!("{rel}m")
            }
            Self::MoveTo { pos } => {
                format!("({}m, {}m)", pos.x(), pos.y())
            }
            Self::TurnBy { angle } => {
                format!("{} deg", angle.to_degrees().round())
            }
            Self::TurnTo { heading } => {
                format!("{} deg", heading.to_degrees().round())
            }
        }
    }
    pub const fn modifiers(&self) -> &str {
        match self {
            Self::StartAt { .. } | Self::MoveTo { .. } | Self::TurnTo { .. } => "Absolute",
            Self::MoveRel { .. } | Self::TurnBy { .. } => "Relative",
            Self::MoveRelAbs { .. } => "Relative (precomputed)",
        }
    }
    pub const fn description(&self) -> &str {
        match self {
            Self::StartAt { .. } => "Sets the robots current position to the specified",
            Self::MoveRel { .. } => "Moves the robot forwards or backwards by the amount specified, where a negative number will make the robot go backwards. This is a relative command meaning it will generate the target point based on the current position given by odometry. Currently this will move the robot in a straight line.",
            Self::MoveRelAbs { .. } => "Moves the robot forwards or backwards by the amount specified, where a negative number will make the robot go backwards. This is a precomputed relative command (not to be confused with relative command) meaning it will generate the target point based on where the robot should be *in theory* meaning it does not need odometry to calculate the target point and instead can calculate it ahead of time. You probably don't want to use this after using MoveRel. Currently this will move the robot in a straight line.",
            Self::MoveTo { .. } => "Moves the robot to the specified target position. Currently this will move the robot in a straight line.",
            Self::TurnBy { .. } => "Turns the robot clockwise or counter-clockwise by the amount specified, where a negative number will make the robot turn clockwise. This is a relative command meaning it will generate the target heading based on the current heading given by odometry.",
            Self::TurnTo { .. } => "Turns the robot to the specified target heading.",
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
                *pos.mut_x() += heading.cos() * rel;
                *pos.mut_y() += heading.sin() * rel;
            }
            Self::MoveTo { pos: new_pos } => {
                let del_x = new_pos.x() - pos.x();
                let del_y = new_pos.y() - pos.y();
                *heading = del_y.atan2(del_x);
                *pos = *new_pos;
            }
            Self::TurnBy { angle } => {
                *heading += angle;
            }
            Self::TurnTo {
                heading: new_heading,
            } => {
                *heading = *new_heading;
            }
        }
    }
}

impl TryFrom<&ActionBuilderMenu> for Action {
    type Error = ActionCreationError;

    fn try_from(builder: &ActionBuilderMenu) -> Result<Self, Self::Error> {
        match &builder.action {
            Action::StartAt { pos: _, heading: _ } => Ok(Action::StartAt {
                pos: Vec2([
                    builder.field_inputs[0].trim().parse()?,
                    builder.field_inputs[1].trim().parse()?,
                ]),
                heading: builder.field_inputs[0].trim().parse::<f64>()?.to_radians(),
            }),

            Action::MoveRel { rel: _ } => Ok(Action::MoveRel {
                rel: builder.field_inputs[0].trim().parse()?,
            }),

            Action::MoveRelAbs { rel: _ } => Ok(Action::MoveRelAbs {
                rel: builder.field_inputs[0].trim().parse()?,
            }),

            Action::MoveTo { pos: _ } => Ok(Action::MoveTo {
                pos: Vec2([
                    builder.field_inputs[0].trim().parse()?,
                    builder.field_inputs[1].trim().parse()?,
                ]),
            }),

            Action::TurnBy { angle: _ } => Ok(Action::TurnBy {
                angle: builder.field_inputs[0].trim().parse::<f64>()?.to_radians(),
            }),

            Action::TurnTo { heading: _ } => Ok(Action::TurnTo {
                heading: builder.field_inputs[0].trim().parse::<f64>()?.to_radians(),
            }),
        }
    }
}
