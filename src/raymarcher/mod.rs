use crate::primitives::{Color, UnitVec3};
use crate::scene::Scene;
use crate::{Config, ImageSettings, Point3, RenderSettings, Vec3};
use itertools::Itertools;

mod ray;
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
        generate_pixel(ray, &config.render_settings, scene)
    })
}

fn generate_pixel<'a>(
    ray: Ray,
    render_settings: &'a RenderSettings,
    scene: &'a Scene<'a>,
) -> Color {
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

            phong(
                material,
                scene,
                &point,
                &render_settings.find_target_settings,
            )
        })
        .unwrap_or_else(|| background.value_at(&ray))
}

fn phong<'a>(
    material: &'a dyn Material,
    scene: &'a Scene<'a>,
    p: &Point3,
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

fn filter_lights<'a>(
    lights: &'a [Light],
    p: &'a Point3,
    sdf: &'a dyn Sdf,
    find_target_settings: &'a FindTargetSettings,
) -> impl Iterator<Item = &'a Light> + 'a {
    lights.iter().filter(move |&l| {
        let direction = p.as_ref() - l.location.as_ref();
        let settings = FindTargetSettings {
            t_max: direction.length(),
            ..(*find_target_settings.clone())
        };
        let origin = l.location.clone();
        let r = Ray { origin, direction };
        !r.is_shadow(&settings, sdf)
    })
}

fn phong_single_element<'a, E: Fn(&Color) -> f64, L: Iterator<Item = &'a Light>>(
    f: E,
    ambient_color: &Color,
    material: &dyn Material,
    lights: L,
    p: &Point3,
    normal: &UnitVec3,
    v: &UnitVec3,
) -> f64 {
    let ambient_contribution = f(ambient_color) * f(&material.ambient());

    let light_contribution = lights
        .map(|light| {
            let l: UnitVec3 = (light.location.as_ref() - p.as_ref()).unit();
            let l_dot_normal: f64 = l.as_ref().dot(normal.as_ref());
            let r: Vec3 = normal.as_ref() * (2.0 * (l_dot_normal)) - l.as_ref();
            let diffuse = if l_dot_normal > 0.0 {
                f(&material.diffuse()) * l_dot_normal * f(&light.diffuse)
            } else {
                0.0
            };
            let specular_dot = r.dot(v.as_ref());
            let specular = if specular_dot > 0.0 && l_dot_normal > 0.0 {
                f(&material.specular())
                    * specular_dot.powf(material.shininess())
                    * f(&light.specular)
            } else {
                0.0
            };
            diffuse + specular
        })
        .fold(0.0, |acc, l| acc + l);
    ambient_contribution + light_contribution
}
