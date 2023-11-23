/* 
- #![crate_attribute]  // 作用于整个crate
- #[crate_attribute] // 作用于模块或者项

几种形式（并不是都通用）：
- #[attribute = "value"]
- #[attribute(key = "value")]
- #[attribute(value)]

多个属性时
- #[attribute(value1, value2)]
- #[attribute(value1, value2
              value3)] // 允许跨行
*/
fn main() {
    println!("Hello, world!");
}
