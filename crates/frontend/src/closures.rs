use std::marker::PhantomData;

pub struct Recursive<'a, A, R> {
    pub function: &'a dyn Fn(&Self, A) -> R,
    _marker: PhantomData<(A, R)>,
}

impl<'a, A, R> Recursive<'a, A, R> {
    pub fn new(f: &'a dyn Fn(&Self, A) -> R) -> Recursive<'a, A, R> {
        Recursive {
            function: f,
            _marker: PhantomData,
        }
    }

    pub fn invoke(&self, args: A) -> R {
        (self.function)(self, args)
    }
}
