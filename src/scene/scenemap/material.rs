use crate::Color;

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
}

#[derive(Debug, Clone)]
pub struct SingleColorMaterial {
    pub specular: Color,
    pub diffuse: Color,
    pub ambient: Color,
    pub shininess: f64,
}

impl Material for SingleColorMaterial {
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

// TODO
// pub struct Normal;
//
// impl Material for Normal {
//     fn value_at(&self, p: &Point3, sdf: &dyn Sdf) -> Color {
//         let normal = sdf.estimate_normal(p);
//         (Color::white() + Color(normal.0)) * constants::half()
//     }
//
//     fn specular(&self) -> f64 {
//         1.0
//     }
//
//     fn diffuse(&self) -> f64 {
//         1.0
//     }
//
//     fn ambient(&self) -> f64 {
//         1.0
//     }
//
//     fn shininess(&self) -> f64 {
//         1.0
//     }
// }

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
