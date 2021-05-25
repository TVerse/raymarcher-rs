pub mod camera;
pub mod scenemap;

use crate::primitives::Color;
use crate::scene::camera::Camera;
use crate::scene::scenemap::SceneMap;
use crate::Ray;

pub struct Scene<'a> {
    pub camera: Camera,
    pub scene_map: SceneMap<'a>,
    pub background: Box<dyn Background>,
}

pub trait Background {
    fn value_at(&self, r: &Ray) -> Color;
}

pub struct VerticalGradientBackground {
    pub from: Color,
    pub to: Color,
}

impl Background for VerticalGradientBackground {
    fn value_at(&self, r: &Ray) -> Color {
        let t = 0.5 * (r.direction.unit().as_ref().y + 1.0);
        &self.from * (1.0 - t) + &self.to * t
    }
}

pub struct ConstantBackground {
    pub color: Color,
}

impl Background for ConstantBackground {
    fn value_at(&self, _r: &Ray) -> Color {
        self.color.clone()
    }
}
