use std::{
    ops::{Mul, Not, Sub},
    simd::{cmp::SimdPartialOrd, LaneCount, Mask, Simd, SimdElement, SupportedLaneCount},
};

use super::{Pixel, Point};

#[macro_export]
macro_rules! simd_triangle_rasterizer {
    ($elem:ty,$lanes:literal,$data:ident,$pixel:expr) => {{
        use std::simd::Simd;
        use $crate::rast::simd::*;

        let width = $data.block.max.x - $data.block.min.x;
        let height = $data.block.max.y - $data.block.min.y;

        for tri in $data.list {
            for i in 0..width * height {
                let x = i % width;
                let y = i / width;

                let x_vec = Simd::<$elem, $lanes>::from_slice(
                    Vec::from_iter(x as $elem..(x as usize + $lanes) as $elem).as_slice(),
                );
                let y_vec = Simd::<$elem, $lanes>::from_slice(&[x as $elem; $lanes]);

                let mask = triangle_mask(
                    x_vec,
                    y_vec,
                    [
                        tri[0].clone().into(),
                        tri[1].clone().into(),
                        tri[2].clone().into(),
                    ],
                );

                if mask.any() {
                    $data.frame.dst[y as usize + $data.frame.height as usize * x as usize] =
                        u32::MAX;
                }
            }
        }
    }};
}

#[inline(always)]
fn edge<T, const N: usize>(
    x: Simd<T, N>,
    y: Simd<T, N>,
    v1: (Simd<T, N>, Simd<T, N>),
    v2: (Simd<T, N>, Simd<T, N>),
) -> Simd<T, N>
where
    Simd<T, N>: Sub<Output = Simd<T, N>> + Mul<Output = Simd<T, N>>,
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    super::edge(x, y, v1.0, v1.1, v2.0 - v1.0, v2.1 - v1.1)
}

#[inline(always)]
pub fn triangle_mask<T, const N: usize>(
    x: Simd<T, N>,
    y: Simd<T, N>,
    tri: [Point<T>; 3],
) -> Mask<T::Mask, N>
where
    Simd<T, N>: Sub<Output = Simd<T, N>>
        + Mul<Output = Simd<T, N>>
        + Not<Output = Simd<T, N>>
        + SimdPartialOrd<Mask = Mask<T::Mask, N>>,
    LaneCount<N>: SupportedLaneCount,
    T: Default + SimdElement,
{
    let v = tri.map(|v| {
        (
            Simd::<T, N>::from_slice(&[v.x; N]),
            Simd::<T, N>::from_slice(&[v.y; N]),
        )
    });

    let edge1 = edge(x, y, v[0], v[1]);
    let edge2 = edge(x, y, v[1], v[2]);
    let edge3 = edge(x, y, v[2], v[0]);

    let mask = Simd::<T, N>::default();

    edge1.simd_ge(mask) & edge2.simd_ge(mask) & edge3.simd_ge(mask)
}
