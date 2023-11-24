/* 
where从句
约束也可以用where从句形式表示，其外置写在{前，而不是泛型参数第一次出现的<>里；
where可以用于约束任意类型，不仅仅是泛型参数

作用：
- 泛型类型与约束分开，结构更清晰
- 由于可约束任意类型，适用场景更多，可直接表意
*/
#![allow(dead_code)]

use std::fmt::Debug;

trait Trait1 {}
trait Trait2 {}

// #[derive(Debug)]
struct A;


fn f1 <T: Trait1+Trait2, U: Trait1+Trait2> (){}
fn f2 <T, U> () where 
    T: Trait1+Trait2,
    U: Trait1+Trait2 {}

// trait意图将任意类型的量通过以Option形式显示
trait PrintInOption {
    fn print_in_opt(self);
}

// 需要直接显示的是Option类型，而不是T；所以这里是对Option进行约束
impl<T> PrintInOption for T where
    Option<T>: Debug{
    fn print_in_opt(self) {
        println!("{:?}", Some(self))
    }
}
    
fn main () {
    1.print_in_opt();
    // A.print_in_opt() // 错误， Option<A>未实现Debug trait
}