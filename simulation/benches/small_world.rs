use criterion::{criterion_group, criterion_main, Criterion};
use macroquad::math::ivec2;

fn benchmark(c: &mut Criterion) {
    let world_size = ivec2(32, 32);
    if let Ok(mut simulation) = simulation::Simulation::new(world_size) {
        for y in 0..world_size.y {
            for x in 0..world_size.x {
                let cell = match rand::random::<u32>() % 4 {
                    0 => simulation::Cell::AIR,
                    1 => simulation::Cell::SAND,
                    2 => simulation::Cell::STONE,
                    3 => simulation::Cell::WATER,
                    _ => simulation::Cell::AIR
                };
                simulation.write_cell(ivec2(x, y), cell);
            }
        }
        let id = format!("world_size={}x{}", world_size.x, world_size.y);
        c.bench_function(&id, |b| b.iter(||simulation.tick()));
    }
}

criterion_main!(benches);
criterion_group!(benches, benchmark);