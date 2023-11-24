/* 
约束：泛型参数经常会用trait进行约束
struct S<T: trait>(T);
fn f<T: trait>(t: T) {}

作用：
1 约束将泛型限制为符合要求的类型, 进一步精确泛型类型
2 存在约束的泛型实例，能够调用对应trait的方法
*/

use std::fmt::{Display, Debug};

struct S1<T: Display>(T);

struct S2<T:Debug>(T);

fn display_t<T: Display> (t: T) {
    println!("{}", t)
}

// fn display_s1<T: Display> (t: T) {
//     println!("{:?}", t)
// } // 错误，T为实现Debug trait

fn main () {
    S1(1);
    // S1(vec![1]); // 错误 vec未实现Display trait
    S2(vec![1]); // 默认实现Debu trait

    display_t(1);
}