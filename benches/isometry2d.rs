#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_isometry2_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("isometry2 return self");
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Isometry2;
        bench_unop!(b, op => ret_self, ty => Isometry2)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::WIsometry2;
        bench_unop4!(b, op => ret_self, ty => WIsometry2)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Isometry2;
        bench_unop!(b, op => ret_self, ty => Isometry2<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Isometry2;
        use simba::simd::f32x4;
        bench_unop4!(b, op => ret_self, ty => Isometry2<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Isometry2;
        use simba::simd::f32x8;
        bench_unop8!(b, op => ret_self, ty => Isometry2<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Isometry2;
        use simba::simd::f32x16;
        bench_unop16!(b, op => ret_self, ty => Isometry2<f32x16>)
    });
    group.finish();
}

fn bench_isometry2_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("isometry2 inverse");
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Isometry2;
        bench_unop!(b, op => inversed, ty => Isometry2)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::WIsometry2;
        bench_unop4!(b, op => inversed, ty => WIsometry2)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Isometry2;
        bench_unop!(b, op => inverse, ty => Isometry2<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Isometry2;
        use simba::simd::f32x4;
        bench_unop4!(b, op => inverse, ty => Isometry2<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Isometry2;
        use simba::simd::f32x8;
        bench_unop8!(b, op => inverse, ty => Isometry2<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Isometry2;
        use simba::simd::f32x16;
        bench_unop16!(b, op => inverse, ty => Isometry2<f32x16>)
    });
    group.finish();
}

fn bench_isometry2_mul_isometry2(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("isometry2 mul isometry2");
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Isometry2;
        bench_binop!(b, op => mul, ty1 => Isometry2, ty2 => Isometry2)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::WIsometry2;
        bench_binop4!(b, op => mul, ty1 => WIsometry2, ty2 => WIsometry2)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Isometry2;
        bench_binop!(b, op => mul, ty1 => Isometry2<f32>, ty2 => Isometry2<f32>, param => by_ref)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Isometry2;
        use simba::simd::f32x4;
        bench_binop4!(b, op => mul, ty1 => Isometry2<f32x4>, ty2 => Isometry2<f32x4>, param => by_ref)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Isometry2;
        use simba::simd::f32x8;
        bench_binop8!(b, op => mul, ty1 => Isometry2<f32x8>, ty2 => Isometry2<f32x8>, param => by_ref)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Isometry2;
        use simba::simd::f32x16;
        bench_binop16!(b, op => mul, ty1 => Isometry2<f32x16>, ty2 => Isometry2<f32x16>, param => by_ref)
    });
    group.finish();
}

criterion_group!(
    isometry2d_benches,
    bench_isometry2_ret_self,
    bench_isometry2_inverse,
    bench_isometry2_mul_isometry2,
);
criterion_main!(isometry2d_benches);
