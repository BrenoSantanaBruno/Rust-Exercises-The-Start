#![allow(unused)]

use std::io;
use rand::Rng;

fn main () {
    let x =rand::thread_rng().gen_range(1..101);
    let y =3;

    let x_float:  f64 = x as f64;
    let y_float:  f64 = y as f64;

    println!("{} + {} = {}", x, y, x+y);
    println!("{} * {} = {}", x, y, x*y);
    println!("{} / {} = {}", x, y, x_float/y_float);
    println!("{} % {} = {}", x, y, x % y); //remainder
    println!("{} - {} = {}", x, y, x - y);
    println!("{} ^ {} = {}", x, y, i32::pow(x,  y.try_into().unwrap()));
}
