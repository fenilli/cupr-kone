use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use rand::Rng;

use crate::core::world::World;

mod core;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    let mut world = World::new();
    let mut rng = rand::rng();

    for _ in 0..100000 {
        let e = world.spawn();

        world.insert(
            e,
            Position {
                x: rng.random_range(-100.0..100.0),
                y: rng.random_range(-100.0..100.0),
                z: rng.random_range(-100.0..100.0),
            },
        );

        world.insert(
            e,
            Velocity {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        );
    }

    let ticks_per_second: f64 = 60.0;
    let frame_duration = Duration::from_secs_f64(1.0 / ticks_per_second);

    let mut last_report = Instant::now();
    let mut tick_count = 0;

    loop {
        let frame_start = Instant::now();

        tick_count += 1;
        update(&mut world);

        if last_report.elapsed() >= Duration::from_secs(1) {
            println!("Ticks in last second: {}", tick_count);
            tick_count = 0;
            last_report = Instant::now();
        }

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            sleep(frame_duration - elapsed);
        }
    }
}

fn update(world: &mut World) {
    if let Some(mut set) = world.query_mut::<Velocity>() {
        for (_, vel) in set.iter_mut() {
            vel.x += 1.0;
            vel.y += 2.0;
            vel.z += 3.0;
        }
    }

    if let (Some(aset), Some(bset)) = (world.query_mut::<Position>(), world.query::<Velocity>()) {
        if aset.len() <= bset.len() {
            let (mut small, large) = (aset, bset);

            for (entity, pos) in small.iter_mut() {
                if let Some(vel) = large.get(entity) {
                    pos.x += vel.x;
                    pos.y += vel.y;
                    pos.z += vel.z;
                }
            }
        } else {
            let (small, mut large) = (bset, aset);

            for (entity, vel) in small.iter() {
                if let Some(pos) = large.get_mut(entity) {
                    pos.x += vel.x;
                    pos.y += vel.y;
                    pos.z += vel.z;
                }
            }
        }
    }
}
