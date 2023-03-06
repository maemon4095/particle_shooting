use std::{
    iter::Sum,
    mem::{self, MaybeUninit},
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::Sqrt;

pub struct Vector<T: Sized, const N: usize>([T; N]);

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Copy + Add, const N: usize> Add for Vector<T, N>
where
    T::Output: Sized,
{
    type Output = Vector<T::Output, N>;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Vector::unsafe_from_iter(self.iter().zip(rhs.iter()).map(|(&l, &r)| l + r)) }
    }
}

impl<T: Copy + AddAssign, const N: usize> AddAssign for Vector<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] += rhs[i];
        }
    }
}

impl<T: Copy + Sub, const N: usize> Sub for Vector<T, N>
where
    T::Output: Sized,
{
    type Output = Vector<T::Output, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Vector::unsafe_from_iter(self.iter().zip(rhs.iter()).map(|(&l, &r)| l - r)) }
    }
}

impl<T: Copy + SubAssign, const N: usize> SubAssign for Vector<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] -= rhs[i];
        }
    }
}

impl<T: Copy + Mul, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Vector<T::Output, N>;

    fn mul(self, rhs: T) -> Self::Output {
        unsafe { Vector::unsafe_from_iter(self.iter().map(|&e| e * rhs)) }
    }
}
impl<T: Copy + MulAssign, const N: usize> MulAssign<T> for Vector<T, N> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            self[i] *= rhs;
        }
    }
}

impl<T: Copy + Neg, const N: usize> Neg for Vector<T, N> {
    type Output = Vector<T::Output, N>;

    fn neg(self) -> Self::Output {
        unsafe { Vector::unsafe_from_iter(self.iter().map(|&e| -e)) }
    }
}

impl<T: Copy, const N: usize> Copy for Vector<T, N> {}
impl<T: Clone, const N: usize> Clone for Vector<T, N> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;

    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Vector<T, N> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Vector<T, N> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: Default + Copy, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Self([Default::default(); N])
    }
}

impl<T, const N: usize> Vector<T, N> {
    pub const fn size(&self) -> usize {
        N
    }

    pub fn new(array: [T; N]) -> Vector<T, N> {
        return Vector(array);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }

    unsafe fn unsafe_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vector<T, N> {
        let mut array: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
        let iter = iter.into_iter();
        for (i, e) in iter.enumerate() {
            array[i].write(e);
        }
        Vector(mem::transmute_copy(&array))
    }
}

impl<T: Copy + Mul, const N: usize> Vector<T, N>
where
    T::Output: Sum,
{
    pub fn dot(self, rhs: Self) -> <T as Mul>::Output {
        self.iter().zip(rhs.iter()).map(|(&l, &r)| l * r).sum()
    }

    pub fn square_length(self) -> T::Output {
        self.iter().map(|&x| x * x).sum()
    }
}

impl<T: Copy + Mul, const N: usize> Vector<T, N>
where
    T::Output: Sum + Sqrt,
{
    pub fn length(self) -> <T::Output as Sqrt>::Output {
        self.square_length().sqrt()
    }
}

impl<T: Copy + Mul<Output = T> + Sum + Sqrt<Output = T> + Div, const N: usize> Vector<T, N> {
    pub fn normalized(self) -> Vector<<T as Div>::Output, N> {
        let len = self.length();
        unsafe { Vector::unsafe_from_iter(self.iter().map(|&e| e / len)) }
    }
}
#[macro_export]
macro_rules! vector {
    ($($x:expr),*) => {
        Vector::new([$($x),*])
    };
    ($e:expr; $n:expr) => {
        Vector::new([$e; $n])
    };
}
