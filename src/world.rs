use crate::ray::Ray;
use crate::collision::{Collision, Collidable};


pub struct World {
    collidable_objects: Vec<Box<dyn Collidable>>,
}


impl World {
    pub fn new(collidable_objects: Vec<Box<dyn Collidable>>) -> World {
        World {
            collidable_objects: collidable_objects,
        }
    }

    pub fn check_collision(self: &Self, ray: Ray, tmin: f64, tmax: f64) -> Option<Collision> {
        let mut closest_collision: Option<Collision> = None;
        for collidable in self.collidable_objects.iter() {
            if let Some(collision) = collidable.check_collision(ray, tmin, tmax) {
                closest_collision = match closest_collision {
                    Some(closest_collision) => {
                        if closest_collision.time < collision.time {
                            Some(closest_collision)
                        } else {
                            Some(collision)
                        }
                    },
                    None => Some(collision),
                };
            }
        }
        closest_collision
    }
}
