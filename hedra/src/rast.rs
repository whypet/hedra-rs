use std::ops::{Mul, Sub};

#[cfg(feature = "simd")]
pub mod simd;

pub type Pixel = Point<u32>;

pub trait TriangleRasterizer: FnMut(TriangleRasterizerData<'_>) {}

pub struct Frame<'a> {
    pub dst: &'a mut [u32],
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub struct Block {
    pub min: Pixel,
    pub max: Pixel,
}

pub struct TriangleRasterizerData<'a> {
    pub frame: Frame<'a>,
    pub block: Block,
    pub list: &'a [[Pixel; 3]],
}

impl From<Pixel> for Point<i32> {
    fn from(value: Pixel) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

#[inline(always)]
fn edge<F: Sub<Output = F> + Mul<Output = F>>(x: F, y: F, v1_x: F, v1_y: F, dvx: F, dvy: F) -> F {
    (dvx * (y - v1_y)) - (dvy * (x - v1_x))
}

/*
fn triangle_bounds(frame: &Frame<'_>, tri: &[Pixel; 3]) {
    let min = (
        tri.iter().min_by(|a, b| a.x.cmp(&b.x)),
        tri.iter().min_by(|a, b| a.y.cmp(&b.y)),
    );
    let max = (
        tri.iter().max_by(|a, b| a.x.cmp(&b.x)),
        tri.iter().max_by(|a, b| a.y.cmp(&b.y)),
    );
}
*/
