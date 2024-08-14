use crate::{
    math::Vec2,
    raster::{Rasterizer, Tile},
    shader::{PixelShader, VertexShader},
};

pub trait VertexState {
    type Vertex;

    fn get_vertex_index(&self) -> usize;
    fn get_vertex(&self) -> Self::Vertex;
}

pub trait PixelState {
    type Pixel;

    fn get_pixel(&self) -> Self::Pixel;
}

pub trait Pipeline<'a, T>: Rasterizer<'a, T> + VertexShader<T> + PixelShader<T> {
    fn render(&mut self, _tile: Tile<'a>, mut _list: &'a [Vec2<T>]) {
        // self.rasterize(tile, list, pixel)
    }
}
