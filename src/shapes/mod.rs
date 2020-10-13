use crate::ray::Ray;
use crate::geom::{Point3, Vector3};

pub mod sphere;


pub struct Intersection {
    pub t: f64,
    pub p: Point3,
    pub normal: Vector3,
    // pub ray_hit_outer_surface: bool
}

impl Intersection {
    pub fn new(r: &Ray, t: f64, p: Point3, normal: Vector3) -> Intersection {
        // which side of object did ray hit?
        let ray_hit_outer_surface = r.direction.dot(normal) < 0.0;
        let new_normal = if ray_hit_outer_surface {normal} else {-normal};
        Intersection { t, p, normal: new_normal }
    }
}

pub trait Hittable {
    /// Returns the interesction between a ray a shape, if there is one
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}


pub struct HittableObjects<T: Hittable> {
    // The HittableObjects list will own its objects, so no lifetime 
    // parameter needed
    pub objects: Vec<T> 
}

impl<T: Hittable> HittableObjects<T> {
    pub fn new() -> HittableObjects<T> {
        HittableObjects { objects: Vec::new() }
    }

    /// Add item
    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

}

impl<T: Hittable> Hittable for HittableObjects<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut closest_intersection: Option<Intersection> = None;
        let mut closest_hit = t_max;

        for object in self.objects.iter() {
            let intersection = object.hit(r, t_min, t_max);
            match intersection {
                Some(intersect) => {
                    if intersect.t < closest_hit {
                        closest_hit = intersect.t;
                        closest_intersection = Some(intersect);
                    }
                },
                None => {}
            }
        }

        return closest_intersection;
    }
}