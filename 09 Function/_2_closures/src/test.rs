// 无报错
pub fn test () {
    let a = 1;
    let cf = || {
        println!("{}", a);
    };
    is_fn(cf);
    is_fn(cf);
}

fn is_fn<F>(_: F) where F: Fn() -> () {}