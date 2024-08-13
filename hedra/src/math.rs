use std::ops::{Add, Div, Mul, Sub};

macro_rules! num_trait_impl {
    (f<$t:tt>) => {
        impl Zero for $t {
            const ZERO: Self = 0.0;
        }
        impl One for $t {
            const ONE: Self = 1.0;
        }
    };

    (f32) => {
        num_trait_impl!(f<f32>);
    };
    (f64) => {
        num_trait_impl!(f<f64>);
    };

    ($t:tt) => {
        impl Zero for $t {
            const ZERO: Self = 0;
        }
        impl One for $t {
            const ONE: Self = 1;
        }
    };

    () => {
        num_trait_impl!(i8);
        num_trait_impl!(i16);
        num_trait_impl!(i32);
        num_trait_impl!(i64);
        num_trait_impl!(i128);
        num_trait_impl!(isize);
        num_trait_impl!(u8);
        num_trait_impl!(u16);
        num_trait_impl!(u32);
        num_trait_impl!(u64);
        num_trait_impl!(u128);
        num_trait_impl!(usize);
        num_trait_impl!(f32);
        num_trait_impl!(f64);
    };
}

#[cfg(feature = "simd")]
macro_rules! num_trait_simd_impl {
    (f<$t:tt,$n:literal>) => {
        impl Zero for std::simd::Simd<$t, $n> {
            const ZERO: Self = std::simd::Simd::<$t, $n>::from_slice(&[0.0; $n]);
        }
        impl One for std::simd::Simd<$t, $n> {
            const ONE: Self = std::simd::Simd::<$t, $n>::from_slice(&[1.0; $n]);
        }
    };

    (f32,$n:literal) => {
        num_trait_simd_impl!(f<f32,$n>);
    };
    (f64,$n:literal) => {
        num_trait_simd_impl!(f<f64,$n>);
    };

    ($t:tt,$n:literal) => {
        impl Zero for std::simd::Simd<$t, $n> {
            const ZERO: Self = std::simd::Simd::<$t, $n>::from_slice(&[0; $n]);
        }
        impl One for std::simd::Simd<$t, $n> {
            const ONE: Self = std::simd::Simd::<$t, $n>::from_slice(&[1; $n]);
        }
    };

    ($t:tt) => {
        num_trait_simd_impl!($t, 1);
        num_trait_simd_impl!($t, 2);
        num_trait_simd_impl!($t, 4);
        num_trait_simd_impl!($t, 8);
        num_trait_simd_impl!($t, 16);
        num_trait_simd_impl!($t, 32);
        num_trait_simd_impl!($t, 64);
    };

    () => {
        num_trait_simd_impl!(f32);
        num_trait_simd_impl!(f64);
        num_trait_simd_impl!(i8);
        num_trait_simd_impl!(i16);
        num_trait_simd_impl!(i32);
        num_trait_simd_impl!(i64);
        num_trait_simd_impl!(isize);
        num_trait_simd_impl!(u8);
        num_trait_simd_impl!(u16);
        num_trait_simd_impl!(u32);
        num_trait_simd_impl!(u64);
        num_trait_simd_impl!(usize);
    };
}

pub trait Zero {
    const ZERO: Self;
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

num_trait_impl!();
#[cfg(feature = "simd")]
num_trait_simd_impl!();

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
