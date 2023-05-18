#![allow(unused)]

use std::env::{args, args_os};
use std::io;
use rand::Rng;

fn main() {

    // Match - matching arm & all possible values must be covered

    let candidacy_age: i32 =32;

    match candidacy_age{
        1..=24 => println!("Cannot hold office"),
        25 | 26 | 27 | 28 | 29 => println!("Can run for the house"),
        30..=34 => println!("Can run for the house or senate"),
        35..=i32::MAX => println!("Can run for the President."),
        _ => println!("Are you an infant?")
    };
}