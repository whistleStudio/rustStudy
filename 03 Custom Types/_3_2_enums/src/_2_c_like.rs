/* 
- C风格经典enum; 关键词对应隐式辨别值（默认从0开始）
*/
#![allow(dead_code)] // 全局宏定义attribute

enum Number {
    Zero,
    One,
    Two
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff
}

pub fn show () {
    // 先用 as 转化为整数类型，再在print里决定以何种进制显示
    println!("Zero: {}", Number::Zero as u8); 
    println!("Red: {:06X}", Color::Red as u32); 
}