use crate::color::*;
use crate::geom::*;
use crate::ray::Ray;
use crate::shapes::Intersection;
use rand::prelude::*;
use rand::rngs::ThreadRng;

/// Different types of material
///
/// A material will produce a scattered ray (or say it absorbed the incident ray).
/// If scattered, say how much the ray should be attenuated.

#[derive(Debug, Clone, Copy)]
pub enum Material {
    DiffuseNonMetal(Color),
    Metal(Color, f64),
    // get known refractive indices from https://en.wikipedia.org/wiki/List_of_refractive_indices
    Dielectric(f64, Color),
}

fn dielectric_reflectance(cosine: f64, ref_index: f64) -> f64 {
    let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material {
    pub fn scatter(&self, incident_ray: Ray, intersect: &Intersection) -> Option<(Ray, &Color)> {
        match self {
            Material::DiffuseNonMetal(albedo) => {
                let scatter_direction = intersect.normal + random_unit_vector();
                let scattered_ray = Ray::new(intersect.p, scatter_direction);
                Some((scattered_ray, albedo))
            }

            Material::Metal(albedo, fuzz) => {
                let reflection = incident_ray
                    .direction
                    .to_unit_vector()
                    .reflect(&intersect.normal);
                let direction = reflection + (*fuzz) * random_point_in_unit_sphere();
                let scattered_ray = Ray::new(intersect.p, direction);

                if scattered_ray.direction.dot(&intersect.normal) > 0.0 {
                    Some((scattered_ray, albedo))
                } else {
                    None
                }
            }

            Material::Dielectric(index_of_refraction, attenuation) => {
                let refraction_ratio = if intersect.ray_hit_outer_surface {
                    1.0 / index_of_refraction
                } else {
                    *index_of_refraction
                };

                let incident_direction = incident_ray.direction.to_unit_vector();

                let cos_theta = intersect.normal.dot(&-incident_direction).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let mut rng = rand::thread_rng();
                let condition = cannot_refract
                    || dielectric_reflectance(cos_theta, refraction_ratio) > rng.gen();

                let refracted_direction = if condition {
                    incident_direction.reflect(&intersect.normal)
                } else {
                    incident_direction.refract(&intersect.normal, refraction_ratio)
                };

                let scattered_ray = Ray::new(intersect.p, refracted_direction);
                Some((scattered_ray, attenuation))
            }
        }
    }
}

/// Selects a material based on the provided probability and random number generator.
///
/// # Arguments
///
/// * `p_material` - A float representing the probability of selecting a particular material.
/// * `rng` - A mutable reference to a ThreadRng instance for generating random numbers.
///
/// # Returns
///
/// * `Material` - The selected material.
///
/// # Example
///
/// ```
/// let mut rng = rand::thread_rng();
/// let p_material: f64 = rng.gen();
/// let material = select_material(p_material, &mut rng);
/// ```
pub fn select_material(p_material: f64, rng: &mut ThreadRng) -> Material {
    if p_material < 0.1 {
        // dielectric => cinnabar
        Material::Dielectric(3.02, Color::CINNABAR)
    } else if p_material < 0.2 {
        // dielectric => diamond
        Material::Dielectric(3.02, Color::DIAMOND)
    } else if p_material < 0.8 {
        // diffuse non-metal
        Material::DiffuseNonMetal(Color::diffuse_albedo())
    } else if p_material < 0.95 {
        // metal
        let fuzz: f64 = rng.gen_range(0. ..0.5);
        Material::Metal(Color::metal_albedo(), fuzz)
    } else {
        // dielectric
        Material::Dielectric(1.5, Color::WHITE)
    }
}
