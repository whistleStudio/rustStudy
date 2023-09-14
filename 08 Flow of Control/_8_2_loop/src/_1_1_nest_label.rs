/* 
- 可以通过加标签'label: loop 跳出指定循环
*/
#![allow(unused_labels, unreachable_code, dead_code)]

pub fn show () {
    'a: loop {
        println!("enter a");
        'b: loop {
            println!("enter b");
            break 'a;
        }
        break 'a
    }
    println!("xxxx")
}