use crate::references_and_borrowing::references_and_borrowing;
use crate::structs::structs;
use crate::rectangles::rectangles;

mod references_and_borrowing;
mod structs;
mod rectangles;

fn main() {
    references_and_borrowing();
    structs();
    rectangles();
}
