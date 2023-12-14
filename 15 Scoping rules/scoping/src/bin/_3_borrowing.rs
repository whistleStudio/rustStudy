/* 
- 数据只允许同时存在多个不可变借用或一个可变借用
*/
#![allow(unused_variables, unused_mut, unused_assignments)]


fn main () {
    let mut a = 1;
    let b = &a;
    let ref c = a;
    // let d = &mut a; // 报错
    println!("{}", b)
}