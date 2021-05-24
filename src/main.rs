use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use raymarcher_rs::scene::camera::Camera;
use raymarcher_rs::scene::scenemap::sdf::{Intersect, Translate, Union, UnitCube, UnitSphere, Arbitrary, ScaleUniform};
use raymarcher_rs::scene::scenemap::SceneMap;
use raymarcher_rs::scene::{Scene, VerticalGradientBackground};
use raymarcher_rs::{
    render, Color, Config, ImageSettings, MaterialOverride, Point3, RGBColor, RenderSettings, Vec3,
};

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let config: Config<f64> = Config {
        image_settings: ImageSettings {
            width: image_width,
            height: image_height,
        },
        render_settings: RenderSettings {
            max_marching_steps: 1000,
            max_light_recursions: 50,
            t_min: 0.001,
            t_max: 100.0,
            epsilon: 1e-5,
            material_override: Some(MaterialOverride::Normal),
        },
    };

    let floor = Arbitrary{
        s: {
            |p: &Point3<f64>| {
                let v: &Vec3<f64> = &p.0;
                v.y - (v.x.sin() + v.z.sin())
            }
        }
    };

    let scaled_floor = ScaleUniform {
        a: &floor,
        f: 0.1,
    };

    let sdf = Union  {
        a: &Union{
            a:&UnitCube,
            b: &Translate{
                a: &Intersect {
                    a: &UnitSphere,
                    b: &Translate {
                        a: &ScaleUniform {
                            a: &scaled_floor,
                            f: 2.0,
                        },
                        v: &Vec3::new(0.0, 0.5, 0.0),
                    }
                },
                v: &Vec3::new(0.0, 1.5, 0.0),
            }
        },
        b: &scaled_floor,
    };

    let scene = Scene {
        camera: Camera::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(3.0, 3.0, 5.0),
            Vec3::new(0.0, 1.0, 0.0),
            45.0,
            aspect_ratio,
        ),
        scene_map: SceneMap { sdf: Box::new(sdf) },
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
