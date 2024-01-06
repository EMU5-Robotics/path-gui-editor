use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Vec2(pub [f64; 2]);

impl Vec2 {
    pub const NONE: Self = Vec2([f64::NAN; 2]);

    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn mut_x(&mut self) -> &mut f64 {
        &mut self.0[0]
    }
    pub fn mut_y(&mut self) -> &mut f64 {
        &mut self.0[1]
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1]
    }
    pub fn mag_sq(&self) -> f64 {
        self.dot(self)
    }
    pub fn mag(&self) -> f64 {
        self.mag_sq().sqrt()
    }
    pub fn normalised(self) -> Self {
        self / self.mag()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let Self(a) = self;
        let Self(b) = other;
        Self([a[0] + b[0], a[1] + b[1]])
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let Self(a) = self;
        let Self(b) = other;
        Self([a[0] - b[0], a[1] - b[1]])
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, b: f64) -> Self {
        Self([self.0[0] / b, self.0[1] / b])
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, b: f64) -> Self {
        Self([self.0[0] * b, self.0[1] * b])
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self([-self.0[0], -self.0[1]])
    }
}

impl From<[f64; 2]> for Vec2 {
    fn from(v: [f64; 2]) -> Self {
        Self(v)
    }
}

impl Into<[f64; 2]> for Vec2 {
    fn into(self) -> [f64; 2] {
        self.0
    }
}
