use criterion::{criterion_group, criterion_main, Criterion};
use rust_core::graph::bubble::BubbleStyle;
use rust_core::graph::Graph;
use rust_core::graphics::colors;
use rust_core::physics;
use std::hint::black_box;

fn bench_physics(c: &mut Criterion) {
    let mut graph = Graph::new();
    graph.init(
        BubbleStyle::Player,
        BubbleStyle::Standard {
            size: 1,
            color: colors::TURQUOISE,
        },
    );

    // Warm up and grow the graph to a reasonable size to make the benchmark meaningful
    for _ in 0..15 {
        let vkeys: Vec<_> = graph.vertices.keys().collect();
        // Just pick the first vertex and spawn a bubble on it
        graph.spawn(
            vkeys[0],
            BubbleStyle::Standard {
                size: 1,
                color: colors::ROSE,
            },
        );
        // Advance physics a few times so the new bubbles settle
        for _ in 0..20 {
            physics::advance_frame(&mut graph);
        }
    }

    // Now we have a graph with ~17 bubbles. Let's benchmark the physics tick.
    c.bench_function("advance_frame_medium_graph", |b| {
        b.iter(|| {
            physics::advance_frame(black_box(&mut graph));
        })
    });
}

criterion_group!(benches, bench_physics);
criterion_main!(benches);
