use approx::assert_ulps_eq;
use cgmath;
use glam;
use nalgebra;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

const NUM_ITERS: usize = 1024;

fn random_vec3<R>(rng: &mut R) -> glam::Vec3
where
    R: Rng,
{
    rng.gen()
}

pub fn random_nonzero_vec3<R>(rng: &mut R) -> glam::Vec3
where
    R: Rng,
{
    loop {
        let v: glam::Vec3 = rng.gen();
        if v.length_squared() > 0.01 {
            return v;
        }
    }
}

pub fn random_quat<R>(rng: &mut R) -> glam::Quat
where
    R: Rng,
{
    let yaw = rng.gen();
    let pitch = rng.gen();
    let roll = rng.gen();
    glam::Quat::from_rotation_ypr(yaw, pitch, roll)
}

pub fn random_mat4<R>(rng: &mut R) -> glam::Mat4
where
    R: Rng
{
    glam::Mat4::from_scale_rotation_translation(
        random_nonzero_vec3(rng),
        random_quat(rng),
        random_vec3(rng),
        )
}

#[macro_export]
macro_rules! semi_implicit_euler {
    ($delta_secs: expr, $accel: expr, $vel: expr, $pos: expr) => {{
        for ((position, acceleration), velocity) in
            $pos.iter_mut().zip($accel.iter()).zip($vel.iter_mut())
        {
            *velocity += *acceleration * $delta_secs;
            *position += *velocity * $delta_secs;
        }
    }};
}

fn mat2_mul_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let glam1 = rng.gen::<glam::Mat2>();
    let glam2 = rng.gen::<glam::Mat2>();
    // TODO: order is not consistent
    let glam3 = glam1 * glam2;

    let mint1: mint::ColumnMatrix2<f32> = glam1.into();
    let mint2: mint::ColumnMatrix2<f32> = glam2.into();

    let nalg1: nalgebra::Matrix2<f32> = mint1.into();
    let nalg2: nalgebra::Matrix2<f32> = mint2.into();
    // column vector multiplication order is right to left
    let nalg3 = nalg1 * nalg2;

    let cgm1: cgmath::Matrix2<f32> = mint1.into();
    let cgm2: cgmath::Matrix2<f32> = mint2.into();
    let cgm3 = cgm1 * cgm2;

    // use nalgebra as assumed correct answer
    let mint3: mint::ColumnMatrix2<f32> = nalg3.into();

    assert_ulps_eq!(cgm3, mint3.into());
    assert_ulps_eq!(glam3, mint3.into());
}

fn mat3_mul_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let glam1 = rng.gen::<glam::Mat3>();
    let glam2 = rng.gen::<glam::Mat3>();
    let glam3 = glam1 * glam2;

    let mint1: mint::ColumnMatrix3<f32> = glam1.into();
    let mint2: mint::ColumnMatrix3<f32> = glam2.into();

    let nalg1: nalgebra::Matrix3<f32> = mint1.into();
    let nalg2: nalgebra::Matrix3<f32> = mint2.into();
    // column vector multiplication order is right to left
    let nalg3 = nalg1 * nalg2;

    let cgm1: cgmath::Matrix3<f32> = mint1.into();
    let cgm2: cgmath::Matrix3<f32> = mint2.into();
    let cgm3 = cgm1 * cgm2;

    // use nalgebra as assumed correct answer
    let mint3: mint::ColumnMatrix3<f32> = nalg3.into();

    assert_ulps_eq!(cgm3, mint3.into());
    assert_ulps_eq!(glam3, mint3.into());
}

fn mat4_mul_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let glam1 = rng.gen::<glam::Mat4>();
    let glam2 = rng.gen::<glam::Mat4>();
    let glam3 = glam1 * glam2;

    let mint1: mint::ColumnMatrix4<f32> = glam1.into();
    let mint2: mint::ColumnMatrix4<f32> = glam2.into();

    let nalg1: nalgebra::Matrix4<f32> = mint1.into();
    let nalg2: nalgebra::Matrix4<f32> = mint2.into();
    // column vector multiplication order is right to left
    let nalg3 = nalg1 * nalg2;

    let cgm1: cgmath::Matrix4<f32> = mint1.into();
    let cgm2: cgmath::Matrix4<f32> = mint2.into();
    let cgm3 = cgm1 * cgm2;

    // use nalgebra as assumed correct answer
    let mint3: mint::ColumnMatrix4<f32> = nalg3.into();

    assert_ulps_eq!(cgm3, mint3.into());
    assert_ulps_eq!(glam3, mint3.into());
}

