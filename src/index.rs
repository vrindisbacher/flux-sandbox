#![allow(unused)]
use std::ops::Range;
use std::slice::SliceIndex;

#[flux_rs::extern_spec(std::ops)]
trait Index<Idx> {
    #[flux_rs::sig(fn(&Self, Idx) -> &Self::Output)]
    fn index(&self, index: Idx) -> &Self::Output;
}

#[flux_rs::extern_spec(std::ops)]
trait IndexMut<Idx> {
    #[flux_rs::sig(fn(&mut Self, Idx) -> &mut Self::Output)]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}

#[flux_rs::extern_spec(std::slice)]
#[flux::generics(Self as base, T as base)]
#[flux::assoc(fn in_bounds(idx: Self, v: T) -> bool)]
trait SliceIndex<T>
where
    T: ?Sized,
{
}

#[flux_rs::extern_spec(std::ops)]
#[generics(I as base)]
impl<T, I: SliceIndex<[T]>> Index<I> for [T] {
    #[flux::sig(fn (&[T][@len], {I[@idx] | <I as SliceIndex<[T]>>::in_bounds(idx, len)}) -> _)]
    fn index(&self, index: I) -> &<I as SliceIndex<[T]>>::Output;
}

#[flux_rs::extern_spec(std::ops)]
#[flux_rs::generics(I as base)]
impl<T, I: SliceIndex<[T]>> IndexMut<I> for [T] {
    #[flux::sig(fn (&mut [T][@len], {I[@idx] | <I as SliceIndex<[T]>>::in_bounds(idx, len)}) -> _)]
    fn index_mut(&mut self, idx: I) -> &mut <I as SliceIndex<[T]>>::Output;
}

#[flux_rs::extern_spec(std::slice)]
#[flux::assoc(fn in_bounds(idx: Range<int>, len: int) -> bool { idx.start >= 0 && idx.start <= idx.end && idx.end < len } )]
impl<T> SliceIndex<[T]> for Range<usize> {
    #[flux_rs::sig(fn(Range<usize>[@start, @end], &[T][@len]) -> Option<&[T][end - start]>[start >= 0 && start <= end && end < len])]
    fn get(self, slice: &[T]) -> Option<&[T]>;

    #[flux_rs::sig(fn(Range<usize>[@start, @end], &mut [T][@len]) -> Option<&mut [T][end - start]>[start >= 0 && start <= end && end < len])]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]>;

    #[flux_rs::sig(fn(Range<usize>[@start, @end], &[T][@len]) -> &[T][end - start])]
    fn index(self, slice: &[T]) -> &[T];

    #[flux_rs::sig(fn(Range<usize>[@start, @end], &mut [T][@len]) -> &mut [T][end - start])]
    fn index_mut(self, slice: &mut [T]) -> &mut [T];
}
