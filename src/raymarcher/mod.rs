use crate::primitives::Color;
use crate::scene::scenemap::Sdf;
use crate::scene::Scene;
use crate::{Config, ImageSettings, MaterialOverride, Point3, Ray, RenderSettings};
use itertools::Itertools;
use num_traits::Float;

pub fn render<'a, F: Float>(
    config: &'a Config<F>,
    scene: &'a Scene<'a, F>,
) -> impl Iterator<Item = Color<F>> + 'a {
    let ImageSettings { width, height } = config.image_settings;
    let height_iter = (0..height).rev();
    let width_iter = 0..width;

    let pixels = height_iter.cartesian_product(width_iter);

    pixels.map(move |(j, i)| generate_pixel(i, j, width, height, &config.render_settings, scene))
}

fn generate_pixel<'a, F: Float>(
    i: usize,
    j: usize,
    image_width: usize,
    image_height: usize,
    render_settings: &'a RenderSettings<F>,
    scene: &'a Scene<'a, F>,
) -> Color<F> {
    let i = F::from(i).unwrap();
    let j = F::from(j).unwrap();

    let u = i / (F::from(image_width).unwrap() - F::one());
    let v = j / (F::from(image_height).unwrap() - F::one());

    let ray = scene.camera.get_ray(u, v);

    let Scene {
        camera: _,
        scene_map,
        background,
    } = scene;

    DepthIterator {
        sdf: scene_map.sdf.as_ref(),
        r: &ray,
        cur_depth: render_settings.t_min,
    }
    .into_iter()
    .take(render_settings.max_marching_steps)
    .filter(|DepthResult { total_depth, .. }| total_depth < &render_settings.t_max)
    .find(|DepthResult { dist, .. }| dist.abs() < render_settings.epsilon)
    .map(
        |DepthResult { point, .. }| match render_settings.material_override {
            Some(MaterialOverride::Normal) => {
                let normal = scene_map.sdf.estimate_normal(point);
                let half = F::one() / (F::one() + F::one());
                (Color::white() + Color(normal.0)) * half
            }
            None => Color::purple(),
        },
    )
    .unwrap_or_else(|| background.value_at(&ray))
}

struct DepthIterator<'a, F> {
    sdf: &'a dyn Sdf<F>,
    r: &'a Ray<F>,
    cur_depth: F,
}

struct DepthResult<F> {
    point: Point3<F>,
    dist: F,
    total_depth: F,
}

impl<'a, F: Float> Iterator for DepthIterator<'a, F> {
    type Item = DepthResult<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = Point3(self.r.origin.as_ref() + &self.r.direction * self.cur_depth);
        let dist = self.sdf.value_at(&point);
        let total_depth = dist + self.cur_depth;

        self.cur_depth = total_depth;

        Some(Self::Item {
            point,
            dist,
            total_depth,
        })
    }
}
