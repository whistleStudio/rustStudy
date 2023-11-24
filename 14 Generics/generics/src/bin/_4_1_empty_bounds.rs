/* 
空约束: trait不实现功能，仅起到泛型限制作用
*/
#![allow(dead_code)]
trait EB {}

struct A;
struct B;

impl EB for A {}

fn f_eb <T: EB> (_: &T) {}
fn main () {
    f_eb(&A);
    // f_eb(&B); // 错误，B未实现EB trait, 不满足约束条件
}