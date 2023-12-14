/* 
- 变量负责释放它们拥有的资源，每个资源只会存在一个拥有者；RAII变量离开作用域时，释放资源
- 所有权机制，避免了悬垂指针(dangling pointer)问题：指针指向无效资源
- 所有权转移时，可变性可能发生改变
- 当对象其中数据部分发生move时，其对象本身也无法使用
*/
#![allow(unused_variables, dead_code, unused_assignments)]

fn destroy_box (c: Box<i32>) {}

#[derive(Debug)]
struct A {
    name: String,
    age: i32
}

fn main () {
    let a = Box::new(1);
    let b = a; // 指针复制到栈，同时其指向的堆上资源所有权move到b
    destroy_box(b); // 指针复制到栈，指向的堆上资源所有权再次转移到c
    // println!("{}", b) // 报错

    let c = Box::new(2);
    // c = Box::new(1); // 报错
    let mut d = c;
    d = Box::new(1);

    let e = A{name: "".to_string(), age: 1};
    let f = e.name;
    // println!("{:?}", e) // 报错
}