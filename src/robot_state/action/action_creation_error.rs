use std::num::ParseFloatError;

pub enum ActionCreationError {
    NoStart,
    ExtraStart,
    InvalidField,
    OutOfBounds,
}

impl From<ParseFloatError> for ActionCreationError {
    fn from(_: ParseFloatError) -> Self {
        ActionCreationError::InvalidField
    }
}

impl ActionCreationError {
    pub fn message(&self) -> &'static str {
        match self {
            ActionCreationError::NoStart => "Start First!",
            ActionCreationError::ExtraStart => "Can Only Start Once!",
            ActionCreationError::InvalidField => "Error in Fields!",
            ActionCreationError::OutOfBounds => "Out of Bounds!",
        }
    }
}
