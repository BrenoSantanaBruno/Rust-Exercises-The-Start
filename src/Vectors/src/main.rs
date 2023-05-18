#![allow(unused)]

use std::io;

fn main() {
    //Vectors are like arrays, but they can grow or shrink in size
    //Vectors are stored on the heap, not the stack
    //Vectors can only store values of the same type
    //Vectors are created using the vec! macro
    //Vectors are useful when you don't know how many items you will have
    //Vectors are useful when you want to add or remove items from the middle of the list
    //Vectors are useful when you want to share data between different parts of your program
    //Vectors are useful when you want to store data that is not known at compile time
    //Vectors are useful when you want to own all of the data and you want to ensure that the data will be valid for the entire lifetime of the program
    //Vectors are useful when you want to have a collection of items, but you don't want to copy the data for each item
    //Vectors are useful when you want to keep track of the order of the items
    //Slow to add or remove items from the middle of the list
    //Fast to add or remove items from the end of the list
    //Fast to look up items by index
    //Slow to iterate over the items in the list

    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1, 2, 3];

    vec1.push(1);
    vec2.push(4);

    let second_element: &i32 = &vec2[1];
    println!("The second element is {}", second_element);

    println!("The lenght of the the vector is {}", vec2.len());

    for element in vec2.iter() {
        println!("Element: {}", element);

    }


}