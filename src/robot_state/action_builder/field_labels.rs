#[derive(Default)]
pub enum FieldLabel {
    X,
    Y,
    Heading,
    Distance,
    Angle,
    #[default]
    None,
}

impl FieldLabel {
    pub fn label(&self) -> &'static str {
        match self {
            FieldLabel::X => "Select x:",
            FieldLabel::Y => "Select y:",
            FieldLabel::Heading => "Select heading (deg):",
            FieldLabel::Distance => "Select distance:",
            FieldLabel::Angle => "Select angle (deg):",
            FieldLabel::None => "",
        }
    }

    pub fn enabled(&self) -> bool {
        !matches!(self, FieldLabel::None)
    }
}
