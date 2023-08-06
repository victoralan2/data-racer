mod rng;
use rand::{Rng};
use rng::DataraceRNG;


fn main() {
    let mut random_number_generator = DataraceRNG::default(); // Initialize the random number generator and make it mutable

    // Initialize a mutable slice
    let mut char_slice: [char; 128] = ['a'; 128];
    let mut bool_slice: [bool; 128] = [true; 128];
    let mut u32_slice: [u32; 128] = [0; 128];

    random_number_generator.fill(&mut char_slice);
    random_number_generator.fill(&mut bool_slice);
    random_number_generator.fill(&mut u32_slice);

}