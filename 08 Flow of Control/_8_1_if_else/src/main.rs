/* 
- 条件是表达式，不用加括号
- 每个分支都是一个代码块{}
- 所有分支必须返回相同类型
*/

fn main() {
    let n = 5;
    if n < 0 {
        println!("xxx");
        // 1 // 错误，表达式希望返回（）
    }

    let bigger_n = if n > 0 {
        n + 1
    } else {0}; // 不加else，会默认返回(), 类型不匹配

    println!("{}", bigger_n)
}
