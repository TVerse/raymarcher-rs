use crate::scene::scenemap::sdf::Sdf;

pub mod sdf;

pub struct SceneMap<'a, F> {
    pub sdf: Box<dyn Sdf<F> + 'a>,
    // pub material: Box<dyn Material<F> + 'a>
    // pub lights: Vec<dyn Light<F> + 'a>,
}
