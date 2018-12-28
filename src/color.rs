use crate::vector::Vector;
use std::ops::{Add, Div, Mul};
use std::iter::Sum;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}


impl Color {
    pub fn default() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }

    pub fn from_vector(vector: Vector) -> Color {
        Color {
            red: vector.x,
            green: vector.y,
            blue: vector.z,
        }
    }

    pub fn antialias(colors: &[Color]) -> Color {
        let mut red: f64 = 0.0;
        let mut green: f64 = 0.0;
        let mut blue: f64 = 0.0;

        let len = colors.len();
        for color in colors {
            red += color.red;
            green += color.green;
            blue += color.blue;
        }
        Color::new(
            red / len as f64,
            green / len as f64,
            blue / len as f64,
        )
    }

    pub fn gamma_correct(&self) -> Color {
        Color {
            red: self.red.sqrt(),
            green: self.green.sqrt(),
            blue: self.blue.sqrt(),
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Div<usize> for Color {
    type Output = Self;

    fn div(self, rhs: usize) -> Self {
        Color {
            red: self.red / rhs as f64,
            green: self.green / rhs as f64,
            blue: self.blue / rhs as f64,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<Vector> for Color {
    type Output = Self;

    fn mul(self, rhs: Vector) -> Self {
        Color {
            red: self.red * rhs.x,
            green: self.green * rhs.y,
            blue: self.blue * rhs.z,
        }
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item=Color>>(iter: I) -> Self {
        iter.fold(Color::default(), |acc, color| acc + color)
    }
}
