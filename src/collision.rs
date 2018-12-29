use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;
use std::marker::Sync;

pub trait Collidable: Sync {
    fn check_collision(
        self: &Self,
        ray: Ray,
        tmix: f64,
        tmax: f64,
    ) -> Option<(f64, Vector, Vector, String)>;
    fn assign_material(&mut self, material: String);
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
