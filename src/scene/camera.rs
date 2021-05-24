use crate::primitives::{Point3, Vec3};
use crate::{constants, Ray};
use num_traits::Float;

pub struct Camera<F> {
    pub origin: Point3<F>,
    pub lower_left_corner: Point3<F>,
    pub horizontal: Vec3<F>,
    pub vertical: Vec3<F>,
}

impl<F: Float> Camera<F> {
    pub fn new(
        look_at: Point3<F>,
        look_from: Point3<F>,
        up: Vec3<F>,
        vfov: F,
        aspect_ratio: F,
    ) -> Self {
        let theta = vfov.to_radians();
        let two = constants::two();
        let h = (theta / two).tan();
        let viewport_height = two * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from.as_ref() - look_at.as_ref()).unit();
        let u = up.cross(w.as_ref()).unit();
        let v = w.as_ref().cross(u.as_ref()).unit();

        let horizontal = u.as_ref() * viewport_width;
        let vertical = v.as_ref() * viewport_height;

        let lower_left_corner =
            look_from.as_ref() - &horizontal / two - &vertical / two - w.as_ref();
        let lower_left_corner = lower_left_corner.into();

        Self {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: F, v: F) -> Ray<F> {
        let origin = (&self.origin).clone();
        let direction = self.lower_left_corner.as_ref() + &self.horizontal * u + &self.vertical * v
            - self.origin.as_ref();
        Ray { origin, direction }
    }
}
