#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// returns self to check overhead of benchmark
fn bench_quaternion_nop(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("quaternion return self");
    bench_glam!(group, |b| {
        use glam::Quat;
        bench_unop!(b, op => ret_self, ty => Quat)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Quaternion;
        bench_unop!(b, op => ret_self, ty => Quaternion<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Rotor3;
        bench_unop!(b, op => ret_self, ty => Rotor3)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::WRotor3;
        bench_unop4!(b, op => ret_self, ty => WRotor3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::UnitQuaternion;
        bench_unop!(b, op => ret_self, ty => UnitQuaternion<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x4;
        bench_unop4!(b, op => ret_self, ty => UnitQuaternion<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x8;
        bench_unop8!(b, op => ret_self, ty => UnitQuaternion<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x16;
        bench_unop16!(b, op => ret_self, ty => UnitQuaternion<f32x16>)
    });
    bench_vek!(group, |b| {
        use vek::Quaternion;
        bench_unop!(b, op => ret_self, ty => Quaternion<f32>)
    });
    group.finish();
}

fn bench_quaternion_conjugate(c: &mut Criterion) {
    let mut group = c.benchmark_group("quaternion conjugate");
    bench_glam!(group, |b| {
        use glam::Quat;
        bench_unop!(b, op => conjugate, ty => Quat)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Quaternion;
        bench_unop!(b, op => conjugate, ty => Quaternion<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Rotor3;
        bench_unop!(b, op => reversed, ty => Rotor3)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::WRotor3;
        bench_unop4!(b, op => reversed, ty => WRotor3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::UnitQuaternion;
        bench_unop!(b, op => conjugate, ty => UnitQuaternion<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x4;
        bench_unop4!(b, op => conjugate, ty => UnitQuaternion<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x8;
        bench_unop8!(b, op => conjugate, ty => UnitQuaternion<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x16;
        bench_unop16!(b, op => conjugate, ty => UnitQuaternion<f32x16>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Rotation3D, UnknownUnit};
        // euclid inverse assumes normalized quaternion, so it's just a conjugate
        bench_unop!(b, op => inverse, ty => Rotation3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Quaternion;
        bench_unop!(b, op => conjugate, ty => Quaternion<f32>)
    });
    group.finish();
}

fn bench_quaternion_mul_quaternion(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("quaternion mul quaternion");
    bench_glam!(group, |b| {
        use glam::Quat;
        bench_binop!(b, op => mul, ty1 => Quat, ty2 => Quat)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Quaternion;
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Quaternion<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Rotor3;
        bench_binop!(b, op => mul, ty1 => Rotor3, ty2 => Rotor3)
    });
    bench_ultraviolet_f32x4!(group, |b| {
        use ultraviolet::WRotor3;
        bench_binop4!(b, op => mul, ty1 => WRotor3, ty2 => WRotor3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::UnitQuaternion;
        bench_binop!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => UnitQuaternion<f32>)
    });
    bench_nalgebra_f32x4!(group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x4;
        bench_binop4!(b, op => mul, ty1 => UnitQuaternion<f32x4>, ty2 => UnitQuaternion<f32x4>)
    });
    bench_nalgebra_f32x8!(group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x8;
        bench_binop8!(b, op => mul, ty1 => UnitQuaternion<f32x8>, ty2 => UnitQuaternion<f32x8>)
    });
    bench_nalgebra_f32x16!(group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x16;
        bench_binop16!(b, op => mul, ty1 => UnitQuaternion<f32x16>, ty2 => UnitQuaternion<f32x16>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Rotation3D, UnknownUnit};
        bench_binop!(b, op => pre_rotate, ty => Rotation3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench_vek!(group, |b| {
        use vek::Quaternion;
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Quaternion<f32>);
    });
    group.finish();
}

fn bench_quaternion_mul_vector3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("quaternion mul vector3");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, |b| {
            use glam::{Quat, Vec3};
            bench_binop!(b, op => mul, ty1 => Quat, ty2 => Vec3)
        });
        bench_cgmath!(group, |b| {
            use cgmath::{Quaternion, Vector3};
            bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Vector3<f32>)
        });
        bench_ultraviolet!(group, |b| {
            use ultraviolet::{Rotor3, Vec3};
            bench_binop!(b, op => mul, ty1 => Rotor3, ty2 => Vec3)
        });
        bench_ultraviolet_f32x4!(group, |b| {
            use ultraviolet::{WRotor3, Wec3};
            bench_binop4!(b, op => mul, ty1 => WRotor3, ty2 => Wec3)
        });
        bench_nalgebra!(group, |b| {
            use nalgebra::{UnitQuaternion, Vector3};
            bench_binop!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => Vector3<f32>)
        });
        bench_nalgebra_f32x4!(group, |b| {
            use nalgebra::{UnitQuaternion, Vector3};
            use simba::simd::f32x4;
            bench_binop4!(b, op => mul, ty1 => UnitQuaternion<f32x4>, ty2 => Vector3<f32x4>)
        });
        bench_nalgebra_f32x8!(group, |b| {
            use nalgebra::{UnitQuaternion, Vector3};
            use simba::simd::f32x8;
            bench_binop8!(b, op => mul, ty1 => UnitQuaternion<f32x8>, ty2 => Vector3<f32x8>)
        });
        bench_nalgebra_f32x16!(group, |b| {
            use nalgebra::{UnitQuaternion, Vector3};
            use simba::simd::f32x16;
            bench_binop16!(b, op => mul, ty1 => UnitQuaternion<f32x16>, ty2 => Vector3<f32x16>)
        });
        bench_euclid!(group, |b| {
            use euclid::{Point3D, Rotation3D, UnknownUnit};
            bench_binop!(b, op => transform_point3d, ty1 => Rotation3D<f32, UnknownUnit, UnknownUnit>, ty2 => Point3D<f32, UnknownUnit>)
        });
        bench_vek!(group, |b| {
            use vek::{Quaternion, Vec3};
            bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Vec3<f32>)
        });
    }
    group.finish();
}

criterion_group!(
    quaternion_benches,
    bench_quaternion_nop,
    bench_quaternion_conjugate,
    bench_quaternion_mul_quaternion,
    bench_quaternion_mul_vector3,
);
criterion_main!(quaternion_benches);
