/* 
- enum关键词允许所有合法的struct
- type类型别名，类型名称比较长时可以替换，impl默认实现type Self = 实现类型名称
*/
use WebEvent::*; // 把enum关键词全部引入，省去前缀WebEvent::
// use WebEvent::{PageLoad, KeyPress}; // 部分引入

#[allow(dead_code)]
enum WebEvent {
    PageLoad, // 单元结构体
    KeyPress(char), // 元组结构体
    Click{x: i64, y: i64} //一般结构体
}

#[derive(Debug)]
#[allow(dead_code)]
enum IHaveAVeryVeryLongName {
    Foo,
    Bar
}

type  ShortName = IHaveAVeryVeryLongName;

impl IHaveAVeryVeryLongName {
    fn show_sth (&self) {
        match self {
            Self::Foo => println!("foo! {:?}", self),
            Self::Bar => println!("bar! {:?}", self)
        }
    }
}

fn inspect(event: WebEvent) {
    match event {
        PageLoad => println!("page load"),
        KeyPress(c) => println!("{}", c),
        Click { x, y } => println!("{},{}", x, y)
    }
}

#[allow(dead_code)]
pub fn show () {
    let pressed = KeyPress('a');
    inspect(pressed);

    let foo = ShortName::Foo;
    foo.show_sth();
}