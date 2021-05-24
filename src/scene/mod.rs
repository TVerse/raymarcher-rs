pub mod camera;
pub mod scenemap;

use crate::primitives::Color;
use crate::scene::camera::Camera;
use crate::scene::scenemap::SceneMap;
use crate::Ray;
use num_traits::Float;

pub struct Scene<'a, F> {
    pub camera: Camera<F>,
    pub scene_map: SceneMap<'a, F>,
    pub background: Box<dyn Background<F>>,
}

pub trait Background<F> {
    fn value_at(&self, r: &Ray<F>) -> Color<F>;
}

pub struct VerticalGradientBackground<F> {
    pub from: Color<F>,
    pub to: Color<F>,
}

impl<F: Float> Background<F> for VerticalGradientBackground<F> {
    fn value_at(&self, r: &Ray<F>) -> Color<F> {
        let t = (F::one() / (F::one() + F::one())) * (r.direction.unit().as_ref().y + F::one());
        &self.from * (F::one() - t) + &self.to * t
    }
}
