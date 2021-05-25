use crate::scene::scenemap::lights::{AmbientLight, Light};
use crate::scene::scenemap::sdf::Sdf;

pub mod lights;
pub mod material;
pub mod sdf;

pub struct SceneMap<'a, F> {
    pub sdf: &'a dyn Sdf<F>,
    pub materials: &'a material::MaterialList<F>,
    pub ambient_light: AmbientLight<F>,
    pub lights: &'a [Light<F>],
}
