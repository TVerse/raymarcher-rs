use crate::primitives::{Color, UnitVec3};
use crate::scene::Scene;
use crate::{constants, Config, ImageSettings, Point3, RenderSettings, Vec3};
use itertools::Itertools;
use num_traits::Float;

mod ray;
use crate::scene::scenemap::lights::Light;
use crate::scene::scenemap::material::{DefaultMaterial, Material};
use crate::scene::scenemap::SceneMap;
pub use ray::FindTargetSettings;
pub use ray::Ray;
use std::slice::Iter;
use crate::scene::scenemap::sdf::Sdf;
use float_cmp::ApproxEq;

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
        scene_map,
        background,
        ..
    } = scene;

    ray.find_target(&render_settings.find_target_settings, scene_map.sdf)
        .map(|(point, mat)| {
            let mat = if let Some(m) = &render_settings.material_override {
                Some(m)
            } else {
                mat.as_ref()
            };
            let material = mat
                .and_then(|m| scene_map.materials.get(*m))
                .unwrap_or(&DefaultMaterial);

            phong(material, scene, &point, &render_settings.find_target_settings)
        })
        .unwrap_or_else(|| background.value_at(&ray))
}

fn phong<'a, F: Float>(
    material: &'a dyn Material<F>,
    scene: &'a Scene<'a, F>,
    p: &Point3<F>,
    find_target_settings: &FindTargetSettings<F>,
) -> Color<F> {
    let Scene {
        scene_map, camera, ..
    } = scene;

    let SceneMap {
        ambient_light,
        lights,
        sdf,
        ..
    } = scene_map;

    let normal = sdf.estimate_normal(p);

    let v = (camera.origin.as_ref() - p.as_ref()).unit();

    let r = phong_single_element(
        |c| c.r(),
        &ambient_light.0,
        material,
        filter_lights(lights, p, *sdf, find_target_settings),
        &p,
        &normal,
        &v,
    );
    let g = phong_single_element(
        |c| c.g(),
        &ambient_light.0,
        material,
        filter_lights(lights, p, *sdf, find_target_settings),
        &p,
        &normal,
        &v,
    );
    let b = phong_single_element(
        |c| c.b(),
        &ambient_light.0,
        material,
        filter_lights(lights, p, *sdf, find_target_settings),
        &p,
        &normal,
        &v,
    );
    Color::new(r, g, b)
}

fn filter_lights<'a, F: Float>(lights: &'a [Light<F>], p: &'a Point3<F>, sdf: &'a dyn Sdf<F>, find_target_settings: &'a FindTargetSettings<F>) -> impl Iterator<Item=&'a Light<F>> {
    let p = p.clone();
    lights
        .iter()
        .filter(|&l| {
            let direction = l.location.as_ref() - p.as_ref();
            let origin = p.clone();
            let r = Ray{
                origin,
                direction,
            };
            let target = r.find_target(find_target_settings, sdf);
            match target {
                Some(t) => true, // TODO
                None => false
            }
        })
}

fn phong_single_element<'a, F: Float + 'a, E: Fn(&Color<F>) -> F, L: Iterator<Item=&'a Light<F>>>(
    f: E,
    ambient_color: &Color<F>,
    material: &dyn Material<F>,
    lights: L,
    p: &Point3<F>,
    normal: &UnitVec3<F>,
    v: &UnitVec3<F>,
) -> F {
    let ambient_contribution = f(ambient_color) * f(&material.ambient());

    let light_contribution = lights
        .map(|light| {
            let l: UnitVec3<F> = (light.location.as_ref() - p.as_ref()).unit();
            let l_dot_normal: F = l.as_ref().dot(normal.as_ref());
            let r: Vec3<F> =
                normal.as_ref() * (constants::two::<F>() * (l_dot_normal)) - l.as_ref();
            let diffuse = if l_dot_normal > F::zero() {
                f(&material.diffuse()) * l_dot_normal * f(&light.diffuse)
            } else {
                F::zero()
            };
            let specular_dot = r.dot(v.as_ref());
            let specular = if specular_dot > F::zero() && l_dot_normal > F::zero() {
                f(&material.specular())
                    * specular_dot.powf(material.shininess())
                    * f(&light.specular)
            } else {
                F::zero()
            };
            diffuse + specular
        })
        .fold(F::zero(), |acc, l| acc + l);
    ambient_contribution + light_contribution
}
