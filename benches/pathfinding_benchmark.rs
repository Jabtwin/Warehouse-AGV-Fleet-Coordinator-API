use agv_coordinator::models::{Coordinate, Grid};
use agv_coordinator::pathfinding::a_star;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashSet;

fn benchmark_a_star(c: &mut Criterion) {
    let grid = Grid::new(1000, 1000);
    let start = Coordinate { x: 0, y: 0 };
    let goal = Coordinate { x: 999, y: 999 };
    let occupied_coords = HashSet::new();

    c.bench_function("a_star_1000x1000", |b| {
        b.iter(|| {
            a_star(
                black_box(&grid),
                black_box(start),
                black_box(goal),
                black_box(&occupied_coords),
            )
        })
    });
}

criterion_group!(benches, benchmark_a_star);
criterion_main!(benches);