fn mat2_det_compare() {
    use cgmath::prelude::*;

    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let gm1 = rng.gen::<glam::Mat2>();
    let gmd = gm1.determinant();

    let mm1: mint::ColumnMatrix2<f32> = gm1.into();

    let nm1: nalgebra::Matrix2<f32> = mm1.into();
    let nmd = nm1.determinant();

    let cm1: cgmath::Matrix2<f32> = mm1.into();
    let cmd = cm1.determinant();

    // use nalgebra as assumed correct answer
    assert_ulps_eq!(cmd, nmd);
    assert_ulps_eq!(gmd, nmd);
}

fn mat2_inv_compare() {
    use cgmath::prelude::*;

    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mut invertible_mat2 = || loop {
        let tmp = rng.gen::<glam::Mat2>();
        if tmp.determinant() != 0.0 {
            return tmp;
        }
    };
    let gm1 = invertible_mat2();
    let gmi = gm1.inverse();

    let mm1: mint::ColumnMatrix2<f32> = gm1.into();

    let nm1: nalgebra::Matrix2<f32> = mm1.into();
    let nmi = nm1.try_inverse();
    assert!(nmi.is_some());

    // use nalgebra as assumed correct answer
    let mmi: mint::ColumnMatrix2<f32> = nmi.unwrap().into();

    let cm1: cgmath::Matrix2<f32> = mm1.into();
    let cmi = cm1.invert();
    assert!(cmi.is_some());

    assert_ulps_eq!(cmi.unwrap(), mmi.into());
    assert_ulps_eq!(gmi, mmi.into());
}

fn mat3_inv_compare() {
    use cgmath::prelude::*;

    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mut invertible_mat3 = || loop {
        let tmp = rng.gen::<glam::Mat3>();
        if tmp.determinant() != 0.0 {
            return tmp;
        }
    };
    let gm1 = invertible_mat3();
    let gmi = gm1.inverse();

    let mm1: mint::ColumnMatrix3<f32> = gm1.into();

    let nm1: nalgebra::Matrix3<f32> = mm1.into();
    let nmi = nm1.try_inverse();
    assert!(nmi.is_some());

    // use nalgebra as assumed correct answer
    let mmi: mint::ColumnMatrix3<f32> = nmi.unwrap().into();

    let cm1: cgmath::Matrix3<f32> = mm1.into();
    let cmi = cm1.invert();
    assert!(cmi.is_some());

    assert_ulps_eq!(cmi.unwrap(), mmi.into());
    assert_ulps_eq!(gmi, mmi.into());
}

fn mat4_inv_compare() {
    use cgmath::prelude::*;

    let seed = rand::random();
    let mut rng = Xoshiro256Plus::seed_from_u64(seed);

    let gm1 = random_mat4(&mut rng);
    let gmi = gm1.inverse();

    let mm1: mint::ColumnMatrix4<f32> = gm1.into();

    let nm1: nalgebra::Matrix4<f32> = mm1.into();
    let nmi = nm1.try_inverse();
    assert!(nmi.is_some());

    let cm1: cgmath::Matrix4<f32> = mm1.into();
    let cmi = cm1.invert();
    assert!(cmi.is_some());

    // use nalgebra as assumed correct answer
    let mmi: mint::ColumnMatrix4<f32> = nmi.unwrap().into();

    assert_ulps_eq!(cmi.unwrap(), mmi.into(), epsilon = 0.0001);
    assert_ulps_eq!(gmi, mmi.into(), epsilon = 0.0001);
}

#[test]
fn test_mat2_mul() {
    for _ in 0..NUM_ITERS {
        mat2_mul_compare();
    }
}

#[test]
fn test_mat3_mul() {
    for _ in 0..NUM_ITERS {
        mat3_mul_compare();
    }
}

#[test]
fn test_mat4_mul() {
    for _ in 0..NUM_ITERS {
        mat4_mul_compare();
    }
}

#[test]
fn test_mat2_det() {
    for _ in 0..NUM_ITERS {
        mat2_det_compare();
    }
}

#[test]
fn test_mat2_inverse() {
    for _ in 0..NUM_ITERS {
        mat2_inv_compare();
    }
}

#[test]
fn test_mat3_inverse() {
    for _ in 0..NUM_ITERS {
        mat3_inv_compare();
    }
}

#[test]
fn test_mat4_inverse() {
    for _ in 0..NUM_ITERS {
        mat4_inv_compare();
    }
}
