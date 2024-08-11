use std::ops::{Mul, Sub};

#[cfg(feature = "simd")]
pub mod simd;

pub type Pixel = Point<usize>;

pub trait Rasterizer<'a, T> {
    fn rasterize(&mut self, frame: Frame<'a>, block: Block, list: &'a [[Point<T>; 3]]);
}

#[derive(Debug)]
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

#[derive(Debug)]
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
