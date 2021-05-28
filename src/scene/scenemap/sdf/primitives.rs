use crate::scene::scenemap::material::MaterialIndex;
use crate::scene::scenemap::sdf::Sdf;
use crate::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Sphere {
    radius: f64,
    center: Point3,
}

impl Sphere {
    pub fn new(radius: f64, center: Point3) -> Self {
        Self { radius, center }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(1.0, Point3::ORIGIN)
    }
}

impl Sdf for Sphere {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        (
            (p - self.center.as_ref()).as_ref().length() - self.radius,
            None,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Cube {
    half_side_length: f64,
    center: Point3,
}

impl Cube {
    pub fn new(side_length: f64, center: Point3) -> Self {
        Self::new_half_side(side_length / 2.0, center)
    }

    pub fn new_half_side(half_side_length: f64, center: Point3) -> Self {
        Self {
            half_side_length,
            center,
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new(2.0, Point3::ORIGIN)
    }
}

impl Sdf for Cube {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        let d = (p - self.center.as_ref()).as_ref().abs()
            - Vec3::new(
                self.half_side_length,
                self.half_side_length,
                self.half_side_length,
            );
        let inside_distance = d.max_component().min(0.0);
        let outside_distance = d.max(&Vec3::new(0.0, 0.0, 0.0)).length();
        (inside_distance + outside_distance, None)
    }
}

#[derive(Debug, Clone)]
pub struct Arbitrary<S> {
    s: S,
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

pub struct NegY;

impl Sdf for NegY {
    fn value_at(&self, p: &Point3) -> (f64, Option<MaterialIndex>) {
        (p.0.y, None)
    }
}
