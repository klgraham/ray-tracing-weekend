use crate::geom::{Point3, Vector3};
use crate::material::Material;
use crate::ray::Ray;
use std::rc::Rc;

pub mod sphere;

/// Records the details of a `Ray` hitting a `Hittable` shape (with
/// normal vector `normal`, made of a particular `Material`) at point p, at `t
/// `.
pub struct Intersection {
    pub t: f64,
    pub p: Point3,
    pub normal: Vector3,
    pub ray_hit_outer_surface: bool,
    /// object that is hit by a ray
    pub object: Rc<dyn Hittable>,
}

impl Intersection {
    pub fn new(
        r: &Ray,
        t: f64,
        p: Point3,
        normal: Vector3,
        object: Rc<dyn Hittable>,
    ) -> Intersection {
        // which side of object did ray hit?
        let ray_hit_outer_surface = r.direction.dot(&normal) < 0.0;
        let new_normal = if ray_hit_outer_surface {
            normal
        } else {
            -normal
        };
        Intersection {
            t,
            p,
            normal: new_normal,
            ray_hit_outer_surface,
            object,
        }
    }
}

pub trait Hittable {
    fn get_material(&self) -> Material;
    /// Returns the interesction between a ray a shape, if there is one
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

pub struct HittableObjects {
    // The HittableObjects list will own its objects, so no lifetime
    // parameter needed
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableObjects {
    pub fn new() -> HittableObjects {
        HittableObjects {
            objects: Vec::new()
        }
    }

    /// Add item
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
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
                }
                None => {}
            }
        }
    
        return closest_intersection;
    }
}

// impl Hittable for HittableObjects {
// }
