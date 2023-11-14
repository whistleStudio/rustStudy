/* 
- 模块中的元素默认私有
- 只有加了pub修饰符的元素，模块外可访问
*/
#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    // out_mod::private_fn(); // wrong
    out_mod::pub_fn();
}

mod out_mod {
    fn private_fn () {println!("out_mod::private_fn")}
    pub fn pub_fn () {println!("out_mod::pub_fn")}
    mod in_mod {

    }
}