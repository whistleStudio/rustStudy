/* 
- 实现fmt::Display自动实现string::ToString
- 实现FromStr, 可以使用parse, 将字符串转化成指定类型（显示标注类型或使用turbofish语法）, 返回Result
其实和TryInto类似只不过一个直接给String实现，
FromStr给自定义类实习，String被动能使用parse,另外借助turbofish，调用更友好些
*/
use std::{fmt::{self, Display}, str::FromStr};

#[derive(Debug)]
struct EvenNum(u8);

impl Display for EvenNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EvenNum: {}", self.0)
    }
}
impl FromStr for EvenNum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Even" {Ok(EvenNum(2))}
        else {Err(())}
    }
}

fn main() {
    println!("{}", EvenNum(1).to_string());
    let one_way_parse: EvenNum = "Even".parse().unwrap();
    println!("one way parse: {:?}", one_way_parse);
    println!("second way parse: {:?}", "Even".parse::<EvenNum>().unwrap())
}
