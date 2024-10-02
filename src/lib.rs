#![feature(step_trait)]

mod option;
mod range;
mod bound;
mod into_iter;
mod step;
mod iterator;

pub unsafe fn panic_begin(nop: &dyn Fn()) {
    // Let any outstanding uart DMA's finish
    for _ in 0..200000 {
        nop();
    }
}
