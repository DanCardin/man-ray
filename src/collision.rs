use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub trait Collidable {
    fn check_collision(self: &Self, ray: Ray, tmix: f64, tmax: f64) -> Option<Collision>;
}

pub struct Collision<'a> {
    pub time: f64,
    pub point: Vector,
    pub normal: Vector,
    pub material: &'a dyn Material,
}

impl<'a> Collision<'a> {
    pub fn new(time: f64, point: Vector, normal: Vector, material: &'a dyn Material) -> Collision {
        Collision {
            time,
            point,
            normal,
            material,
        }
    }
}
