#![feature(step_trait)]
#![feature(slice_index_methods)]
#![allow(unused)]

use std::{ops::{Bound, Index, IndexMut, Range, RangeBounds}, slice::SliceIndex};

mod bound;
mod index;
mod into_iter;
mod iterator;
mod option;
mod range;
mod step;
mod string;

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

    #[flux_rs::sig(fn<R as base>(&mut MyStruct<T>[@ss], 
                                 { R[@r] | <R as RangeBounds<usize>>::start(r) + 1 <= <R as RangeBounds<usize>>::end(r)
                                 })
                )]
    pub fn slice<R: RangeBounds<usize>>(&mut self, range: R) {
        let start = match range.start_bound() {
            Bound::Included(s) => *s,
            Bound::Excluded(s) => *s + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(e) => *e + 1,
            Bound::Excluded(e) => *e,
            Bound::Unbounded => self.range.end - self.range.start,
        };

        // The start is at most start + (start + 1)
        let new_start = self.range.start + start;
        // 
        let new_end = new_start + (end - start);

        self.range = new_start..new_end;
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

#[flux_rs::generics(I as base)]
impl<'a, T, I> Index<I> for MyStructMut<'a, T>
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;

    #[flux_rs::sig(fn(&MyStructMut<T>[@internal_len, @start, @end], I[@idx]) -> &Self::Output requires <I as SliceIndex<[T]>>::in_bounds(idx, end - start))]
    fn index(&self, idx: I) -> &Self::Output {
        &self.internal[self.range.start..self.range.end][idx]
    }
}

#[flux_rs::generics(I as base)]
impl<'a, T, I> IndexMut<I> for MyStructMut<'a, T>
where
    I: SliceIndex<[T]>,
{
    #[flux_rs::sig(fn(&mut MyStructMut<T>[@internal_len, @start, @end], I[@idx]) -> &mut Self::Output requires <I as SliceIndex<[T]>>::in_bounds(idx, end - start))]
    fn index_mut(&mut self, idx: I) -> &mut Self::Output {
        &mut self.internal[self.range.start..self.range.end][idx]
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
    let iter = slice.iter().zip(slice);
}

#[flux_rs::refined_by(offset: int, index: int, bytes_remaining: bool)]
pub struct WriteStruct {
    #[field(usize[offset])]
    offset: usize,
    #[field(usize[index])]
    index: usize,
    #[field(bool[bytes_remaining])]
    bytes_remaining: bool,
}

impl WriteStruct {
    #[flux_rs::sig(
        fn (
            &mut WriteStruct[@offset, @idx, @br], 
            { 
                &str[@s] | str_len(s) > 0 
                // && 
                //(offset > idx => offset - idx <= str_len(s)) 
            }
        ) -> _
    )]
    fn write_str(&mut self, s: &str) {
        let string_len = s.len();
        let offset = self.offset;
        let index = self.index;
        if index + string_len < offset {
            // We are still waiting for `self.offset` bytes to be send before we
            // actually start printing.
            // self.index += string_len;
            return;
        } else {
            // We need to be printing at least some of this.
            let start = if offset <= index {
                // We're past our offset, so we can display this entire str.
                0
            } else {
                // We want to start in the middle.
                offset - index
            };

            let ret = &(s).as_bytes()[start..string_len];
        }
    }
}

#[flux_rs::sig(fn() -> usize[3])]
pub fn str_len_good() -> usize {
    let x = "hog";
    x.len()
}

