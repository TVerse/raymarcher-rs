//! Contains structs that can combine two SDFs into one.
//!
//! Implementations are encouraged to propagate any [MaterialIndex] whenever possible,
//! but are not required to do so.

use crate::Point3;
use crate::scene::scenemap::material::MaterialIndex;
use crate::scene::scenemap::sdf::Sdf;

#[derive(Debug, Clone)]
pub struct Union<A, B> {
    a: A,
    b: B,
}

impl<A, B> Union<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
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
pub struct Intersect<A, B> {
    a: A,
    b: B,
}

impl<A, B> Intersect<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
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
pub struct Difference<A, B> {
    a: A,
    b: B,
}

impl<A, B> Difference<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: Sdf, B: Sdf> Sdf for Difference<A, B> {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        let da = self.a.value_at(p);
        let db = self.b.value_at(p);
        if da.0 > -db.0 {
            da
        } else {
            (-db.0, db.1)
        }
    }
}
