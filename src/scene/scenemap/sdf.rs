use crate::primitives::{Point3, UnitVec3};
use crate::scene::scenemap::material::MaterialIndex;
use crate::Vec3;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

pub trait Sdf {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>);

    fn estimate_normal(&self, p: &Point3) -> UnitVec3 {
        let normal_epsilon: f64 = 1e-5;
        Vec3 {
            x: self
                .value_at(&Point3(Vec3 {
                    x: p.0.x + normal_epsilon,
                    ..p.0
                }))
                .0
                - self
                    .value_at(&Point3(Vec3 {
                        x: p.0.x - normal_epsilon,
                        ..p.0
                    }))
                    .0,
            y: self
                .value_at(&Point3(Vec3 {
                    y: p.0.y + normal_epsilon,
                    ..p.0
                }))
                .0
                - self
                    .value_at(&Point3(Vec3 {
                        y: p.0.y - normal_epsilon,
                        ..p.0
                    }))
                    .0,
            z: self
                .value_at(&Point3(Vec3 {
                    z: p.0.z + normal_epsilon,
                    ..p.0
                }))
                .0
                - self
                    .value_at(&Point3(Vec3 {
                        z: p.0.z - normal_epsilon,
                        ..p.0
                    }))
                    .0,
        }
        .unit()
    }
}

// pub fn value_at(&self, p: &Point3) -> f64 {
//     match self {
//         Sdf::Primitive(prim) => prim.value_at(p),
//         Sdf::Union(a, b) => a.value_at(p).min(b.value_at(p)),
//         Sdf::Intersect(a, b) => a.value_at(p).max(b.value_at(p)),
//         Sdf::Subtract(a, b) => a.value_at(p).max(-b.value_at(p)),
//         Sdf::Translate(a, v) => a.value_at(&(p.as_ref() - v).into()),
//     }
// }

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

#[derive(Debug, Copy, Clone)]
pub struct UnitSphere;

impl Sdf for UnitSphere {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        (p.as_ref().length() - 1.0, None)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UnitCube;

impl Sdf for UnitCube {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        let d = p.as_ref().abs() - Vec3::new(1.0, 1.0, 1.0);
        let inside_distance = d.max_component().min(0.0);
        let outside_distance = d.max(&Vec3::new(0.0, 0.0, 0.0)).length();
        (inside_distance + outside_distance, None)
    }
}

#[derive(Debug, Clone)]
pub struct Union<A, B> {
    pub a: A,
    pub b: B,
}

impl<A: Sdf, B: Sdf> Sdf for Union<A, B> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        let da = self.a.value_at(p);
        let db = self.b.value_at(p);
        if da.0 < db.0 {
            da
        } else {
            db
        }
    }
}

#[derive(Debug, Clone)]
pub struct Translate<A> {
    pub a: A,
    pub v: Vec3,
}

impl<A: Sdf> Sdf for Translate<A> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        self.a.value_at(&(p.as_ref() - &self.v).into())
    }
}

#[derive(Debug, Clone)]
pub struct Intersect<A, B> {
    pub a: A,
    pub b: B,
}

impl<A: Sdf, B: Sdf> Sdf for Intersect<A, B> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        let da = self.a.value_at(p);
        let db = self.b.value_at(p);
        if da.0 > db.0 {
            da
        } else {
            db
        }
    }
}

#[derive(Debug, Clone)]
pub struct Subtract<A, B> {
    pub a: A,
    pub b: B,
}

impl<A: Sdf, B: Sdf> Sdf for Subtract<A, B> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        let da = self.a.value_at(p);
        let db = self.b.value_at(p);
        if da.0 > -db.0 {
            da
        } else {
            db
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScaleUniform<A> {
    pub a: A,
    pub f: f64,
}

impl<A: Sdf> Sdf for ScaleUniform<A> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        let (f, m) = self.a.value_at(&Point3(p.as_ref() / self.f));
        (f * self.f, m)
    }
}

#[derive(Debug, Clone)]
pub struct WithMaterial<A> {
    pub a: A,
    pub m: MaterialIndex,
}

impl<A: Sdf> Sdf for WithMaterial<A> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        (self.a.value_at(p).0, Some(self.m))
    }
}

#[derive(Debug, Clone)]
pub struct Arbitrary<S> {
    pub s: S,
}

impl<S> Arbitrary<S> {
    pub fn new<'a>(s: S) -> Self
    where
        S: (Fn(&Point3) -> (f64, Option<MaterialIndex>)) + 'a,
    {
        Self { s }
    }
}

impl<'a, S> Sdf for Arbitrary<S>
where
    S: (Fn(&Point3) -> (f64, Option<MaterialIndex>)) + 'a,
{
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        (self.s)(p)
    }
}

pub mod halfplane {
    use crate::scene::scenemap::material::MaterialIndex;
    use crate::scene::scenemap::sdf::Sdf;
    use crate::Point3;

    pub struct NegY;

    impl Sdf for NegY {
        fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
            (p.0.y, None)
        }
    }
}
