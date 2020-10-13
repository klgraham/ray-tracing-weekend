use crate::{Point3, Vector3};
use crate::shapes::{Hittable, Intersection};
use crate::ray::Ray;
use rand::prelude::*;


/// sphere of radius `radius`, with center at `center` 
#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center: center, radius }
    }
    
    pub fn unit_sphere_at_origin() -> Sphere {
        Sphere::new(Point3::origin(), 1.)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        // origin - center
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a*c;
        
        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let t = (-half_b - root) / a;            
            if t < t_max && t > t_min {
                // point where ray hits sphere
                let p = r.at(t);
                let normal: Vector3 = (p - self.center) / self.radius;
                let intersection = Intersection::new(&r, t, p, normal);
                return Some(intersection);
            }

            let t = (-half_b + root) / a;            
            if t < t_max && t > t_min {
                // point where ray hits sphere
                let p = r.at(t);
                let normal: Vector3 = (p - self.center) / self.radius;
                let intersection = Intersection::new(&r, t, p, normal);
                return Some(intersection);
            }
        }

        None
    }
}

