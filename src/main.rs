use std::fs::File;
use std::io::Write;
use std::path::Path;

mod canvas;
mod camera;
mod color;
mod geom;
mod material;
mod ray;
mod shapes;

use camera::{Camera, RenderConfig};
use canvas::{ASPECT_RATIO, Resolution};
use geom::*;

use shapes::make_random_scene;

/// The viewer's eye (the camera) will be at `(0,0,0)`. The screen will
/// basically be an xy-plane, where the origin is in the lower left corner,
/// the x-axis goes to the right, and the y-axis goes up. The z-axis points
/// out of the screen. The endpoint of the ray on the screen (in the xy-plane)
/// can be denoted with two offset vectors `u` and `v`.

fn main() {
    let resolution = Resolution::_240p;
    let samples_per_pixel: usize = 500;
    let max_depth: i32 = 50;
    let render_config = RenderConfig::new(resolution, samples_per_pixel, max_depth);

    let objects = make_random_scene();

    // Camera
    let look_from = Point3::new(13., 2., 3.);
    let look_at = Point3::origin();
    let view_up = Vector3::new(0., 1., 0.);
    let dist_to_focus = 10_f64;
    let aperture = 0.1_f64;

    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render

    let binary_pixels = camera.render(&objects, render_config);

    // Write pixels to PPM file in P6 format, P6 format is a little simpler than P3 format
    let filename = format!("scene_{}p.ppm", render_config.height);
    let path = Path::new(&filename);
    let mut file = File::create(path).expect("Failed to create file.");

    let header = format!("P6\n{} {}\n255\n", render_config.width, render_config.height);
    file.write_all(header.as_bytes())
        .expect("Failed to write PPM header.");

    file.write_all(&binary_pixels)
        .expect("Failed to write color map to PPM.");
}


