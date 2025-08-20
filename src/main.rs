pub mod ownership;
pub mod structs;
pub mod enums_and_pattern_matching;
pub mod collections;
pub mod errors;

// use ownership::references_and_borrowing;
// use structs::defining_structs;
// use structs::rectangles;
// use enums_and_pattern_matching::enums;
// use enums_and_pattern_matching::pattern_matching;
// use collections::vectors;
// use collections::strings;
// use collections::hashmaps;
// use errors::unrecoverable;
use errors::recoverable;

fn main() {
    // references_and_borrowing::run();
    // defining_structs::run();
    // rectangles::run();
    // enums::run();
    // pattern_matching::run();
    // vectors::run();
    // strings::run();
    // hashmaps::run();
    // unrecoverable::run();
    recoverable::run();
}
