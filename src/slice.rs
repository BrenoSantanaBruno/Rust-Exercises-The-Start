#![allow(unused)]

use std::io;

pub(crate) fn main(){
    //let mut input: String = String::new();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; // [i32; 5]
    let slice: &mut[i32] = &mut arr[1..3];     // [2, 3]

    slice[0] = 6;
    slice[1] = 7;

    println!("{:?}", arr);
}