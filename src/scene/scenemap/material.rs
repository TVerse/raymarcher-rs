use crate::scene::scenemap::sdf::Sdf;
use crate::{Color, Point3, Ray};

/*
TODO

* Add refraction. Idea: offset target point based on normal (maybe? t_min?), then invert SDF and continue as normal. Is it possible to distinguish inside/outside (for refractive index) then? Unless that's also inverted in inverted SDFs.
* Allow changing of material based on point and/or normals (for normal shade override)

 */

#[derive(Debug, Copy, Clone)]
pub struct MaterialIndex(usize);

#[derive(Default)]
pub struct MaterialList {
    mats: Vec<Material>,
}

impl MaterialList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, m: Material) -> MaterialIndex {
        self.mats.push(m);
        MaterialIndex(self.mats.len() - 1)
    }

    pub fn get(&self, idx: MaterialIndex) -> Option<&Material> {
        self.mats.get(idx.0)
    }
}

pub struct Material {
    pub specular: Color,
    pub diffuse: Color,
    pub ambient: Color,
    pub shininess: f64,
    pub reflectivity: f64,
}

impl Material {
    pub const DEFAULT: Material = Self::new(Color::PURPLE, Color::PURPLE, Color::PURPLE, 1.0, 0.0);

    pub const fn new(
        specular: Color,
        diffuse: Color,
        ambient: Color,
        shininess: f64,
        reflectivity: f64,
    ) -> Self {
        Self {
            specular,
            diffuse,
            ambient,
            shininess,
            reflectivity,
        }
    }

    pub const fn pure_reflective() -> Self {
        Self::new(Color::BLACK, Color::BLACK, Color::BLACK, 0.0, 1.0)
    }

    pub fn specular(&self) -> Color {
        self.specular.clone()
    }
    pub fn diffuse(&self) -> Color {
        self.diffuse.clone()
    }
    pub fn ambient(&self) -> Color {
        self.ambient.clone()
    }
    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn reflectivity(&self) -> f64 {
        self.reflectivity
    }

    pub fn child_ray(&self, sdf: &dyn Sdf, p: &Point3, incoming: &Ray) -> Option<Ray> {
        let normal = sdf.estimate_normal(p);
        let reflected_direction = incoming.direction().as_ref().reflect(&normal);
        Some(Ray::new_unnormalized(p.clone(), reflected_direction))
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::DEFAULT
    }
}
