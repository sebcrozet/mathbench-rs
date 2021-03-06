#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};

const UPDATE_RATE: f32 = 1.0 / 60.0;

macro_rules! bench_euler {
    ($b: ident, $size:expr, ty => $t: ty, zero => $zero: expr, dt => $dt: expr) => {{
        struct TestData {
            acc: Vec<$t>,
            vel: Vec<$t>,
            pos: Vec<$t>,
        };

        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let mut data = TestData {
            acc: vec![<$t as mathbench::BenchValue>::random_value(&mut rng); *$size],
            vel: vec![<$t as mathbench::BenchValue>::random_value(&mut rng); *$size],
            pos: vec![<$t as mathbench::BenchValue>::random_value(&mut rng); *$size],
        };

        let dt = $dt;
        $b.iter(|| {
            for ((position, acceleration), velocity) in
                data.pos.iter_mut().zip(&data.acc).zip(&mut data.vel)
            {
                *velocity = *velocity + *acceleration * dt;
                *position = *position + *velocity * dt;
            }
        })
    }};
}

fn bench_euler_3d(c: &mut Criterion) {
    let mut group = c.benchmark_group("euler 3d");
    for size in [10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::Vec3;
            bench_euler!(b, size, ty => Vec3, zero => Vec3::zero(), dt => Vec3::splat(UPDATE_RATE))
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{prelude::*, Vector3};
            bench_euler!(b, size, ty => Vector3<f32>, zero => Vector3::zero(), dt => UPDATE_RATE)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::Vec3;
            bench_euler!(b, size, ty => Vec3, zero => Vec3::zero(), dt => UPDATE_RATE)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{f32x4, Wec3};
            bench_euler!(b, &((*size as f32 / 4.0).ceil() as usize), ty => Wec3, zero => Wec3::zero(), dt => f32x4::from(UPDATE_RATE))
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{zero, Vector3};
            bench_euler!(b, size, ty => Vector3<f32>, zero => zero(), dt => UPDATE_RATE);
        });
        bench_nalgebra_f32x4!(group, size, |b, size| {
            use nalgebra::{zero, Vector3};
            use simba::simd::{SimdValue, f32x4};
            bench_euler!(b, &((*size as f32 / 4.0).ceil() as usize), ty => Vector3<f32x4>, zero => zero(), dt => f32x4::splat(UPDATE_RATE));
        });
        bench_nalgebra_f32x8!(group, size, |b, size| {
            use nalgebra::{zero, Vector3};
            use simba::simd::{SimdValue, f32x8};
            bench_euler!(b, &((*size as f32 / 8.0).ceil() as usize), ty => Vector3<f32x8>, zero => zero(), dt => f32x8::splat(UPDATE_RATE));
        });
        bench_nalgebra_f32x16!(group, size, |b, size| {
            use nalgebra::{zero, Vector3};
            use simba::simd::{SimdValue, f32x16};
            bench_euler!(b, &((*size as f32 / 16.0).ceil() as usize), ty => Vector3<f32x16>, zero => zero(), dt => f32x16::splat(UPDATE_RATE));
        });
        bench_euclid!(group, size, |b, size| {
            use euclid::{UnknownUnit, Vector3D};
            bench_euler!(b, size, ty => Vector3D<f32, UnknownUnit>, zero => Vector3D::zero(), dt => UPDATE_RATE);
        });
        bench_vek!(group, size, |b, size| {
            use vek::Vec3;
            bench_euler!(b, size, ty => Vec3<f32>, zero => Vec3::zero(), dt => Vec3::broadcast(UPDATE_RATE))
        });
        bench_pathfinder!(group, size, |b, size| {
            use pathfinder_geometry::vector::Vector4F;
            bench_euler!(b, size, ty => Vector4F, zero => Vector4F::splat(0.0), dt => Vector4F::splat(UPDATE_RATE))
        });
    }
    group.finish();
}

fn bench_euler_2d(c: &mut Criterion) {
    let mut group = c.benchmark_group("euler 2d");
    for size in [10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::Vec2;
            bench_euler!(b, size, ty => Vec2, zero => Vec2::zero(), dt => Vec2::splat(UPDATE_RATE))
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{prelude::*, Vector2};
            bench_euler!(b, size, ty => Vector2<f32>, zero => Vector2::zero(), dt => UPDATE_RATE)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::Vec2;
            bench_euler!(b, size, ty => Vec2, zero => Vec2::zero(), dt => UPDATE_RATE)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{f32x4, Wec2};
            bench_euler!(b, &((*size as f32 / 4.0).ceil() as usize), ty => Wec2, zero => Wec2::zero(), dt => f32x4::from(UPDATE_RATE))
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{zero, Vector2};
            bench_euler!(b, size, ty => Vector2<f32>, zero => zero(), dt => UPDATE_RATE);
        });
        bench_nalgebra_f32x4!(group, size, |b, size| {
            use nalgebra::{zero, Vector2};
            use simba::simd::{SimdValue, f32x4};
            bench_euler!(b, &((*size as f32 / 4.0).ceil() as usize), ty => Vector2<f32x4>, zero => zero(), dt => f32x4::splat(UPDATE_RATE));
        });
        bench_nalgebra_f32x8!(group, size, |b, size| {
            use nalgebra::{zero, Vector2};
            use simba::simd::{SimdValue, f32x8};
            bench_euler!(b, &((*size as f32 / 8.0).ceil() as usize), ty => Vector2<f32x8>, zero => zero(), dt => f32x8::splat(UPDATE_RATE));
        });
        bench_nalgebra_f32x16!(group, size, |b, size| {
            use nalgebra::{zero, Vector2};
            use simba::simd::{SimdValue, f32x16};
            bench_euler!(b, &((*size as f32 / 16.0).ceil() as usize), ty => Vector2<f32x16>, zero => zero(), dt => f32x16::splat(UPDATE_RATE));
        });
        bench_euclid!(group, size, |b, size| {
            use euclid::{UnknownUnit, Vector2D};
            bench_euler!(b, size, ty => Vector2D<f32, UnknownUnit>, zero => Vector2D::zero(), dt => UPDATE_RATE);
        });
        bench_vek!(group, size, |b, size| {
            use vek::Vec2;
            bench_euler!(b, size, ty => Vec2<f32>, zero => Vec2::zero(), dt => Vec2::broadcast(UPDATE_RATE))
        });
        bench_pathfinder!(group, size, |b, size| {
            use pathfinder_geometry::vector::Vector2F;
            bench_euler!(b, size, ty => Vector2F, zero => Vector2F::splat(0.0), dt => Vector2F::splat(UPDATE_RATE))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_euler_2d, bench_euler_3d,);

criterion_main!(benches);
