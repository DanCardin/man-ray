use crate::color::Color;
use crate::vector::Vector;

pub struct Light {
    pub direction: Vector,
    pub color: Color,
    pub intensity: f64,
}
