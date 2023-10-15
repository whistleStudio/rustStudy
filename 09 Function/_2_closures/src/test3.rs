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
    let f1 = || {};

    let mut count = 0;
    let mut f2 = || count+=1 ;


    let s = "".to_string();
    let f3 = || std::mem::drop(s);

    // let mut f4 = || count+=1 ; // count作为可变引用不能多次借用

    is_fn(f1);

    is_fn_mut(f1);
    is_fn_mut(f2);
   
    is_fn_once(f1);
    // is_fn_once(f2);
    is_fn_once(f3);

    println!("{}", count);

}

fn is_fn<F>(_: F) where F: Fn() -> () {}

fn is_fn_mut<F>(_: F) where F: FnMut() -> () {}

fn is_fn_once<F>(_: F) where F: FnOnce() -> () {}
