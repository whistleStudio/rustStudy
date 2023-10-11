/* 
- Fn FnMut FnOnce 闭包定义
- 捕获变量时，有无Copy trait, move情况不同
- 派生关系
  FnOnce
  | FnMut
    | Fn
- &F 实现了 Fn
*/

pub fn test3 () {
    let a = 1;

    let cf = || {
        println!("{}", a);
    };

    let mut b = 1;
    let mut cf2 = || {
       b += 1;
    };

    let c = "x".to_string();
    let cf3 = || {
        println!("{}", c);
        std::mem::drop(c); // 自动加move?
    };

    is_fn(cf);

    is_fnmut(cf);
    is_fnmut(&mut cf2);

    is_fnonce(cf);
    is_fnonce(&mut cf2);
    is_fnonce(cf3);

    // fn f1 ()->i32 {a}
    // let f2 = |a:i32|;



}

fn is_fn<F>(_: F) where F: Fn() -> () {}

fn is_fnmut<F>(_: F) where F: FnMut() -> () {}

fn is_fnonce<F>(_: F) where F: FnOnce() -> () {}
