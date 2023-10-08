/* 
if let 
- match的精简版，无需考虑穷尽性；
- if let 左值 = 右值；左值相当于match的pattern, 右值相当于需要match匹配的值
- if let {}应当返回()
- 无需实现PartialEq，即可实现非参数化枚举的比较
*/
#![allow(dead_code, unused_variables)]

fn main() {
    let n = 1;
    
    if let 1 = n {
        println!("right")
    }

    // #[derive(PartialEq)]
    enum Foo {
        A (i32, i32),
        B {x: i32, y: i32},
        C
    }

    let f = Foo::A(1, 2);

    if let Foo::A(x, _) = f {
        println!("A first el is {}", x);
    } else if let Foo::B { x, y } = f {}
    else {println!("nothing happens")}

    // if Foo::A(1, 2) == f {println!("eq")} // 为实现PartialEq trait时，无法比较报错

    struct Bar(i32);

    let b = Bar(1);

    if let Bar(1) = b {}
}
