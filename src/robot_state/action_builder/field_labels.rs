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
            FieldLabel::X => "X:",
            FieldLabel::Y => "Y:",
            FieldLabel::Heading => "Heading:",
            FieldLabel::Distance => "Distance:",
            FieldLabel::Angle => "Angle:",
            FieldLabel::None => "",
        }
    }

    pub fn enabled(&self) -> bool {
        !matches!(self, FieldLabel::None)
    }
}
