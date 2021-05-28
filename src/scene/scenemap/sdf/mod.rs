use crate::primitives::{Point3, UnitVec3};
use crate::scene::scenemap::material::MaterialIndex;
use crate::Vec3;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

pub mod combinators;
pub mod positioners;
pub mod primitives;

pub trait Sdf {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>);

    /// Does a 6-point numerical gradient by default.
    /// Implementors can assume this is only called when value_at(p) is close to zero.
    fn estimate_normal(&self, p: &Point3) -> UnitVec3 {
        let normal_epsilon: f64 = 1e-5;
        let dx =
            self.value_at(&Point3(Vec3 {
                x: p.0.x + normal_epsilon,
                ..p.0
            }))
            .0 - self
                .value_at(&Point3(Vec3 {
                    x: p.0.x - normal_epsilon,
                    ..p.0
                }))
                .0;
        let dy =
            self.value_at(&Point3(Vec3 {
                y: p.0.y + normal_epsilon,
                ..p.0
            }))
            .0 - self
                .value_at(&Point3(Vec3 {
                    y: p.0.y - normal_epsilon,
                    ..p.0
                }))
                .0;
        let dz =
            self.value_at(&Point3(Vec3 {
                z: p.0.z + normal_epsilon,
                ..p.0
            }))
            .0 - self
                .value_at(&Point3(Vec3 {
                    z: p.0.z - normal_epsilon,
                    ..p.0
                }))
                .0;
        Vec3::new(dx, dy, dz).unit()
    }
}

impl<'a, A: Sdf> Sdf for &'a A {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        (*self).value_at(p)
    }
}

impl<A: Sdf + ?Sized> Sdf for Box<A> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        (self.deref()).value_at(p)
    }
}

impl<A: Sdf + ?Sized> Sdf for Rc<A> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        (self.deref()).value_at(p)
    }
}

impl<A: Sdf + ?Sized> Sdf for Arc<A> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        (self.deref()).value_at(p)
    }
}

#[derive(Debug, Clone)]
pub struct WithMaterial<A> {
    a: A,
    m: MaterialIndex,
}

impl<A> WithMaterial<A> {
    pub fn new(a: A, m: MaterialIndex) -> Self {
        Self { a, m }
    }
}

impl<A: Sdf> Sdf for WithMaterial<A> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        (self.a.value_at(p).0, Some(self.m))
    }
}
