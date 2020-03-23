#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// returns self to check overhead of benchmark
fn bench_matrix2_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("matrix2 return self");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_unop!(b, op => ret_self, ty => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix2;
        bench_unop!(b, op => ret_self, ty => Matrix2<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat2;
        bench_unop!(b, op => ret_self, ty => Mat2)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::Wat2;
        bench_unop4!(b, op => ret_self, ty => Wat2)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_unop!(b, op => ret_self, ty => Matrix2<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Matrix2;
        use simba::simd::f32x4;
        bench_unop4!(b, op => ret_self, ty => Matrix2<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Matrix2;
        use simba::simd::f32x8;
        bench_unop8!(b, op => ret_self, ty => Matrix2<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Matrix2;
        use simba::simd::f32x16;
        bench_unop16!(b, op => ret_self, ty => Matrix2<f32x16>)
    });
    bench_vek!(group, |b| {
        use vek::Mat2;
        bench_unop!(b, op => ret_self, ty => Mat2<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Matrix2x2F;
        bench_unop!(b, op => ret_self, ty => Matrix2x2F)
    });
    group.finish();
}

fn bench_matrix2_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix2 transpose");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_unop!(b, op => transpose, ty => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix2};
        bench_unop!(b, op => transpose, ty => Matrix2<f32>)
    });
    //    bench_ultraviolet!(group, |b| {
    //        use ultraviolet::Mat2;
    //        bench_unop!(b, op => transpose, ty => Mat2)
    //    });
    //    bench_ultraviolet_f32x4!(group, |b| {
    //        use ultraviolet::Wat2;
    //        bench_unop!(b, op => transpose, ty => Wat2)
    //    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_unop!(b, op => transpose, ty => Matrix2<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Matrix2;
        use simba::simd::f32x4;
        bench_unop4!(b, op => transpose, ty => Matrix2<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Matrix2;
        use simba::simd::f32x8;
        bench_unop8!(b, op => transpose, ty => Matrix2<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Matrix2;
        use simba::simd::f32x16;
        bench_unop16!(b, op => transpose, ty => Matrix2<f32x16>)
    });
    bench_vek!(group, |b| {
        use vek::Mat2;
        bench_unop!(b, op => transposed, ty => Mat2<f32>)
    });
    group.finish();
}

fn bench_matrix2_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix2 determinant");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_unop!(b, op => determinant, ty => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix2};
        bench_unop!(b, op => determinant, ty => Matrix2<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_unop!(b, op => determinant, ty => Matrix2<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat2;
        bench_unop!(b, op => determinant, ty => Mat2<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Matrix2x2F;
        bench_unop!(b, op => det, ty => Matrix2x2F)
    });
    group.finish();
}

fn bench_matrix2_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix2 inverse");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_unop!(b, op => inverse, ty => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix2};
        bench_unop!(b, op => invert, ty => Matrix2<f32>)
    });
    //    bench_ultraviolet!(group, |b| {
    //        use ultraviolet::Mat2;
    //        bench_unop!(b, op => inversed, ty => Mat2)
    //    });
    //    bench_ultraviolet_f32x4!(group, |b| {
    //        use ultraviolet::Wat2;
    //        bench_unop!(b, op => inversed, ty => Wat2)
    //    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_unop!(b, op => try_inverse, ty => Matrix2<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Matrix2x2F;
        bench_unop!(b, op => inverse, ty => Matrix2x2F)
    });
    group.finish();
}

fn bench_matrix2_mul_matrix2(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("matrix2 mul matrix2");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_binop!(b, op => mul, ty1 => Mat2, ty2 => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix2;
        bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Matrix2<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat2;
        bench_binop!(b, op => mul, ty1 => Mat2, ty2 => Mat2)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::Wat2;
        bench_binop4!(b, op => mul, ty1 => Wat2, ty2 => Wat2)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Matrix2<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::Matrix2;
        use simba::simd::f32x4;
        bench_binop4!(b, op => mul, ty1 => Matrix2<f32x4>, ty2 => Matrix2<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::Matrix2;
        use simba::simd::f32x8;
        bench_binop8!(b, op => mul, ty1 => Matrix2<f32x8>, ty2 => Matrix2<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::Matrix2;
        use simba::simd::f32x16;
        bench_binop16!(b, op => mul, ty1 => Matrix2<f32x16>, ty2 => Matrix2<f32x16>)
    });
    bench_vek!(group, |b| {
        use vek::Mat2;
        bench_binop!(b, op => mul, ty1 => Mat2<f32>, ty2 => Mat2<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Matrix2x2F;
        bench_binop!(b, op => mul, ty1 => Matrix2x2F, ty2 => Matrix2x2F)
    });
    group.finish();
}

fn bench_matrix2_mul_vector2(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("matrix2 mul vector2");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::{Mat2, Vec2};
            bench_binop!(b, size, op => mul, ty1 => Mat2, ty2 => Vec2)
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{Matrix2, Vector2};
            bench_binop!(b, size, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Mat2, Vec2};
            bench_binop!(b, size, op => mul, ty1 => Mat2, ty2 => Vec2)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{Wat2, Wec2};
            bench_binop4!(b, size, op => mul, ty1 => Wat2, ty2 => Wec2)
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Matrix2, Vector2};
            bench_binop!(b, size, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
        });
        bench_nalgebra_f32x4!(group, size, |b, size| {
            use nalgebra::{Matrix2, Vector2};
            use simba::simd::f32x4;
            bench_binop4!(b, size, op => mul, ty1 => Matrix2<f32x4>, ty2 => Vector2<f32x4>)
        });
        bench_nalgebra_f32x8!(group, size, |b, size| {
            use nalgebra::{Matrix2, Vector2};
            use simba::simd::f32x8;
            bench_binop8!(b, size, op => mul, ty1 => Matrix2<f32x8>, ty2 => Vector2<f32x8>)
        });
        bench_nalgebra_f32x16!(group, size, |b, size| {
            use nalgebra::{Matrix2, Vector2};
            use simba::simd::f32x16;
            bench_binop16!(b, size, op => mul, ty1 => Matrix2<f32x16>, ty2 => Vector2<f32x16>)
        });
        bench_vek!(group, size, |b, size| {
            use vek::{Mat2, Vec2};
            bench_binop!(b, size, op => mul, ty1 => Mat2<f32>, ty2 => Vec2<f32>)
        });
        bench_pathfinder!(group, size, |b, size| {
            use pathfinder_geometry::{transform2d::Matrix2x2F, vector::Vector2F};
            bench_binop!(b, size, op => mul, ty1 => Matrix2x2F, ty2 => Vector2F)
        });
    }
    group.finish();
}

criterion_group!(
    matrix2_benches,
    bench_matrix2_ret_self,
    bench_matrix2_transpose,
    bench_matrix2_determinant,
    bench_matrix2_inverse,
    bench_matrix2_mul_matrix2,
    bench_matrix2_mul_vector2,
);
criterion_main!(matrix2_benches);
