extern crate criterion;
extern crate simdeez;
extern crate cursednoise;

use std::time::Duration;

use criterion::*;
use simdeez::prelude::*;

use cursednoise::intrinsics::*;
use cursednoise::*;

fn d3(c: &mut Criterion) {
    let setting = NoiseBuilder::fbm_3d(64, 64, 64).wrap();
    let mut group = c.benchmark_group("fbm3d");
    group.bench_function("avx2 3d", move |b| {
        b.iter(|| unsafe { avx2::get_3d_noise::<simdeez::Avx2>(&setting) })
    });
    group
        .sample_size(10)
        .warm_up_time(Duration::from_millis(1))
        .measurement_time(Duration::from_secs(5));
}

fn d2(c: &mut Criterion) {
    let setting = NoiseBuilder::fbm_2d(3840, 2160).wrap();
    let mut group = c.benchmark_group("fbm2d");
    group.bench_function("avx2 2d", move |b| {
        b.iter(|| unsafe { avx2::get_2d_noise::<simdeez::Avx2>(&setting) })
    });
    group
        .sample_size(10)
        .warm_up_time(Duration::from_millis(1))
        .measurement_time(Duration::from_secs(5));
}

fn d1(c: &mut Criterion) {
    let setting = NoiseBuilder::fbm_1d(1024).wrap();
    let mut group = c.benchmark_group("fbm1d");
    group.bench_function("avx2 1d", move |b| {
        b.iter(|| unsafe { avx2::get_1d_noise::<simdeez::Avx2>(&setting) })
    });
    group
        .sample_size(10)
        .warm_up_time(Duration::from_millis(1))
        .measurement_time(Duration::from_secs(5));
}

criterion_group!(benches, d3, d2, d1);
criterion_main!(benches);
