#![feature(test)]

mod primitives;
mod raymarcher;
pub mod scene;

use num_traits::Float;
pub use primitives::{Color, Point3, Ray, Vec3};
pub use raymarcher::render;

mod constants {
    use num_traits::Float;

    pub fn two<F: Float>() -> F {
        F::one() + F::one()
    }

    pub fn half<F: Float>() -> F {
        F::one() / two()
    }
}

pub struct Config<F> {
    pub image_settings: ImageSettings,
    pub render_settings: RenderSettings<F>,
}

pub struct ImageSettings {
    pub width: usize,
    pub height: usize,
}

pub struct RenderSettings<F> {
    pub max_marching_steps: usize,
    pub max_light_recursions: usize,
    pub t_min: F,
    pub t_max: F,
    pub epsilon: F,
    pub material_override: Option<MaterialOverride>,
}

pub enum MaterialOverride {
    Normal,
}

pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBColor {
    fn convert_to_rgb_byte<F: Float>(f: F) -> u8 {
        let res: F = f * F::from(255.999).unwrap();
        if res >= F::from(256.0).unwrap() {
            255
        } else if res < F::zero() {
            0
        } else {
            res.to_u8().unwrap()
        }
    }
}

impl<F: Float> From<Color<F>> for RGBColor {
    fn from(c: Color<F>) -> Self {
        Self {
            r: RGBColor::convert_to_rgb_byte(c.r()),
            g: RGBColor::convert_to_rgb_byte(c.g()),
            b: RGBColor::convert_to_rgb_byte(c.b()),
        }
    }
}
