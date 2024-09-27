#![allow(unused)]
use core::ops::Range;
use std::iter::Step;

#[flux_rs::extern_spec]
#[flux_rs::refined_by(b: bool)]
enum Option<T> {
    #[variant(Option<T>[false])]
    None,
    #[variant({T} -> Option<T>[true])]
    Some(T),
}

#[flux_rs::extern_spec]
impl<T> Option<T> {
    #[sig(fn(&Option<T>[@b]) -> bool[b])]
    const fn is_some(&self) -> bool;

    #[sig(fn(&Option<T>[@b]) -> bool[!b])]
    const fn is_none(&self) -> bool;
}

#[flux_rs::extern_spec(core::ops)]
#[flux_rs::refined_by(start: Idx, end: Idx)]
struct Range<Idx> {
    #[field(Idx[start])]
    start: Idx,
    #[field(Idx[end])]
    end: Idx,
}

#[flux_rs::extern_spec(std::iter)]
#[generics(Self as base)]
#[flux_rs::assoc(fn done(self: Self) -> bool )]
#[flux_rs::assoc(fn step(self: Self, other: Self) -> bool )]
trait Iterator {
    #[flux_rs::sig(fn(self: &strg Self[@curr_range]) -> Option<Self::Item>[!<Self as Iterator>::done(curr_range)] ensures self: Self{next_range: <Self as Iterator>::step(curr_range, next_range)})]
    fn next(&mut self) -> Option<Self::Item>;
}

#[flux_rs::extern_spec(core::ops)]
#[generics(Idx as base)]
#[flux_rs::assoc(fn done(r: Range<Idx>) -> bool { r.start == r.end } )]
#[flux_rs::assoc(fn step(r1: Range<Idx>, r2: Range<Idx>) -> bool { r1.start + 1 == r2.start } )]
impl<Idx: Step> Iterator for Range<Idx> {
    #[flux_rs::sig(fn(self: &strg Range<Idx>[@curr_range]) -> Option<_>[curr_range.start < curr_range.end] ensures self: Range<Idx>{next_range: curr_range.start + 1 == next_range.start})]
    fn next(&mut self) -> Option<Idx>;
}
