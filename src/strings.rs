#![allow(unused)]

use std::io;
use std::str;
use std::string::String;

//
// pub fn strings() {
//     //Strings - there are several ways to create a string
//     // 1. String::new()
//     let mut s = String::new();
//     // 2. to_string()
//     let mut s = "Hello".to_string();
//     // 3. from()
//     let mut s = String::from("Hello");
//     // 4. with_capacity()
//     let mut s = String::with_capacity(10);
//     // 5. from_utf8()
//     let mut s = String::from_utf8(vec![72, 101, 108, 108, 111]).unwrap();
//     // 6. from_utf8_lossy()
//     let mut s = String::from_utf8_lossy(&[72, 101, 108, 108, 111]);
//     // 7. from_raw_parts()
//     let mut s = unsafe { String::from_raw_parts(ptr, length, capacity) };
//     // 8. from_raw_parts_mut()
//     let mut s = unsafe { String::from_raw_parts_mut(ptr, length, capacity) };
//     // 9. from_char()
//     let mut s = String::from_char(5, 'a');
//     // 10. from_chars()
//     let mut s = String::from_chars(&['a', 'b', 'c']);
//     // 11. from_utf16()
//     let mut s = String::from_utf16(&[0x0048, 0x0065, 0x006c, 0x006c, 0x006f]);
//     // 12. from_utf16_lossy()
//     let mut s = String::from_utf16_lossy(&[0x0048, 0x0065, 0x006c, 0x006c, 0x006f]);
//     // 13. from_utf32()
//     let mut s = String::from_utf32(&[0x00000048, 0x00000065, 0x0000006c, 0x0000006c, 0x0000006f]);
//     // 14. from_utf32_lossy()
//     let mut s = String::from_utf32_lossy(&[0x00000048, 0x00000065, 0x0000006c, 0x0000006c, 0x0000006f]);
//     // 15. from_utf8_unchecked()
//     let mut s = unsafe { String::from_utf8_unchecked(vec![72, 101, 108, 108, 111]) };
//     // 16. from_utf16_unchecked()
//     let mut s = unsafe { String::from_utf16_unchecked(&[0x0048, 0x0065, 0x006c, 0x006c, 0x006f]) };
//     // 17. from_utf32_unchecked()
//     let mut s = unsafe { String::from_utf32_unchecked(&[0x00000048, 0x00000065, 0x0000006c, 0x0000006c, 0x0000006f]) };
//     // 18. from_raw_buf()
//     let mut s = unsafe { String::from_raw_buf(ptr, length) };
//     // 19. from_raw_buf_mut()
//     let mut s = unsafe { String::from_raw_buf_mut(ptr, length) };
//     // 20. from_boxed_bytes()
//     let mut s = unsafe { String::from_boxed_bytes(Box::new([72, 101, 108, 108, 111])) };
//     // 21. from_boxed_utf8_unchecked()
//     let mut s = unsafe { String::from_boxed_utf8_unchecked(Box::new([72, 101, 108, 108, 111])) };
//     // 22. from_boxed_utf16_unchecked()
//     let mut s = unsafe { String::from_boxed_utf16_unchecked(Box::new([0x0048, 0x0065, 0x006c, 0x006c, 0x006f])) };
//     // 23. from_boxed_utf32_unchecked()
//     let mut s = unsafe { String::from_boxed_utf32_unchecked(Box::new([0x00000048, 0x00000065, 0x0000006c, 0x0000006c, 0x0000006f])) };
//     // 24. from_raw_vec()
//     let mut s = unsafe { String::from_raw_vec(vec![72, 101, 108, 108, 111]) };
//     // 25. from_raw_vec_unchecked()
//     let mut s = unsafe { String::from_raw_vec_unchecked(vec![72, 101, 108, 108, 111]) };
//     // 26. from_utf8_unchecked_mut()
//     let mut s = unsafe { String::from_utf8_unchecked_mut(vec![72, 101, 108, 108, 111]) };
//     // 27. from_utf16_unchecked_mut()
//     let mut s = unsafe { String::from_utf16_unchecked_mut(&mut [0x0048, 0x0065, 0x006c, 0x006c, 0x006f]) };
//     // 28. from_utf32_unchecked_mut()
//     let mut s = unsafe { String::from_utf32_unchecked_mut(&mut [0x00000048, 0x00000065, 0x0000006c, 0x0000006c, 0x0000006f]) };
// }

pub(crate) fn main() {
    // let name = "Heath".tostring();
    let name: String = String::from("Heath");


    let mut name: String = String::new();
    name.push_str("test");
    println!("{}", name);
}