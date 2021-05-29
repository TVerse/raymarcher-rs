use std::ops::{Add, Mul, Sub};

mod quaternion;
mod vec;
#[cfg(test)]
use float_cmp::{ApproxEq, F64Margin};
pub use quaternion::Quaternion;
pub use vec::Vec3;

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct UnitVec3(pub Vec3);

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct Point3(pub Vec3);

impl Point3 {
    pub const ORIGIN: Self = Self(Vec3::ZERO);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl Sub<&Vec3> for &Point3 {
    type Output = Point3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Point3(&self.0 - rhs)
    }
}

#[cfg(test)]
impl<'a> ApproxEq for &'a Point3 {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        self.0.approx_eq(&other.0, margin)
    }
}

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct Color(pub Vec3);

impl Color {
    pub const WHITE: Color = Self::new(1.0, 1.0, 1.0);

    pub const BLACK: Color = Self(Vec3::ZERO);

    pub const PURPLE: Color = Self::new(1.0, 0.0, 1.0);

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
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

impl Mul<Color> for &Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.g() * rhs.g())
    }
}

impl Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.g() * rhs.g())
    }
}
