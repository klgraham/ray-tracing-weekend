use crate::geom::{Point3, Vector3};
use crate::material::Material;
use crate::ray::Ray;
// use std::rc::Rc;

#[derive(Debug)]
pub enum Shape {
    Sphere(Point3, f64, Point3, f64, f64, Material),
}

pub fn make_sphere(center: Point3, radius: f64, material: Material) -> Shape {
    Shape::Sphere(center, 0f64, center, 0f64, radius, material)
}

/// Sphere with center that can move linearly from center0 at time0 to center1 at time1.
pub fn make_moving_sphere(
    center0: Point3,
    time0: f64,
    center1: Point3,
    time1: f64,
    radius: f64,
    material: Material,
) -> Shape {
    Shape::Sphere(center0, time0, center1, time1, radius, material)
}

impl Shape {
    pub fn center(&self, time: f64) -> Point3 {
        match self {
            Shape::Sphere(center0, time0, center1, time1, _radius, _material) => {
                let r = (time - time0) / (time1 - time0);
                return *center0 + r * (*center1 - *center0);
            }
        }
    }
}

impl Hittable for Shape {
    fn get_material<'a>(&'a self) -> &'a Material {
        match self {
            Shape::Sphere(_center0, _time0, _center1, _time1, _radius, material) => material,
        }
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        match self {
            Shape::Sphere(center0, _time0, _center1, _time1, radius, _material) => {
                let center_at_time = self.center(r.time);
                let oc = r.origin - center_at_time;
                let a = r.direction.length_squared();
                let half_b = r.direction.dot(&oc);
                let c = oc.length_squared() - radius * radius;
                let discriminant: f64 = half_b * half_b - a * c;

                if discriminant > 0.0 {
                    let root = discriminant.sqrt();

                    let t = (-half_b - root) / a;
                    if t < t_max && t > t_min {
                        // point where ray hits sphere
                        let p = r.at(t);
                        let outward_normal: Vector3 = (p - center_at_time) / *radius;
                        let intersection = Intersection::new(&r, t, p, outward_normal, self);
                        return Some(intersection);
                    }

                    let t = (-half_b + root) / a;
                    if t < t_max && t > t_min {
                        // point where ray hits sphere
                        let p = r.at(t);
                        let outward_normal: Vector3 = (p - center_at_time) / *radius;
                        let intersection = Intersection::new(&r, t, p, outward_normal, self);
                        return Some(intersection);
                    }
                }

                None
            }
        }
    }
}

/// Records the details of a `Ray` hitting a `Hittable` shape (with
/// normal vector `normal`, made of a particular `Material`) at point p, at `t
/// `.
pub struct Intersection<'a> {
    pub t: f64,
    pub p: Point3,
    pub normal: Vector3,
    pub ray_hit_outer_surface: bool,
    /// object that is hit by a ray
    pub object: &'a Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(r: &Ray, t: f64, p: Point3, normal: Vector3, object: &'a Shape) -> Intersection<'a> {
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
    fn get_material<'a>(&'a self) -> &'a Material;
    /// Returns the interesction between a ray a shape, if there is one
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

pub struct HittableObjects {
    // The HittableObjects list will own its objects, so no lifetime
    // parameter needed
    pub objects: Vec<Shape>,
}

impl HittableObjects {
    pub fn new() -> HittableObjects {
        HittableObjects {
            objects: Vec::new(),
        }
    }

    /// Add item
    pub fn add(&mut self, object: Shape) {
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
