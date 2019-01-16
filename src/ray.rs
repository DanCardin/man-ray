use crate::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at_distance(self: &Self, distance: f64) -> Vector {
        self.origin + self.direction * distance
    }
}
