/* 
- 程序通常由语句expressions构成
- 语句常见两种形式：
1 声明变量绑定+; 
2 表达式+;
- {}代码块blocks也是表达式；代码块里最后一个表达式会作为返回值，若是语句则为（）
*/
#![allow(path_statements, unused_must_use)]

fn main() {
    // 语句 声明变量绑定;
    let _x = 5;
    // 语句 表达式;
    _x;
    _x + 1;
    1;

    let x = {
        let x = 1;
        x+1
    };
    println!("{}", x);
}
