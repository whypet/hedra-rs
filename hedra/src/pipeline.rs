use std::marker::PhantomData;

use crate::{
    raster::Rasterizer,
    shader::{PixelShader, VertexShader},
};

pub struct Pipeline<'a, T, R: Rasterizer<'a, T>, VS: VertexShader<T>, PS: PixelShader<VS, T>> {
    _a: PhantomData<&'a ()>,
    _t: PhantomData<T>,
    rasterizer: R,
    vertex_shader: VS,
    pixel_shader: PS,
}

impl<'a, T, R: Rasterizer<'a, T>, VS: VertexShader<T>, PS: PixelShader<VS, T>>
    Pipeline<'a, T, R, VS, PS>
{
    pub fn new(rasterizer: R, vertex_shader: VS, pixel_shader: PS) -> Self {
        Self {
            _a: PhantomData,
            _t: PhantomData,
            rasterizer,
            vertex_shader,
            pixel_shader,
        }
    }
}
