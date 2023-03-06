pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

macro_rules! impl_sqrt {
    ($id:path) => {
        impl Sqrt for $id {
            type Output = $id;

            fn sqrt(self) -> Self::Output {
                Self::sqrt(self)
            }
        }
    };

    ($id:ident, $($rests:ident),+) => {
        impl_sqrt!($id);
        impl_sqrt!($($rests),*);
    };
}

impl_sqrt!(f64, f32);
