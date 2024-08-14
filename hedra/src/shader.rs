#[macro_export]
macro_rules! shader_params {
    (struct $t:tt $($(@$attr:tt)? $field:ident:$type:tt),*($($vis:tt)*)) => {
        #[derive(Default)]
        $($vis)* struct $t { $(pub $field: $type),* }
    };

    (impl $t:tt<$state_arg:ident:$state:ty>{$($body:tt)*}) => {
        impl std::convert::From<$state> for $t {
            #[allow(unused_variables)]
            fn from($state_arg: $state) -> Self {
                #[allow(clippy::needless_update)]
                Self {
                    $($body)*
                    ..Self::default()
                }
            }
        }
    };

    (impl $t:tt<$state_arg:ident:$state:ty>{$($body:tt)*}@vertex_index $field:ident:$type:tt $($tail:tt)*) => {
        use $crate::pipeline::VertexIndex;
        shader_params!(impl $t<$state_arg: $state> {
            $field: $state_arg.get_vertex_index() as $type,
            $($body)*
        } $($tail)*);
    };

    (impl $t:tt<$state_arg:ident:$state:ty>{$($body:tt)*}@pixel $field:ident:$type:tt $($tail:tt)*) => {
        use $crate::raster::RasterState;
        shader_params!(impl $t<$state_arg: $state> {
            $field: $state_arg.get_pixel(),
            $($body)*
        } $($tail)*);
    };

    (impl $t:tt<$state:ty> {$($body:tt)*} $field:ident:$type:tt $($tail:tt)*) => {
        shader_params!(impl $t<state: $state> { $($body)* } $($tail)*);
    };

    ($t:tt<$state:ty>{$($x:tt)*}) => {
        shader_params!(struct $t $($x)* ());
        shader_params!(impl $t<state: $state> {} $($x)*);
    };

    (pub $t:tt<$state:ty>{$($x:tt)*}) => {
        shader_params!(struct $t $($x)* (pub));
        shader_params!(impl $t<state: $state> {} $($x)*);
    };

    (pub($($vis:tt)+) $t:tt<$state:ty>{$($x:tt)*}) => {
        shader_params!(struct $t $($x)* (pub($($vis)+)));
        shader_params!(impl $t<$state> {} $($x)*);
    };
}

#[derive(Debug)]
pub enum Interpolation<T> {
    Flat(T),
    Linear(T),
    // to be implemented
    // Perspective(T),
}

pub trait VertexShader<T> {
    type VertexInput;
    type VertexOutput;

    fn vertex(input: Self::VertexInput) -> Self::VertexOutput;
}

pub trait PixelShader<T> {
    type PixelInput;
    type PixelOutput;

    fn pixel(input: Self::PixelInput) -> Self::PixelOutput;
}
