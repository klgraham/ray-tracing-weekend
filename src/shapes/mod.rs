use crate::color::Color;
use crate::geom::{Point3, Vector3};
use crate::material::{select_material, Material};
use crate::ray::Ray;
use rand::prelude::*;

pub const INFINITY: f64 = f64::INFINITY;

/// Determines degree of membership in a real-valued
/// interval with a minimum and a maximum
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

pub const EMPTY_INTERVAL: Interval = Interval{min: INFINITY, max:-INFINITY};
pub const UNIVERSE: Interval = Interval{min: -INFINITY, max: INFINITY};

pub trait Hittable {
    fn get_material(&self) -> &Material;
    /// Computes the intersection between a ray and a shape at t
    fn compute_intersection(&self, r: &Ray, t: f64) -> Intersection;
    /// Returns the intersection between a ray and a shape, if there is one
    fn hit(&self, r: &Ray, interval: Interval) -> Option<Intersection>;
}

/// Shape structs

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

// #[derive(Debug, Copy, Clone)]
// pub struct Triangle {
//     vertices: (Point3, Point3, Point3),
//     material: Material,
// }

/// `Shape` represents a geometric shape in the scene which can be hit by rays.
/// Using an enum gives us a Shape type without needing generics, which would
/// make a list of Hittable objects need dyn

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Sphere(Sphere),
    // Triangle(Triangle),
}

/// Records the details of a `Ray` hitting a `Hittable` shape (with
/// normal vector `normal`, made of a particular `Material`) at point p, at `t`
#[derive(Debug)]
pub struct Intersection<'a> {
    /// time point when interection occurs
    pub t: f64,
    /// point where ray hits shape
    pub p: Point3,
    pub normal: Vector3,
    pub ray_hit_outer_surface: bool,
    /// object that is hit by a ray
    pub material: &'a Material, // TODO: replace with material, since that's all we need for now?
}

/// Shape struct impls

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn get_material(&self) -> &Material {
        &self.material
    }

    fn compute_intersection(&self, r: &Ray, t: f64) -> Intersection {
        let intersection_point = r.at(t);
        let normal: Vector3 = (intersection_point - self.center) / self.radius;
        Intersection::new(r, t, intersection_point, normal, self.get_material())
    }

    fn hit(&self, r: &Ray, interval: Interval) -> Option<Intersection> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = r.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;
            if interval.surrounds(t) {
                return Some(self.compute_intersection(r, t));
            }
            let t = (-half_b + root) / a;
            if interval.surrounds(t) {
                return Some(self.compute_intersection(r, t));
            }
        }

        None
    }
}

impl Hittable for Shape {
    fn get_material(&self) -> &Material {
        match self {
            Shape::Sphere(sphere) => sphere.get_material(),
        }
    }

    fn compute_intersection(&self, r: &Ray, t: f64) -> Intersection {
        match self {
            Shape::Sphere(sphere) => sphere.compute_intersection(r, t),
        }
    }

    fn hit(&self, r: &Ray, interval: Interval) -> Option<Intersection> {
        match self {
            Shape::Sphere(sphere) => sphere.hit(r, interval),
        }
    }
}

impl<'a> Intersection<'a> {
    pub fn new(
        r: &Ray,
        t: f64,
        p: Point3,
        normal: Vector3,
        material: &'a Material,
    ) -> Intersection<'a> {
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
            material,
        }
    }
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

    pub fn hit(&self, r: &Ray, interval: Interval) -> Option<Intersection> {
        let mut closest_intersection: Option<Intersection> = None;
        let mut closest_hit = interval.max;

        for object in self.objects.iter() {
            let reduced_interval = Interval::new(interval.min, closest_hit);
            let maybe_intersection = object.hit(r, reduced_interval);
            if let Some(intersection) = maybe_intersection {
                if intersection.t < closest_hit {
                    closest_hit = intersection.t;
                    closest_intersection = Some(intersection);
                }
            }
        }
        closest_intersection
    }

    pub fn compute_ray_color(&self, r: Ray, depth: i32) -> Color {
        if depth <= 0 {
            // This gives us an end to the recursion.
            return Color::BLACK;
        }

        let intersection = self.hit(&r, Interval::new(0_f64, INFINITY));

        match intersection {
            Some(intersect) => {
                let intersection_material = intersect.material;
                let ray_and_color = intersection_material.scatter(r, &intersect);

                match ray_and_color {
                    Some((scattered_ray, attenuation)) => {
                        attenuation.mult(self.compute_ray_color(scattered_ray, depth - 1))
                    }
                    None => Color::BLACK,
                }
            }
            None => {
                let ray_direction = r.direction.to_unit_vector();
                // y is [-1,1], so t is [0,1]
                let t = 0.5 * (ray_direction.y + 1.0);
                // linear interpolation between while and a light blue, based on y-component of ray
                // blendedValue = (1−t)*startValue + t * endValue
                (1.0 - t) * Color::WHITE + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}

pub fn make_random_scene<'a>() -> HittableObjects {
    let mut objects = HittableObjects::new();

    let ground_material = Material::DiffuseNonMetal(Color::new(0.5, 0.5, 0.5));
    let mut sphere = Sphere::new(Point3::new(0., -1000., 0.), 1000., ground_material);
    objects.add(Shape::Sphere(sphere));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let p_material: f64 = rng.gen();
            let i: f64 = rng.gen();
            let k: f64 = rng.gen();
            let x = (a as f64) + 0.9 * i;
            let z = (b as f64) + 0.9 * k;
            let center = Point3::new(x, 0.2, z);

            if (center - Point3::new(4., 0.2, 0.)).norm() > 0.9 {
                let sphere_material = select_material(p_material, &mut rng);
                let sphere = Sphere::new(center, 0.2, sphere_material);
                objects.add(Shape::Sphere(sphere));
            }
        }
    }

    let material1 = Material::Dielectric(1.5, Color::WHITE);
    sphere = Sphere::new(Point3::new(0., 1., 0.), 1., material1);
    objects.add(Shape::Sphere(sphere));

    let albedo = Color::new(0.4, 0.2, 0.1);
    let material2 = Material::DiffuseNonMetal(albedo);
    sphere = Sphere::new(Point3::new(-4., 1., 0.), 1., material2);
    objects.add(Shape::Sphere(sphere));

    let material3 = Material::Metal(Color::new(0.7, 0.6, 0.5), 0.);
    sphere = Sphere::new(Point3::new(4., 1., 0.), 1., material3);
    objects.add(Shape::Sphere(sphere));

    objects
}
