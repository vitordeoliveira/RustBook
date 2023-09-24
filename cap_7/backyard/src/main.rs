use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};

// NEXT
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// The Glob Operator
use std::collections::*;
// The glob operator is often used when testing to bring everything under test into the tests module;   