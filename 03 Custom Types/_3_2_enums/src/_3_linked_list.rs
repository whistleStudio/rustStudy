/* 
- 通过枚举实现链表
*/
#![allow(dead_code, unused_variables)]

use List::*;
enum List {
    Cons(i32, Box<List>),
    Nil
}

impl List {
    fn new () -> List {
        Nil
    }
    // 添加新节点，新节点指针指向当前节点
    fn prepend(self, el: i32) -> List {
        List::Cons(el, Box::new(self))
    }
    // 不希望求len时，使List丢失所有权，所以使用&self;
    // 
    // fn len(&self) -> u32 {

    // }
}

fn hello(arg: &Option<String>) {
    match arg {
        Some(name) => println!("Hello {}!", name),
        None => println!("I don't know who you are."),
    }
}


pub fn show () {
    let d = "xxxx".to_string();
    let a = Some(d);
    let b = &a;
    // let c = d;
    // println!("{}", b);

    // hello(&Some("xxx".to_string()))
}