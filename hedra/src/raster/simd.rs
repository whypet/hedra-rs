use std::{
    ops::{Mul, Sub},
    simd::{cmp::SimdPartialOrd, LaneCount, Mask, Simd, SimdElement, SupportedLaneCount},
};

use super::Point;

#[macro_export]
macro_rules! simd_triangle_rasterizer {
    ($type:ident<$elem:ty,$lanes:literal>) => {
        struct $type {
            n_vec: std::simd::Simd<$elem, $lanes>,
        }

        impl $crate::raster::Rasterizer<'_, $elem> for $type {
            fn new() -> Self {
                use std::simd::Simd;

                Self {
                    n_vec: Simd::<$elem, $lanes>::from_slice(
                        &(0..$lanes).map(|i| i as $elem).collect::<Vec<$elem>>(),
                    ),
                }
            }

            fn rasterize(&mut self, frame: Frame<'_>, block: Block, list: &'_ [[Point<$elem>; 3]]) {
                use std::simd::{Mask, Simd};
                use $crate::raster::simd::*;

                let i = block.min.x * block.min.y;
                let width = block.max.x - block.min.x;
                let height = block.max.y - block.min.y;

                let width_vec = Simd::<$elem, $lanes>::from_slice(&[width as $elem; $lanes]);

                for tri in list {
                    for i in (i..i + width * height).step_by($lanes) {
                        let i_vec = self.n_vec + &[i as $elem; $lanes].into();
                        let x_vec = i_vec % width_vec;
                        let y_vec = i_vec / width_vec;

                        let mask = triangle_mask(Point { x: x_vec, y: y_vec }, tri);

                        if mask.any() {
                            let x = unsafe {
                                std::mem::transmute::<Simd<$elem, $lanes>, [$elem; $lanes]>(x_vec)
                            }[0] as usize;
                            let y = unsafe {
                                std::mem::transmute::<Simd<$elem, $lanes>, [$elem; $lanes]>(y_vec)
                            }[0] as usize;

                            let white = !Simd::<u32, $lanes>::default();
                            white.store_select(&mut frame.dst[y * frame.width + x..], mask);
                        }
                    }
                }
            }
        }
    };
}

#[inline(always)]
fn edge<T, const N: usize>(
    p: Point<Simd<T, N>>,
    v1: Point<Simd<T, N>>,
    v2: Point<Simd<T, N>>,
) -> Simd<T, N>
where
    Simd<T, N>: Sub<Output = Simd<T, N>> + Mul<Output = Simd<T, N>>,
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    super::edge(
        p,
        v1,
        Point {
            x: v2.x - v1.x,
            y: v2.y - v1.y,
        },
    )
}

#[inline(always)]
pub fn triangle_mask<T, const N: usize>(
    p: Point<Simd<T, N>>,
    tri: &[Point<T>; 3],
) -> Mask<T::Mask, N>
where
    Simd<T, N>: Sub<Output = Simd<T, N>>
        + Mul<Output = Simd<T, N>>
        + SimdPartialOrd<Mask = Mask<T::Mask, N>>,
    LaneCount<N>: SupportedLaneCount,
    T: Default + SimdElement,
{
    let v = tri.map(|v| Point {
        x: Simd::<T, N>::from_slice(&[v.x; N]),
        y: Simd::<T, N>::from_slice(&[v.y; N]),
    });

    let edge1 = edge(p, v[0], v[1]);
    let edge2 = edge(p, v[1], v[2]);
    let edge3 = edge(p, v[2], v[0]);

    let zero = Simd::<T, N>::default();

    edge1.simd_ge(zero) & edge2.simd_ge(zero) & edge3.simd_ge(zero)
}
