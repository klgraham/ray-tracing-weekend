use std::fs::{File};
use std::path::Path;
use std::io::{Write};

mod color;
mod geom;
mod ray;
use color::{Color, Colors};
use ray::Ray;
use geom::{Point3, Vector3};

/// The viewer's eye (the camera) will be at `(0,0,0)`. The screen will 
/// basically be an xy-plane, where the origin is in the lower left corner, 
/// the x-axis goes to the right, and the y-axis goes up. The z-axis points 
/// out of the screen. The endpoint of the ray on the screen (in the xy-plane) 
/// can be denoted with two offset vectors `u` and `v`.

fn color_ray(r: Ray) -> Color {
    let ray_direction = r.direction.to_unit_vector();
    // y is [-1,1], so t is [0,1]
    let t = 0.5 * (ray_direction.y + 1.0);
    // linear interpolation between while and blue, based on y-component of ray
    // blendedValue = (1−t)*startValue + t * endValue
    return (1.0-t) * Colors::White.value() + t * Colors::Blue.value();
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: usize = 400;
    let height: usize = (width as f64 / aspect_ratio) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::origin();
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    // the eye is at the center of the screen, at z = focal_length
    // This position is relative to the center of the screen. So, to get to the 
    // eye from the center, you go left and down by one half and then move 
    // towards the eye by focal length. 
    let lower_left_corner = origin - 0.5 * horizontal - 0.5 * vertical - 
        Vector3::new(0.0, 0.0, focal_length);
    
    // Render a PPM file in P6 format, P6 format is a little simpler than P3 format
    let path = Path::new("./test_file.ppm");
    let mut file = File::create(path).expect("Failed to create file.");
    
    let header = format!("P6\n{} {}\n255\n", width, height);
    file.write(header.as_bytes()).expect("Failed to write PPM header.");

    let mut binary_pixels: Vec<u8> = Vec::with_capacity(width * height);
    let w = (width as f64) - 1.0;
    let h = (height as f64) - 1.0;

    // Note that the height coordinate is written backwards
    for j in (0..height).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..width {
            let u = (i as f64) / w;
            let v = (j as f64) / h;
            let direction = lower_left_corner - origin + u * horizontal + v * vertical;
            let r = Ray::new(origin, direction);
            let pixel = color_ray(r).to_pixel();
            binary_pixels.push(pixel.0);
            binary_pixels.push(pixel.1);
            binary_pixels.push(pixel.2);
        }
    }
    println!("Done!");
    file.write(&binary_pixels).expect("Failed to write color map to PPM.");
}
