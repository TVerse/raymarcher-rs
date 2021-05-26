use crate::primitives::{Point3, Vec3};
use crate::Ray;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(look_at: Point3, look_from: Point3, up: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from.as_ref() - look_at.as_ref()).unit();
        let u = up.cross(w.as_ref()).unit();
        let v = w.as_ref().cross(u.as_ref()).unit();

        let horizontal = u.as_ref() * viewport_width;
        let vertical = v.as_ref() * viewport_height;

        let lower_left_corner =
            look_from.as_ref() - &horizontal / 2.0 - &vertical / 2.0 - w.as_ref();
        let lower_left_corner = lower_left_corner.into();

        Self {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let origin = (&self.origin).clone();
        let direction = self.lower_left_corner.as_ref() + &self.horizontal * u + &self.vertical * v
            - self.origin.as_ref();
        Ray::new_unnormalized(origin, direction)
    }
}
