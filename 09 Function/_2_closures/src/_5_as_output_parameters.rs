/* 
- 闭包同样可以作为返回值，但因为类型模糊，需要用impl Fn* 约束
- 在发生捕获时，需要添加move; 
因为闭包捕获的是创建它的函数作用域中的变量，当函数调用完毕时，作用域销毁，其中变量也会被丢弃，此时闭包留下无效的引用；
而 加了move捕获将通过值的形式进行，具有所有权的变量存在于闭包的作用域中
*/

pub fn show () {
    let a = return_fn();
    a();
    a();
}

fn return_fn () -> impl Fn() {
    let b = 0;
    // let a = "".to_string();
    move || println!("{}", b)
}