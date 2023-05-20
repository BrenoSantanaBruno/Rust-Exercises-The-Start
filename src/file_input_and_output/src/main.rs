#![allow(unused)]

use std::{fs, io};

use std::fs::{File, OpenOptions};

use std::io::prelude::*;

use std::io::Read;

use std::fs::remove_file;

fn main(){
    // let mut file: File = File::create( "src/helllo.txt").expect( "Could not create file");
    // file.write_all( b"Hello, world!").expect("Could not write file" );

    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open("src/helllo.txt")
    //     .expect("Could not open file");
   // file.write_all(b"hello Again\r\n").expect("Could not write file");
   //  file.write_all(b"hello Againaaaaa\r\n").expect("Could not write file");
   //  file.write_all(b"hello Againaaaaa\r\n").expect("Could not write file");
   //
   //  let mut file = File::open("src/helllo.txt").expect("Could not open file");
   //  let mut file_content: String = String::new();
   //  file.read_to_string(&mut file_content).unwrap();
   //  println!("{}", file_content);

    fs::remove_file("src/helllo.txt").expect("Could not delete file");

}