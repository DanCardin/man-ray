use crate::collision::Collidable;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere {
    center: Vector,
    radius: f64,
    material_name: Option<String>,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
            material_name: None,
        }
    }

    pub fn with_material<S>(mut self, material_name: S) -> Self
    where
        S: Into<String>,
    {
        self.material_name = Some(material_name.into());
        self
    }
}

impl Collidable for Sphere {
    fn check_collision(
        self: &Self,
        ray: &Ray,
        min_distance: f64,
        max_distance: f64,
    ) -> Option<f64> {
        let to_sphere_center = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = to_sphere_center.dot(&ray.direction);
        let c = to_sphere_center.dot(&to_sphere_center) - self.radius.powi(2);

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let distance = (-b - discriminant.sqrt()) / a;
            if min_distance < distance && distance < max_distance {
                return Some(distance);
            }
            let distance = (-b + discriminant.sqrt()) / a;
            if min_distance < distance && distance < max_distance {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(self: &Self, collision_point: &Vector) -> Vector {
        (*collision_point - self.center).to_unit()
    }

    fn get_material_name(self: &Self) -> Option<&str> {
        self.material_name.as_ref().map(|s| s.as_ref())
    }
}
