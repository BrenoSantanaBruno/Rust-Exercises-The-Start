#![allow(unused)]

use std::arch::x86_64::_mm_insert_ps;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main () {
    //Loops - While, FOr, Infinite loop
    // For loops - start to finish of an iterate

    let mut vegetables: [&str; 3] = ["Cumcuber", "Spinach", "Cabbage"];

    for veg in vegetables.iter() {
        println!("Vegetable: {}", veg);
    }


//While loop - execute as long as condition is true

    let mut number: i32 = 1;

    while number < 10 {
        println!("{}!", number);

        number += 1;
    }

    //loop - execute until break

    let mut v: i32 = 0;
    println!("Let's count until infinity!");
    loop {
        v += 1;
        if v == 3 {
            println!("three");
            continue;
        }
        println!("{}", v);
        if v == 5 {
            println!("Ok, that's enough");
            continue;
        }

        if v == 10 {
            println!("Ok, that's enough");
            continue;
        }

        if v == 20 {
            println!("Reched 20, exiting");
        }
        break;
    }

}

