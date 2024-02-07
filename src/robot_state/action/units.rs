use std::{f64::consts::PI, fmt::Display};

pub trait Unit {
    fn modifier(&self) -> f64;
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum LengthUnit {
    #[default]
    Metre,
    Inch,
    Foot,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum AngleUnit {
    Degree,
    #[default]
    Radian,
}

impl Unit for LengthUnit {
    fn modifier(&self) -> f64 {
        match self {
            LengthUnit::Metre => 1.,
            LengthUnit::Inch => 0.0254,
            LengthUnit::Foot => 0.3048,
        }
    }
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LengthUnit::Metre => "Metres",
                LengthUnit::Inch => "Inches",
                LengthUnit::Foot => "Feet",
            }
        )
    }
}

impl Unit for AngleUnit {
    fn modifier(&self) -> f64 {
        match self {
            AngleUnit::Degree => PI / 180.,
            AngleUnit::Radian => 1.,
        }
    }
}

impl Display for AngleUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AngleUnit::Degree => "Degrees",
                AngleUnit::Radian => "Radians",
            }
        )
    }
}
