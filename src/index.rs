#![allow(unused)]
use std::slice::SliceIndex;
use std::ops::Range;

#[flux_rs::extern_spec(std::ops)]
#[generics(Idx as base)]
trait Index<Idx> {
    #[flux_rs::sig(fn(&Self, Idx) -> &Self::Output)]
    fn index(&self, index: Idx) -> &Self::Output;
}

#[flux_rs::extern_spec(std::ops)]
#[generics(T as base)]
impl<T, I: SliceIndex<[T]>> Index<I> for [T] { 
    #[flux_rs::sig(fn(&[T], I) -> &I::Output)]
    fn index(&self, index: I) -> &I::Output;
}

#[flux_rs::extern_spec(std::slice)]
trait SliceIndex<T> {
    #[flux_rs::sig(fn(Self, &T) -> &Self::Output)]
    fn index(self, slice: &T) -> &Self::Output where Self: Sized;
}

#[flux_rs::extern_spec(std::slice)]
impl<T> SliceIndex<[T]> for Range<usize> {
    #[flux_rs::sig(fn(Range<usize>[@start, @end], &[T][@len]) -> &[T][end - start])]
    fn index(self, slice: &[T]) -> &[T];
}
