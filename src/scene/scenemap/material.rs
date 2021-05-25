use crate::raymarcher::FindTargetSettings;
use crate::scene::scenemap::sdf::Sdf;
use crate::{Color, Point3};

#[derive(Copy, Clone)]
pub struct MaterialIndex(usize);

pub struct MaterialList<F> {
    mats: Vec<Box<dyn Material<F>>>,
}

impl<F> MaterialList<F> {
    pub fn new() -> Self {
        Self {
            mats: Vec::new(),
        }
    }

    pub fn insert(&mut self, m: Box<dyn Material<F>>) -> MaterialIndex {
        self.mats.push(m);
        MaterialIndex(self.mats.len() - 1)
    }

    pub fn get(&self, idx: MaterialIndex) -> Option<&dyn Material<F>> {
        self.mats.get(idx.0).map(|m| m.as_ref())
    }
}

pub trait Material<F> {
    fn value_at(
        &self,
        p: &Point3<F>,
        sdf: &dyn Sdf<F>,
        find_target_settings: &FindTargetSettings<F>,
    ) -> Option<Color<F>>;
}

pub struct SingleColorMaterial<F> {
    pub color: Color<F>,
}

impl<F: Clone> Material<F> for SingleColorMaterial<F> {
    fn value_at(
        &self,
        _p: &Point3<F>,
        _sdf: &dyn Sdf<F>,
        _find_target_settings: &FindTargetSettings<F>,
    ) -> Option<Color<F>> {
        Some(self.color.clone())
    }
}
