#![allow(unused)]

use std::io;

fn main(){
    //Structures are like tuples, but you can name each field and you can have different types
//Structures are similar to objects in JavaScript
//Structures are similar to classes in Python
//Structures are similar to dictionaries in Python
//Structures are similar to hashes in Ruby
//Structures are similar to records in C
//Structures are similar to structs in C++
//Structures are similar to objects in Java
//Structures are similar to objects in C#
//Structures are similar to objects in PHP
//Structures are similar to objects in Go
//Structures are similar to objects in Scala
//Structures are similar to objects in Kotlin
//Structures are similar to objects in Swift
//Structures are similar to objects in TypeScript
//Structures are similar to objects in Visual Basic .NET
//Structures are similar to objects in Objective-C
//Structures are similar to objects in R
//Structures are similar to objects in Perl
//Structures are similar to objects in Rust
//Structures are similar to objects in MATLAB
//Structures are similar to objects in Lua
//Structures are similar to objects in Delphi
//Structures are similar to objects in Pascal
//Structures are similar to objects in Ada
//Structures are similar to objects in Fortran
//Structures are similar to objects in COBOL
//Structures are similar to objects in PL/SQL
//Structures are similar to objects in Transact-SQL


    struct Car{
        make: String,
        model: String,
        year: u32,
        price: f64,

    }

    let mut huracan: Car = Car{
        make: String::from("Lamborghini"),
        model: String::from("Huracan"),
        year: 2019,
        price: 261274.00,
    };
    println!("The cost of a {} {} {} is {}.", huracan.year, huracan.make, huracan.model, huracan.price);

    let mut gallardo: Car = Car{
        make: String::from("Lamborghini"),
        model: String::from("Gallardo"),
        year: 2014,
        price: 181900.00,
    };
    println!("The cost of a {} {} {} is {}.", gallardo.year, gallardo.make, gallardo.model, gallardo.price);

    struct Rectangle{
        width: u32,
        height: u32,
    }
    impl Rectangle{
        fn area(&self) -> u32{
            self.width * self.height
        }
    }
    let rect = Rectangle {width: 30, height: 50};

    let area: u32 = rect.area();

    println!("The area of the rectangle is {}.", area);
}