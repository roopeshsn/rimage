use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rimage::{decoders::decode_image, encoders::encode_image};

fn bench_encode_png_1(c: &mut Criterion) {
    let (pixels, width, height) = decode_image(&PathBuf::from("test/large_test1.jpg")).unwrap();
    c.bench_function("en lt1png", |b| {
        b.iter(|| {
            encode_image(
                black_box(&PathBuf::from("en_lt1png")),
                black_box(&pixels),
                black_box("png"),
                black_box(width),
                black_box(height),
                black_box(0.75),
            )
        })
    });
}

fn bench_encode_png_2(c: &mut Criterion) {
    let (pixels, width, height) = decode_image(&PathBuf::from("test/large_test2.jpg")).unwrap();
    c.bench_function("en lt2png", |b| {
        b.iter(|| {
            encode_image(
                black_box(&PathBuf::from("en_lt2png")),
                black_box(&pixels),
                black_box("png"),
                black_box(width),
                black_box(height),
                black_box(0.75),
            )
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_encode_png_1, bench_encode_png_2
);
criterion_main!(benches);
