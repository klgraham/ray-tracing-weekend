use crate::geom::{Point3, Vector3};

// ray.rs

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3
}

impl Ray {
    /// Creates a ray at `origin` along direction `direction`
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray {origin, direction}
    }

    /// Gives the ray at `t`.
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}


#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::geom::{Point3, Vector3};


    #[test]
    fn can_create_rays() {
        let origin = Point3::new(1., 2., 3.);
        let direction = Vector3::new(4., 5., 6.);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn can_compute_ray_position_in_time() {
        let p = Point3::new(2., 3., 4.);
        let v = Vector3::new(1., 0., 0.);
        let r = Ray::new(p, v);
        assert_eq!(r.at(1.0), Point3::new(3., 3., 4.));
        assert_eq!(r.at(-1.0), Point3::new(1., 3., 4.));
        assert_eq!(r.at(2.5), Point3::new(4.5, 3., 4.));
    }
}