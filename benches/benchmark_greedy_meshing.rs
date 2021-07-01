use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xp_vox_engine::{chunker::greedy_meshing::greedy_mesh, registry::Registry, vox};

fn criterion_benchmark(c: &mut Criterion) {
    let mut vox_models = Registry::new();
    let vox_handle = vox::load_vox(
        &dot_vox::load_bytes(
            std::fs::read("res/vox-models/#treehouse/#treehouse.vox")
                .unwrap()
                .as_slice(),
        )
        .unwrap(),
        &mut vox_models,
    );
    if let Some(vox) = vox_models.get(&vox_handle) {
        println!("vox size x: {}, y: {}, z: {}", vox.x_size, vox.y_size, vox.z_size);
        c.bench_function("greedy_meshing", |b| b.iter(|| greedy_mesh(vox)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
