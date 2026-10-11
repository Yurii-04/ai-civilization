use criterion::{criterion_group, criterion_main, Criterion};
use sim_core::{step, Balance, World};

fn tick_placeholder_world(c: &mut Criterion) {
    let mut world = World::new(7, Balance::embedded().expect("balance"));
    c.bench_function("tick_placeholder_world", |b| {
        b.iter(|| step(&mut world, &[]));
    });
}

criterion_group!(benches, tick_placeholder_world);
criterion_main!(benches);
