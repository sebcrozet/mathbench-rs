#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// returns self to check overhead of benchmark
fn bench_vector3_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("vector3 return self");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_unop!(b, op => ret_self, ty => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Vector3;
        bench_unop!(b, op => ret_self, ty => Vector3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Vec3;
        bench_unop!(b, op => ret_self, ty => Vec3)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::Wec3;
        bench_unop4!(b, op => ret_self, ty => Wec3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => ret_self, ty => Vector3<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x4;
        bench_unop4!(b, op => ret_self, ty => Vector3<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x8;
        bench_unop8!(b, op => ret_self, ty => Vector3<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x16;
        bench_unop16!(b, op => ret_self, ty => Vector3<f32x16>)
    });
    bench_vek!(group, |b| {
        use vek::Vec3;
        bench_unop!(b, op => ret_self, ty => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector3 length");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_unop!(b, op => length, ty => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_unop!(b, op => magnitude, ty => Vector3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Vec3;
        bench_unop!(b, op => mag, ty => Vec3)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::Wec3;
        bench_unop4!(b, op => mag, ty => Wec3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => magnitude, ty => Vector3<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x4;
        bench_unop4!(b, op => magnitude, ty => Vector3<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x8;
        bench_unop8!(b, op => magnitude, ty => Vector3<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x16;
        bench_unop16!(b, op => magnitude, ty => Vector3<f32x16>)
    });
    bench_euclid!(group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_unop!(b, op => length, ty => Vector3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Vec3;
        bench_unop!(b, op => magnitude, ty => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_normalize(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector3 normalize");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_unop!(b, op => normalize, ty => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_unop!(b, op => normalize, ty => Vector3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Vec3;
        bench_unop!(b, op => normalized, ty => Vec3)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::Wec3;
        bench_unop4!(b, op => normalized, ty => Wec3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => normalize, ty => Vector3<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x4;
        bench_unop4!(b, op => normalize, ty => Vector3<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x8;
        bench_unop8!(b, op => normalize, ty => Vector3<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x16;
        bench_unop16!(b, op => normalize, ty => Vector3<f32x16>)
    });
    bench_euclid!(group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_unop!(b, op => normalize, ty => Vector3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Vec3;
        bench_unop!(b, op => normalized, ty => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_dot(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector3 dot");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_binop!(b, op => dot, ty1 => Vec3, ty2 => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_binop!(b, op => dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Wec3;
        bench_binop!(b, op => dot, ty1 => Wec3, ty2 => Wec3)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::Wec3;
        bench_binop4!(b, op => dot, ty1 => Wec3, ty2 => Wec3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_binop!(b, op => dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>, param => by_ref)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x4;
        bench_binop4!(b, op => dot, ty1 => Vector3<f32x4>, ty2 => Vector3<f32x4>, param => by_ref)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x8;
        bench_binop8!(b, op => dot, ty1 => Vector3<f32x8>, ty2 => Vector3<f32x8>, param => by_ref)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x16;
        bench_binop16!(b, op => dot, ty1 => Vector3<f32x16>, ty2 => Vector3<f32x16>, param => by_ref)
    });
    bench_euclid!(group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_binop!(b, op => dot, ty1 => Vector3D<f32, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Vec3;
        bench_binop!(b, op => dot, ty1 => Vec3<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_cross(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector3 cross");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_binop!(b, op => cross, ty1 => Vec3, ty2 => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Vector3;
        bench_binop!(b, op => cross, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Vec3;
        bench_binop!(b, op => dot, ty1 => Vec3, ty2 => Vec3)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::Wec3;
        bench_binop4!(b, op => dot, ty1 => Wec3, ty2 => Wec3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_binop!(b, op => dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>, param => by_ref)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x4;
        bench_binop4!(b, op => dot, ty1 => Vector3<f32x4>, ty2 => Vector3<f32x4>, param => by_ref)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x8;
        bench_binop8!(b, op => dot, ty1 => Vector3<f32x8>, ty2 => Vector3<f32x8>, param => by_ref)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Vector3;
        use simba::simd::f32x16;
        bench_binop16!(b, op => dot, ty1 => Vector3<f32x16>, ty2 => Vector3<f32x16>, param => by_ref)
    });
    bench_euclid!(group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_binop!(b, op => cross, ty1 => Vector3D<f32, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Vec3;
        bench_binop!(b, op => cross, ty1 => Vec3<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

criterion_group!(
    vector3_benches,
    bench_vector3_ret_self,
    bench_vector3_length,
    bench_vector3_normalize,
    bench_vector3_dot,
    bench_vector3_cross
);
criterion_main!(vector3_benches);
