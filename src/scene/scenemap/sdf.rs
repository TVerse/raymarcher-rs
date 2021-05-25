use num_traits::Float;

use crate::primitives::{Point3, UnitVec3};
use crate::scene::scenemap::material::MaterialIndex;
use crate::Vec3;

pub trait Sdf<F: Float> {
    fn value_at(&self, p: &Point3<F>) -> (F, Option<MaterialIndex>);

    fn estimate_normal(&self, p: Point3<F>) -> UnitVec3<F> {
        let normal_epsilon: F = F::from(1e-5).unwrap();
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

pub struct UnitSphere;

impl<F: Float> Sdf<F> for UnitSphere {
    fn value_at(&self, p: &Point3<F>) -> (F, Option<MaterialIndex>) {
        (p.as_ref().length() - F::one(), None)
    }
}

pub struct UnitCube;

impl<F: Float> Sdf<F> for UnitCube {
    fn value_at(&self, p: &Point3<F>) -> (F, Option<MaterialIndex>) {
        let d = p.as_ref().abs() - Vec3::new(F::one(), F::one(), F::one());
        let inside_distance = d.max_component().min(F::zero());
        let outside_distance = d.max(&Vec3::new(F::zero(), F::zero(), F::zero())).length();
        (inside_distance + outside_distance, None)
    }
}

pub struct Union<'a, A, B> {
    pub a: &'a A,
    pub b: &'a B,
}

impl<'a, F: Float, A: Sdf<F>, B: Sdf<F>> Sdf<F> for Union<'a, A, B> {
    fn value_at(&self, p: &Point3<F>) -> (F, Option<MaterialIndex>) {
        let da = self.a.value_at(p);
        let db = self.b.value_at(p);
        if da.0 < db.0 {
            da
        } else {
            db
        }
    }
}

pub struct Translate<'a, F, A> {
    pub a: &'a A,
    pub v: &'a Vec3<F>,
}

impl<'a, F: Float, A: Sdf<F>> Sdf<F> for Translate<'a, F, A> {
    fn value_at(&self, p: &Point3<F>) -> (F, Option<MaterialIndex>) {
        self.a.value_at(&(p.as_ref() - self.v).into())
    }
}

pub struct Intersect<'a, A, B> {
    pub a: &'a A,
    pub b: &'a B,
}

impl<'a, F: Float, A: Sdf<F>, B: Sdf<F>> Sdf<F> for Intersect<'a, A, B> {
    fn value_at(&self, p: &Point3<F>) -> (F, Option<MaterialIndex>) {
        let da = self.a.value_at(p);
        let db = self.b.value_at(p);
        if da.0 > db.0 {
            da
        } else {
            db
        }
    }
}

pub struct Subtract<'a, A, B> {
    pub a: &'a A,
    pub b: &'a B,
}

impl<'a, F: Float, A: Sdf<F>, B: Sdf<F>> Sdf<F> for Subtract<'a, A, B> {
    fn value_at(&self, p: &Point3<F>) -> (F, Option<MaterialIndex>) {
        let da = self.a.value_at(p);
        let db = self.b.value_at(p);
        if da.0 > -db.0 {
            da
        } else {
            db
        }
    }
}

pub struct ScaleUniform<'a, A, F> {
    pub a: &'a A,
    pub f: F,
}

impl<'a, F: Float, A: Sdf<F>> Sdf<F> for ScaleUniform<'a, A, F> {
    fn value_at(&self, p: &Point3<F>) -> (F, Option<MaterialIndex>) {
        let (f, m) = self.a.value_at(&Point3(p.as_ref() / self.f));
        (f * self.f, m)
    }
}

pub struct WithMaterial<'a, A> {
    pub a: &'a A,
    pub m: Option<MaterialIndex>
}

impl<'a, F: Float, A: Sdf<F>> Sdf<F> for WithMaterial<'a, A> {
    fn value_at(&self, p: &Point3<F>) -> (F, Option<MaterialIndex>) {
        (self.a.value_at(p).0, self.m)
    }
}

pub struct Arbitrary<S> {
    pub s: S,
}

impl<S> Arbitrary<S> {
    pub fn new<'a, F>(s: S) -> Self
    where
        F: Float,
        S: (Fn(&Point3<F>) -> (F, Option<MaterialIndex>)) + 'a,
    {
        Self { s }
    }
}

impl<'a, F, S> Sdf<F> for Arbitrary<S>
where
    F: Float,
    S: (Fn(&Point3<F>) -> (F, Option<MaterialIndex>)) + 'a,
{
    fn value_at(&self, p: &Point3<F>) -> (F, Option<MaterialIndex>) {
        (self.s)(p)
    }
}

pub mod halfplane {
    use num_traits::Float;

    use crate::scene::scenemap::material::MaterialIndex;
    use crate::scene::scenemap::sdf::Sdf;
    use crate::Point3;

    pub struct NegY;

    impl<F: Float> Sdf<F> for NegY {
        fn value_at(&self, p: &Point3<F>) -> (F, Option<MaterialIndex>) {
            (p.0.y, None)
        }
    }
}
