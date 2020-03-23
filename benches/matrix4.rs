#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// returns self to check overhead of benchmark
fn bench_matrix4_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("matrix4 return self");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => ret_self, ty => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix4;
        bench_unop!(b, op => ret_self, ty => Matrix4<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat4;
        bench_unop!(b, op => ret_self, ty => Mat4)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::Wat4;
        bench_unop4!(b, op => ret_self, ty => Wat4)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => ret_self, ty => Matrix4<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::f32x4;
        bench_unop4!(b, op => ret_self, ty => Matrix4<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::f32x8;
        bench_unop8!(b, op => ret_self, ty => Matrix4<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::f32x16;
        bench_unop16!(b, op => ret_self, ty => Matrix4<f32x16>)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => ret_self, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix4 transpose");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => transpose, ty => Mat4);
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix4};
        bench_unop!(b, op => transpose, ty => Matrix4<f32>);
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat4;
        bench_unop!(b, op => transposed, ty => Mat4)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::Wat4;
        bench_unop4!(b, op => transposed, ty => Wat4)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => transpose, ty => Matrix4<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::f32x4;
        bench_unop4!(b, op => transpose, ty => Matrix4<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::f32x8;
        bench_unop8!(b, op => transpose, ty => Matrix4<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::f32x16;
        bench_unop16!(b, op => transpose, ty => Matrix4<f32x16>)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => transposed, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix4 determinant");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => determinant, ty => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix4, SquareMatrix};
        bench_unop!(b, op => determinant, ty => Matrix4<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => determinant, ty => Matrix4<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => determinant, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => determinant, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix4 inverse");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => inverse, ty => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix4, SquareMatrix};
        bench_unop!(b, op => invert, ty => Matrix4<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat4;
        bench_unop!(b, op => inversed, ty => Mat4)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::Wat4;
        bench_unop4!(b, op => inversed, ty => Wat4)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => try_inverse, ty => Matrix4<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => inverse, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => inverted, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_mul_matrix4(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("matrix4 mul matrix4");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix4;
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>, param => by_ref)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Mat4)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::Wat4;
        bench_binop4!(b, op => mul, ty1 => Wat4, ty2 => Wat4)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::f32x4;
        bench_binop4!(b, op => mul, ty1 => Matrix4<f32x4>, ty2 => Matrix4<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::f32x8;
        bench_binop8!(b, op => mul, ty1 => Matrix4<f32x8>, ty2 => Matrix4<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::f32x16;
        bench_binop16!(b, op => mul, ty1 => Matrix4<f32x16>, ty2 => Matrix4<f32x16>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_binop!(b, op => post_transform, ty => Transform3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4<f32>, ty2 => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_mul_vector4(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("matrix4 mul vector4");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::{Mat4, Vec4};
            bench_binop!(b, size, op => mul, ty1 => Mat4, ty2 => Vec4)
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{Matrix4, Vector4};
            bench_binop!(b, size, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Mat4, Vec4};
            bench_binop!(b, size, op => mul, ty1 => Mat4, ty2 => Vec4)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{Wat4, Wec4};
            bench_binop4!(b, size, op => mul, ty1 => Wat4, ty2 => Wec4)
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Matrix4, Vector4};
            bench_binop!(b, size, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
        });
        bench_nalgebra_f32x4!(group, size, |b, size| {
            use nalgebra::{Matrix4, Vector4};
            use simba::simd::f32x4;
            bench_binop4!(b, size, op => mul, ty1 => Matrix4<f32x4>, ty2 => Vector4<f32x4>)
        });
        bench_nalgebra_f32x8!(group, size, |b, size| {
            use nalgebra::{Matrix4, Vector4};
            use simba::simd::f32x8;
            bench_binop8!(b, size, op => mul, ty1 => Matrix4<f32x8>, ty2 => Vector4<f32x8>)
        });
        bench_nalgebra_f32x16!(group, size, |b, size| {
            use nalgebra::{Matrix4, Vector4};
            use simba::simd::f32x16;
            bench_binop16!(b, size, op => mul, ty1 => Matrix4<f32x16>, ty2 => Vector4<f32x16>)
        });
        bench_vek!(group, size, |b, size| {
            use vek::{Mat4, Vec4};
            bench_binop!(b, size, op => mul, ty1 => Mat4<f32>, ty2 => Vec4<f32>)
        });
    }
    group.finish();
}

criterion_group!(
    matrix4_benches,
    bench_matrix4_ret_self,
    bench_matrix4_transpose,
    bench_matrix4_determinant,
    bench_matrix4_inverse,
    bench_matrix4_mul_matrix4,
    bench_matrix4_mul_vector4,
);
criterion_main!(matrix4_benches);
