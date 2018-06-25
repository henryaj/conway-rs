extern crate pxl;

use pxl::*;

fn main() {
    pxl::run::<Conway>();
}

struct Conway {}

impl Program for Conway {
    fn new() -> Self {
        return Conway{};
    }

    fn dimensions() -> (usize, usize) {
        return (700, 500);
    }
}