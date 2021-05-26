use crate::scene::scenemap::sdf::Sdf;
use crate::{Color, Point3, Ray};

#[derive(Debug, Copy, Clone)]
pub struct MaterialIndex(usize);

#[derive(Default)]
pub struct MaterialList {
    mats: Vec<Box<dyn Material>>,
}

impl MaterialList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, m: Box<dyn Material>) -> MaterialIndex {
        self.mats.push(m);
        MaterialIndex(self.mats.len() - 1)
    }

    pub fn get(&self, idx: MaterialIndex) -> Option<&dyn Material> {
        self.mats.get(idx.0).map(|m| m.as_ref())
    }
}

pub trait Material {
    fn specular(&self) -> Color;
    fn diffuse(&self) -> Color;
    fn ambient(&self) -> Color;
    fn shininess(&self) -> f64;

    fn child_ray(&self, _sdf: &dyn Sdf, _p: &Point3, _incoming: &Ray) -> Option<Ray> {
        None
    }
}

pub struct PhongMaterial {
    pub specular: Color,
    pub diffuse: Color,
    pub ambient: Color,
    pub shininess: f64,
}

impl Material for PhongMaterial {
    fn specular(&self) -> Color {
        self.specular.clone()
    }
    fn diffuse(&self) -> Color {
        self.diffuse.clone()
    }
    fn ambient(&self) -> Color {
        self.ambient.clone()
    }
    fn shininess(&self) -> f64 {
        self.shininess
    }
}

pub struct ReflectiveMaterial;

impl Material for ReflectiveMaterial {
    fn specular(&self) -> Color {
        Color::black()
    }

    fn diffuse(&self) -> Color {
        Color::black()
    }

    fn ambient(&self) -> Color {
        Color::black()
    }

    fn shininess(&self) -> f64 {
        0.0
    }

    fn child_ray(&self, sdf: &dyn Sdf, p: &Point3, incoming: &Ray) -> Option<Ray> {
        let normal = sdf.estimate_normal(p);
        let reflected_direction = incoming.direction().as_ref().reflect(&normal);
        Some(Ray::new_unnormalized(p.clone(), reflected_direction))
    }
}

pub struct DefaultMaterial;

impl Material for DefaultMaterial {
    fn specular(&self) -> Color {
        Color::purple()
    }

    fn diffuse(&self) -> Color {
        Color::purple()
    }

    fn ambient(&self) -> Color {
        Color::purple()
    }

    fn shininess(&self) -> f64 {
        1.0
    }
}
