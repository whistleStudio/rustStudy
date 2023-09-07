/* 
可在任何作用域内声明，包括全局；
都需要显示声明类型，无法隐式类型推断
- const 不可变
- static 可mut; 特殊情况可变，有'static生命周期
*/
#![allow(dead_code)]

const AGE: u8 = 1;
static mut LANG: &str = "rust";
fn main() {
    println!("const Age: {}", AGE);
}
