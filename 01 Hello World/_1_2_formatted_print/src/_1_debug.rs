//! - `{}`标准库里的类型(整型，浮点，字符，字符串，字符串切片)自动实现了`std::fmt::Display`
//! - `{:?}`所有类型都实现了`std::fmt::Debug`，未实现Display的要加`#[derive(Debug)]`
//! - `{:#?}`Debug时美化打印

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
  name: String,
  age: u8
}

#[allow(dead_code)]
pub fn show () {
  println!("{:?}", 1); // 直接可以
  println!("{:?}", (1,)); // 元组必须用{:?}

  let p = Person{
    name: "wsh".to_string(),
    age: 11
  };

  println!("{:?}", p);
  println!("pretty print:");
  println!("{:#?}", p);
}