use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use raymarcher_rs::{Color, Config, ImageSettings, Point3, render, RenderSettings, RGBColor, Vec3};
use raymarcher_rs::scene::{Scene, VerticalGradientBackground};
use raymarcher_rs::scene::camera::Camera;
use raymarcher_rs::scene::scenemap::lights::{AmbientLight, Light};
use raymarcher_rs::scene::scenemap::material::{MaterialList, PhongMaterial, ReflectiveMaterial};
use raymarcher_rs::scene::scenemap::SceneMap;
use raymarcher_rs::scene::scenemap::sdf::{
    Arbitrary, halfplane, Intersect, ScaleUniform, Translate, Union, UnitCube, UnitSphere,
    WithMaterial,
};

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let mut material_list = MaterialList::new();

    let sphere_top_material = material_list.insert(Box::new(PhongMaterial {
        specular: Color::new(0.9, 0.9, 0.9),
        diffuse: Color::new(0.9, 0.9, 0.9),
        ambient: Color::new(0.9, 0.9, 0.9),
        shininess: 10.0,
    }));
    let red = material_list.insert(Box::new(PhongMaterial {
        specular: Color::new(0.9, 0.1, 0.1),
        diffuse: Color::new(0.5, 0.5, 0.5),
        ambient: Color::new(0.9, 0.1, 0.1),
        shininess: 5.0,
    }));
    let floor_material = material_list.insert(Box::new(PhongMaterial {
        specular: Color::new(0.1, 0.1, 0.1),
        diffuse: Color::new(0.1, 0.1, 0.1),
        ambient: Color::black(),
        shininess: 32.0,
    }));

    let sphere_outside_material = material_list.insert(Box::new(ReflectiveMaterial));

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

    let sine_wave = WithMaterial {
        a: ScaleUniform {
            a: sine_wave,
            f: 0.1,
        },
        m: sphere_top_material,
    };

    let floor = WithMaterial {
        a: Intersect {
            a: halfplane::NegY,
            b: ScaleUniform {
                a: UnitCube,
                f: 10.0,
            },
        },
        m: floor_material,
    };

    let wavy_sphere = Intersect {
        a: WithMaterial {
            a: ScaleUniform {
                a: UnitSphere,
                f: 0.9,
            },
            m: sphere_outside_material,
        },
        b: Translate {
            a: ScaleUniform {
                a: &sine_wave,
                f: 2.0,
            },
            v: Vec3::new(0.0, 0.5, 0.0),
        },
    };

    let wavy_cube = Intersect {
        a: WithMaterial {
            a: ScaleUniform {
                a: UnitCube,
                f: 0.9,
            },
            m: sphere_outside_material,
        },
        b: Translate {
            a: ScaleUniform {
                a: &sine_wave,
                f: 2.0,
            },
            v: Vec3::new(0.0, 0.5, 0.0),
        },
    };
    let sdf = Union {
        a: Union {
            a: WithMaterial {
                a: Translate { a: ScaleUniform { a: UnitCube, f: 2.0 }, v: Vec3::new(0.0, -1.0, 0.0) },
                m: red,
            },
            b: Union {
                a: Translate {
                    a: wavy_sphere,
                    v: Vec3::new(1.0, 1.5, 1.0),
                },
                b: Translate {
                    a: wavy_cube,
                    v: Vec3::new(-1.0, 1.5, -1.0),
                },
            },
        },
        b: floor,
    };

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

fn write_iter<W: Write, I: Iterator<Item=RGBColor>>(
    writer: &mut W,
    iter: I,
) -> std::io::Result<()> {
    let mut buf_writer = BufWriter::with_capacity(1024, writer);

    for RGBColor { r, g, b } in iter {
        buf_writer.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;
    }

    Ok(())
}
