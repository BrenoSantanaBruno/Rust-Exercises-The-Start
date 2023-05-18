#![allow(unused)]

use std::io;
use rand::Rng;

fn main(){
    let a: i32 = 5;
    let b: i32 = 10;
    let c: bool = true;
    let d: bool = false;

    println!("a > b: {}", a > b); // false
    println!("a >= b: {}", a >= b); // false
    println!("a < b: {}", a < b); // true
    println!("a <= b: {}", a <= b); // true
    println!("a == b: {}", a == b); // false
    println!("a != b: {}", a != b); // true
    println!("True or False: {}", c || d); // true
    println!("True or True: {}", c || c); // true
    println!("False or False: {}", d || d); // false
    println!("True and False: {}", c && d); // false
    println!("True and True: {}", c && c); // true
    println!("False and False: {}", d && d); // false

}