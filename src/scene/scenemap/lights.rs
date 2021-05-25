use crate::{Color, Point3};

#[derive(Debug, Clone)]
pub struct AmbientLight<F>(pub Color<F>);

impl<F> AmbientLight<F> {
    pub fn new(c: Color<F>) -> Self {
        Self(c)
    }
}

#[derive(Debug, Clone)]
pub struct Light<F> {
    pub location: Point3<F>,
    pub specular: Color<F>,
    pub diffuse: Color<F>,
}
