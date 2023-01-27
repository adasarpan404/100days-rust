// The From trait allows for a type to define how to create itself from another type
// hence providing a very simple mechanism for converting between several types. 
// There are numerous implementations of this trait within the standard library for conversion of primitive and common types.

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}

