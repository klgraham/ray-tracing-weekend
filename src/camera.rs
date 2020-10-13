use crate::geom::{Point3, Vector3};
use crate::ray::Ray;

pub struct Camera {
    // the eye is at the center of the screen, at z = focal_length
    // This position is relative to the center of the screen. So, to get to the 
    // eye from the center, you go left and down by one half and then move 
    // towards the eye by focal length. 
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    origin: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Vector3    
}


impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;        
        let origin = Point3::origin();
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = Vector3::new(origin.x, origin.y, origin.z) - 0.5 * horizontal - 0.5 * vertical - Vector3::new(0.0, 0.0, focal_length);

        Camera {
            aspect_ratio, viewport_height, viewport_width, 
            focal_length, origin, horizontal, vertical, lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical;
        Ray::new(self.origin, direction)
    }
}