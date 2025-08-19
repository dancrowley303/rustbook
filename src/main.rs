pub mod ownership;
pub mod structs;
pub mod enums_and_pattern_matching;

use ownership::references_and_borrowing;
use structs::defining_structs;
use structs::rectangles;
use enums_and_pattern_matching::enums;
use enums_and_pattern_matching::pattern_matching;

fn main() {
    references_and_borrowing::run();
    defining_structs::run();
    rectangles::run();
    enums::run();
    pattern_matching::run();
}
