use crate::camera::random_in_unit_sphere;
use crate::collision::Collision;
use crate::ray::Ray;
use crate::vector::Vector;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::marker::Sync;

pub trait Material: Sync {
    fn name(&self) -> String;
    fn scatter(
        self: &Self,
        ray: Ray,
        collision: Collision,
        rng: &mut SmallRng,
    ) -> Option<MaterialEffect>;
}

#[derive(Copy, Clone, Debug)]
pub struct MaterialEffect {
    pub scatter: Ray,
    pub attenuation: Vector,
}

fn reflect(light: Vector, normal: Vector) -> Vector {
    light - (normal * light.dot(normal) * 2.0)
}

fn refract(light: Vector, normal: Vector, refraction_index: f64) -> Option<Vector> {
    let uv = light.to_unit();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - refraction_index.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        return Some(((light - normal * dt) * refraction_index) - normal * discriminant.sqrt());
    }
    None
}

fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r1 = r0.powi(2);
    r1 + (1.0 - r1) * (1.0 - cosine).powi(5)
}

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    albedo: Vector,
}

impl Lambertian {
    pub fn new(albedo: Vector) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn name(&self) -> String {
        "lamb".to_string()
    }
    fn scatter(
        self: &Self,
        _ray: Ray,
        collision: Collision,
        rng: &mut SmallRng,
    ) -> Option<MaterialEffect> {
        let target = collision.point + collision.normal + random_in_unit_sphere(rng);
        Some(MaterialEffect {
            scatter: Ray::new(collision.point, target - collision.point),
            attenuation: self.albedo,
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Metal {
    albedo: Vector,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector, fuzz: f64) -> Metal {
        let fuzz = if fuzz > 1.0 { 1.0 } else { fuzz };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn name(&self) -> String {
        "metal".to_string()
    }
    fn scatter(
        self: &Self,
        ray: Ray,
        collision: Collision,
        rng: &mut SmallRng,
    ) -> Option<MaterialEffect> {
        let reflected = reflect(ray.direction.to_unit(), collision.normal);
        let scatter = Ray::new(
            collision.point,
            reflected + random_in_unit_sphere(rng) * self.fuzz,
        );
        if scatter.direction.dot(collision.normal) > 0.0 {
            Some(MaterialEffect {
                scatter,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Dialectic {
    refraction_index: f64,
}

impl Dialectic {
    pub fn new(refraction_index: f64) -> Dialectic {
        Dialectic { refraction_index }
    }
}

impl Material for Dialectic {
    fn name(&self) -> String {
        "dial".to_string()
    }
    fn scatter(
        self: &Self,
        ray: Ray,
        collision: Collision,
        rng: &mut SmallRng,
    ) -> Option<MaterialEffect> {
        let outward_normal;
        let refraction_index;
        let cosine;
        let reflect_probability;
        let mut scatter_direction;

        if ray.direction.dot(collision.normal) > 0.0 {
            outward_normal = collision.normal * -1.0;
            refraction_index = self.refraction_index;
            cosine =
                refraction_index * ray.direction.dot(collision.normal) / ray.direction.length();
        } else {
            outward_normal = collision.normal;
            refraction_index = 1.0 / self.refraction_index;
            cosine = ray.direction.dot(collision.normal) / ray.direction.length() * -1.0;
        }

        if let Some(refraction) = refract(ray.direction, outward_normal, refraction_index) {
            reflect_probability = schlick(cosine, refraction_index);
            scatter_direction = refraction;
        } else {
            reflect_probability = 1.0;
            scatter_direction = reflect(ray.direction, collision.normal);
        }

        if rng.gen::<f64>() > reflect_probability {
            scatter_direction = reflect(ray.direction, collision.normal);
        }
        Some(MaterialEffect {
            scatter: Ray::new(collision.point, scatter_direction),
            attenuation: Vector::unit(),
        })
    }
}
