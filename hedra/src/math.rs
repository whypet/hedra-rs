use std::ops::{Add, Div, Mul, Sub};

macro_rules! one_impl {
    (f<$t:tt>) => {
        impl One for $t {
            const ONE: Self = 1.0;
        }
    };

    (f32) => {
        one_impl!(f<f32>);
    };
    (f64) => {
        one_impl!(f<f64>);
    };

    ($t:tt) => {
        impl One for $t {
            const ONE: Self = 1;
        }
    };

    () => {
        one_impl!(i8);
        one_impl!(i16);
        one_impl!(i32);
        one_impl!(i64);
        one_impl!(i128);
        one_impl!(isize);
        one_impl!(u8);
        one_impl!(u16);
        one_impl!(u32);
        one_impl!(u64);
        one_impl!(u128);
        one_impl!(usize);
        one_impl!(f32);
        one_impl!(f64);
    };
}

#[cfg(feature = "simd")]
macro_rules! one_simd_impl {
    (f<$t:tt,$n:literal>) => {
        impl One for std::simd::Simd<$t, $n> {
            const ONE: Self = std::simd::Simd::<$t, $n>::from_slice(&[1.0; $n]);
        }
    };

    (f32,$n:literal) => {
        one_simd_impl!(f<f32,$n>);
    };
    (f64,$n:literal) => {
        one_simd_impl!(f<f64,$n>);
    };

    ($t:tt,$n:literal) => {
        impl One for std::simd::Simd<$t, $n> {
            const ONE: Self = std::simd::Simd::<$t, $n>::from_slice(&[1; $n]);
        }
    };

    ($t:tt) => {
        one_simd_impl!($t, 1);
        one_simd_impl!($t, 2);
        one_simd_impl!($t, 4);
        one_simd_impl!($t, 8);
        one_simd_impl!($t, 16);
        one_simd_impl!($t, 32);
        one_simd_impl!($t, 64);
    };

    () => {
        one_simd_impl!(f32);
        one_simd_impl!(f64);
        one_simd_impl!(i8);
        one_simd_impl!(i16);
        one_simd_impl!(i32);
        one_simd_impl!(i64);
        one_simd_impl!(isize);
        one_simd_impl!(u8);
        one_simd_impl!(u16);
        one_simd_impl!(u32);
        one_simd_impl!(u64);
        one_simd_impl!(usize);
    };
}

pub trait One {
    const ONE: Self;
}

pub trait Cartesian<T> {
    fn to_barycentric(self, v1: Vec2<T>, v2: Vec2<T>, v3: Vec2<T>) -> Vec3<T>;
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

one_impl!();
#[cfg(feature = "simd")]
one_simd_impl!();

impl<T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>>
    Cartesian<T> for Vec2<T>
{
    fn to_barycentric(self, v1: Vec2<T>, v2: Vec2<T>, v3: Vec2<T>) -> Vec3<T> {
        let x_x3 = self.x - v3.x;
        let x1_x3 = v1.x - v3.x;
        let x3_x2 = v3.x - v2.x;
        let y_y3 = self.y - v3.y;
        let y1_y3 = v1.y - v3.y;
        let y2_y3 = v2.y - v3.y;
        let y3_y1 = v3.y - v1.y;

        let l1_dividend = y2_y3 * x_x3 + x3_x2 * y_y3;
        let l2_dividend = y3_y1 * x_x3 + x1_x3 * y_y3;
        let divisor = y2_y3 * x1_x3 + x3_x2 * y1_y3;

        let l1 = l1_dividend / divisor;
        let l2 = l2_dividend / divisor;

        Vec3 {
            x: l1,
            y: l2,
            z: T::ONE - l1 - l2,
        }
    }
}
