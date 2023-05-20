#![allow(unused)]

use std::fmt::Error;
use std::io;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::fs;

fn main () {
    // HELPER METHODS - unwarp(), expect(), and panic!() - are not the best way to handle errors
    // unwarp() - will panic!() if the Result is an Err value
    // expect() - allows you to choose the panic!() error message
    // panic!() - will print a backtrace and exit the program
    // Two types of errors: recoverable and unrecoverable
    // Recoverable errors are those that a program can be expected to be able to recover from ... e.g. a file not found
    // Unrecoverable errors are those that a program cannot be expected to be able to recover from ... e.g. a bug
    // Rust does not have exceptions
    // Rust uses the Result<T, E> enum to encode error handling information
    // Result<T, E> has two variants: Ok(T) and Err(E)
    // The T and E are generic type parameters
    // The T represents the type of the value that will be returned in a success case within the Ok variant
    // The E represents the type of the error that will be returned in a failure case within the Err variant
    // The Result<T, E> enum is defined as follows:
    // enum Result<T, E> { Ok(T), Err(E), }
    // The std::io::Result<T> type is a type alias for Result<T, io::Error>
    // The io::Error type is a struct that represents an error that can occur when performing I/O operations
    // The io::Error struct is defined as follows:
    // pub struct Error { repr: Repr, }
    // enum Repr { Custom(Box<Custom>), Os(i32), }
    // struct Custom { kind: ErrorKind, error: Box<error::Error + Send + Sync>, }
    // enum ErrorKind { NotFound, PermissionDenied, ConnectionRefused, ConnectionReset, ConnectionAborted, NotConnected, AddrInUse, AddrNotAvailable, BrokenPipe, AlreadyExists, WouldBlock, InvalidInput, InvalidData, TimedOut, WriteZero, Interrupted, Other, }
    // The io::ErrorKind enum is defined as follows: ... see above
    // The io::ErrorKind enum is a list of the different kinds of errors that can occur when performing I/O operations
    // The io::ErrorKind enum is used to create an io::Error value
    // Option and Result are both enumerations that encode error handling information
    // Option<T> has two variants: Some(T) and None

    // let result: Result<i32, String> = divide(10, 0);
    // match result {
    //     Ok(value) => println!("The result is {}", value),
    //     Err(msg) => println!("Error: {}", msg),
    // }

    // let result: Result<i32, String> = divide(10, 2).unwrap();
    // let result = divide(10, 0).unwrap();
    // println!("The result is {}", result);
// ? Operator - can only be used in functions that return Result or Option
// ? Operator - can only be used in functions that return Result or Option
    // let result = divide(10, 0)?;
    // println!("The result is {}", result);


//     let result = divide(10, 2);
//     println!("The result is {}", result);
//
//
//     println!("The show must go on!");
// }
//
// fn divide(x: i32, y: i32) -> i32 {
//     if y== 0 {
//         panic!("Cannot divide by zero");
//
//    }
// fn divide(x: i32, y: i32) -> i32 {
//     if y== 0 {
//         return Err(String::from("Cannot divide by zero"));
//
//     }
//     x / y

    let result = read_file("test.txt");
    match result {
        Ok(contents) => println!("The file contains: {}", contents),
        Err(err: Error) => println!("Error: {}", err),
    }

}

fn read_file(path: &str) -> Result<String, std::io::Error>{
    let mut file: File = match File::open(path) {
        Ok(file: File) => file,
        Err(e: Error) => return Err(e),
    };

    let mut contents: String = String::new();
    match file.read_to_string(&mut contents) {
        OK(_) => Ok(contents),
        Err(e: Error) => Err(e),

    }
    }
