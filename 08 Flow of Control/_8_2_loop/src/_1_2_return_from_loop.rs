/* 
- loop作为表达式，用break val返回值
*/
#![allow(dead_code)]

pub fn show () {
    let mut count = 3;

    let res = loop {
        println!("{}", count);
        count -= 1;
        if count == 0 {
            break "over"
        }
    };

    println!("{}", res);
}