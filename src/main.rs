extern crate hackvm;

use std::io::prelude::*;
use std::io::{stdin, stdout};

fn main() {
    let mut input = String::new();

    print!("Enter something: "); // The print! macro doesn't flush stdout, for some reason...
    stdout().flush().ok().expect("Error flushing stdout");
    stdin().read_line(&mut input).ok().expect("Error receiving user input");

    println!("You entered: {}", input);
}
