#![allow(unused)]
use core::ops::{Range, RangeBounds};
use std::iter::Step;

#[flux_rs::extern_spec(core::ops)]
#[flux_rs::refined_by(start: Idx, end: Idx)]
struct Range<Idx> {
    #[field(Idx[start])]
    start: Idx,
    #[field(Idx[end])]
    end: Idx,
}

#[flux_rs::extern_spec(core::ops)]
#[generics(Self as base)]
#[flux_rs::assoc(fn steps_between(start: Self, end: Self) -> bool )]
#[flux_rs::assoc(fn can_step_forward(start: Self, count: int) -> bool )]
#[flux_rs::assoc(fn step_forward(start: Self, count: int) -> Self )]
#[flux_rs::assoc(fn can_step_backward(start: Self, count: int) -> bool )]
#[flux_rs::assoc(fn step_backward(start: Self, count: int) -> Self )]
trait Step {
    #[flux_rs::sig(fn(&Self[@start], &Self[@end]) -> Option<usize>[<Self as Step>::steps_between(start, end)])]
    fn steps_between(start: &Self, end: &Self) -> Option<usize>;

    #[flux_rs::sig(fn(Self[@start], usize[@n]) -> Option<Self>[<Self as Step>::can_step_forward(start, n)])]
    fn forward_checked(start: Self, count: usize) -> Option<Self>;

    #[flux_rs::sig(fn(Self[@start], usize[@n]) -> Option<Self>[<Self as Step>::can_step_backward(start, n)])]
    fn backward_checked(start: Self, count: usize) -> Option<Self>;
}

#[flux_rs::extern_spec(core::ops)]
#[generics(Self as base)]
trait RangeBounds<T> {
    #[flux_rs::sig(fn(&Self) -> Bound<&T>)]
    fn start_bound(&self) -> Bound<&T>;
    #[flux_rs::sig(fn(&Self) -> Bound<&T>)]
    fn end_bound(&self) -> Bound<&T>;
}

#[flux_rs::extern_spec(std::iter)]
#[flux_rs::refined_by(idx: int, inner: I)]
struct Enumerate<I>;

#[flux_rs::extern_spec(std::iter)]
#[flux_rs::generics(Self as base)]
#[flux_rs::assoc(fn done(self: Self) -> bool  )]
#[flux_rs::assoc(fn step(self: Self, other: Self) -> bool )]
trait Iterator {
    #[flux_rs::sig(fn(self: &strg Self[@curr_s]) -> Option<Self::Item>[!<Self as Iterator>::done(curr_s)] ensures self: Self{next_s: <Self as Iterator>::step(curr_s, next_s)})]
    fn next(&mut self) -> Option<Self::Item>;

    #[flux_rs::sig(fn(Self[@s]) -> Enumerate<Self>[0, s])]
    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized;
}

#[flux_rs::extern_spec(std::iter)]
#[generics(Self as base)]
trait IntoIterator {
    #[flux_rs::sig(fn(self: Self) -> Self::IntoIter)]
    fn into_iter(self) -> Self::IntoIter
    where
        Self: Sized;
}

#[flux_rs::extern_spec(std::iter)]
#[flux_rs::assoc(fn steps_between(start: int, end: int) -> bool { start <= end } )]
#[flux_rs::assoc(fn can_step_forward(start: int, count: int) -> bool { start + count <= usize::MAX } )]
#[flux_rs::assoc(fn step_forward(start: int, count: int) -> int { start + count } )]
#[flux_rs::assoc(fn can_step_backward(start: int, count: int) -> bool { start - count >= usize::MIN } )]
#[flux_rs::assoc(fn step_backward(start: int, count: int) -> int { start - count } )]
impl Step for usize {
    #[sig(fn(&usize[@start], &usize[@end]) -> Option<usize[end - start]>[start < end])]
    fn steps_between(start: &usize, end: &usize) -> Option<usize>;

    #[sig(fn(usize[@start], usize[@n]) -> Option<usize[start + n]>[start + n <= usize::MAX])]
    fn forward_checked(start: usize, count: usize) -> Option<usize>;

    #[sig(fn(usize[@start], usize[@n]) -> Option<usize[start - n]>[start - n >= usize::MIN])]
    fn backward_checked(start: usize, count: usize) -> Option<usize>;
}

#[flux_rs::extern_spec(core::ops)]
#[generics(T as base)]
impl<T> RangeBounds<T> for Range<T> {
    #[flux_rs::sig(fn(&Range<T>[@r]) -> Bound<&T>[true, false])]
    fn start_bound(&self) -> Bound<&T>;
    #[flux_rs::sig(fn(&Range<T>[@r]) -> Bound<&T>[true, false])]
    fn end_bound(&self) -> Bound<&T>;
}

#[flux_rs::extern_spec(core::ops)]
#[generics(A as base)]
#[flux_rs::assoc(fn done(r: Range<A>) -> bool { r.start == r.end } )]
#[flux_rs::assoc(fn step(self: Range<A>, other: Range<A>) -> bool { <A as Step>::can_step_forward(self.start, 1) => other.start == <A as Step>::step_forward(self.start, 1) } )]
impl<A: Step> Iterator for Range<A> {
    #[flux_rs::sig(
        fn(self: &strg Range<A>[@old_range]) -> Option<A>[old_range.start < old_range.end]
            ensures self: Range<A>{r: <A as Step>::can_step_forward(old_range.start, 1) => r.start == <A as Step>::step_forward(old_range.start, 1) }
    )]
    fn next(&mut self) -> Option<A>;
}

#[flux_rs::extern_spec(core::ops)]
#[generics(I as base)]
impl <I: Iterator> IntoIterator for I {
    #[flux_rs::sig(fn(self: I) -> I)]
    fn into_iter(self) -> I;
}

#[flux_rs::refined_by(start: int, end: int)]
struct StructWithRange {
    #[field(Range<usize>[start, end])]
    range: Range<usize>,
}

fn test_iter_range() {
    let start: usize = 0;
    let end: usize = 10;
    for _ in start..end {}
}
