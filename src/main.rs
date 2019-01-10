use ggez::GameResult;
use itertools::iproduct;
use man_ray::camera::Camera;
use man_ray::material::{Dialectic, Lambertian, Metal};
use man_ray::shapes::{plane::Plane, sphere::Sphere};
use man_ray::vector::Vector;
use man_ray::window::Window;
use man_ray::world::World;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::thread_rng;

fn create_world(n: usize) -> World {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    let mut world = World::new();

    world.push_material("ground", Lambertian::new(Vector::new(0.5, 0.5, 0.5)));
    world.push_object(
        "ground",
        Plane::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, -1.0, 0.0)).with_material("ground"),
    );

    world.push_material("dialectic", Dialectic::new(1.5));
    world.push_object(
        "dialectic",
        Sphere::new(Vector::new(0.0, 1.0, 0.0), 1.0).with_material("dialectic"),
    );

    world.push_material("lambertian", Lambertian::new(Vector::new(0.4, 0.2, 0.1)));
    world.push_object(
        "lambertian",
        Sphere::new(Vector::new(4.0, 1.0, 0.0), 1.0).with_material("lambertian"),
    );

    world.push_material("metal", Metal::new(Vector::new(0.7, 0.6, 0.5), 0.0));
    world.push_object(
        "metal",
        Sphere::new(Vector::new(-4.0, 1.0, 0.0), 1.0).with_material("metal"),
    );

    for i in 0..50 {
        world.push_material(
            format!("lamb{}", i),
            Lambertian::new(Vector::new(
                rng.gen::<f64>() * rng.gen::<f64>(),
                rng.gen::<f64>() * rng.gen::<f64>(),
                rng.gen::<f64>() * rng.gen::<f64>(),
            )),
        );
    }
    for i in 0..10 {
        world.push_material(
            format!("metal{}", i),
            Metal::new(
                Vector::new(
                    0.5 * (1.0 + rng.gen::<f64>()),
                    0.5 * (1.0 + rng.gen::<f64>()),
                    0.5 * (1.0 + rng.gen::<f64>()),
                ),
                0.5 * rng.gen::<f64>(),
            ),
        );
    }
    for i in 0..10 {
        world.push_material(format!("dial{}", i), Dialectic::new(1.5));
    }

    let mut random_materials = world.get_materials();
    random_materials.shuffle(&mut rng);
    for ((i, e), material_name) in iproduct!(0..n, 0..n).zip(random_materials.into_iter()) {
        let center = Vector::new(
            i as f64 - (n as f64) / 2.0 + 0.9 + rng.gen::<f64>(),
            0.2,
            e as f64 - (n as f64) / 2.0 + 0.9 * rng.gen::<f64>(),
        );

        world.push_object(
            format!("{},{}", i, e),
            Sphere::new(center, 0.2).with_material(material_name),
        );
    }
    world
}

fn main() -> GameResult<()> {
    let world = create_world(10);
    let camera = Camera::default()
        .with_origin(Vector::new(8.0, 2.0, 3.0))
        .with_target(Vector::new(0.0, 1.0, 0.0))
        .with_scale(600);

    Window::new(world, camera).run()
}
