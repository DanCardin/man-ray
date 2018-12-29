use criterion::{criterion_group, criterion_main, Benchmark, Criterion};

use itertools::iproduct;
use man_ray::camera::Camera;
use man_ray::collision::Collidable;
use man_ray::material::{Dialectic, Lambertian, Metal};
use man_ray::shape::Sphere;
use man_ray::vector::Vector;
use man_ray::world::World;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::thread_rng;

fn big_scene<'a>(world: &World, scale: usize) {
    let origin = Vector::new(8.0, 2.0, 3.0);
    let target = Vector::new(0.0, 1.0, 0.0);
    let vup = Vector::new(0.0, 1.0, 0.0);
    let field_of_view = 33.0;
    let aspect_ratio = 4.0 / 3.0;
    let apurture = 0.0;
    let camera = Camera::new(origin, target, vup, field_of_view, aspect_ratio, apurture);
    camera.render(&world, scale);
}

fn setup(n: usize, rng: &mut SmallRng) -> World {
    let mut world = World::new();

    world.push_material("ground".into(), Lambertian::new(Vector::new(0.5, 0.5, 0.5)));
    world.push_object(
        "ground".into(),
        Sphere::new_with_material(Vector::new(0.0, -1000.0, 0.0), 1000.0, "ground".into()),
    );

    world.push_material("dialectic".into(), Dialectic::new(1.5));
    world.push_object(
        "dialectic".into(),
        Sphere::new_with_material(Vector::new(0.0, 1.0, 0.0), 1000.0, "dialectic".into()),
    );

    world.push_material(
        "lambertian".into(),
        Lambertian::new(Vector::new(0.4, 0.2, 0.1)),
    );
    world.push_object(
        "lambertian".into(),
        Sphere::new_with_material(Vector::new(-4.0, 1.0, 0.0), 1000.0, "lambertian".into()),
    );

    world.push_material("metal".into(), Metal::new(Vector::new(0.7, 0.6, 0.5), 0.0));
    world.push_object(
        "metal".into(),
        Sphere::new_with_material(Vector::new(4.0, 1.0, 0.0), 1000.0, "metal".into()),
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
    random_materials.shuffle(rng);
    for ((i, e), material_name) in iproduct!(0..n, 0..n).zip(random_materials.into_iter()) {
        let center = Vector::new(
            i as f64 - (n as f64) / 2.0 + 0.9 + rng.gen::<f64>(),
            0.2,
            e as f64 - (n as f64) / 2.0 + 0.9 * rng.gen::<f64>(),
        );

        world.push_object(
            format!("{},{}", i, e),
            Sphere::new_with_material(center, 0.2, material_name),
        );
    }
    world
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    let world = setup(1, &mut rng);
    const scale: usize = 50;

    c.bench(
        "routines",
        Benchmark::new("big scene", move |b| b.iter(|| big_scene(&world, scale))).sample_size(50),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
