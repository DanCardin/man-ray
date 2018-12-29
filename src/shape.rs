use crate::collision::{Collidable, Collision};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Collidable for Sphere {
    fn check_collision(self: &Self, ray: Ray, tmin: f64, tmax: f64) -> Option<Collision> {
        let offset = ray.origin - self.center;

        let a = ray.direction.dot(ray.direction);
        let b = offset.dot(ray.direction);
        let c = offset.dot(offset) - self.radius.powi(2);

        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let time = (-b - discriminant.sqrt()) / a;
            if tmin < time && time < tmax {
                let point = ray.point_at_parameter(time);
                let normal = (point - self.center) / self.radius;
                return Some(Collision::new(time, point, normal, &*self.material));
            }
            let time = (-b + discriminant.sqrt()) / a;
            if tmin < time && time < tmax {
                let point = ray.point_at_parameter(time);
                let normal = (point - self.center) / self.radius;
                return Some(Collision::new(time, point, normal, &*self.material));
            }
        }
        None
    }
}
