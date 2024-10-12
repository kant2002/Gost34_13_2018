use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_cc_sBox(c: &mut Criterion) {
    let din = [0u8; 32];
    let mut dout = [0u8; 32];
    c.bench_function("cc_sBox", |b| b.iter(|| gost::cc_sBox(&din, &mut dout, 32)));
}

fn criterion_sBox(c: &mut Criterion) {
    let din = [0u8; 32];
    let mut dout = [0u8; 32];
    c.bench_function("sBox", |b| {
        b.iter(|| gost::qalqan::sBox(&din, &mut dout, 32))
    });
}

fn criterion_sBoxU(c: &mut Criterion) {
    let din = [0u8; 32];
    let mut dout = [0u8; 32];
    c.bench_function("sBoxU", |b| {
        b.iter(|| gost::qalqan::sBoxU(&din, &mut dout, 32))
    });
}

criterion_group!(benches, criterion_cc_sBox, criterion_sBox, criterion_sBoxU);
criterion_main!(benches);
