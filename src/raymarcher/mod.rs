use crate::primitives::Color;
use crate::scene::Scene;
use crate::{constants, Config, ImageSettings, MaterialOverride, RenderSettings};
use itertools::Itertools;
use num_traits::Float;

mod ray;
pub use ray::Ray;

pub fn render<'a, F: Float>(
    config: &'a Config<F>,
    scene: &'a Scene<'a, F>,
) -> impl Iterator<Item = Color<F>> + 'a {
    let ImageSettings { width, height } = config.image_settings;
    let height_iter = (0..height).rev();
    let width_iter = 0..width;

    let pixels = height_iter.cartesian_product(width_iter);

    pixels.map(move |(j, i)| {
        let i = F::from(i).unwrap();
        let j = F::from(j).unwrap();

        let u = i / (F::from(width).unwrap() - F::one());
        let v = j / (F::from(height).unwrap() - F::one());

        let ray = scene.camera.get_ray(u, v);
        generate_pixel(ray, &config.render_settings, scene)
    })
}

fn generate_pixel<'a, F: Float>(
    ray: Ray<F>,
    render_settings: &'a RenderSettings<F>,
    scene: &'a Scene<'a, F>,
) -> Color<F> {
    let Scene {
        camera: _,
        scene_map,
        background,
    } = scene;

    ray.find_target(&render_settings, scene_map.sdf.as_ref())
        .map(|point| match render_settings.material_override {
            Some(MaterialOverride::Normal) => {
                let normal = scene_map.sdf.estimate_normal(point);
                (Color::white() + Color(normal.0)) * constants::half()
            }
            None => Color::purple(),
        })
        .unwrap_or_else(|| background.value_at(&ray))
}
