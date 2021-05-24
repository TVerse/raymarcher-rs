use std::ops::{Add, Mul};
use num_traits::Float;

mod vec;
pub use vec::Vec3;

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct UnitVec3<F>(pub Vec3<F>);

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct Point3<F>(pub Vec3<F>);

impl<F: Float> Point3<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct Color<F>(pub Vec3<F>);

impl<F: Float> Color<F> {
    pub fn white() -> Self {
        Self::new(F::one(), F::one(), F::one())
    }

    pub fn black() -> Self {
        Self(Vec3::zero())
    }

    pub fn purple() -> Self {
        Self::new(F::one(), F::zero(), F::one())
    }

    pub fn new(r: F, g: F, b: F) -> Self {
        Self(Vec3 { x: r, y: g, z: b })
    }

    pub fn r(&self) -> F {
        self.0.x
    }

    pub fn g(&self) -> F {
        self.0.y
    }

    pub fn b(&self) -> F {
        self.0.z
    }
}

impl<F: Float> Add<Color<F>> for Color<F> {
    type Output = Color<F>;

    fn add(self, rhs: Color<F>) -> Self::Output {
        Self::Output::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl<F: Float> Mul<F> for &Color<F> {
    type Output = Color<F>;

    fn mul(self, rhs: F) -> Self::Output {
        Self::Output::new(self.r() * rhs, self.g() * rhs, self.b() * rhs)
    }
}

impl<F: Float> Mul<F> for Color<F> {
    type Output = Color<F>;

    fn mul(self, rhs: F) -> Self::Output {
        &self * rhs
    }
}
