#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_transform_point3(c: &mut Criterion) {
    let mut group = c.benchmark_group("isometry transform point3");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Isometry3, Vec3};
            bench_binop!(b, size, op => transform_vec, ty1 => Isometry3, ty2 => Vec3)
        });
        //        bench_ultraviolet_f32x4!(group, size, |b, size| {
        //            use ultraviolet::{WIsometry3, Wec3};
        //            bench_binop!(b, size, op => transform_vec, ty1 => WIsometry3, ty2 => Wec3)
        //        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Isometry3, Point3};
            bench_binop!(b, size, op => transform_point, ty1 => Isometry3<f32>, ty2 => Point3<f32>, param => by_ref)
        });
        //        bench_nalgebra_f32x4!(group, size, |b, size| {
        //            use nalgebra::{Isometry3, Point3};
        //            use simba::simd::f32x4;
        //            bench_binop!(b, size, op => transform_point, ty1 => Isometry3<f32x4>, ty2 => Point3<f32x4>, param => by_ref)
        //        });
        //        bench_nalgebra_f32x8!(group, size, |b, size| {
        //            use nalgebra::{Isometry3, Point3};
        //            use simba::simd::f32x8;
        //            bench_binop!(b, size, op => transform_point, ty1 => Isometry3<f32x8>, ty2 => Point3<f32x8>, param => by_ref)
        //        });
        //        bench_nalgebra_f32x16!(group, size, |b, size| {
        //            use nalgebra::{Isometry3, Point3};
        //            use simba::simd::f32x16;
        //            bench_binop!(b, size, op => transform_point, ty1 => Isometry3<f32x16>, ty2 => Point3<f32x16>, param => by_ref)
        //        });
    }
    group.finish();
}

fn bench_transform_point2(c: &mut Criterion) {
    let mut group = c.benchmark_group("isometry transform point2");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Isometry2, Vec2};
            bench_binop!(b, size, op => transform_vec, ty1 => Isometry2, ty2 => Vec2)
        });
        //        bench_ultraviolet_f32x4!(group, size, |b, size| {
        //            use ultraviolet::{WIsometry2, Wec2};
        //            bench_binop!(b, size, op => transform_vec, ty1 => WIsometry2, ty2 => Wec2)
        //        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Isometry2, Point2};
            bench_binop!(b, size, op => transform_point, ty1 => Isometry2<f32>, ty2 => Point2<f32>, param => by_ref)
        });
        //        bench_nalgebra_f32x4!(group, size, |b, size| {
        //            use nalgebra::{Isometry2, Point2};
        //            use simba::simd::f32x4;
        //            bench_binop!(b, size, op => transform_point, ty1 => Isometry2<f32x4>, ty2 => Point2<f32x4>, param => by_ref)
        //        });
        //        bench_nalgebra_f32x8!(group, size, |b, size| {
        //            use nalgebra::{Isometry2, Point2};
        //            use simba::simd::f32x8;
        //            bench_binop!(b, size, op => transform_point, ty1 => Isometry2<f32x8>, ty2 => Point2<f32x8>, param => by_ref)
        //        });
        //        bench_nalgebra_f32x16!(group, size, |b, size| {
        //            use nalgebra::{Isometry2, Point2};
        //            use simba::simd::f32x16;
        //            bench_binop!(b, size, op => transform_point, ty1 => Isometry2<f32x16>, ty2 => Point2<f32x16>, param => by_ref)
        //        });
    }
    group.finish();
}

criterion_group!(
    transformation_benches,
    bench_transform_point2,
    bench_transform_point3,
);
criterion_main!(transformation_benches);
