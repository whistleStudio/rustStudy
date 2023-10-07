/* 
- 解引用：* 右值，操作值
- 解构：&, ref, ref mut 左值
  （& 引用 》解构》非引用；ref 非引用 》解构》引用）
- 右值或操作值出现&时，作为引用标识
*/
pub fn show () {
    let reference = &3;

    match reference {
        // v => {}, // v: &i32
        // 解构↓
        &v => println!("{}", v) 
    }

    // 解引用↓
    match *reference {
        v => {}
    }

    // 赋值操作时用ref、ref mut解构，使变量类型转化为对值的引用
    let ref mut reference2 = 3; 
    let reference3 = &mut 3;

    let mut not_ref = 1;
    match not_ref {
        ref mut v => *v += 1
    }
    println!("after ref mut: {}", not_ref)
}