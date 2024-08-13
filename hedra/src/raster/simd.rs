use std::{
    ops::{Add, BitOr, Div, Mul, Neg, Rem, Sub},
    simd::{cmp::SimdPartialOrd, LaneCount, Mask, Simd, SimdElement, SupportedLaneCount},
};

use crate::{
    math::{One, Zero},
    raster::TriangleEdgeState,
    NumberCast, SimdTransmute,
};

use super::{Rasterizer, Tile, Vec2};

/*
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
*/

#[derive(Debug, Clone)]
pub struct SimdTriangleRastState<T, const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    pub pixel: Vec2<Simd<T, N>>,
}

#[derive(Debug)]
pub struct SimdTriangleRasterizer<T, const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    n_vec: Simd<T, N>,
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
    T: Default + SimdElement + NumberCast<usize>,
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
    type State = SimdTriangleRastState<T, N>;
    type Color = Simd<u32, N>;

    fn rasterize<F: Fn(&Self::State) -> Self::Color>(
        &mut self,
        tile: Tile<'_>,
        list: &'_ [Vec2<T>],
        pixel: F,
    ) {
        debug_assert!(list.len() % 3 == 0);

        let start = tile.position.y * tile.dimensions.x + tile.position.x;
        let end = (tile.position.y + tile.dimensions.y) * tile.dimensions.x
            + (tile.position.x + tile.dimensions.x);
        let edge_width = tile.dimensions.x / N;

        let start_vec = self.n_vec + [start.to_num(); N].into();
        let width_vec = Simd::<T, N>::from_slice(&[tile.dimensions.x.to_num(); N]);

        let state = SimdTriangleRastState {
            pixel: Vec2 {
                x: start_vec % width_vec,
                y: start_vec / width_vec,
            },
        };

        let mut iter = list.iter().map(|v| Vec2 {
            x: Simd::<T, N>::from_slice(&[v.x; N]),
            y: Simd::<T, N>::from_slice(&[v.y; N]),
        });

        while let (Some(v1), Some(v2), Some(v3)) = (iter.next(), iter.next(), iter.next()) {
            let mut state = state.clone();

            let mut edge = TriangleEdgeState::new(
                edge_width,
                Vec2 {
                    x: state.pixel.x,
                    y: state.pixel.y,
                },
                v1,
                v2,
                v3,
            );

            for i in (start..end).step_by(N) {
                let mask = edge.mask();

                edge.step();

                if mask.any() {
                    let i_vec = self.n_vec + [i.to_num(); N].into();

                    state.pixel = Vec2 {
                        x: i_vec % width_vec,
                        y: i_vec / width_vec,
                    };

                    let x = NumberCast::<usize>::to_num(unsafe { state.pixel.x.transmute() }[0]);
                    let y = NumberCast::<usize>::to_num(unsafe { state.pixel.y.transmute() }[0]);

                    let color = pixel(&state);

                    color.store_select(&mut tile.dst[y * tile.dst_width + x..], mask.into());
                }
            }
        }
    }
}
