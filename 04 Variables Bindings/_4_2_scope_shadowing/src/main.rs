/* 
- 用{}标定作用域]
*/
#![allow(unused_assignments)]

fn main() {
    let mut shadow_binding = 1;
    {
        shadow_binding = 2;

    }
    println!("{}", shadow_binding);
    {
        let shadow_binding = 3;
        // 打印 {} 连续两
        println!("in 2nd:{{}} {}", shadow_binding)
    }
    println!("{}", shadow_binding)
}
