use itertools::iproduct;
use rand::prelude::*;
use rand::rngs::SmallRng;

use crate::color::Color;
use crate::ray::Ray;
use crate::vector::Vector;
use crate::world::World;
use rayon::prelude::*;
use std::f64;
use std::ops::Div;

pub struct Camera {
    origin: Vector,
    target: Vector,
    vup: Vector,

    apurture: f64,
    fov: f64,
    aspect_ratio: f64,
    scale: usize,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vector::default(),
            target: Vector::new(0.0, 0.0, -1.0),
            vup: Vector::new(0.0, 1.0, 0.0),

            apurture: 0.0,
            fov: 33.0,
            aspect_ratio: 4.0 / 3.0,
            scale: 600,
        }
    }
}

impl Camera {
    pub fn with_origin(mut self, origin: Vector) -> Self {
        self.origin = origin;
        self
    }

    pub fn with_target(mut self, target: Vector) -> Self {
        self.target = target;
        self
    }

    pub fn with_scale(mut self, scale: usize) -> Self {
        self.scale = scale;
        self
    }

    pub fn set_origin(&mut self, origin: Vector) {
        self.origin = origin;
    }

    fn focus_dist(&self) -> f64 {
        (self.origin - self.target).length()
    }

    fn theta(&self) -> f64 {
        self.fov * f64::consts::PI / 180.0
    }

    fn height(&self) -> f64 {
        (self.theta()).tan()
    }

    fn width(&self) -> f64 {
        self.aspect_ratio * self.height()
    }

    fn get_ray(self: &Self, s: f64, t: f64, rng: &mut SmallRng) -> Ray {
        let w = (self.origin - self.target).to_unit();
        let u = self.vup.cross(&w).to_unit();
        let v = w.cross(&u);

        let focus_dist = self.focus_dist();
        let horizontal_part = u * (self.width() / 2.0) * focus_dist;
        let vertical_part = v * (self.height() / 2.0) * focus_dist;
        let depth_part = w * focus_dist;

        let lower_left_corner = self.origin - horizontal_part - vertical_part - depth_part;
        let horizontal = horizontal_part * 2.0;
        let vertical = vertical_part * 2.0;

        let random_disc = random_in_unit_disc(rng) * self.apurture / 2.0;
        let offset = u * random_disc.x + v * random_disc.y;
        Ray::new(
            self.origin + offset,
            lower_left_corner + (horizontal * s) + (vertical * t) - self.origin - offset,
        )
    }

    pub fn pixel_width(&self) -> usize {
        self.scale
    }

    pub fn pixel_height(&self) -> usize {
        (self.scale as f64 / self.aspect_ratio) as usize
    }

    pub fn render(self: &Self, world: &World) -> Vec<Color> {
        let width = self.pixel_width();
        let height = self.pixel_height();
        let sub_pixels = 1;

        iproduct!(0..height, 0..width)
            .collect::<Vec<(usize, usize)>>()
            .par_iter()
            .map(|(i, e)| {
                let mut rng = SmallRng::from_rng(thread_rng()).expect("couldn't get randomness");
                (0..sub_pixels)
                    .map(|_| {
                        let u_rand: f64 = rng.gen();
                        let u = (*e as f64 + u_rand) / width as f64;

                        let v_rand: f64 = rng.gen();
                        let v = (((height as f64) - (*i as f64)) + v_rand) / (height as f64);
                        let ray = self.get_ray(u, v, &mut rng);
                        calc_color(world, &ray, 0, &mut rng)
                    })
                    .sum::<Color>()
                    .div(sub_pixels)
                    .gamma_correct()
            })
            .collect()
    }
}

fn calc_color(world: &World, ray: &Ray, depth: i32, rng: &mut SmallRng) -> Color {
    let collision = match world.check_collision(&ray, 0.001, f64::MAX) {
        Some(collision) => collision,
        None => return background(ray),
    };

    if depth < 2 {
        match collision.material.scatter(ray, collision, rng) {
            Some(effect) => calc_color(world, &effect.scatter, depth + 1, rng) * effect.attenuation,
            None => Color::default(),
        }
    } else {
        Color::default()
    }
}

pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Vector {
    let mut point;
    loop {
        point = Vector::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - Vector::unit();
        if point.dot(&point) >= 1.0 {
            break;
        }
    }
    point
}

pub fn random_in_unit_disc(rng: &mut SmallRng) -> Vector {
    let mut point;
    loop {
        point = Vector::new(rng.gen(), rng.gen(), 0.0) * 2.0 - Vector::new(1.0, 1.0, 0.0);
        if point.dot(&point) >= 1.0 {
            break;
        }
    }
    point
}

fn background(ray: &Ray) -> Color {
    let unit_direction = ray.direction.to_unit();
    let t = 0.5 * unit_direction.y + 1.0;
    let vector = Vector::unit() * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t;
    Color::from_vector(vector)
}
