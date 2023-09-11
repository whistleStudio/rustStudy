/* 
- 实现A::from(B), 自动实现B.into()到A
- 实现实现B.into()到A， 不会自动实现A::from(B)
- from into 两个trait不能同时存在，会conflict
*/
#![allow(dead_code)]

use std::convert::From;

#[derive(Debug)]
struct Number(i32);

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number(value)
    }
}


#[derive(Debug)]
struct Integer {
    v: u8
}

impl Into<Integer> for u8 {
    fn into(self) -> Integer {
        Integer { v: self }
    }
}

// impl From<u8> for Integer {
//     fn from(value: u8) -> Self {
//         Integer { v: value }
//     }
// }

fn main() {
    let mut num = Number::from(1);
    println!("{:?}", num);
    num = 2.into();
    println!("{:?}", num);

    let i: Integer = 2u8.into();
    println!("{:?}", i);
    // print!("{:?}", Integer::from(1u8))
}
