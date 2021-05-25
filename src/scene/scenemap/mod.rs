use crate::scene::scenemap::sdf::Sdf;

pub mod material;
pub mod sdf;

pub struct SceneMap<'a, F> {
    pub sdf: &'a dyn Sdf<F>,
    pub materials: &'a material::MaterialList<F>,
    // pub lights: Vec<dyn Light<F> + 'a>,
}
