use std::ops::{Add, Mul};

mod vec;
pub use vec::Vec3;

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct UnitVec3(pub Vec3);

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct Point3(pub Vec3);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct Color(pub Vec3);

impl Color {
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self(Vec3::zero())
    }

    pub fn purple() -> Self {
        Self::new(1.0, 0.0, 1.0)
    }

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3 { x: r, y: g, z: b })
    }

    pub fn r(&self) -> f64 {
        self.0.x
    }

    pub fn g(&self) -> f64 {
        self.0.y
    }

    pub fn b(&self) -> f64 {
        self.0.z
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Self::Output::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.r() * rhs, self.g() * rhs, self.b() * rhs)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}
