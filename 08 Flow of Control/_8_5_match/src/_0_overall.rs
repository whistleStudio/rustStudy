/* 
- match 所有可能结果都必须在pattern中有体现
- match 分支返回值类型必须相同
- match 第一个匹配的分支的返回值 将作为 match表达式返回值返回
*/
pub fn show () {
    let num = 2;

    match num {
        1 => {println!("1")},
        2 | 3 => {println!("2 | 3")},
        3 ..= 19 => {println!(">=3 <=19")},
        _ => {}
    }
}