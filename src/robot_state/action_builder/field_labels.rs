#[derive(Default)]
pub enum FieldLabel {
    X,
    Y,
    Heading,
    Distance,
    #[default]
    None,
}

impl FieldLabel {
    pub fn label(&self) -> &'static str {
        match self {
            FieldLabel::X => "Select x:",
            FieldLabel::Y => "Select y:",
            FieldLabel::Heading => "Select Heading (rad):",
            FieldLabel::Distance => "Select distance:",
            FieldLabel::None => "",
        }
    }

    pub fn enabled(&self) -> bool {
        !matches!(self, FieldLabel::None)
    }
}
