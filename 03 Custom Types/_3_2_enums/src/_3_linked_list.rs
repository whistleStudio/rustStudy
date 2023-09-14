/* 
- 通过枚举实现链表
*/
#![allow(dead_code, unused_variables)]

use List::*;
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

impl List {
    fn new () -> List {
        Nil
    }
    // 添加新节点，新节点指针指向当前节点; 创建智能指针还是相当于引用，不发生所有权转移，所以用self
    fn prepend(self, head: i32) -> List {
        List::Cons(head, Box::new(self))
    }

    // 不希望求len时，使List丢失所有权，所以使用&self;
    fn len(&self) -> u32 {
        match self {
            // 递归非Nil +1, 获取下个节点再调用len
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    // 显示链表关系
    fn stringify (&self) -> String {
        match self {
            Cons(head, tail) => format!("{}->{}", head, tail.stringify()),
            Nil => "Nil".to_string()
        }
    }
}

// // 资料：https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#nicer-match-bindings
fn about_match_ref () {
    // 新版本特性 match Option引用的话，pattern不加&，自动当作&引用处理，并且内部也自动ref匹配；加&就没用了，该写的ref还得写
    let a = Some("x".to_string());
    let b = &a;
    match b {
        Some(v) => {},
        None => {}
    }
    println!("{:?}", a);

    // // old edition
    // let a = Some("x".to_string());
    // let b = &a;
    // match b {
    //     &Some(ref v) => {},
    //     &None => {}
    // }
    // println!("{:?}", a);

}


// fn foo (ref mut a: &mut i32) {
    
// }

pub fn show () {
    let list = List::new().prepend(7).prepend(8).prepend(9);
    println!("len: {}", list.len());
    println!("stringify: {}", list.stringify());
}