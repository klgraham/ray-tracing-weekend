use crate::geom::{random_in_unit_disk, Point3, Vector3};
use crate::ray::Ray;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Camera {
    // the eye is at the center of the screen, at z = focal_length
    // This position is relative to the center of the screen. So, to get to the
    // eye from the center, you go left and down by one half and then move
    // towards the eye by focal length.
    origin: Point3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
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
        // shutter open time
        time0: f64,
        // shutter close time
        time1: f64,
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
            origin.to_vector() - 0.5 * horizontal - 0.5 * vertical - focus_dist * w;
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
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let direction = self.lower_left_corner - self.origin.to_vector() - offset
            + s * self.horizontal
            + t * self.vertical;
        let time: f64 = thread_rng().gen_range(self.time0, self.time1);
        return Ray::new_with_time(self.origin + offset, direction, time);
    }
}
