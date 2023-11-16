/*
super 和 self 作用 
- 消除路径歧义
- 避免不必要的路径硬编码；绝对路径，模块移植时发生问题
*/

fn f () {println!("f")}

mod my {
    fn f () {println!("my::f")}
    mod in_my {
        pub fn f () {println!("my::in_my::f")}
    }
    pub fn indirect_f () {
        self::f(); // 当前模块作用域，和直接调用f一样
        in_my::f();
        super::f(); // 当前模块的父级作用域
    }
}

fn main() {
    my::indirect_f();
}
