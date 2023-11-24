/* 
关联类型 type XXX
作用： 增加代码整体可读性，将外部的泛型参数移入trait内部

为什么使用关联类型
存在的问题：
因为Rust要求 —— 如果容器的类型约束(trait)是泛型的, 则必须明确地标出所有泛型类型
*/

use std::fmt::Debug;

trait T {}

fn main () {
}