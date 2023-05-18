#![allow(unused)]

use std::io;

fn main() {
    println!("Who goes there?");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name);

    let enter: &str = "You may now Enter";
    println!("Hello there {}. {}", name.trim_end(), enter);
}