#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_isometry3_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("isometry3 return self");
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Isometry3;
        bench_unop!(b, op => ret_self, ty => Isometry3)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::WIsometry3;
        bench_unop4!(b, op => ret_self, ty => WIsometry3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Isometry3;
        bench_unop!(b, op => ret_self, ty => Isometry3<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Isometry3;
        use simba::simd::f32x4;
        bench_unop4!(b, op => ret_self, ty => Isometry3<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Isometry3;
        use simba::simd::f32x8;
        bench_unop8!(b, op => ret_self, ty => Isometry3<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Isometry3;
        use simba::simd::f32x16;
        bench_unop16!(b, op => ret_self, ty => Isometry3<f32x16>)
    });
    group.finish();
}

fn bench_isometry3_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("isometry3 inverse");
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Isometry3;
        bench_unop!(b, op => inversed, ty => Isometry3)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::WIsometry3;
        bench_unop4!(b, op => inversed, ty => WIsometry3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Isometry3;
        bench_unop!(b, op => inverse, ty => Isometry3<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Isometry3;
        use simba::simd::f32x4;
        bench_unop4!(b, op => inverse, ty => Isometry3<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Isometry3;
        use simba::simd::f32x8;
        bench_unop8!(b, op => inverse, ty => Isometry3<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Isometry3;
        use simba::simd::f32x16;
        bench_unop16!(b, op => inverse, ty => Isometry3<f32x16>)
    });
    group.finish();
}

fn bench_isometry3_mul_isometry3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("isometry3 mul isometry3");
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Isometry3;
        bench_binop!(b, op => mul, ty1 => Isometry3, ty2 => Isometry3)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::WIsometry3;
        bench_binop4!(b, op => mul, ty1 => WIsometry3, ty2 => WIsometry3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Isometry3;
        bench_binop!(b, op => mul, ty1 => Isometry3<f32>, ty2 => Isometry3<f32>, param => by_ref)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Isometry3;
        use simba::simd::f32x4;
        bench_binop4!(b, op => mul, ty1 => Isometry3<f32x4>, ty2 => Isometry3<f32x4>, param => by_ref)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Isometry3;
        use simba::simd::f32x8;
        bench_binop8!(b, op => mul, ty1 => Isometry3<f32x8>, ty2 => Isometry3<f32x8>, param => by_ref)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Isometry3;
        use simba::simd::f32x16;
        bench_binop16!(b, op => mul, ty1 => Isometry3<f32x16>, ty2 => Isometry3<f32x16>, param => by_ref)
    });
    group.finish();
}

criterion_group!(
    isometry3d_benches,
    bench_isometry3_ret_self,
    bench_isometry3_inverse,
    bench_isometry3_mul_isometry3,
);
criterion_main!(isometry3d_benches);
