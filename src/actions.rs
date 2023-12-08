use std::f64::consts::PI;

pub enum Action {
    StartAt { pos: [f64; 2], heading: f64 },
    MoveRel { rel: f64 },
    MoveRelAbs { rel: f64 },
    MoveTo { pos: [f64; 2] },
}

impl Action {
    pub fn action_name(&self) -> &str {
        match self {
            Self::StartAt { .. } => "Start At",
            Self::MoveRel { .. } | Self::MoveRelAbs { .. } => "Move",
            Self::MoveTo { .. } => "Move To",
        }
    }
    pub fn action_value(&self) -> String {
        match self {
            Self::StartAt { pos, heading } => {
                format!("({}m, {}m) @ {} deg", pos[0], pos[1], heading * 180.0 / PI)
            }
            Self::MoveRel { rel } | Self::MoveRelAbs { rel } => {
                format!("{rel}m")
            }
            Self::MoveTo { pos } => {
                format!("({}m, {}m)", pos[0], pos[1])
            }
        }
    }
    pub fn action_modifiers(&self) -> &str {
        match self {
            Self::StartAt { .. } | Self::MoveTo { .. } => "Absolute",
            Self::MoveRel { .. } => "Relative",
            Self::MoveRelAbs { .. } => "Relative (abs)",
        }
    }
}
