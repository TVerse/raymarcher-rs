use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::primitives::UnitVec3;
#[cfg(test)]
use float_cmp::{ApproxEq, F64Margin};

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> UnitVec3 {
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

    pub fn max_component(&self) -> f64 {
        self.x.max(self.y.max(self.z))
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    pub fn reflect(&self, n: &UnitVec3) -> Self {
        self - &n.0 * 2.0 * self.dot(&n.0)
    }
}

#[cfg(test)]
impl<'a> ApproxEq for &'a Vec3 {
    type Margin = F64Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin)
            && self.y.approx_eq(other.y, margin)
            && self.z.approx_eq(other.z, margin)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::black_box;
    use test::Bencher;

    use crate::test_constants::MARGIN;
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn arb_vec3()(x in -10.0..10.0, y in -10.0..10.0, z in -10.0..10.0) -> Vec3 {
            Vec3{
                x,
                y,
                z,
            }
        }
    }

    proptest! {
        #[test]
        fn dot_product_commutative(a in arb_vec3(), b in arb_vec3()) {
            assert!(a.dot(&b).approx_eq(b.dot(&a), MARGIN))
        }

        #[test]
        fn dot_product_distributive(a in arb_vec3(), b in arb_vec3(), c in arb_vec3()) {
            assert!(a.dot(&(&b + &c)).approx_eq(a.dot(&b) + a.dot(&c), MARGIN))
        }

        #[test]
        fn dot_product_bilinear(a in arb_vec3(), b in arb_vec3(), c in arb_vec3(), r in -10.0..10.0) {
            assert!(a.dot(&((&b * r) + &c)).approx_eq(r * a.dot(&b) + a.dot(&c), MARGIN))
        }
    }

    proptest! {
        #[test]
        fn scalar_triple_product(a in arb_vec3(), b in arb_vec3(), c in arb_vec3()) {
            let first = a.dot(&b.cross(&c));
            let second = b.dot(&c.cross(&a));
            let third = c.dot(&a.cross(&b));
            assert!(first.approx_eq(second,  MARGIN));
            assert!(second.approx_eq(third,  MARGIN));
            assert!(third.approx_eq(first,  MARGIN));
        }

        #[test]
        fn cross_product_self(a in arb_vec3()) {
            assert!(a.cross(&a).approx_eq(&Vec3::new(0.0, 0.0, 0.0), MARGIN))
        }

        #[test]
        fn cross_anticommute(a in arb_vec3(), b in arb_vec3()) {
            assert!(a.cross(&b).approx_eq(&-b.cross(&a), MARGIN))
        }

        #[test]
        fn cross_jacobi(a in arb_vec3(), b in arb_vec3(), c in arb_vec3()) {
            assert!((a.cross(&b.cross(&c)) + b.cross(&c.cross(&a)) + c.cross(&a.cross(&b))).approx_eq(&Vec3::ZERO, MARGIN))
        }
    }

    #[test]
    fn dot_product_distributive_failures() {
        let a = Vec3 {
            x: 0.0,
            y: -8.921582397006445,
            z: 7.573710202022724,
        };
        let b = Vec3 {
            x: 0.0,
            y: -6.876151810718301,
            z: -1.414153466732282,
        };
        let c = Vec3 {
            x: 0.0,
            y: 3.484237948533316,
            z: -2.88375132389473,
        };
        assert!(a.dot(&(&b + &c)).approx_eq(a.dot(&b) + a.dot(&c), MARGIN));
        let a = Vec3 {
            x: 8.645810033587026,
            y: -8.633742886820283,
            z: 6.564002568600472,
        };
        let b = Vec3 {
            x: 4.388393866612459,
            y: -0.1128526180901801,
            z: 6.320195101647585,
        };
        let c = Vec3 {
            x: -7.565850100949318,
            y: 8.369558824873508,
            z: 8.620789029486183,
        };
        assert!(a.dot(&(&b + &c)).approx_eq(a.dot(&b) + a.dot(&c), MARGIN));
    }

    #[bench]
    fn length(b: &mut Bencher) {
        b.iter(|| {
            let v: Vec3 = Vec3::new(1.0, 2.0, -3.0);
            black_box(v).length()
        });
    }

    #[bench]
    fn unit(b: &mut Bencher) {
        b.iter(|| {
            let v: Vec3 = Vec3::new(1.0, 2.0, -3.0);
            black_box(v).unit()
        });
    }

    #[bench]
    fn dot(b: &mut Bencher) {
        b.iter(|| {
            let v: Vec3 = Vec3::new(1.0, 2.0, -3.0);
            let w: Vec3 = Vec3::new(-10.0, 8.2, 4.6);
            black_box(v).dot(black_box(&w))
        });
    }

    #[bench]
    fn cross(b: &mut Bencher) {
        b.iter(|| {
            let v: Vec3 = Vec3::new(1.0, 2.0, -3.0);
            let w: Vec3 = Vec3::new(-10.0, 8.2, 4.6);
            black_box(v).cross(black_box(&w))
        });
    }
}
