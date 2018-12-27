use crate::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn point_at_parameter(self: &Self, t: f64) -> Vector {
        self.origin + self.direction * t
    }
}
