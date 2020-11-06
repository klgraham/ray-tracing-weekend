use pbr::ProgressBar;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod camera;
mod color;
mod geom;
mod material;
mod ray;
mod shapes;

use camera::Camera;
use color::{Color, Colors};
use geom::*;
use material::Material;
use ray::Ray;
use rayon::prelude::*;
use shapes::{make_sphere, Hittable, HittableObjects, Shape};

/// The viewer's eye (the camera) will be at `(0,0,0)`. The screen will
/// basically be an xy-plane, where the origin is in the lower left corner,
/// the x-axis goes to the right, and the y-axis goes up. The z-axis points
/// out of the screen. The endpoint of the ray on the screen (in the xy-plane)
/// can be denoted with two offset vectors `u` and `v`.

struct World {
    pub objects: HittableObjects,
}

fn make_random_scene() -> World {
    let mut objects = HittableObjects::new();

    let ground_material = Material::DiffuseNonMetal(Color::new(0.5, 0.5, 0.5));
    objects.add(make_sphere(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    ));

    let mut rng = thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let p_material: f64 = rng.gen();
            let i: f64 = rng.gen();
            let k: f64 = rng.gen();
            let x = (a as f64) + 0.9 * i;
            let z = (b as f64) + 0.9 * k;
            let center = Point3::new(x, 0.2, z);

            if (center - Point3::new(4., 0.2, 0.)).norm() > 0.9 {
                if p_material < 0.1 {
                    // dielectric => cinnabar
                    let cinnabar = Color::new(0.73, 0.27, 0.21);
                    let sphere_material = Material::Dielectric(3.02, cinnabar);
                    objects.add(make_sphere(center, 0.1, sphere_material));
                } else if p_material < 0.2 {
                    // dielectric => diamond
                    let diamond = Color::new(0.78, 0.88, 0.91);
                    let sphere_material = Material::Dielectric(3.02, diamond);
                    objects.add(make_sphere(center, 0.1, sphere_material));
                } else if p_material < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Material::DiffuseNonMetal(albedo);
                    objects.add(make_sphere(center, 0.2, sphere_material));
                } else if p_material < 0.95 {
                    // metal
                    let albedo = Color::random();
                    let fuzz: f64 = rng.gen_range(0., 0.5);
                    let sphere_material = Material::Metal(albedo, fuzz);
                    objects.add(make_sphere(center, 0.2, sphere_material));
                } else {
                    // dielectric
                    let sphere_material = Material::Dielectric(1.5, Colors::White.value());
                    objects.add(make_sphere(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Material::Dielectric(1.5, Colors::White.value());
    objects.add(make_sphere(Point3::new(0., 1., 0.), 1., material1));

    let albedo = Color::new(0.4, 0.2, 0.1);
    let material2 = Material::DiffuseNonMetal(albedo);
    objects.add(make_sphere(Point3::new(-4., 1., 0.), 1., material2));

    let material3 = Material::Metal(Color::new(0.7, 0.6, 0.5), 0.);
    objects.add(make_sphere(Point3::new(4., 1., 0.), 1., material3));

    return World { objects };
}

fn compute_ray_color(r: Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        // This gives us an end to the recursion.
        return Colors::Black.value();
    }

    let intersection = world.objects.hit(&r, 1e-3, f64::MAX);

    match intersection {
        Some(intersect) => {
            let intersection_material = intersect.object.get_material();
            let ray_and_color = intersection_material.scatter(r, &intersect);

            match ray_and_color {
                Some((scattered_ray, attenuation)) => {
                    return attenuation.mult(compute_ray_color(scattered_ray, world, depth - 1));
                }
                None => return Colors::Black.value(),
            }
        }
        None => {
            let ray_direction = r.direction.to_unit_vector();
            // y is [-1,1], so t is [0,1]
            let t = 0.5 * (ray_direction.y + 1.0);
            // linear interpolation between while and a light blue, based on y-component of ray
            // blendedValue = (1−t)*startValue + t * endValue
            return (1.0 - t) * Colors::White.value() + t * Color::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let height: usize = 1080;
    let width: usize = ((height as f64) * aspect_ratio) as usize;
    let samples_per_pixel: usize = 500;
    let max_depth: i32 = 50;

    // World
    let world = make_random_scene();

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
        aspect_ratio,
        aperture,
        dist_to_focus,
        0f64,
        0f64,
    );

    // Render
    // Render a PPM file in P6 format, P6 format is a little simpler than P3 format
    let filename = format!("scene_{}p.ppm", height);
    let path = Path::new(&filename);
    let mut file = File::create(path).expect("Failed to create file.");

    let header = format!("P6\n{} {}\n255\n", width, height);
    file.write(header.as_bytes())
        .expect("Failed to write PPM header.");

    let mut binary_pixels: Vec<u8> = Vec::with_capacity(width * height);
    let w = (width as f64) - 1.0;
    let h = (height as f64) - 1.0;

    let mut progress_bar = ProgressBar::new(height as u64);

    // Note that the height coordinate is written backwards
    // Should be able to parallelize the i and j loops. The sampling loop can't be though.
    for j in (0..height).rev() {
        for i in 0..width {
            let samples: Vec<usize> = (0..samples_per_pixel).collect();
            let color = samples
                .par_iter()
                .map(|_| sample_pixel(i, j, &camera, &world, max_depth, w, h))
                .collect::<Vec<Color>>()
                .iter()
                .sum::<Color>();

            let pixel = color.sample_pixel(samples_per_pixel as u32);
            binary_pixels.push(pixel.0);
            binary_pixels.push(pixel.1);
            binary_pixels.push(pixel.2);
        }
        progress_bar.inc();
    }
    progress_bar.finish_print("Done.");
    file.write(&binary_pixels)
        .expect("Failed to write color map to PPM.");
}

fn sample_pixel(
    i: usize,
    j: usize,
    camera: &Camera,
    world: &World,
    max_depth: i32,
    w: f64,
    h: f64,
) -> Color {
    let x = thread_rng().gen::<f64>();
    let u = ((i as f64) + x) / w;
    let y = thread_rng().gen::<f64>();
    let v = ((j as f64) + y) / h;
    let r = camera.get_ray(u, v);
    return compute_ray_color(r, world, max_depth);
}
