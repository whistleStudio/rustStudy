/* 
mod my;
声明会查找同级目录名为"my.rs"或"my/mod.rs"的文件，
（另外这种目录结构也是允许的:
-my
--m1.rs
--m2.rs
-my.rs
-main.rs 
）
并将文件内容放到当前作用域名为my的模块里
*/

mod my;
fn main() {
    println!("Hello, world!");
    my::m1::show();
    // my::m2::show(); // 错误， super show仅在my(mod)模块内可见
    my::show();
}
