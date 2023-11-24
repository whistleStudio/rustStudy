/* 
多重约束
- 单一类型多个约束用+连接
<T: Trait1+Trait2>
- 多个类型存在约束，用,隔开
<T: TraitT, U: TraitU>
*/

use std::fmt::{Display, Debug};

fn compare_prints<T: Display+Debug> (t: &T) {
    println!("{}", t);
    println!("{:?}", t)
}

fn compare_types<T: Debug, U: Debug> (t: &T, u: &U) {
    println!("{:?}", t);
    println!("{:?}", u);
}
fn main () {
    let str = "[1]";
    let arr = [1];
    compare_prints(&str);
    compare_types(&str, &arr);
}