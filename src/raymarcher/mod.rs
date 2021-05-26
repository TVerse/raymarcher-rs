use crate::primitives::{Color, UnitVec3};
use crate::scene::Scene;
use crate::{Config, ImageSettings, Point3, RenderSettings, Vec3};
use itertools::Itertools;

mod ray;
use crate::raymarcher::ray::FindTargetResult;
use crate::scene::scenemap::lights::Light;
use crate::scene::scenemap::material::{DefaultMaterial, Material};
use crate::scene::scenemap::sdf::Sdf;
use crate::scene::scenemap::SceneMap;
pub use ray::FindTargetSettings;
pub use ray::Ray;

pub fn render<'a>(config: &'a Config, scene: &'a Scene<'a>) -> impl Iterator<Item = Color> + 'a {
    let ImageSettings { width, height } = config.image_settings;
    let height_iter = (0..height).rev();
    let width_iter = 0..width;

    let pixels = height_iter.cartesian_product(width_iter);

    pixels.map(move |(j, i)| {
        let i = i as f64;
        let j = j as f64;

        let u = i / ((width as f64) - 1.0);
        let v = j / ((height as f64) - 1.0);

        let ray = scene.camera.get_ray(u, v);
        generate_pixel(&ray, &config.render_settings, scene, config.render_settings.max_recursions)
    })
}

fn generate_pixel<'a>(
    ray: &Ray,
    render_settings: &'a RenderSettings,
    scene: &'a Scene<'a>,
    remaining_depth: usize,
) -> Color {
    if remaining_depth == 0 {
        return Color::black()
    }
    let Scene {
        scene_map,
        background,
        ..
    } = scene;

    let sdf = scene_map.sdf;

    ray.find_target(&render_settings.find_target_settings, scene_map.sdf)
        .map(
            |FindTargetResult {
                 point,
                 material_index,
             }| {
                let mat = if let Some(m) = &render_settings.material_override {
                    Some(m)
                } else {
                    material_index.as_ref()
                };
                let material = mat
                    .and_then(|m| scene_map.materials.get(*m))
                    .unwrap_or(&DefaultMaterial);

                let phong_contribution = phong(
                    material,
                    scene,
                    &point,
                    &render_settings.find_target_settings,
                );

                material
                    .child_ray(sdf, &point, &ray)
                    .map(|r| generate_pixel(&r, render_settings, scene, remaining_depth - 1))
                    .unwrap_or(phong_contribution)
            },
        )
        .unwrap_or_else(|| background.value_at(&ray))
}

fn phong<'a>(
    material: &'a dyn Material,
    scene: &'a Scene<'a>,
    point: &Point3,
    find_target_settings: &FindTargetSettings,
) -> Color {
    let Scene {
        scene_map, camera, ..
    } = scene;

    let SceneMap {
        ambient_light,
        lights,
        sdf,
        ..
    } = scene_map;

    let normal = sdf.estimate_normal(point);

    let v = (camera.origin.as_ref() - point.as_ref()).unit();

    let red = phong_single_element(
        |c| c.r(),
        &ambient_light.0,
        material,
        find_light_factors(lights, point, *sdf, find_target_settings),
        &point,
        &normal,
        &v,
    );
    let green = phong_single_element(
        |c| c.g(),
        &ambient_light.0,
        material,
        find_light_factors(lights, point, *sdf, find_target_settings),
        &point,
        &normal,
        &v,
    );
    let blue = phong_single_element(
        |c| c.b(),
        &ambient_light.0,
        material,
        find_light_factors(lights, point, *sdf, find_target_settings),
        &point,
        &normal,
        &v,
    );
    Color::new(red, green, blue)
}

fn find_light_factors<'a>(
    lights: &'a [Light],
    p: &'a Point3,
    sdf: &'a dyn Sdf,
    find_target_settings: &'a FindTargetSettings,
) -> impl Iterator<Item = (&'a Light, f64)> + 'a {
    lights.iter().map(move |l| {
        let direction = l.location.as_ref() - p.as_ref();
        let settings = FindTargetSettings {
            t_max: direction.length(),
            ..find_target_settings.clone()
        };
        let origin = p.clone();
        let r = Ray::new(origin, direction.unit());
        (
            l,
            l.strength * r.soft_shadow(&settings, sdf, l.shadow_hardness),
        )
    })
}

fn phong_single_element<'a, E: Fn(&Color) -> f64, L: Iterator<Item = (&'a Light, f64)>>(
    extract: E,
    ambient_color: &Color,
    material: &dyn Material,
    light_factors: L,
    p: &Point3,
    normal: &UnitVec3,
    v: &UnitVec3,
) -> f64 {
    let ambient_contribution = extract(ambient_color) * extract(&material.ambient());

    let light_contribution = light_factors
        .map(|(light, factor)| {
            let l: UnitVec3 = (light.location.as_ref() - p.as_ref()).unit();
            let l_dot_normal: f64 = l.as_ref().dot(normal.as_ref());
            let r: Vec3 = normal.as_ref() * (2.0 * (l_dot_normal)) - l.as_ref();
            let diffuse = if l_dot_normal > 0.0 {
                extract(&material.diffuse()) * l_dot_normal * extract(&light.diffuse)
            } else {
                0.0
            };
            let specular_dot = r.dot(v.as_ref());
            let specular = if specular_dot > 0.0 && l_dot_normal > 0.0 {
                extract(&material.specular())
                    * specular_dot.powf(material.shininess())
                    * extract(&light.specular)
            } else {
                0.0
            };
            // println!("{}", factor);
            factor * (diffuse + specular)
        })
        .fold(0.0, |acc, l| acc + l);
    ambient_contribution + light_contribution
}
