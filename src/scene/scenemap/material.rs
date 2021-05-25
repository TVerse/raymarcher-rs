use crate::raymarcher::FindTargetSettings;
use crate::scene::camera::Camera;
use crate::scene::scenemap::sdf::Sdf;
use crate::{constants, Color, Point3};
use num_traits::Float;

#[derive(Debug, Copy, Clone)]
pub struct MaterialIndex(usize);

pub struct MaterialList<F> {
    mats: Vec<Box<dyn Material<F>>>,
}

impl<F> MaterialList<F> {
    pub fn new() -> Self {
        Self { mats: Vec::new() }
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
    fn specular(&self) -> Color<F>;
    fn diffuse(&self) -> Color<F>;
    fn ambient(&self) -> Color<F>;
    fn shininess(&self) -> F;
}

#[derive(Debug, Clone)]
pub struct SingleColorMaterial<F> {
    pub specular: Color<F>,
    pub diffuse: Color<F>,
    pub ambient: Color<F>,
    pub shininess: F,
}

impl<F: Copy> Material<F> for SingleColorMaterial<F> {
    fn specular(&self) -> Color<F> {
        self.specular.clone()
    }
    fn diffuse(&self) -> Color<F> {
        self.diffuse.clone()
    }
    fn ambient(&self) -> Color<F> {
        self.ambient.clone()
    }
    fn shininess(&self) -> F {
        self.shininess.clone()
    }
}

// TODO
// pub struct Normal;
//
// impl<F: Float> Material<F> for Normal {
//     fn value_at(&self, p: &Point3<F>, sdf: &dyn Sdf<F>) -> Color<F> {
//         let normal = sdf.estimate_normal(p);
//         (Color::white() + Color(normal.0)) * constants::half()
//     }
//
//     fn specular(&self) -> F {
//         F::one()
//     }
//
//     fn diffuse(&self) -> F {
//         F::one()
//     }
//
//     fn ambient(&self) -> F {
//         F::one()
//     }
//
//     fn shininess(&self) -> F {
//         F::one()
//     }
// }

pub struct DefaultMaterial;

impl<F: Float> Material<F> for DefaultMaterial {
    fn specular(&self) -> Color<F> {
        Color::purple()
    }

    fn diffuse(&self) -> Color<F> {
        Color::purple()
    }

    fn ambient(&self) -> Color<F> {
        Color::purple()
    }

    fn shininess(&self) -> F {
        F::one()
    }
}
