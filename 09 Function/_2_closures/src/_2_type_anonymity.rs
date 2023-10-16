/* 
- 闭包作为函数参数时，需要泛型
- 闭包的泛型必须用Fn FnMut FnOnce一种约束；决定变量以何种方式调用
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