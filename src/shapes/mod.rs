use crate::ray::Ray;
use crate::geom::{Point3, Vector3};


pub mod sphere;


pub struct Intersection {
    pub t: f64,
    pub p: Point3,
    pub normal: Vector3
}

impl Intersection {
    pub fn new(t: f64, p: Point3, normal: Vector3) -> Intersection {
        Intersection { t, p, normal }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}