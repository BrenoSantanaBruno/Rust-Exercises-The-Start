#![allow(unused)]

use std::io;
use rand::Rng;

fn main () {
    println!("How Much Money Do You Have?");
    let mut input_money:String = String::new();
    io::stdin().read_line(&mut input_money).expect("Failed to read line");

    let input_money:u32 = input_money.trim().parse().expect("Please type a number!");

    println!("How Old Are You?");
    let mut input_age:String = String::new();
    io::stdin().read_line(&mut input_age).expect("Failed to read line");

    let age: i32 = input_age.trim().parse().expect("Please type a number!");

    if (age >= 21) && (input_money >= 100) {
        println!("You can enter the club!");
    } else if (age >= 21) && (input_money < 100) {
        println!("You can enter the club, but you need to pay the entrance fee!");
    } else {
        println!("You can't enter the club!");
    }

}

