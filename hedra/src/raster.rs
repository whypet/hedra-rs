use std::ops::{Mul, Sub};

#[cfg(feature = "simd")]
pub mod simd;

pub type Pixel = Point<usize>;

pub trait Rasterizer<'a, T> {
    fn new() -> Self;
    fn rasterize(&mut self, frame: Frame<'a>, block: Block, list: &'a [[Point<T>; 3]]);
}

pub struct Frame<'a> {
    pub dst: &'a mut [u32],
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub struct Block {
    pub min: Pixel,
    pub max: Pixel,
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
fn edge<T: Sub<Output = T> + Mul<Output = T>>(p: Point<T>, v1: Point<T>, dv: Point<T>) -> T {
    (dv.x * (p.y - v1.y)) - (dv.y * (p.x - v1.x))
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
