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
#[generics(I as base)]
impl<T, I: SliceIndex<[T]>> Index<I> for [T] { 
    // problem here: How can I refine `I`???
    #[flux_rs::sig(fn(&[T][@slice], idx: I) -> &I::Output::indexed(slice, idx))]
    fn index(&self, index: I) -> &I::Output;
}

#[flux_rs::extern_spec(std::slice)]
#[generics(T as base)]
#[flux_rs::assoc(fn indexed(slice: T, idx: Self) -> int)]
trait SliceIndex<T> {
    #[flux_rs::sig(fn(Self, &T) -> &Self::Output)]
    fn index(self, slice: &T) -> &Self::Output where Self: Sized;
}

#[flux_rs::extern_spec(std::slice)]
#[flux_rs::assoc(fn indexed(slice: int, idx: Range<int>) -> int { idx.end - idx.start })]
impl<T> SliceIndex<[T]> for Range<usize> {
    #[flux_rs::sig(fn(Range<usize>[@start, @end], &[T][@len]) -> &[T][end - start])]
    fn index(self, slice: &[T]) -> &[T];
}
