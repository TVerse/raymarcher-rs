use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use raymarcher_rs::scene::camera::Camera;
use raymarcher_rs::scene::scenemap::{Intersect, SceneMap, Translate, Union, UnitCube, UnitSphere};
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

    let sphere = UnitSphere;
    let translated_sphere = Translate {
        a: &sphere,
        v: &Vec3::new(0.0, 1.0, 0.0),
    };
    let spheres = Intersect {
        a: &sphere,
        b: &translated_sphere,
    };
    let cube = UnitCube;
    let sdf = Union {
        a: &Translate {
            a: &spheres,
            v: &Vec3::new(1.0, 0.0, 0.0),
        },
        b: &cube,
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
