use crate::collision::Collidable;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Plane {
    pub point: Vector,
    pub normal: Vector,
    pub material_name: Option<String>,
}

impl Plane {
    pub fn new(point: Vector, normal: Vector) -> Plane {
        Plane {
            point,
            normal,
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

impl Collidable for Plane {
    fn check_collision(
        self: &Self,
        ray: &Ray,
        min_distance: f64,
        max_distance: f64,
    ) -> Option<f64> {
        let denom = self.normal.dot(&ray.direction);

        let v = self.point - ray.origin;
        let distance = v.dot(&self.normal) / denom;

        if min_distance < distance && distance < max_distance {
            return Some(distance);
        }
        None
    }

    fn surface_normal(self: &Self, _collision_point: &Vector) -> Vector {
        self.normal * -1.0
    }

    fn get_material_name(self: &Self) -> Option<&str> {
        self.material_name.as_ref().map(|s| s.as_ref())
    }
}
