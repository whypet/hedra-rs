use std::{
    ops::{Mul, Sub},
    simd::{cmp::SimdPartialOrd, LaneCount, Mask, Simd, SimdElement, SupportedLaneCount},
};

use super::{Block, Frame, Point};

macro_rules! simd_triangle_rasterizer {
    (overflow_check($frame:ident,$x:ident,$y:ident)) => {
        if $x > $frame.width {
            continue;
        }
        if $y > $frame.height {
            continue;
        }
    };

    (overflow_check<i8>($frame:ident,$x:ident,$y:ident)) => {
        simd_triangle_rasterizer!(overflow_check($frame,$x,$y))
    };
    (overflow_check<$t:tt>($frame:ident,$x:ident,$y:ident)) => {};

    ($type:ident<$elem:tt,$lanes:literal>) => {
        impl Default for $type<$elem, $lanes> {
            fn default() -> Self {
                Self {
                    n_vec: Simd::<$elem, $lanes>::from_slice(
                        &(0..$lanes).map(|i| i as $elem).collect::<Vec<$elem>>(),
                    ),
                }
            }
        }

        impl $crate::raster::Rasterizer<'_, $elem> for $type<$elem, $lanes> {
            fn rasterize(&mut self, frame: Frame<'_>, block: Block, list: &'_ [[Point<$elem>; 3]]) {
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

                            simd_triangle_rasterizer!(overflow_check<$elem>(frame, x, y));

                            let white = !Simd::<u32, $lanes>::default();
                            white.store_select(&mut frame.dst[y * frame.width + x..], mask.into());
                        }
                    }
                }
            }
        }
    };

    ($type:ident<$elem:tt>) => {
        simd_triangle_rasterizer!($type<$elem, 1>);
        simd_triangle_rasterizer!($type<$elem, 2>);
        simd_triangle_rasterizer!($type<$elem, 4>);
        simd_triangle_rasterizer!($type<$elem, 8>);
        simd_triangle_rasterizer!($type<$elem, 16>);
        simd_triangle_rasterizer!($type<$elem, 32>);
        simd_triangle_rasterizer!($type<$elem, 64>);
    };

    ($type:ident) => {
        simd_triangle_rasterizer!($type<f32>);
        simd_triangle_rasterizer!($type<f64>);
        simd_triangle_rasterizer!($type<i8>);
        simd_triangle_rasterizer!($type<i16>);
        simd_triangle_rasterizer!($type<i32>);
        simd_triangle_rasterizer!($type<i64>);
    };
}

pub struct SimdRasterizer<T, const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    n_vec: std::simd::Simd<T, N>,
}

simd_triangle_rasterizer!(SimdRasterizer);

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
