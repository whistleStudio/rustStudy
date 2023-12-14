/* 
编译器（中的借用检查器）用生命周期确保借用的有效性。变量生命周期为其创建到销毁的期间。
Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域

大部分情况可推断，有时需要用生命周期 参数 标注，以明确运行时的有效性，好让借用检查器通过

函数：
由于rust函数无法通过除传参以外的方式获取外部变量，所以当返回值为引用时，确保运行正常，只有可能是传递的参数引用或静态'static 引用；
否则使用函数内部的局部变量引用，就会导致悬垂指针错误产生；
因而函数的生命周期参数有了相应的一些省略规则

函数生命周期参数省略三条规则：(引用作为参数传递时，同时会将生命周期传递给生命周期参数)
1 每一个是引用的参数都有它自己的生命周期参数
fn foo (x: &i32，y: &i32)->(){}
编译时会自动添加参数 fn foo<'a, 'b> (x: &'a i32，y: &'b i32)->(){}

2 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
fn foo (x: &i32)-> &i32 {} 
自动添加: fn foo<'a> (x: &'a i32)-> &'a i32 {} 

3 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method), 那么所有输出生命周期参数被赋予 self 的生命周期
trait A {
    fn a(&self, b: &i32) -> &i32;
}
这种无法推断，就需要显示标注
fn foo (x: &i32, y: &i32)-> &i32 {}
通过自动添加只能走到这：fn foo<'a, 'b>(x: &'a i32, y: &'b i32)-> &i32 {}

如果显示标注：fn foo<'a, 'b>(x: &'a i32, y: &'b i32)-> &'a i32 {}, 会在传参时将x的生命周期传递给返回值

如果显示标注：fn foo<'a>(x: &'a i32, y: &'a i32)-> &'a i32 {} 会将x与y重叠的生命周期传递

结构体：
引用的生命周期参数必须显示标注

方法:
impl 也需要一样进行生命周期的标注, 若结构体有的话

约束:
1 T: 'a  
T中所有引用必须保证在'a生命周期内有效
2 T: Trait + 'a 
T类型实现Trait 且T中所用引用必须在'a生命周期内有效
*/
#![allow(unused_variables, dead_code)]

struct A<'a>(&'a str); //引用的生命周期限制了结构体A的使用范围

impl<'a> A<'a> {
    fn a () {}
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

struct B<'a, T> (&'a T);
struct C<'a, T: 'a> (&'a T);

fn main () {}