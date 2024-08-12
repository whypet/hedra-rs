#[macro_export]
macro_rules! shader_attr {
    (VertexIndex<$t:tt.$field:ident>) => {
        impl $crate::shader::attr::VertexIndex for $t {
            fn vert_index(&mut self) -> Option<&mut usize> {
                Some(&mut self.$field)
            }
        }
    };
}

pub trait VertexIndex {
    fn vert_index(&mut self) -> Option<&mut usize>;
}

pub struct Test {}
