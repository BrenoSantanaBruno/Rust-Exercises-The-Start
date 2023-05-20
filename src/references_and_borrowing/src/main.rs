#![allow(unused)]

use std::io;

fn main(){

    //References and Borrowing
//The issue with the previous section is that we’re trying to use a value after it has been moved.
//Rust has a feature called references that lets you refer to some value without taking ownership
//of it. Instead of passing a value by value, we can pass it by reference.
//The issue with the previous section is that we’re trying to use a value after it has been moved.

    //References &
    // Borrowing has to match the mutability of the borrowed value
    let a: String = String::from("Breno");
    let b: &String  = &a;
    // let b: String = a;
    println!("b = {}", *b);
    // println!("a = {}", a);


    //Mutable References


    let s1: String = String::from("Hello");
    let len: usize = calculate_lenght(&s1);
    println!("The lenght of '{}' is {}.", s1, len);

    let x: i32 = 10;
    let
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}