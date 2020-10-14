
/// Different types of material
///
/// A matrial will produce a scattered ray (or say it absorbed the incident ray).
/// If scattered, say how much the ray should be attenuated.

use crate::shapes::Intersection;
use crate::ray::Ray;
use crate::color::*;
use crate::geom::*;


pub trait Material {
    fn scatter(&self, incident_ray: Ray, intersect: &Intersection) -> Option<(Ray, Color)>;
}

/// A Lambertian (diffuse) material
#[derive(Debug, Clone, Copy)]
pub struct DiffuseNonMetal {
    pub albedo: Color
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub index_of_refraction: f64
}

impl DiffuseNonMetal {
    pub fn new(albedo: Color) -> DiffuseNonMetal {
        DiffuseNonMetal { albedo }
    }
}

impl Material for DiffuseNonMetal {
    // Each time a ray hits an object made of a diffuse material, we a create 
    // a ray with random direction. This simulates the random reflection and 
    // absorption that happens when light hits a diffuse material.
    fn scatter(&self, incident_ray: Ray, intersect: &Intersection) -> Option<(Ray, Color)> {
        let scatter_direction = intersect.normal + random_unit_vector();
        let scattered_ray = Ray::new(intersect.p, scatter_direction);
        return Some((scattered_ray, self.albedo))
    }    
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, incident_ray: Ray, intersect: &Intersection) -> Option<(Ray, Color)> {
        let reflection = incident_ray.direction.to_unit_vector().reflect(intersect.normal);
        let direction = reflection + self.fuzz * random_point_in_unit_sphere();
        let scattered_ray = Ray::new(intersect.p, direction);

        if scattered_ray.direction.dot(intersect.normal) > 0.0 {
            Some((scattered_ray, self.albedo))
        } else {
            None
        }
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric { index_of_refraction }
    }
}

impl Material for Dielectric {
    fn scatter(&self, incident_ray: Ray, intersect: &Intersection) -> Option<(Ray, Color)> {
        let attenuation = Colors::White.value();
        let refraction_ratio = if intersect.ray_hit_outer_surface {1.0 / self.index_of_refraction} else {self.index_of_refraction};
        let incident_direction = incident_ray.direction.to_unit_vector();
        let refracted_direction = incident_direction.refract(intersect.normal, refraction_ratio);
        let scattered_ray = Ray::new(intersect.p, refracted_direction);
        Some((scattered_ray, attenuation))
    }
}