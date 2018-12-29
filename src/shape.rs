use crate::collision::Collidable;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Option<String>,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
            material: None,
        }
    }

    pub fn new_with_material(center: Vector, radius: f64, material: String) -> Sphere {
        Sphere {
            center,
            radius,
            material: Some(material),
        }
    }
}

impl Collidable for Sphere {
    fn check_collision(
        self: &Self,
        ray: Ray,
        tmin: f64,
        tmax: f64,
    ) -> Option<(f64, Vector, Vector, String)> {
        let to_sphere_center = ray.origin - self.center;

        let a = ray.direction.dot(ray.direction);
        let b = to_sphere_center.dot(ray.direction);
        let c = to_sphere_center.dot(to_sphere_center) - self.radius.powi(2);

        let discriminant = b * b - a * c;

        let material = match self.material.clone() {
            Some(material) => material,
            None => return None,
        };

        if discriminant > 0.0 {
            let time = (-b - discriminant.sqrt()) / a;
            if tmin < time && time < tmax {
                let point = ray.point_at_parameter(time);
                let normal = (point - self.center) / self.radius;
                return Some((time, point, normal, material));
            }
            let time = (-b + discriminant.sqrt()) / a;
            if tmin < time && time < tmax {
                let point = ray.point_at_parameter(time);
                let normal = (point - self.center) / self.radius;
                return Some((time, point, normal, material));
            }
        }
        None
    }

    fn assign_material(&mut self, material: String) {
        self.material = Some(material);
    }
}
