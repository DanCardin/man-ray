use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;
use std::marker::Sync;

pub trait Collidable: Sync {
    fn check_collision(self: &Self, ray: &Ray, min_distance: f64, max_distance: f64)
        -> Option<f64>;
    fn surface_normal(self: &Self, collision_point: Vector) -> Vector;
    fn get_material_name(self: &Self) -> Option<&str>;
}

pub struct Collision<'a> {
    pub distance: f64,
    pub normal: Vector,
    pub material: &'a dyn Material,
}

impl<'a> Collision<'a> {
    pub fn new(distance: f64, normal: Vector, material: &'a dyn Material) -> Collision {
        Collision {
            distance,
            normal,
            material,
        }
    }
}
