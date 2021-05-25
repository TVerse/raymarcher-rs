use std::fs::File;
use std::io::{BufWriter, Write};
use std::rc::Rc;
use std::time::Instant;

use raymarcher_rs::scene::camera::Camera;
use raymarcher_rs::scene::scenemap::lights::{AmbientLight, Light};
use raymarcher_rs::scene::scenemap::material::{MaterialIndex, MaterialList, SingleColorMaterial};
use raymarcher_rs::scene::scenemap::sdf::{
    Arbitrary, Intersect, ScaleUniform, Sdf, Subtract, Translate, Union, UnitCube, UnitSphere,
    WithMaterial,
};
use raymarcher_rs::scene::scenemap::SceneMap;
use raymarcher_rs::scene::{ConstantBackground, Scene, VerticalGradientBackground};
use raymarcher_rs::{render, Color, Config, ImageSettings, Point3, RGBColor, RenderSettings, Vec3};

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let mut material_list = MaterialList::new();

    let white = material_list.insert(Box::new(SingleColorMaterial {
        specular: Color::new(0.9, 0.9, 0.9),
        diffuse: Color::new(0.9, 0.9, 0.9),
        ambient: Color::new(0.9, 0.9, 0.9),
        shininess: 10.0,
    }));
    let red = material_list.insert(Box::new(SingleColorMaterial {
        specular: Color::new(0.9, 0.1, 0.1),
        diffuse: Color::new(0.9, 0.1, 0.1),
        ambient: Color::new(0.9, 0.1, 0.1),
        shininess: 5.0,
    }));

    let config: Config<f64> = Config::new(
        ImageSettings::new(image_width, image_height),
        RenderSettings::new(0.001, 1000.0, 1e-5, 1000, None),
        // RenderSettings::new(0.001, 1000.0, 1e-5, 500, None),
    );

    let camera = Camera::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.5, 5.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        aspect_ratio,
    );

    let infinite_floor = Arbitrary::new(|p| {
        let v: &Vec3<f64> = &p.0;
        // Divide by 2 is to reduce holes in the floor at the cost of slower rendering
        ((v.y - (v.x.sin() + v.z.sin())) / 2.0, None)
    });

    let infinite_floor = Rc::new(ScaleUniform {
        a: infinite_floor,
        f: 0.1,
    });

    let floor = Intersect {
        a: infinite_floor.clone(),
        b: ScaleUniform {
            a: UnitCube,
            f: 50.0,
        },
    };

    let sdf = Union {
        a: Union {
            a: WithMaterial {
                a: UnitCube,
                m: Some(red),
            },
            b: Translate {
                a: Intersect {
                    a: WithMaterial {
                        a: UnitSphere,
                        m: Some(white),
                    },
                    b: Translate {
                        a: ScaleUniform {
                            a: infinite_floor,
                            f: 2.0,
                        },
                        v: Vec3::new(0.0, 0.5, 0.0),
                    },
                },
                v: Vec3::new(0.0, 1.5, 0.0),
            },
        },
        b: floor,
    };

    let ambient_light = AmbientLight::new(Color::new(0.5, 0.5, 0.5));

    let lights = vec![
        Light {
            location: Point3::new(0.0, 2.0, 10.0),
            specular: Color::new(0.4, 0.4, 0.4),
            diffuse: Color::new(0.4, 0.4, 0.4),
        },
        Light {
            location: Point3::new(0.0, 5.0, 0.0),
            specular: Color::new(0.4, 0.9, 0.4),
            diffuse: Color::new(0.4, 0.4, 0.4),
        },
        Light {
            location: Point3::new(3.0, 2.0, 0.0),
            specular: Color::new(0.1, 0.1, 0.1),
            diffuse: Color::new(0.1, 0.1, 0.9),
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
        // background: Box::new(ConstantBackground {
        //     color: Color::purple(),
        // }),
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
