use std::fs::File;
use std::f64;
use std::path::Path;
use std::io;
use std::io::{BufWriter, Write};
use rand::prelude::*;
use rand::rngs::SmallRng;

use crate::color::Color;
use crate::vector::Vector;
use crate::world::World;
use crate::ray::Ray;


#[derive(Clone, Copy, Debug)]
pub struct CameraColor {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}


impl CameraColor {
    pub fn from_color(color: Color) -> CameraColor {
        CameraColor {
            red: (color.red * 255.99) as u8,
            green: (color.green * 255.99) as u8,
            blue: (color.blue * 255.99) as u8,
        }
    }

}



pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    aspect: f64,
    lens_radius: f64,
}


impl Camera {
    pub fn new(origin: Vector, target: Vector, vup: Vector, fov: f64, aspect: f64, apurture: f64) -> Camera {
        let focus_dist = (origin - target).length();
        let lens_radius = apurture / 2.0;
        let theta = fov * f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (origin - target).to_unit();
        let u = vup.cross(w).to_unit();
        let v = w.cross(u);

        let horizontal_part = u * half_width * focus_dist;
        let vertical_part = v * half_height * focus_dist;
        let depth_part = w * focus_dist;

        let lower_left_corner = origin - horizontal_part - vertical_part - depth_part;
        let horizontal = horizontal_part * 2.0;
        let vertical = vertical_part * 2.0;
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            aspect: aspect,
            lens_radius: lens_radius,
        }
    }

    fn get_ray(self: &Self, s: f64, t: f64, rng: &mut SmallRng) -> Ray {
        let random_disc = random_in_unit_disc(rng) * self.lens_radius;
        let offset = self.u * random_disc.x + self.v * random_disc.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner +
            (self.horizontal * s) +
            (self.vertical * t) -
            self.origin - offset,
        )
    }

    pub fn render(self: &Self, world: &World, scale: usize, filename: &str, mut rng: &mut SmallRng) -> io::Result<()> {
        let f = File::create(Path::new(filename)).expect("Unable to open file");
        let mut f = BufWriter::new(f);

        let width = scale;
        let height = (scale as f64 / self.aspect) as usize;
        println!("{}, {}", height, width);

        let sub_pixels = 100;
        let mut pixels: Vec<Color> = Vec::with_capacity(width * height);
        for i in 0..height {
            for e in 0..width {
                let mut colors = Vec::new();
                for _ in 0..sub_pixels {
                    let u_rand: f64 = rng.gen();
                    let u = (e as f64 + u_rand) / width as f64;

                    let v_rand: f64 = rng.gen();
                    let v = (((height as f64) - (i as f64)) + v_rand) / (height as f64);
                    let ray = self.get_ray(u, v, &mut rng);

                    let color = calc_color(world, ray, 0, &mut rng);
                    colors.push(color);
                }
                pixels.push(Color::gamma_correct(Color::antialias(colors.as_ref())));
            }
        }

        f.write("P3\n".as_bytes())?;
        f.write(format!("{} {}\n", width, height).as_bytes())?;
        f.write(format!("{}\n", 255).as_bytes())?;

        for row in pixels.chunks(width) {
            for pixel in row {
                let camera_color = CameraColor::from_color(*pixel);
                f.write(
                    format!(
                        "{} {} {}\n",
                        camera_color.red,
                        camera_color.green,
                        camera_color.blue,
                        )
                    .as_bytes()
                )?;
            }
            f.write("\n".as_bytes())?;
        }
        Ok(())
    }
}

fn calc_color(world: &World, ray: Ray, depth: i32, rng: &mut SmallRng) -> Color {
    if let Some(collision) = world.check_collision(ray, 0.001, f64::MAX) {
        if depth < 50 {
            match collision.material.scatter(ray, collision, rng) {
                Some(effect) => {
                    calc_color(
                        world,
                        effect.scatter,
                        depth + 1,
                        rng
                    ) * effect.attenuation
                }
                None => Color::default(),
            }
        } else {
            Color::default()
        }
    } else {
        background(ray)
    }
}

pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Vector {
    let mut point;
    loop {
        point = Vector::new(
            rng.gen(),
            rng.gen(),
            rng.gen(),
        ) * 1.5 - Vector::unit();
        if point.dot(point) >= 1.0 {
            break;
        }
    };
    point
}

pub fn random_in_unit_disc(rng: &mut SmallRng) -> Vector {
    let mut point;
    loop {
        point = Vector::new(
            rng.gen(),
            rng.gen(),
            0.0,
        ) * 2.0 - Vector::new(1.0, 1.0, 0.0);
        if point.dot(point) >= 1.0 {
            break;
        }
    };
    point
}

fn background(ray: Ray) -> Color {
    let unit_direction = ray.direction.to_unit();
    let t = 0.5 * unit_direction.y + 1.0;
    let vector = Vector::unit() * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t;
    Color::from_vector(vector)
}
