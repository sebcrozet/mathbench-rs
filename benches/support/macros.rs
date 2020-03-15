#[macro_export]
macro_rules! bench_lib {
    ($libname:literal, $group:ident, $size:expr, $closure:expr) => {
        #[cfg(feature = $libname)]
        $group.bench_with_input(
            criterion::BenchmarkId::new($libname, $size),
            $size,
            $closure,
        )
    };
    ($libname:literal, $group:ident, $closure:expr) => {
        #[cfg(feature = $libname)]
        $group.bench_function($libname, $closure)
    };
}

#[macro_export]
macro_rules! bench_glam {
    ($group:ident, $closure:expr) => {
        // bench_lib!("glam", $group, $closure)
        $group.bench_function("glam", $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        // bench_lib!("glam", $group, $size, $closure)
        $group.bench_with_input(criterion::BenchmarkId::new("glam", $size), $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_cgmath {
    ($group:ident, $closure:expr) => {
        bench_lib!("cgmath", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("cgmath", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_euclid {
    ($group:ident, $closure:expr) => {
        bench_lib!("euclid", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("euclid", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_ultraviolet {
    ($group:ident, $closure:expr) => {
        bench_lib!("ultraviolet", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("ultraviolet", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra_f32x4 {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra_f32x4", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra_f32x4", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra_f32x8 {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra_f32x8", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra_f32x8", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra_f32x16 {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra_f32x16", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra_f32x16", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_vek {
    ($group:ident, $closure:expr) => {
        bench_lib!("vek", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("vek", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_pathfinder {
    ($group:ident, $closure:expr) => {
        #[cfg(feature = "pathfinder_geometry")]
        $group.bench_function("pathfinder", $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        #[cfg(feature = "pathfinder_geometry")]
        $group.bench_with_input(
            criterion::BenchmarkId::new("pathfinder", $size),
            $size,
            $closure,
        )
    };
}

#[macro_export]
macro_rules! bench_unop {
    ($b: ident, op => $unop: ident, ty => $t:ty) => {{
        const SIZE: usize = 1 << 13;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let inputs = criterion::black_box(
            (0..SIZE)
                .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        // pre-fill output vector with some random value
        let mut outputs = vec![<$t as mathbench::BenchValue>::random_value(&mut rng).$unop(); SIZE];
        let mut i = 0;
        $b.iter(|| {
            i = (i + 16) & (SIZE - 1);
            unsafe { *outputs.get_unchecked_mut(i + 00) = inputs.get_unchecked(i + 00).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 01) = inputs.get_unchecked(i + 01).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 02) = inputs.get_unchecked(i + 02).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 03) = inputs.get_unchecked(i + 03).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 04) = inputs.get_unchecked(i + 04).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 05) = inputs.get_unchecked(i + 05).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 06) = inputs.get_unchecked(i + 06).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 07) = inputs.get_unchecked(i + 07).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 08) = inputs.get_unchecked(i + 08).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 09) = inputs.get_unchecked(i + 09).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 10) = inputs.get_unchecked(i + 10).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 11) = inputs.get_unchecked(i + 11).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 12) = inputs.get_unchecked(i + 12).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 13) = inputs.get_unchecked(i + 13).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 14) = inputs.get_unchecked(i + 14).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 15) = inputs.get_unchecked(i + 15).$unop() }
        });
        criterion::black_box(outputs);
    }};
}

#[macro_export]
macro_rules! bench_unop4 {
    ($b: ident, op => $unop: ident, ty => $t:ty) => {{
        const SIZE: usize = 1 << 13;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let inputs = criterion::black_box(
            (0..SIZE)
                .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        // pre-fill output vector with some random value
        let mut outputs = vec![<$t as mathbench::BenchValue>::random_value(&mut rng).$unop(); SIZE];
        let mut i = 0;
        $b.iter(|| {
            i = (i + 4) & (SIZE - 1);
            unsafe { *outputs.get_unchecked_mut(i + 00) = inputs.get_unchecked(i + 00).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 01) = inputs.get_unchecked(i + 01).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 02) = inputs.get_unchecked(i + 02).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 03) = inputs.get_unchecked(i + 03).$unop() }
        });
        criterion::black_box(outputs);
    }};
}

#[macro_export]
macro_rules! bench_unop8 {
    ($b: ident, op => $unop: ident, ty => $t:ty) => {{
        const SIZE: usize = 1 << 13;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let inputs = criterion::black_box(
            (0..SIZE)
                .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        // pre-fill output vector with some random value
        let mut outputs = vec![<$t as mathbench::BenchValue>::random_value(&mut rng).$unop(); SIZE];
        let mut i = 0;
        $b.iter(|| {
            i = (i + 2) & (SIZE - 1);
            unsafe { *outputs.get_unchecked_mut(i + 00) = inputs.get_unchecked(i + 00).$unop() }
            unsafe { *outputs.get_unchecked_mut(i + 01) = inputs.get_unchecked(i + 01).$unop() }
        });
        criterion::black_box(outputs);
    }};
}

#[macro_export]
macro_rules! bench_unop16 {
    ($b: ident, op => $unop: ident, ty => $t:ty) => {{
        const SIZE: usize = 1 << 13;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let inputs = criterion::black_box(
            (0..SIZE)
                .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        // pre-fill output vector with some random value
        let mut outputs = vec![<$t as mathbench::BenchValue>::random_value(&mut rng).$unop(); SIZE];
        let mut i = 0;
        $b.iter(|| {
            i = (i + 1) & (SIZE - 1);
            unsafe { *outputs.get_unchecked_mut(i + 00) = inputs.get_unchecked(i + 00).$unop() }
        });
        criterion::black_box(outputs);
    }};
}

#[macro_export]
macro_rules! by_value {
    ($e:expr) => {
        *$e
    };
}

#[macro_export]
macro_rules! by_ref {
    ($e:expr) => {
        $e
    };
}

#[macro_export]
macro_rules! bench_binop {
    ($b: ident, $size:expr, op => $binop: ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {{
        const SIZE: usize = 1 << 13;
        let batch_size = SIZE * $size;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        // generate input arrays
        let inputs1 = criterion::black_box(
            (0..batch_size)
                .map(|_| <$t1 as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        let inputs2 = criterion::black_box(
            (0..batch_size)
                .map(|_| <$t2 as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        // pre-fill output vector with some random value
        let mut outputs = vec![<$t1 as mathbench::BenchValue>::random_value(&mut rng).$binop($param!(&<$t2 as mathbench::BenchValue>::random_value(&mut rng))); batch_size];
        let mut i = 0;
        if *$size == 1usize {
            $b.iter(|| {
                // minimise overhead of accessing random data using get unchecked
                i = (i + 1) & (SIZE - 1);
                unsafe {
                    *outputs.get_unchecked_mut(i) = inputs1.get_unchecked(i).$binop($param!(inputs2.get_unchecked(i)))
                }
            })
        } else {
            $b.iter(|| {
                // minimise overhead of accessing random data using get unchecked
                i = (i + 1) & (SIZE - 1);
                let start = i * $size;
                let end = start + $size;
                for j in start..end {
                    unsafe {
                        *outputs.get_unchecked_mut(j) = inputs1.get_unchecked(j).$binop($param!(inputs2.get_unchecked(j)))
                    }
                }
            })
        }
    }};
    ($b: ident, op => $binop: ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {{
        bench_binop!($b, &1, op => $binop, ty1 => $t1, ty2 => $t2, param => $param)
    }};
    ($b: ident, $size: expr, op => $binop: ident, ty1 => $ty1:ty, ty2 => $ty2:ty) => {{
        bench_binop!($b, $size, op => $binop, ty1 => $ty1, ty2 => $ty2, param => by_value)
    }};
    ($b: ident, $size:expr, op => $binop: ident, ty1 => $ty1:ty, ty2 => $ty2:ty) => {{
        bench_binop!($b, $size, op => $binop, ty1 => $ty1, ty2 => $ty2, param => by_value)
    }};
    ($b: ident, op => $binop: ident, ty1 => $ty1:ty, ty2 => $ty2:ty) => {{
        bench_binop!($b, op => $binop, ty1 => $ty1, ty2 => $ty2, param => by_value)
    }};
    ($b: ident, $size:expr, op => $binop: ident, ty => $ty:ty, param => $param:tt) => {{
        bench_binop!($b, $size, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    }};
    ($b: ident, op => $binop: ident, ty => $ty:ty, param => $param:tt) => {{
        bench_binop!($b, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    }};
}
