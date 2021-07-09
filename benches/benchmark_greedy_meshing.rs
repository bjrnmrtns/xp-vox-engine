use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xp_vox_engine::{registry::Registry, vox, world::Chunker};

fn criterion_benchmark(c: &mut Criterion) {
    let mut vox_models = Registry::new();
    let mut chunker = Chunker::new(32);
    let vox_handle = vox::load_vox(
        &dot_vox::load_bytes(
            std::fs::read("res/vox-models/#treehouse/#treehouse.vox")
                .unwrap()
                .as_slice(),
        )
        .unwrap(),
        &mut vox_models,
    );
    chunker.add(vox_handle.clone(), [0, 0, 0], &vox_models);
    chunker.add(vox_handle.clone(), [0, 0, 128], &vox_models);
    chunker.add(vox_handle.clone(), [128, 0, 128], &vox_models);
    chunker.add(vox_handle.clone(), [128, 0, 0], &vox_models);

    chunker.add(vox_handle.clone(), [128, 0, 0], &vox_models);
    chunker.add(vox_handle.clone(), [128, 0, 128], &vox_models);
    chunker.add(vox_handle.clone(), [256, 0, 128], &vox_models);
    chunker.add(vox_handle.clone(), [256, 0, 0], &vox_models);

    chunker.add(vox_handle.clone(), [0, 0, 128], &vox_models);
    chunker.add(vox_handle.clone(), [0, 0, 256], &vox_models);
    chunker.add(vox_handle.clone(), [128, 0, 256], &vox_models);
    chunker.add(vox_handle.clone(), [128, 0, 128], &vox_models);

    chunker.add(vox_handle.clone(), [128, 0, 128], &vox_models);
    chunker.add(vox_handle.clone(), [128, 0, 256], &vox_models);
    chunker.add(vox_handle.clone(), [256, 0, 256], &vox_models);
    chunker.add(vox_handle.clone(), [256, 0, 128], &vox_models);
    c.bench_function("generate_chunk", |b| {
        b.iter(|| chunker.generate_chunk(&vox_models, (0, 0, 0)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
