#![allow(unused)]

use std::io;
use std::ops::Add;
fn main () {
//Generics are a way to abstract over types (and other things) while keeping type safety.
//Generics are abstract stand-ins for concrete types or other properties.
//When we use generics, we don’t need to repeat ourselves and do something like create multiple
//functions that are the same except for the types of the values passed in to the functions.
//Instead, we can use generics to create definitions for items like functions, structs, methods,
//traits, and enums.
//Generics are abstract stand-ins for concrete types or other properties.

    //Generic Data Types
    //In the same way that we can define functions to work with any type, we can also define
    //structs, enums, methods, and even modules that can work with any type.
    //Structs and enums are the most commonly used generics because they provide custom
    //types for a particular use without the need to repeat code.
    //The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to
    //bring it into scope explicitly. In addition, so are its variants: you can use Some and None
    //directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and
    //Some(T) and None are still variants of type Option<T>.
    //The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of
    //type Option<T>.

    fn sum<T:Add <Output=T>> (a: T, b: T) -> T {
        a + b
    }

    let x: i32 = sum(1,2);
    let y: f64 = sum(1.0,2.0);
    let z = println!("1,2");
    println!("x = {}", x);
    println!("x = {}", y);
    println!("The Sum of 3 + 2 = {}", sum(3,2));


    struct Items<T>{
        x: T,
        y: T,
    }

    let i = Items {x: 1.0, y: 2.0};
println!("{}, {}",i.x, i.y)


}