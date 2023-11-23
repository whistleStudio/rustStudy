/* 
条件校验可通过两种操作符实现
- 属性形式：#[cfg(...)]
- 宏形式(用在布尔表达式)：cfg!(...) 

差异:
- 属性形式的校验发生在编译阶段，符合条件的完成编译，反之不会编译；
- 宏形式的校验发生在运行阶段，对编译过程不产生影响，不会移除任何代码，仅运行条件成立部分

适用场景：
- 属性通常是运行时需要唯一性的地方用
- 宏通常是允许运行时多可能的地方
*/
fn main () {
    am_i_linux();

    // cfg!()这里是固定宏的写法，不是比较运算target_os == "linux"
    if cfg!(target_os = "linux") {
        println!("im linux !")
    } else {println!("im not linux!")}

}

#[cfg(target_os = "linux")]
fn am_i_linux () {
    println!("im linux")
}

#[cfg(not(target_os = "linux"))]
fn am_i_linux () {
    println!("im not linux")
}