use crate::primitives::{Quaternion, UnitVec3};
use crate::scene::scenemap::material::MaterialIndex;
use crate::scene::scenemap::sdf::Sdf;
use crate::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Translate<A> {
    a: A,
    v: Vec3,
}

impl<A> Translate<A> {
    pub fn new(a: A, v: Vec3) -> Self {
        Self { a, v }
    }
}

impl<A: Sdf> Sdf for Translate<A> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        self.a.value_at(&(p.as_ref() - &self.v).into())
    }
}

#[derive(Debug, Clone)]
pub struct ScaleUniform<A> {
    a: A,
    f: f64,
}

impl<A> ScaleUniform<A> {
    pub fn new(a: A, f: f64) -> Self {
        Self { a, f }
    }
}

impl<A: Sdf> Sdf for ScaleUniform<A> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        let (f, m) = self.a.value_at(&Point3(p.as_ref() / self.f));
        (f * self.f, m)
    }
}

#[derive(Debug, Clone)]
pub struct Rotate<A> {
    a: A,
    q: Quaternion,
}

impl<A> Rotate<A> {
    pub fn new(a: A, angle: f64, axis: UnitVec3) -> Self {
        Self {
            a,
            q: Quaternion::for_rotation(-angle, axis), // Need to invert the rotation!
        }
    }

    pub fn new_degrees(a: A, angle: f64, axis: UnitVec3) -> Self {
        Self::new(a, angle.to_radians(), axis)
    }
}

impl<A: Sdf> Sdf for Rotate<A> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        let v = p.as_ref();
        let q = &self.q;
        let q_inv = q.conjugate();
        let qv = Quaternion::new(0.0, v.clone());
        let p_new = Point3((q * qv * q_inv).vec().clone());
        self.a.value_at(&p_new)
    }
}
