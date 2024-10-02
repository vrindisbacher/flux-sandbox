#![allow(unused)]
use std::slice::SliceIndex;

// #[flux_rs::extern_spec(std::ops)]
// #[flux_rs::assoc(fn slice_index_resulting_len(inp: Self:Output,  res: Self::Output, Self) -> bool)]
// trait SliceIndex<T> {
//     #[flux_rs::sig(fn(Self, &T) -> &Self::Output)]
//     fn index(self, slice: &T) -> &Self::Output;
// }

#[flux_rs::extern_spec(std::ops)]
#[generics(T as base)]
impl<T> SliceIndex<[T]> for Range<usize> {
    #[flux_rs::sig(fn(Range<usize>[@start, @end], &[T][@len]) -> &[T][end - start])]
    fn index(self, slice: &[T]) -> &[T];
}
