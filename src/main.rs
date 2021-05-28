use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use raymarcher_rs::scene::camera::Camera;
use raymarcher_rs::scene::scenemap::lights::{AmbientLight, Light};
use raymarcher_rs::scene::scenemap::material::{Material, MaterialList};
use raymarcher_rs::scene::scenemap::sdf::combinators::{Difference, Intersect, Union};
use raymarcher_rs::scene::scenemap::sdf::positioners::{ScaleUniform, Translate};
use raymarcher_rs::scene::scenemap::sdf::primitives::{Arbitrary, Cube, Sphere};
use raymarcher_rs::scene::scenemap::sdf::WithMaterial;
use raymarcher_rs::scene::scenemap::SceneMap;
use raymarcher_rs::scene::{Scene, VerticalGradientBackground};
use raymarcher_rs::{render, Color, Config, ImageSettings, Point3, RGBColor, RenderSettings, Vec3};

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let mut material_list = MaterialList::new();

    let top_material = material_list.insert(Material::new(
        Color::new(0.9, 0.9, 0.9),
        Color::new(0.9, 0.9, 0.9),
        Color::new(0.9, 0.9, 0.9),
        10.0,
        0.3,
    ));
    let red = material_list.insert(Material::new(
        Color::new(0.9, 0.1, 0.1),
        Color::new(0.5, 0.5, 0.5),
        Color::new(0.9, 0.1, 0.1),
        5.0,
        0.5,
    ));
    let floor_material = material_list.insert(Material::new(
        Color::new(0.1, 0.1, 0.1),
        Color::new(0.1, 0.1, 0.1),
        Color::BLACK,
        32.0,
        0.1,
    ));

    let sphere_outside_material = material_list.insert(Material::new(
        Color::new(0.9, 0.1, 0.1),
        Color::new(0.9, 0.1, 0.1),
        Color::BLACK,
        32.0,
        0.1,
    ));

    let box_outside_material = material_list.insert(Material::pure_reflective());

    let lattice_material = material_list.insert(Material::new(
        Color::new(0.2, 0.2, 0.2),
        Color::new(0.2, 0.2, 0.2),
        Color::new(0.2, 0.2, 0.2),
        0.0,
        0.0,
    ));

    let config: Config = Config::new(
        ImageSettings::new(image_width, image_height),
        RenderSettings::new(0.001, 100.0, 1e-4, 100, None),
    );

    let camera = Camera::new(
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(-2.0, 2.5, 5.0),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        aspect_ratio,
    );

    let sine_wave = Arbitrary::new(|p| {
        let v: &Vec3 = &p.0;
        // Divide by 2 is to reduce holes in the floor at the cost of slower rendering
        ((v.y - (v.x.sin() + v.z.sin())) / 2.0, None)
    });

    let sine_wave = WithMaterial::new(ScaleUniform::new(sine_wave, 0.1), top_material);

    let floor = WithMaterial::new(
        Cube::new(20.0, Point3::new(0.0, -10.0, 0.0)),
        floor_material,
    );

    let wavy_sphere = Intersect::new(
        WithMaterial::new(Sphere::new(0.9, Point3::ORIGIN), sphere_outside_material),
        Translate::new(ScaleUniform::new(&sine_wave, 2.0), Vec3::new(0.0, 0.5, 0.0)),
    );

    let translate_cube = |v: Vec3| Cube::new(2.0, Point3(v * 0.2));

    let cross = Union::new(
        translate_cube(Vec3::new(1.0, 0.0, 0.0)),
        Union::new(
            translate_cube(Vec3::new(-1.0, 0.0, 0.0)),
            Union::new(
                translate_cube(Vec3::new(0.0, 1.0, 0.0)),
                Union::new(
                    translate_cube(Vec3::new(0.0, -1.0, 0.0)),
                    Union::new(
                        translate_cube(Vec3::new(0.0, 0.0, 1.0)),
                        translate_cube(Vec3::new(0.0, 0.0, -1.0)),
                    ),
                ),
            ),
        ),
    );

    let lattice = WithMaterial::new(
        Difference::new(Cube::default(), ScaleUniform::new(cross, 0.9)),
        lattice_material,
    );

    let wavy_cube = Intersect::new(
        WithMaterial::new(Cube::new(1.8, Point3::ORIGIN), box_outside_material),
        Translate::new(ScaleUniform::new(&sine_wave, 2.0), Vec3::new(0.0, 0.5, 0.0)),
    );

    let contained_cube = Union::new(lattice, wavy_cube);

    let sdf = Union::new(
        Union::new(
            WithMaterial::new(Cube::new(4.0, Point3::new(0.0, -1.0, 0.0)), red),
            Union::new(
                Translate::new(wavy_sphere, Vec3::new(1.0, 1.5, 1.0)),
                Translate::new(contained_cube, Vec3::new(-1.0, 2.0, -1.0)),
            ),
        ),
        floor,
    );

    let ambient_light = AmbientLight::new(Color::new(0.2, 0.2, 0.2));

    let lights = vec![
        Light {
            location: Point3::new(0.0, 2.0, 10.0),
            specular: Color::new(0.4, 0.4, 0.4),
            diffuse: Color::new(0.4, 0.4, 0.4),
            strength: 6.0,
            shadow_hardness: 2.0,
        },
        Light {
            location: Point3::new(0.0, 5.0, 0.0),
            specular: Color::new(0.4, 0.9, 0.4),
            diffuse: Color::new(0.4, 0.4, 0.4),
            strength: 2.0,
            shadow_hardness: 128.0,
        },
        Light {
            location: Point3::new(3.0, 2.0, 1.5),
            specular: Color::new(0.5, 0.5, 0.5),
            diffuse: Color::new(0.9, 0.9, 0.9),
            strength: 3.0,
            shadow_hardness: 32.0,
        },
    ];

    let scene = Scene {
        camera,
        scene_map: SceneMap {
            sdf: &sdf,
            materials: &material_list,
            ambient_light,
            lights: &lights,
        },
        background: Box::new(VerticalGradientBackground {
            from: Color::new(1.0, 1.0, 1.0),
            to: Color::new(0.5, 0.7, 1.0),
        }),
    };

    let result = render(&config, &scene);

    let mut file = File::create("image.ppm")?;
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n255\n", image_width, image_height).as_bytes())?;
    write_iter(&mut file, result.map(|c| c.into()))?;
    file.sync_all()?;

    let duration = start.elapsed();

    println!("Done! Took {:.3} seconds.", duration.as_secs_f64());

    Ok(())
}

fn write_iter<W: Write, I: Iterator<Item = RGBColor>>(
    writer: &mut W,
    iter: I,
) -> std::io::Result<()> {
    let mut buf_writer = BufWriter::with_capacity(1024, writer);

    for RGBColor { r, g, b } in iter {
        buf_writer.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;
    }

    Ok(())
}
