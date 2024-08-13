use std::{
    ops::{Add, BitOr, Div, Mul, Neg, Rem, Sub},
    simd::{cmp::SimdPartialOrd, LaneCount, Mask, Simd, SimdElement, SupportedLaneCount},
};

use crate::{math::Zero, raster::EdgeState, NumberCast, SimdTransmute};

use super::{Block, Frame, Rasterizer, Vec2};

macro_rules! overflow_check_impl {
    ($t:tt) => {
        impl RasterOverflowCheck for $t {
            #[inline(always)]
            fn overflow_check(_: usize, _: usize, _: usize, _: usize) -> bool {
                false
            }
        }
    };

    () => {
        overflow_check_impl!(i16);
        overflow_check_impl!(i32);
        overflow_check_impl!(i64);
        overflow_check_impl!(isize);
        overflow_check_impl!(u16);
        overflow_check_impl!(u32);
        overflow_check_impl!(u64);
        overflow_check_impl!(usize);
        overflow_check_impl!(f32);
        overflow_check_impl!(f64);
    };
}

trait RasterOverflowCheck {
    fn overflow_check(x: usize, y: usize, width: usize, height: usize) -> bool;
}

impl RasterOverflowCheck for i8 {
    #[inline(always)]
    fn overflow_check(x: usize, y: usize, width: usize, height: usize) -> bool {
        x >= width || y >= height
    }
}

overflow_check_impl!();

#[derive(Debug)]
pub struct SimdTriangleRasterizer<T, const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    n_vec: std::simd::Simd<T, N>,
}

impl<T, const N: usize> Default for SimdTriangleRasterizer<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
    usize: NumberCast<T>,
{
    fn default() -> Self {
        Self {
            n_vec: Simd::<T, N>::from_slice(&(0..N).map(|i| i.to_num()).collect::<Vec<T>>()),
        }
    }
}

impl<T, const N: usize> Rasterizer<'_, T> for SimdTriangleRasterizer<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: Default + SimdElement + NumberCast<usize> + RasterOverflowCheck,
    Simd<T, N>: Zero
        + Add<Output = Simd<T, N>>
        + Sub<Output = Simd<T, N>>
        + Mul<Output = Simd<T, N>>
        + Div<Output = Simd<T, N>>
        + Rem<Output = Simd<T, N>>
        + Neg<Output = Simd<T, N>>
        + BitOr<Output = Simd<T, N>>
        + SimdPartialOrd<Mask = Mask<T::Mask, N>>
        + SimdTransmute<T, N>,
    Mask<i32, N>: From<Mask<T::Mask, N>>,
    Simd<u32, N>: Zero,
    usize: NumberCast<T>,
{
    fn rasterize(&mut self, frame: Frame<'_>, block: Block, list: &'_ [Vec2<T>]) {
        debug_assert!(list.len() % 3 == 0);

        let i = block.min.x * block.min.y;
        let width = block.max.x - block.min.x;
        let height = block.max.y - block.min.y;

        let width_vec = Simd::<T, N>::from_slice(&[width.to_num(); N]);
        let i_vec = self.n_vec + [i.to_num(); N].into();
        let x_vec = i_vec % width_vec;
        let y_vec = i_vec / width_vec;

        let white = !Simd::<u32, N>::ZERO;

        let mut iter = list.iter();

        while let (Some(v1), Some(v2), Some(v3)) = (iter.next(), iter.next(), iter.next()) {
            let v = [v1, v2, v3].map(|v| Vec2 {
                x: Simd::<T, N>::from_slice(&[v.x; N]),
                y: Simd::<T, N>::from_slice(&[v.y; N]),
            });

            let mut edge = EdgeState::new(width / N, Vec2 { x: x_vec, y: y_vec }, v[0], v[1], v[2]);

            for i in (i..i + width * height).step_by(N) {
                let mask = edge.mask();

                edge.step();

                if mask.any() {
                    let x = i % width;
                    let y = i / width;

                    if T::overflow_check(x, y, frame.width, frame.height) {
                        continue;
                    }

                    white.store_select(&mut frame.dst[y * frame.width + x..], mask.into());
                }
            }
        }
    }
}
