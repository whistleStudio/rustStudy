/* 
cargo - package management tool
 - 依赖管理 crates.io
 - 单元测试
 - 基准测试

 1 创建项目
 cargo new foo // 二进制
 cargo new --lib foo // 库

 2 配置文件Cargo.to
 [package]
 name = "" 包名 也是编译输出的二进制文件名
 version = "" 版本
 authors = [""] 作者

 [dependencies]
 clap = "2.27.1" # 来自 crates.io
 rand = { git = "https://github.com/rust-lang-nursery/rand" } # 来自网上的仓库
 bar = { path = "../bar" } # 来自本地文件系统的路径

 3 多个二进制
 foo
 - Cargo.toml
 - src
 - - main.rs
 - - bin
 - - - other_bin.rs
 将文件放在/src/bin目录下，
 两种方法运行:
 - cargo指令 --bin packageName/other_bin
 - Cargo.toml增加default-run字段
 default-run = "packageName" / "other_bin"

 4 测试
 放/tests目录
 cargo test

 5 构建
 需要特殊条件构建
 [package]
 ...
 build = "build.rs"
*/

fn main() {
    println!("Hello, world!");
}
