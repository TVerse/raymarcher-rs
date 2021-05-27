use crate::{Color, Point3};

#[derive(Debug, Clone)]
pub struct AmbientLight(pub Color);

impl AmbientLight {
    pub fn new(c: Color) -> Self {
        Self(c)
    }
}

// TODO think about attenuation and light types (directional/point/spot)
// https://blogs.igalia.com/itoral/2017/07/06/working_lights_shadows_parti_phong_reflection_model/

#[derive(Debug, Clone)]
pub struct Light {
    pub location: Point3,
    pub specular: Color,
    pub diffuse: Color,
    pub strength: f64,
    pub shadow_hardness: f64,
}
