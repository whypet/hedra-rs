pub mod attr;

use crate::math::Vec4;

#[macro_export]
macro_rules! shader_params {
    (struct $t:tt $($(@$attr:tt)? $field:ident:$type:ty),*) => {
        struct $t { $(pub $field: $type),* }
    };

    (impl $t:tt $($(@$attr:tt)? $field:ident:$type:ty)*) => {
        $($($crate::shader_attr!($attr<$t.$field>);)?)*
    };

    ($t:tt{$($x:tt)*}) => {
        shader_params!(struct $t $($x)*);
        shader_params!(impl $t $($x)*);
    };
}

shader_params!(TestShaderParams {
    @VertexIndex index: usize
});

#[derive(Debug)]
pub enum Interpolation<T> {
    Flat(T),
    Linear(T),
    // to be implemented
    // Perspective(T),
}

pub trait Shader {
    type Input;
    type Output;

    fn main(input: Self::Input) -> Self::Output;
}

pub trait VertexShader<T>: Shader {}
pub trait PixelShader<VS: VertexShader<T>, T>:
    Shader<Input = VS::Output, Output = Vec4<T>>
{
}
