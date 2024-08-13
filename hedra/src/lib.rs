#![cfg_attr(feature = "simd", feature(portable_simd))]

pub mod buffer;
pub mod math;
pub mod pipeline;
pub mod raster;
pub mod shader;

use std::mem;

macro_rules! num_cast_impl {
    ($a:tt,$b:tt) => {
        impl NumberCast<$b> for $a {
            #[inline(always)]
            fn to_num(self) -> $b {
                self as $b
            }
        }
    };

    ($t:tt) => {
        num_cast_impl!($t, i8);
        num_cast_impl!($t, i16);
        num_cast_impl!($t, i32);
        num_cast_impl!($t, i64);
        num_cast_impl!($t, i128);
        num_cast_impl!($t, isize);
        num_cast_impl!($t, u8);
        num_cast_impl!($t, u16);
        num_cast_impl!($t, u32);
        num_cast_impl!($t, u64);
        num_cast_impl!($t, u128);
        num_cast_impl!($t, usize);
        num_cast_impl!($t, f32);
        num_cast_impl!($t, f64);
    };

    () => {
        num_cast_impl!(i8);
        num_cast_impl!(i16);
        num_cast_impl!(i32);
        num_cast_impl!(i64);
        num_cast_impl!(i128);
        num_cast_impl!(isize);
        num_cast_impl!(u8);
        num_cast_impl!(u16);
        num_cast_impl!(u32);
        num_cast_impl!(u64);
        num_cast_impl!(u128);
        num_cast_impl!(usize);
        num_cast_impl!(f32);
        num_cast_impl!(f64);
    };
}

macro_rules! simd_transmute_impl {
    ($t:tt,$n:literal) => {
        impl SimdTransmute<$t, $n> for std::simd::Simd<$t, $n> {
            #[inline(always)]
            unsafe fn transmute(self) -> [$t; $n] {
                unsafe { mem::transmute(self) }
            }
        }
    };

    ($t:tt) => {
        simd_transmute_impl!($t, 1);
        simd_transmute_impl!($t, 2);
        simd_transmute_impl!($t, 4);
        simd_transmute_impl!($t, 8);
        simd_transmute_impl!($t, 16);
        simd_transmute_impl!($t, 32);
        simd_transmute_impl!($t, 64);
    };

    () => {
        simd_transmute_impl!(i8);
        simd_transmute_impl!(i16);
        simd_transmute_impl!(i32);
        simd_transmute_impl!(i64);
        simd_transmute_impl!(isize);
        simd_transmute_impl!(u8);
        simd_transmute_impl!(u16);
        simd_transmute_impl!(u32);
        simd_transmute_impl!(u64);
        simd_transmute_impl!(usize);
        simd_transmute_impl!(f32);
        simd_transmute_impl!(f64);
    };
}

// Separate trait as to avoid implementing From/Into for primitives everywhere
pub trait NumberCast<T> {
    fn to_num(self) -> T;
}

#[cfg(feature = "simd")]
pub trait SimdTransmute<T, const N: usize>
where
    T: std::simd::SimdElement,
    std::simd::LaneCount<N>: std::simd::SupportedLaneCount,
{
    /// # Safety
    ///
    /// Makes use of [mem::transmute](std::mem::transmute) internally.
    unsafe fn transmute(self) -> [T; N];
}

num_cast_impl!();
#[cfg(feature = "simd")]
simd_transmute_impl!();
