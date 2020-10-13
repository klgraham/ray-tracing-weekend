use std::fs::{File};
use std::path::Path;
use std::io::{Write};
use rand::prelude::*;


mod color;
mod geom;
mod ray;
mod shapes;
mod camera;

use color::{Color, Colors};
use ray::Ray;
use geom::*;
use camera::Camera;
use shapes::{Hittable, HittableObjects};
use shapes::sphere::{Sphere};


/// The viewer's eye (the camera) will be at `(0,0,0)`. The screen will 
/// basically be an xy-plane, where the origin is in the lower left corner, 
/// the x-axis goes to the right, and the y-axis goes up. The z-axis points 
/// out of the screen. The endpoint of the ray on the screen (in the xy-plane) 
/// can be denoted with two offset vectors `u` and `v`.

fn compute_ray_color<T: Hittable>(r: Ray, world: &HittableObjects<T>, depth: i32) -> Color {    
    if depth <= 0 {
        // This gives us an end to the recursion.
        return Colors::Black.value();
    }

    let intersection = world.hit(&r, 1e-3, f64::MAX);

    match intersection {
        Some(intersect) => {
            // Each time a ray hits something, we a create a ray with random direction. 
            // This simulates the random reflection and absorption that 
            // happens when light hits a diffuse material.
            let target = intersect.p + random_in_hemisphere(&intersect.normal);
            // let target = intersect.p + intersect.normal + random_unit_vector();
            let n = intersect.normal;
            let ray = Ray::new(intersect.p, target - intersect.p);
            return 0.5 * compute_ray_color(ray, world, depth - 1);
        },
        None => {
            let ray_direction = r.direction.to_unit_vector();
            // y is [-1,1], so t is [0,1]
            let t = 0.5 * (ray_direction.y + 1.0);
            // linear interpolation between while and a light blue, based on y-component of ray
            // blendedValue = (1−t)*startValue + t * endValue
            return (1.0-t) * Colors::White.value() + t * Color::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: usize = 400;
    let height: usize = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel: u32 = 100;
    let max_depth: i32 = 50;

    // World
    let mut world: HittableObjects<Sphere> = HittableObjects::new();
    // Note that the order in which the objects are added to the list affects the 
    // order in which a ray hits things.
    world.add(Sphere::new(Point3::new(0.0,-100.5, -1.0), 100.0));
    world.add(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5));

    // Camera
    let camera = Camera::new(aspect_ratio, 2.0, 1.0);
    
    // Render    
    // Render a PPM file in P6 format, P6 format is a little simpler than P3 format
    let path = Path::new("./test_file.ppm");
    let mut file = File::create(path).expect("Failed to create file.");
    
    let header = format!("P6\n{} {}\n255\n", width, height);
    file.write(header.as_bytes()).expect("Failed to write PPM header.");

    let mut binary_pixels: Vec<u8> = Vec::with_capacity(width * height);
    let w = (width as f64) - 1.0;
    let h = (height as f64) - 1.0;
    let mut rng = rand::thread_rng();

    // Note that the height coordinate is written backwards
    for j in (0..height).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..width {
            let mut color = Colors::Black.value();

            for _ in 0..samples_per_pixel {
                let x: f64 = rng.gen();
                let u = ((i as f64) + x) / w;
                let y: f64 = rng.gen();
                let v = ((j as f64) + y) / h;
                let r = camera.get_ray(u, v);
                color += compute_ray_color(r, &world, max_depth);
            }
            let pixel = color.sample_pixel(samples_per_pixel);
            binary_pixels.push(pixel.0);
            binary_pixels.push(pixel.1);
            binary_pixels.push(pixel.2);
        }
    }
    println!("Done!");
    file.write(&binary_pixels).expect("Failed to write color map to PPM.");
}
