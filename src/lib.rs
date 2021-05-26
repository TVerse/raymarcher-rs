#![feature(test)]

mod primitives;
mod raymarcher;
pub mod scene;

use crate::raymarcher::FindTargetSettings;
use crate::scene::scenemap::material::MaterialIndex;
pub use primitives::{Color, Point3, Vec3};
pub use raymarcher::render;
pub use raymarcher::Ray;

pub struct Config {
    image_settings: ImageSettings,
    render_settings: RenderSettings,
}

impl Config {
    pub fn new(image_settings: ImageSettings, render_settings: RenderSettings) -> Self {
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

pub struct RenderSettings {
    find_target_settings: FindTargetSettings,
    material_override: Option<MaterialIndex>,
}

impl RenderSettings {
    pub fn new(
        t_min: f64,
        t_max: f64,
        epsilon: f64,
        material_override: Option<MaterialIndex>,
    ) -> Self {
        Self {
            find_target_settings: FindTargetSettings::new(t_min, t_max, epsilon),
            material_override,
        }
    }
}

pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBColor {
    fn convert_to_rgb_byte(f: f64) -> u8 {
        let res: f64 = f * 255.999;
        if res > 255.0 {
            255
        } else if res < 0.0 {
            0
        } else {
            // TODO NaN
            res as u8
        }
    }
}

impl From<Color> for RGBColor {
    fn from(c: Color) -> Self {
        Self {
            r: RGBColor::convert_to_rgb_byte(c.r()),
            g: RGBColor::convert_to_rgb_byte(c.g()),
            b: RGBColor::convert_to_rgb_byte(c.b()),
        }
    }
}

#[cfg(test)]
pub mod test_constants {
    use float_cmp::F64Margin;

    // Used in macro
    #[allow(dead_code)]
    pub const MARGIN: F64Margin = F64Margin {
        ulps: 0, // TODO improve numerical stability, maybe
        epsilon: 1e-5,
    };
}
