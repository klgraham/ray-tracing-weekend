use crate::canvas::{CanvasConfig, Resolution};
use crate::geom::{random_in_unit_disk, Point3, Vector3};
use crate::ray::Ray;
use crate::shapes::HittableObjects;
use crate::color::Color;

use pbr::ProgressBar;
use rand::prelude::*;
use rayon::prelude::*;


#[derive(Copy, Clone, Debug)]
pub struct RenderConfig {
    pub height: usize,
    pub width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: i32,
}

impl RenderConfig {
    pub fn new(resolution: Resolution, samples_per_pixel: usize, max_depth: i32) -> Self {
        let canvas_config = CanvasConfig { resolution };
        RenderConfig {
            height: canvas_config.height(),
            width: canvas_config.width(),
            samples_per_pixel,
            max_depth,
        }
    }
}


#[derive(Debug)]
pub struct Camera {
    // the eye is at the center of the screen, at z = focal_length
    // This position is relative to the center of the screen. So, to get to the
    // eye from the center, you go left and down by one half and then move
    // towards the eye by focal length.
    /// vertical field-of-view, in degrees
    // vertical_fov: f64,
    // aspect_ratio: f64,
    // aperture: f64,
    // focus_dist: f64,
    // theta: f64,
    // h: f64,
    // viewport_height: f64,
    // viewport_width: f64,
    // focal_length: f64,
    origin: Point3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f64,
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

fn sample_pixel(
    i: usize,
    j: usize,
    camera: &Camera,
    objects: &HittableObjects,
    max_depth: i32,
    w: f64,
    h: f64,
) -> Color {
    let x = rand::thread_rng().gen::<f64>();
    let u = ((i as f64) + x) / w;
    let y = rand::thread_rng().gen::<f64>();
    let v = ((j as f64) + y) / h;
    let r = camera.get_ray(u, v);
    objects.compute_ray_color(r, max_depth)
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        view_up: Vector3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).to_unit_vector();
        let u = view_up.cross(&w).to_unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        let lower_left_corner =
            origin.as_vector() - 0.5 * horizontal - 0.5 * vertical - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let direction = self.lower_left_corner - self.origin.as_vector() - offset
            + s * self.horizontal
            + t * self.vertical;
        Ray::new(self.origin + offset, direction)
    }

    pub fn render(&self, objects: &HittableObjects, render_config: RenderConfig) -> Vec<u8> {
        let width = render_config.width;
        let height = render_config.height;
        let samples_per_pixel = render_config.samples_per_pixel;
        let max_depth = render_config.max_depth;
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
                    .map(|_| sample_pixel(i, j, self, objects, max_depth, w, h))
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
        binary_pixels
    }
}
