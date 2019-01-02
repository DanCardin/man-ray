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

    pub fn check_collision(
        self: &Self,
        ray: &Ray,
        min_distance: f64,
        max_distance: f64,
    ) -> Option<Collision> {
        let mut min_object_distance: Option<(&Box<dyn Collidable>, &str, f64)> = None;
        for object in self.objects.values() {
            let material_name = match object.get_material_name() {
                Some(name) => name,
                None => continue,
            };

            let potential_collision = object
                .check_collision(&ray, min_distance, max_distance)
                .filter(|distance| min_distance < *distance && *distance < max_distance);

            let collision_distance = match potential_collision {
                Some(distance) => distance,
                None => continue,
            };

            min_object_distance = match min_object_distance {
                None => Some((object, material_name, collision_distance)),
                Some((pobject, pmaterial_name, pdistance)) => {
                    if pdistance < collision_distance {
                        Some((pobject, pmaterial_name, pdistance))
                    } else {
                        Some((object, material_name, collision_distance))
                    }
                }
            };
        }

        let (object, material_name, distance) = match min_object_distance {
            Some(data) => data,
            None => return None,
        };

        let point = ray.point_at_distance(distance);
        let normal = object.surface_normal(point);
        let material = self.get_material(material_name);
        Some(Collision::new(distance, normal, material))
    }

    pub fn push_material<T: 'static + Material, S: Into<String>>(&mut self, name: S, material: T) {
        self.materials.insert(name.into(), Box::new(material));
    }

    pub fn get_material<S: AsRef<str>>(&self, name: S) -> &Material {
        &**self.materials.get(name.as_ref()).unwrap()
    }

    pub fn get_materials(&self) -> Vec<String> {
        self.materials.keys().map(|i| i.to_string()).collect()
    }

    pub fn push_object<T, S>(&mut self, name: S, object: T)
    where
        S: Into<String>,
        T: 'static + Collidable,
    {
        self.objects.insert(name.into(), Box::new(object));
    }
}
