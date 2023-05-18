#![allow(unused)]

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //Functions - mini programs, organized blocks of code
    who_am_i();
    add_one_hundred(100);
    add(7,7);
    println!("7 x 7 = {}", multiply(7,7));
    let (added, multiplied) = add_and_multiply(4, 3);
    println!("Added: {}", added);
    println!("Multiplied: {}", multiplied);
}
fn who_am_i() {
    let name: &str = "Breno";
    let age: i32 = 28;
    println!("My name is {} and I am {} years old", name, age);

}


fn add_one_hundred(num: i32) {
    println!("{}", num + 100);
}

fn add(x: i32, y: i32){
    println!("{}", x + y);
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn add_and_multiply(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}