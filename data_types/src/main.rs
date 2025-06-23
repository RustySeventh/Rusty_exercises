mod integers_range;
use crate::integers_range::fits_in_type;
mod compilation_error;
use crate::compilation_error::compilation_error;


fn main() {
    println!("Hello, world!");

    println!("{}", fits_in_type(800, "i8"));
    println!("{}", fits_in_type(100, "i8"));
}
