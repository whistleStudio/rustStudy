/* 
- 闭包定义后，编译器会创建一个匿名的结构体，根据捕获变量的操作实现相应trait
- 因为是匿名结构体（无具体类型名），闭包作函数参数时，需要使用泛型，
但一般泛型又过于模糊，所以使用Fn/FnMut/FnOnce加以约束
*/
pub fn show () {
    let mut n = "".to_string();
    let mut f1 = || {n += "x"};

    // f1();
    is_fn_once(&mut f1);
    is_fn_once(&mut f1);
    println!("n: {}", n);
}

fn is_fn_once<F>(f: F) where F: FnOnce() {f();}