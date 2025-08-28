pub mod ownership;
pub mod structs;
pub mod enums_and_pattern_matching;
pub mod collections;
pub mod errors;
pub mod generics;
pub mod functional_features;

use functional_features::iterators;
fn main() {
    iterators::run();
}
