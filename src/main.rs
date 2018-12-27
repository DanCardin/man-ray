use std::io;

use ray::camera::Camera;
use ray::material::{Lambertian, Metal, Dialectic, Material};
use ray::collision::Collidable;
use ray::shape::Sphere;
use rand::thread_rng;
use rand::rngs::SmallRng;
use rand::prelude::*;
use ray::vector::Vector;
use ray::world::World;

fn main() -> io::Result<()> {
    let n = 300;
    let ns = (n as f64).sqrt() as isize;
    let mut objects: Vec<Box<dyn Collidable>> = Vec::with_capacity(n);

    let ground = Box::new(
        Sphere::new(
            Vector::new(0.0, -1000.0, 0.0),
            1000.0,
            Box::new(Lambertian::new(Vector::new(0.5, 0.5, 0.5))),
        )
    );
    let dialectic = Box::new(
        Sphere::new(
            Vector::new(0.0, 1.0, 0.0),
            1.0,
            Box::new(Dialectic::new(1.5)),
        )
    );
    let lambertian = Box::new(
        Sphere::new(
            Vector::new(-4.0, 1.0, 0.0),
            1.0,
            Box::new(Lambertian::new(Vector::new(0.4, 0.2, 0.1))),
        )
    );
    let metal = Box::new(
        Sphere::new(
            Vector::new(4.0, 1.0, 0.0),
            1.0,
            Box::new(Metal::new(Vector::new(0.7, 0.6, 0.5), 0.0)),
        )
    );
    objects.push(ground);
    objects.push(dialectic);
    objects.push(lambertian);
    objects.push(metal);

    let mut rng = SmallRng::from_rng(thread_rng())?;
    for i in -ns..ns {
        for e in -ns..ns {
            let choose_mat = rng.gen::<f64>();
            let center = Vector::new(i as f64 + 0.9 + rng.gen::<f64>(), 0.2, e as f64 + 0.9 * rng.gen::<f64>());
            let material: Box<dyn Material> = if (center - Vector::new(4.0, 0.2, 0.0)).length() > 0.9 {
                Box::new(Lambertian::new(Vector::new(
                    rng.gen::<f64>() * rng.gen::<f64>(),
                    rng.gen::<f64>() * rng.gen::<f64>(),
                    rng.gen::<f64>() * rng.gen::<f64>(),
                )))
            } else if choose_mat < 0.95 {
                Box::new(Metal::new(
                    Vector::new(
                        0.5 * (1.0 + rng.gen::<f64>()),
                        0.5 * (1.0 + rng.gen::<f64>()),
                        0.5 * (1.0 + rng.gen::<f64>()),
                    ),
                    0.5 * rng.gen::<f64>(),
                ))
            } else {
                Box::new(Dialectic::new(1.5))
            };
            objects.push(Box::new(Sphere::new(center, 0.2, material)));
        }
    }

    let origin = Vector::new(8.0, 2.0, 3.0);
    let target = Vector::new(0.0, 1.0, 0.0);
    let vup = Vector::new(0.0, 1.0, 0.0);
    let field_of_view = 33.0;
    let aspect_ratio = 4.0 / 3.0;
    let apurture = 0.0;
    let camera = Camera::new(origin, target, vup, field_of_view, aspect_ratio, apurture);
    let world = World::new(objects);
    camera.render(&world, 1200, "example.ppm", &mut rng)?;
    Ok(())
}
