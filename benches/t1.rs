use criterion::{ criterion_group, criterion_main, Criterion};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("image", |b| b.iter(|| crate::run() );
    c.bench_function("Bvh", |b| {
        b.iter(|| {
            let mut rng = ChaCha8Rng::seed_from_u64(1);
            bvh_tutorial::run(&mut rng, 640, 640, 64, false, None);        })
    });

    // c.bench_function("Brute Force", |b| {
    //     b.iter(|| {
    //         let mut rng = ChaCha8Rng::seed_from_u64(1);
    //         bvh_tutorial::run(&mut rng, 640, 640, 64, true, None);        })
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
