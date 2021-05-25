#![feature(test)]

mod primitives;
mod raymarcher;
pub mod scene;

use crate::raymarcher::FindTargetSettings;
use num_traits::Float;
pub use primitives::{Color, Point3, Vec3};
pub use raymarcher::render;
pub use raymarcher::Ray;

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
    image_settings: ImageSettings,
    render_settings: RenderSettings<F>,
}

impl<F> Config<F> {
    pub fn new(image_settings: ImageSettings, render_settings: RenderSettings<F>) -> Self {
        Self {
            image_settings,
            render_settings,
        }
    }
}

pub struct ImageSettings {
    width: usize,
    height: usize,
}

impl ImageSettings {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

pub struct RenderSettings<F> {
    find_target_settings: FindTargetSettings<F>,
    material_override: Option<MaterialOverride>,
}

impl<F> RenderSettings<F> {
    pub fn new(
        t_min: F,
        t_max: F,
        epsilon: F,
        max_marching_steps: usize,
        material_override: Option<MaterialOverride>,
    ) -> Self {
        Self {
            find_target_settings: FindTargetSettings::new(
                t_min,
                t_max,
                epsilon,
                max_marching_steps,
            ),
            material_override,
        }
    }
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
