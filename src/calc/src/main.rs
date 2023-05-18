#![allow(unused)]



use std::io;
use rand::{Rng, thread_rng};

fn main () {
    printlm("Give me a value fox x : ");
    let mut input_x: String = String::new();
    io::stdin().read_line(&mut input_x);


    let x: i32 = input_x.trim().parse().expect("Ações não entre com um inteiro");  // let x = 10

    println("Give me a value fox y : ");
}
