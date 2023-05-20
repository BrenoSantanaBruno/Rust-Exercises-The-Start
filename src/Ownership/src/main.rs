#![allow(unused)]

use std::io;

fn main () {
    let name = String::from("Breno");
    let new_name = String::from (name);
    println!("hello, my name is {}", new_name);
    let eagora: String = new_name;
    // println!("hello, my name is {}", eagora);
}