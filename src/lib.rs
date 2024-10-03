#![feature(step_trait)]
#![feature(slice_index_methods)]

use std::ops::Range;

mod bound;
mod index;
mod into_iter;
mod iterator;
mod option;
mod range;
mod step;

#[flux_rs::refined_by(internal_len: int, start: int, end: int)]
pub struct MyStruct<'a, T> {
    #[field({ &[T][internal_len] | internal_len > 0 })]
    internal: &'a [T],
    #[field({ Range<usize>[start, end] | start >= 0 && start < end && end < internal_len })]
    range: Range<usize>,
}

#[flux_rs::refined_by(internal_len: int, start: int, end: int)]
pub struct MyStructMut<'a, T> {
    #[field({ &mut [T][internal_len] | internal_len > 0 })]
    internal: &'a mut [T],
    #[field({ Range<usize>[start, end] | start >= 0 && start < end && end < internal_len })]
    range: Range<usize>,
}

impl<'a, T> MyStruct<'a, T> {
    #[flux_rs::sig(fn(&MyStruct<T>[@internal_len, @start, @end]) -> &[T])]
    fn slice_me_up(&'a self) -> &'a [T] {
        &self.internal[self.range.start..self.range.end]
    }
}

impl<'a, T> MyStructMut<'a, T> {
    #[flux_rs::sig(fn(&mut MyStructMut<T>[@internal_len, @start, @end]) -> &[T])]
    fn slice_me_up(&'a mut self) -> &'a [T] {
        &mut self.internal[self.range.start..self.range.end]
    }
}
