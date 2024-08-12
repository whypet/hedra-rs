use std::{
    ops::{Add, Div, Mul, Rem, Sub},
    simd::{cmp::SimdPartialOrd, LaneCount, Mask, Simd, SimdElement, SupportedLaneCount},
};

use crate::{NumberCast, SimdTransmute};

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
pub struct SimdRasterizer<T, const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    n_vec: std::simd::Simd<T, N>,
}

impl<T, const N: usize> Default for SimdRasterizer<T, N>
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

impl<T, const N: usize> Rasterizer<'_, T> for SimdRasterizer<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: Default + SimdElement + NumberCast<usize> + RasterOverflowCheck,
    usize: NumberCast<T>,
    Simd<T, N>: Add<Output = Simd<T, N>>
        + Sub<Output = Simd<T, N>>
        + Mul<Output = Simd<T, N>>
        + Div<Output = Simd<T, N>>
        + Rem<Output = Simd<T, N>>
        + SimdPartialOrd<Mask = Mask<T::Mask, N>>
        + SimdTransmute<T, N>,
    Mask<i32, N>: From<Mask<T::Mask, N>>,
{
    fn rasterize(&mut self, frame: Frame<'_>, block: Block, list: &'_ [[Vec2<T>; 3]]) {
        let i = block.min.x * block.min.y;
        let width = block.max.x - block.min.x;
        let height = block.max.y - block.min.y;

        let width_vec = Simd::<T, N>::from_slice(&[width.to_num(); N]);

        for tri in list {
            for i in (i..i + width * height).step_by(N) {
                let i_vec = self.n_vec + [i.to_num(); N].into();
                let x_vec = i_vec % width_vec;
                let y_vec = i_vec / width_vec;

                let mask = triangle_mask(Vec2 { x: x_vec, y: y_vec }, tri);

                if mask.any() {
                    let x = NumberCast::<usize>::to_num(unsafe { x_vec.transmute() }[0]);
                    let y = NumberCast::<usize>::to_num(unsafe { y_vec.transmute() }[0]);

                    if T::overflow_check(x, y, frame.width, frame.height) {
                        continue;
                    }

                    let white = !Simd::<u32, N>::default();
                    white.store_select(&mut frame.dst[y * frame.width + x..], mask.into());
                }
            }
        }
    }
}

#[inline(always)]
fn edge<T, const N: usize>(
    p: Vec2<Simd<T, N>>,
    v1: Vec2<Simd<T, N>>,
    v2: Vec2<Simd<T, N>>,
) -> Simd<T, N>
where
    Simd<T, N>: Sub<Output = Simd<T, N>> + Mul<Output = Simd<T, N>>,
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    super::edge(
        p,
        v1,
        Vec2 {
            x: v2.x - v1.x,
            y: v2.y - v1.y,
        },
    )
}

#[inline(always)]
pub fn triangle_mask<T, const N: usize>(p: Vec2<Simd<T, N>>, tri: &[Vec2<T>; 3]) -> Mask<T::Mask, N>
where
    Simd<T, N>: Sub<Output = Simd<T, N>>
        + Mul<Output = Simd<T, N>>
        + SimdPartialOrd<Mask = Mask<T::Mask, N>>,
    LaneCount<N>: SupportedLaneCount,
    T: Default + SimdElement,
{
    let v = tri.map(|v| Vec2 {
        x: Simd::<T, N>::from_slice(&[v.x; N]),
        y: Simd::<T, N>::from_slice(&[v.y; N]),
    });

    let edge1 = edge(p, v[0], v[1]);
    let edge2 = edge(p, v[1], v[2]);
    let edge3 = edge(p, v[2], v[0]);

    let zero = Simd::<T, N>::default();

    edge1.simd_ge(zero) & edge2.simd_ge(zero) & edge3.simd_ge(zero)
}
