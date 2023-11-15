/* 
- use声明将引入路径别名，简化调用时路径的书写
- 配合as，可绑定别名
*/
use m1::m2::m3;
use m1::m2::m3::M3Struct;

mod m1 {
    pub mod m2 {
        pub mod m3 {
            pub fn m3_show () {println!("m3_show")}

            #[derive(Debug)]
            pub struct M3Struct {
                pub v: i32
            }
        }
    }
}
fn main() {
    m3::m3_show();
    let s = M3Struct {v: 1};
    println!("{:#?}", s);
}
