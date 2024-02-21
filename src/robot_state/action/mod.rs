use communication::path::Action;
use std::f64::NAN;

use crate::vec::Vec2;

use self::units::Unit;

use super::ActionBuilderMenu;

mod action_creation_error;
pub use action_creation_error::ActionCreationError;

pub mod units;

pub trait ActionGuiReq {
    const START_AT: Action = Action::StartAt {
        pos: Vec2::NONE.0,
        heading: NAN,
    };
    const MOVE_REL: Action = Action::MoveRel { rel: NAN };
    const MOVE_REL_ABS: Action = Action::MoveRelAbs { rel: NAN };
    const MOVE_TO: Action = Action::MoveTo { pos: Vec2::NONE.0 };
    const TURN_REL: Action = Action::TurnRel { angle: NAN };
    const TURN_REL_ABS: Action = Action::TurnRelAbs { angle: NAN };
    const TURN_TO: Action = Action::TurnTo { heading: NAN };

    fn name(&self) -> &str;
    fn value(&self) -> String;
    fn modifiers(&self) -> &str;
    fn description(&self) -> &str;
    fn modify_position(&self, pos: &mut Vec2, heading: &mut f64);
}

impl ActionGuiReq for Action {
    fn name(&self) -> &str {
        match self {
            Self::StartAt { .. } => "Start At",
            Self::MoveRel { .. } | Self::MoveRelAbs { .. } => "Move",
            Self::MoveTo { .. } => "Move To",
            Self::TurnRel { .. } | Self::TurnRelAbs { .. } => "Turn By",
            Self::TurnTo { .. } => "Turn To",
        }
    }
    fn value(&self) -> String {
        match self {
            Self::StartAt { pos, heading } => {
                format!(
                    "({}m, {}m) @ {} deg",
                    pos[0],
                    pos[1],
                    heading.to_degrees().round(),
                )
            }
            Self::MoveRel { rel } | Self::MoveRelAbs { rel } => {
                format!("{rel}m")
            }
            Self::MoveTo { pos } => {
                format!("({}m, {}m)", pos[0], pos[1])
            }
            Self::TurnRel { angle }
            | Self::TurnRelAbs { angle }
            | Self::TurnTo { heading: angle } => {
                format!("{} deg", angle.to_degrees().round())
            }
        }
    }
    fn modifiers(&self) -> &str {
        match self {
            Self::StartAt { .. } | Self::MoveTo { .. } | Self::TurnTo { .. } => "Absolute",
            Self::MoveRel { .. } | Self::TurnRel { .. } => "Relative",
            Self::MoveRelAbs { .. } | Self::TurnRelAbs { .. } => "Relative (precomputed)",
        }
    }
    fn description(&self) -> &str {
        match self {
            Self::StartAt { .. } => "Sets the robots current position to the specified",
            Self::MoveRel { .. } => "Moves the robot forwards or backwards by the amount specified, where a negative number will make the robot go backwards. This is a relative command meaning it will generate the target point based on the current position given by odometry. Currently this will move the robot in a straight line.",
            Self::MoveRelAbs { .. } => "Moves the robot forwards or backwards by the amount specified, where a negative number will make the robot go backwards. This is a precomputed relative command (not to be confused with relative command) meaning it will generate the target point based on where the robot should be *in theory* meaning it does not need odometry to calculate the target point and instead can calculate it ahead of time. You probably don't want to use this after using MoveRel. Currently this will move the robot in a straight line.",
            Self::MoveTo { .. } => "Moves the robot to the specified target position. Currently this will move the robot in a straight line.",
            Self::TurnRel { .. } => "Turns the robot clockwise or counter-clockwise by the amount specified, where a negative number will make the robot turn clockwise. This is a relative command meaning it will generate the target heading based on the current heading given by odometry.",
            Self::TurnRelAbs { .. } => "Turns the robot clockwise or counter-clockwise by the amount specified, where a negative number will make the robot turn clockwise. This is a precomputed relative command (not to be confused with relative command) meaning it will generate the target point based on where the robot should be *in theory* meaning it does not need odometry to calculate the target point and instead can calculate it ahead of time. You probably don't want to use this after using TurnRel.",
            Self::TurnTo { .. } => "Turns the robot to the specified target heading.",
        }
    }
    fn modify_position(&self, pos: &mut Vec2, heading: &mut f64) {
        match self {
            Self::StartAt {
                pos: start_pos,
                heading: start_heading,
            } => {
                *pos = Vec2(*start_pos);
                *heading = *start_heading;
            }
            Self::MoveRel { rel } | Self::MoveRelAbs { rel } => {
                *pos.mut_x() += heading.cos() * rel;
                *pos.mut_y() += heading.sin() * rel;
            }
            Self::MoveTo { pos: new_pos } => {
                let del_x = new_pos[0] - pos.x();
                let del_y = new_pos[1] - pos.y();
                *heading = del_y.atan2(del_x);
                *pos = Vec2(*new_pos);
            }
            Self::TurnRel { angle } | Self::TurnRelAbs { angle } => {
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
        let length_modifier = builder.length_unit.modifier();
        let angle_modifier = builder.angle_unit.modifier();
        match &builder.action {
            Action::StartAt { pos: _, heading: _ } => Ok(Action::StartAt {
                pos: [
                    builder.field_inputs[0].trim().parse::<f64>()? * length_modifier,
                    builder.field_inputs[1].trim().parse::<f64>()? * length_modifier,
                ],
                heading: builder.field_inputs[0].trim().parse::<f64>()? * angle_modifier,
            }),

            Action::MoveRel { rel: _ } => Ok(Action::MoveRel {
                rel: builder.field_inputs[0].trim().parse::<f64>()? * length_modifier,
            }),

            Action::MoveRelAbs { rel: _ } => Ok(Action::MoveRelAbs {
                rel: builder.field_inputs[0].trim().parse::<f64>()? * length_modifier,
            }),

            Action::MoveTo { pos: _ } => Ok(Action::MoveTo {
                pos: [
                    builder.field_inputs[0].trim().parse::<f64>()? * length_modifier,
                    builder.field_inputs[1].trim().parse::<f64>()? * length_modifier,
                ],
            }),

            Action::TurnRel { angle: _ } => Ok(Action::TurnRel {
                angle: builder.field_inputs[0].trim().parse::<f64>()? * angle_modifier,
            }),

            Action::TurnRelAbs { angle: _ } => Ok(Action::TurnRelAbs {
                angle: builder.field_inputs[0].trim().parse::<f64>()? * angle_modifier,
            }),

            Action::TurnTo { heading: _ } => Ok(Action::TurnTo {
                heading: builder.field_inputs[0].trim().parse::<f64>()? * angle_modifier,
            }),
        }
    }
}
