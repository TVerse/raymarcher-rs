use crate::scene::scenemap::lights::{AmbientLight, Light};
use crate::scene::scenemap::sdf::Sdf;

pub mod lights;
pub mod material;
pub mod sdf;

pub struct SceneMap<'a> {
    pub sdf: &'a dyn Sdf,
    pub materials: &'a material::MaterialList,
    pub ambient_light: AmbientLight,
    pub lights: &'a [Light],
}
