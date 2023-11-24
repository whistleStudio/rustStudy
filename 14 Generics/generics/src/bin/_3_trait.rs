#![allow(path_statements)]

// A, B 为实现copy trait
struct A;
struct B;

trait DoubleDrop<T> {
    fn double_drop (self, _:T);
} // 传参任意类型，使用泛型T

impl<T, U> DoubleDrop<T> for U {
    fn double_drop (self, _:T) {}
} // self任意类型，使用泛型U

// self, T 非引用所有权发生转移，函数调用完毕，函数作用域销毁，内存释放
fn main () {
    let a = A;
    let b = B;
    a.double_drop(b);
    // a; // 错误

    A.double_drop(B); // 这里A，B只是创建两个未命名的临时变量
    A; // 同样，这里也是创建一个未命名的临时变量

}