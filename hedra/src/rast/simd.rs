use std::{
    ops::{Mul, Sub},
    simd::{cmp::SimdPartialOrd, LaneCount, Mask, Simd, SimdElement, SupportedLaneCount},
};

use super::Point;

#[macro_export]
macro_rules! simd_triangle_rasterizer {
    ($elem:ty,$lanes:literal,$data:ident,$pixel:expr) => {{
        use std::simd::{Mask, Simd};
        use $crate::rast::simd::*;

        let width = $data.block.max.x - $data.block.min.x;
        let height = $data.block.max.y - $data.block.min.y;

        for tri in $data.list {
            for i in (0..width * height).step_by($lanes) {
                let x = i % width + $data.block.min.x;
                let y = i / width + $data.block.min.y;

                let x_as = x as $elem;
                let y_as = y as $elem;

                let x_vec = Simd::<$elem, $lanes>::from_slice(
                    &(x_as..x_as + $lanes).collect::<Vec<$elem>>(),
                );
                let y_vec = Simd::<$elem, $lanes>::from_slice(&[y_as; $lanes]);

                let mask = triangle_mask(Point { x: x_vec, y: y_vec }, tri);

                if mask.any() {
                    let i = y * $data.frame.width + x;
                    let white = !Simd::<u32, $lanes>::default();
                    white.store_select(&mut $data.frame.dst[i..i + $lanes], mask);
                }
            }
        }
    }};
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
