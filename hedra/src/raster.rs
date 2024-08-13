use std::ops::{AddAssign, BitOr, Mul, Neg, Sub};

use crate::math::{Vec2, Zero};

#[cfg(feature = "simd")]
pub mod simd;

pub type Pixel = Vec2<usize>;

pub trait Rasterizer<'a, T> {
    type State;
    type Color;

    fn rasterize<F: Fn(&Self::State) -> Self::Color>(
        &mut self,
        tile: Tile<'a>,
        list: &'a [Vec2<T>],
        pixel: F,
    );
}

#[derive(Debug)]
pub struct Tile<'a> {
    pub dst: &'a mut [u32],
    pub dst_width: usize,
    pub position: Vec2<usize>,
    pub dimensions: Vec2<usize>,
}

#[derive(Debug)]
struct TriangleEdgeState<T> {
    i: usize,
    width: usize,
    step: (T, T, T),
    row: (T, T, T),
    edges: (T, T, T),
    last_edges: (T, T, T),
}

impl<T: Copy + AddAssign<T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T>>
    TriangleEdgeState<T>
{
    #[inline(always)]
    pub fn new(width: usize, p: Vec2<T>, v1: Vec2<T>, v2: Vec2<T>, v3: Vec2<T>) -> Self {
        let x1_x3 = v1.x - v3.x;
        let x2_x1 = v2.x - v1.x;
        let x3_x2 = v3.x - v2.x;
        let y1_y3 = v1.y - v3.y;
        let y2_y1 = v2.y - v1.y;
        let y3_y2 = v3.y - v2.y;

        let edges = (
            (x3_x2) * (p.y - v2.y) - (y3_y2) * (p.x - v2.x),
            (x1_x3) * (p.y - v3.y) - (y1_y3) * (p.x - v3.x),
            (x2_x1) * (p.y - v1.y) - (y2_y1) * (p.x - v1.x),
        );

        Self {
            i: 0,
            width,
            step: (-y3_y2, -y1_y3, -y2_y1),
            row: (x3_x2, x1_x3, x2_x1),
            edges,
            last_edges: edges,
        }
    }

    #[inline(always)]
    fn edge_add_step(&mut self) {
        self.edges.0 += self.step.0;
        self.edges.1 += self.step.1;
        self.edges.2 += self.step.2;
    }

    #[inline(always)]
    fn next_row_test(&mut self) {
        if self.i >= self.width {
            self.last_edges.0 += self.row.0;
            self.last_edges.1 += self.row.1;
            self.last_edges.2 += self.row.2;

            self.edges = self.last_edges;

            self.i = 0;
        }
    }

    #[inline(always)]
    pub fn step(&mut self) {
        self.edge_add_step();
        self.i += 1;
        self.next_row_test();
    }

    #[inline(always)]
    pub fn test(&self) -> bool
    where
        T: Zero + BitOr<Output = T> + PartialOrd,
    {
        (self.edges.0 | self.edges.1 | self.edges.2) >= T::ZERO
    }

    #[cfg(feature = "simd")]
    #[inline(always)]
    pub fn mask(&self) -> T::Mask
    where
        T: Zero + BitOr<Output = T> + std::simd::prelude::SimdPartialOrd,
    {
        (self.edges.0 | self.edges.1 | self.edges.2).simd_ge(T::ZERO)
    }
}
