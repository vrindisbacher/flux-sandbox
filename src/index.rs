use std::slice::SliceIndex;

#[flux_rs::extern_spec(std::ops)]
// #[flux_rs::assoc()]
impl<T, I: SliceIndex<[T]>> Index<I> for [T] {
    #[flux_rs::sig(fn(&Self[@len], I) -> &I::Output)]
    fn index(&self, index: I) -> &I::Output;
}

#[flux_rs::extern_spec(std::ops)]
#[generics(T as base)]
impl<T> SliceIndex<[T]> for Range<usize> {
    #[flux_rs::sig(fn(Range<usize>[@start, @end], &[T][@len]) -> &[T][end - start])]
    fn index(self, slice: &[T]) -> &[T];
}
