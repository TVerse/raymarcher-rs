use crate::{Color, Point3};

#[derive(Debug, Clone)]
pub struct AmbientLight(pub Color);

impl AmbientLight {
    pub fn new(c: Color) -> Self {
        Self(c)
    }
}

#[derive(Debug, Clone)]
pub struct Light {
    pub location: Point3,
    pub specular: Color,
    pub diffuse: Color,
    pub strength: f64,
    pub shadow_hardness: f64,
}
