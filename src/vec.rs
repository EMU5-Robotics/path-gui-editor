use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Vec2(pub [f64; 2]);

impl Vec2 {
    pub fn dot(&self, other: &Self) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1]
    }

    pub fn mag_sq(&self) -> f64 {
        self.dot(self)
    }
    
    pub fn mag(&self) -> f64 {
        self.mag_sq().sqrt()
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

impl From<[f64; 2]> for Vec2 {
    fn from(v: [f64; 2]) -> Self {
        Self(v)
    }
}
