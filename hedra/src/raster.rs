use std::ops::{Mul, Sub};

use crate::math::Vec2;

#[cfg(feature = "simd")]
pub mod simd;

pub type Pixel = Vec2<usize>;

pub trait Rasterizer<'a, T> {
    fn rasterize(&mut self, frame: Frame<'a>, block: Block, list: &'a [[Vec2<T>; 3]]);
}

#[derive(Debug)]
pub struct Frame<'a> {
    pub dst: &'a mut [u32],
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct Block {
    pub min: Pixel,
    pub max: Pixel,
}

#[inline(always)]
fn edge<T: Sub<Output = T> + Mul<Output = T>>(p: Vec2<T>, v1: Vec2<T>, dv: Vec2<T>) -> T {
    (dv.x * (p.y - v1.y)) - (dv.y * (p.x - v1.x))
}
