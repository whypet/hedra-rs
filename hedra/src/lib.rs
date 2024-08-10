#![cfg_attr(feature = "simd", feature(portable_simd))]

pub mod buffer;
pub mod raster;

/*
pub struct Pipeline<'a, T, R: Rasterizer<'a, T>> {
    _a: PhantomData<&'a ()>,
    _t: PhantomData<T>,
    rasterizer: R,
}

impl<'a, T, R: Rasterizer<'a, T>> Pipeline<'a, T, R> {
    pub fn new(rasterizer: R) -> Self {
        Self {
            _a: PhantomData,
            _t: PhantomData,
            rasterizer,
        }
    }
}
*/
