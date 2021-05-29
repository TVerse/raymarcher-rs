use crate::primitives::UnitVec3;
use crate::Vec3;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Quaternion {
    a: f64,
    v: Vec3,
}

impl Quaternion {
    pub fn for_rotation(angle: f64, axis: UnitVec3) -> Self {
        Self {
            a: (angle / 2.0).cos(),
            v: axis.as_ref() * (angle / 2.0).sin(),
        }
    }

    pub fn new(a: f64, v: Vec3) -> Self {
        Self { a, v }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            a: self.a,
            v: -(&self.v),
        }
    }

    pub fn vec(&self) -> &Vec3 {
        &self.v
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a - self.v.dot(&rhs.v),
            v: &rhs.v * self.a + &self.v * rhs.a + self.v.cross(&rhs.v),
        }
    }
}

impl Mul<Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a - self.v.dot(&rhs.v),
            v: &rhs.v * self.a + &self.v * rhs.a + self.v.cross(&rhs.v),
        }
    }
}

impl Mul<&Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &Quaternion) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a - self.v.dot(&rhs.v),
            v: &rhs.v * self.a + &self.v * rhs.a + &self.v.cross(&rhs.v),
        }
    }
}

impl Mul<&Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &Quaternion) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a - self.v.dot(&rhs.v),
            v: &rhs.v * self.a + &self.v * rhs.a + self.v.cross(&rhs.v),
        }
    }
}
