#![feature(portable_simd)]

use hedra::{rast::TriangleRasterizerData, simd_triangle_rasterizer};

fn rasterizer(data: TriangleRasterizerData<'_>) {
    simd_triangle_rasterizer!(i32, 16, data, || {})
}

fn main() {}
