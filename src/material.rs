
/// Different types of material
///
/// A matrial will produce a scattered ray (or say it absorbed the incident ray).
/// If scattered, say how much the ray should be attenuated.

use crate::shapes::Intersection;
use crate::ray::Ray;
use crate::color::*;
use crate::geom::*;
use rand::prelude::*;


pub trait Material {
    fn scatter(&self, incident_ray: Ray, intersect: &Intersection) -> Option<(Ray, &Color)>;
}

/// A Lambertian (diffuse) material
#[derive(Debug)]
pub struct DiffuseNonMetal {
    pub albedo: Color
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

#[derive(Debug)]
pub struct Dielectric {
    pub index_of_refraction: f64,
    attenuation: Color
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
    fn scatter(&self, incident_ray: Ray, intersect: &Intersection) -> Option<(Ray, &Color)> {
        let scatter_direction = intersect.normal + random_unit_vector();
        let scattered_ray = Ray::new(intersect.p, scatter_direction);
        return Some((scattered_ray, &self.albedo))
    }    
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, incident_ray: Ray, intersect: &Intersection) -> Option<(Ray, &Color)> {
        let reflection = incident_ray.direction.to_unit_vector().reflect(&intersect.normal);
        let direction = reflection + self.fuzz * random_point_in_unit_sphere();
        let scattered_ray = Ray::new(intersect.p, direction);

        if scattered_ray.direction.dot(&intersect.normal) > 0.0 {
            Some((scattered_ray, &self.albedo))
        } else {
            None
        }
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric { index_of_refraction, attenuation: Colors::White.value() }
    }

    fn reflectance(cosine: f64, ref_index: f64) -> f64 {
        let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
        r0 *= r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, incident_ray: Ray, intersect: &Intersection) -> Option<(Ray, &Color)> {
        let refraction_ratio = if intersect.ray_hit_outer_surface {1.0 / self.index_of_refraction} else {self.index_of_refraction};
        let incident_direction = incident_ray.direction.to_unit_vector();

        let cos_theta = intersect.normal.dot(&-incident_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut rng = rand::thread_rng();
        let condition = cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen();

        let refracted_direction = if condition {
            incident_direction.reflect(&intersect.normal)
        } else {
            incident_direction.refract(&intersect.normal, refraction_ratio)
        };

        let scattered_ray = Ray::new(intersect.p, refracted_direction);
        Some((scattered_ray, &self.attenuation))
    }
}