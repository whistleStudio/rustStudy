/* 
仅在原始rustc时起作用，用cargo时无效
cargo_type 设定crate类型 binary / library
cargo_name 设定crate名称
*/
#![crate_type="lib"] // 就不用在rustc编译时加上--crate-type=lib了

// rustc crate_attr.rs
fn main () {
    // cargo run --bin crate_attr
    println!("use cargo will ignore crate attributes")
}
