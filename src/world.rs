use crate::collision::{Collidable, Collision};
use crate::material::Material;
use crate::ray::Ray;
use std::collections::HashMap;

pub struct World {
    materials: HashMap<String, Box<dyn Material>>,
    objects: HashMap<String, Box<dyn Collidable>>,
}

impl World {
    pub fn new() -> World {
        World {
            materials: HashMap::new(),
            objects: HashMap::new(),
        }
    }

    pub fn check_collision(self: &Self, ray: Ray, tmin: f64, tmax: f64) -> Option<Collision> {
        let mut closest_collision: Option<Collision> = None;
        for (_, collidable) in &self.objects {
            if let Some(collision_data) = collidable.check_collision(ray, tmin, tmax) {
                let (time, point, normal, material_name) = collision_data;
                let collision =
                    Collision::new(time, point, normal, self.get_material(material_name));
                closest_collision = match closest_collision {
                    Some(closest_collision) => {
                        if closest_collision.time < collision.time {
                            Some(closest_collision)
                        } else {
                            Some(collision)
                        }
                    }
                    None => Some(collision),
                };
            }
        }
        closest_collision
    }

    pub fn push_material<T: 'static + Material>(&mut self, name: String, material: T) {
        self.materials.insert(name, Box::new(material));
    }

    pub fn get_material<S: AsRef<str>>(&self, name: S) -> &Material {
        &**self.materials.get(name.as_ref()).unwrap()
    }

    pub fn get_materials(&self) -> Vec<String> {
        self.materials.keys().map(|i| i.to_string()).collect()
    }

    pub fn push_object<T: 'static + Collidable>(&mut self, name: String, object: T) {
        self.objects.insert(name, Box::new(object));
    }
}
