#![allow(unused)]
use std::string::String;
use std::io;

mod slice;
mod strings;
use std::str;
// tuple - () - can store different data types - up to 12 elements
fn main() {
    let student_a: (&str, char, f32) = ("John", 'A', 25.44);
    let name_student_a:&str = student_a.0;
    let grade_student_a: char = student_a.1;
    let gpa_student_a: f32 = student_a.2;
    slice::main();
    strings::main();
// tuple destructuring
    let (name_student_a, grade_student_a, gpa_student_a) = student_a;
    println!("{} {} {}", student_a.0, student_a.1, student_a.2);
    println!("Student A: {} {} {}", name_student_a, grade_student_a, gpa_student_a);
 // Arrays - [] - store up to 32 - similar data types
let students: [&str; 3] =  ["John", "A", "25.44"];
    println!{"the first student in our array is {}", students[0]};
}