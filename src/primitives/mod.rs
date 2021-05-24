#[cfg(test)]
use float_cmp::ApproxEq;

use num_traits::Float;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Vec3<F> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Vec3<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(&self) -> F {
        self.dot(self)
    }

    pub fn length(&self) -> F {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> UnitVec3<F> {
        let length = self.length();
        UnitVec3(self / length)
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn max_component(&self) -> F {
        self.x.max(self.y.max(self.z))
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

#[cfg(test)]
impl<'a, M: Copy + Default, F: Copy + ApproxEq<Margin = M>> ApproxEq for &'a Vec3<F> {
    type Margin = M;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin)
            && self.y.approx_eq(other.y, margin)
            && self.z.approx_eq(other.z, margin)
    }
}

impl<F: Float> Add<Vec3<F>> for Vec3<F> {
    type Output = Vec3<F>;

    fn add(self, rhs: Vec3<F>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<F: Float> Add<Vec3<F>> for &Vec3<F> {
    type Output = Vec3<F>;

    fn add(self, rhs: Vec3<F>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<F: Float> Sub<Vec3<F>> for Vec3<F> {
    type Output = Vec3<F>;

    fn sub(self, rhs: Vec3<F>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<F: Float> Sub<&Vec3<F>> for Vec3<F> {
    type Output = Vec3<F>;

    fn sub(self, rhs: &Vec3<F>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<F: Float> Sub<Vec3<F>> for &Vec3<F> {
    type Output = Vec3<F>;

    fn sub(self, rhs: Vec3<F>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<F: Float> Sub<&Vec3<F>> for &Vec3<F> {
    type Output = Vec3<F>;

    fn sub(self, rhs: &Vec3<F>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<F: Float> Mul<F> for &Vec3<F> {
    type Output = Vec3<F>;

    fn mul(self, rhs: F) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<F: Float> Div<F> for Vec3<F> {
    type Output = Vec3<F>;

    fn div(self, rhs: F) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<F: Float> Div<F> for &Vec3<F> {
    type Output = Vec3<F>;

    fn div(self, rhs: F) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct UnitVec3<F>(pub Vec3<F>);

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct Point3<F>(pub Vec3<F>);

impl<F: Float> Point3<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

pub struct Ray<F> {
    pub origin: Point3<F>,
    pub direction: Vec3<F>,
}

#[derive(Debug, Clone, derive_more::From, derive_more::Into, derive_more::AsRef)]
pub struct Color<F>(pub Vec3<F>);

impl<F: Float> Color<F> {
    pub fn white() -> Self {
        Self::new(F::one(), F::one(), F::one())
    }

    pub fn black() -> Self {
        Self::new(F::zero(), F::zero(), F::zero())
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

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use float_cmp::F64Margin;
    use quickcheck::{Arbitrary, Gen};
    use test::black_box;
    use test::Bencher;

    impl<F: Arbitrary> Arbitrary for Vec3<F> {
        fn arbitrary(g: &mut Gen) -> Self {
            Self {
                x: F::arbitrary(g),
                y: F::arbitrary(g),
                z: F::arbitrary(g),
            }
        }
    }

    #[quickcheck]
    fn dot_product_commutative(va: Vec3<f64>, vb: Vec3<f64>) -> bool {
        va.dot(&vb).approx_eq(
            vb.dot(&va),
            F64Margin {
                ulps: 2,
                epsilon: 0.0,
            },
        )
    }

    #[bench]
    fn length(b: &mut Bencher) {
        b.iter(|| {
            let v: Vec3<f64> = Vec3::new(1.0, 2.0, -3.0);
            black_box(v).length()
        });
    }

    #[bench]
    fn unit(b: &mut Bencher) {
        b.iter(|| {
            let v: Vec3<f64> = Vec3::new(1.0, 2.0, -3.0);
            black_box(v).unit()
        });
    }

    #[bench]
    fn dot(b: &mut Bencher) {
        b.iter(|| {
            let v: Vec3<f64> = Vec3::new(1.0, 2.0, -3.0);
            let w: Vec3<f64> = Vec3::new(-10.0, 8.2, 4.6);
            black_box(v).dot(black_box(&w))
        });
    }

    #[bench]
    fn cross(b: &mut Bencher) {
        b.iter(|| {
            let v: Vec3<f64> = Vec3::new(1.0, 2.0, -3.0);
            let w: Vec3<f64> = Vec3::new(-10.0, 8.2, 4.6);
            black_box(v).cross(black_box(&w))
        });
    }
}
