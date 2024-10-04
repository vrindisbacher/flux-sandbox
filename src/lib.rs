#![feature(step_trait)]
#![feature(slice_index_methods)]
#![allow(unused)]

use std::ops::Range;

mod bound;
mod index;
mod into_iter;
mod iterator;
mod option;
mod range;
mod step;

flux_rs::defs! {
    fn is_proper_usize(x: int) -> bool { x >= usize::MIN && x <= usize::MAX }
}

#[flux_rs::refined_by(internal_len: int, start: int, end: int)]
#[flux_rs::invariant(internal_len > 0)]
#[flux_rs::invariant(start >= 0 && start <= end && end <= internal_len && is_proper_usize(start) && is_proper_usize(end))]
pub struct MyStruct<'a, T> {
    #[field({ &[T][internal_len] | internal_len > 0 })]
    internal: &'a [T],
    #[field({ Range<usize>[start, end] |  start >= 0 && start <= end && end <= internal_len && is_proper_usize(start) && is_proper_usize(end) })]
    range: Range<usize>,
}

#[flux_rs::refined_by(internal_len: int, start: int, end: int)]
#[flux_rs::invariant(internal_len > 0)]
#[flux_rs::invariant(start >= 0 && start <= end && end <= internal_len && is_proper_usize(start) && is_proper_usize(end))]
pub struct MyStructMut<'a, T> {
    #[field({ &mut [T][internal_len] | internal_len > 0 })]
    internal: &'a mut [T],
    #[field({ Range<usize>[start, end] | start >= 0 && start <= end && end <= internal_len && is_proper_usize(start) && is_proper_usize(end) })]
    range: Range<usize>,
}

impl<'a, T> MyStruct<'a, T> {
    #[flux_rs::sig(fn(&MyStruct<T>[@internal_len, @start, @end]) -> &[T])]
    fn slice_me_up(&'a self) -> &'a [T] {
        &self.internal[self.range.start..self.range.end]
    }

    #[flux_rs::sig(fn(&MyStruct<T>[@internal_len, @start, @end]) -> Option<&[T]>[start >= 0 && start <= end && end <= internal_len])]
    fn slice_me_up_safe(&'a self) -> Option<&'a [T]> {
        self.internal.get(self.range.start..self.range.end)
    }

    #[flux_rs::sig(fn(&MyStruct<T>[@internal_len, @start, @end]) -> Option<&T>[start >= 0 && start < internal_len])]
    fn index_me_safe(&'a self) -> Option<&'a T> {
        self.internal.get(self.range.start)
    }
}

impl<'a, T> MyStructMut<'a, T> {
    #[flux_rs::sig(fn(&mut MyStructMut<T>[@internal_len, @start, @end]) -> &[T])]
    fn slice_me_up(&'a mut self) -> &'a [T] {
        &mut self.internal[self.range.start..self.range.end]
    }

    #[flux_rs::sig(fn(&mut MyStructMut<T>[@internal_len, @start, @end]) -> Option<&mut [T]>[start >= 0 && start <= end && end <= internal_len])]
    fn slice_me_up_safe(&'a mut self) -> Option<&'a mut [T]> {
        self.internal.get_mut(self.range.start..self.range.end)
    }
}

fn test_iter<T>(slice: &[T]) {
    for _ in slice.iter() {}
}

fn test_iter_mut<T>(slice: &mut [T]) {
    for _ in slice.iter_mut() {}
}

fn test_enum<T>(slice: &[T]) {
    for _ in slice.iter().enumerate() {}
}

fn test_skip<T>(slice: &[T]) {
    for _ in slice.iter().skip(1) {}
}

fn test_zip<T>(slice: &[T]) {
    // for _ in slice.iter().zip(slice.iter()) {}
}
