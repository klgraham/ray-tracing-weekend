use rand::prelude::*;
use rand::rngs::ThreadRng;
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
use color::Color;
use geom::*;
use material::Material;
use ray::Ray;

use shapes::{HittableObjects, Shape, Sphere};

/// The viewer's eye (the camera) will be at `(0,0,0)`. The screen will
/// basically be an xy-plane, where the origin is in the lower left corner,
/// the x-axis goes to the right, and the y-axis goes up. The z-axis points
/// out of the screen. The endpoint of the ray on the screen (in the xy-plane)
/// can be denoted with two offset vectors `u` and `v`.


/// Selects a material based on the provided probability and random number generator.
///
/// # Arguments
///
/// * `p_material` - A float representing the probability of selecting a particular material.
/// * `rng` - A mutable reference to a ThreadRng instance for generating random numbers.
///
/// # Returns
///
/// * `Material` - The selected material.
///
/// # Example
///
/// ```
/// let mut rng = rand::thread_rng();
/// let p_material: f64 = rng.gen();
/// let material = select_material(p_material, &mut rng);
/// ```
fn select_material(p_material: f64, rng: &mut ThreadRng) -> Material {
    if p_material < 0.1 {
        // dielectric => cinnabar
        Material::Dielectric(3.02, Color::CINNABAR)
    } else if p_material < 0.2 {
        // dielectric => diamond
        Material::Dielectric(3.02, Color::DIAMOND)
    } else if p_material < 0.8 {
        // diffuse non-metal
        Material::DiffuseNonMetal(Color::diffuse_albedo())
    } else if p_material < 0.95 {
        // metal
        let fuzz: f64 = rng.gen_range(0. ..0.5);
        Material::Metal(Color::metal_albedo(), fuzz)
    } else {
        // dielectric
        Material::Dielectric(1.5, Color::WHITE)
    }
}

fn make_random_scene<'a>() -> HittableObjects {
    let mut objects = HittableObjects::new();

    let ground_material = Material::DiffuseNonMetal(Color::new(0.5, 0.5, 0.5));
    let mut sphere = Sphere::new(Point3::new(0., -1000., 0.), 1000., ground_material);
    objects.add(Shape::Sphere(sphere));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let p_material: f64 = rng.gen();
            let i: f64 = rng.gen();
            let k: f64 = rng.gen();
            let x = (a as f64) + 0.9 * i;
            let z = (b as f64) + 0.9 * k;
            let center = Point3::new(x, 0.2, z);

            if (center - Point3::new(4., 0.2, 0.)).norm() > 0.9 {
                let sphere_material = select_material(p_material, &mut rng);
                let sphere = Sphere::new(center, 0.2, sphere_material);
                objects.add(Shape::Sphere(sphere));
            }
        }
    }

    let material1 = Material::Dielectric(1.5, Color::WHITE);
    sphere = Sphere::new(Point3::new(0., 1., 0.), 1., material1);
    objects.add(Shape::Sphere(sphere));

    let albedo = Color::new(0.4, 0.2, 0.1);
    let material2 = Material::DiffuseNonMetal(albedo);
    sphere = Sphere::new(Point3::new(-4., 1., 0.), 1., material2);
    objects.add(Shape::Sphere(sphere));

    let material3 = Material::Metal(Color::new(0.7, 0.6, 0.5), 0.);
    sphere = Sphere::new(Point3::new(4., 1., 0.), 1., material3);
    objects.add(Shape::Sphere(sphere));

    objects
}


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


