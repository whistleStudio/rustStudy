/* 
- 满足闭包约束的函数，同样可以作为参数传递
*/
pub fn show () {
    apply(f1);
}

fn apply<F>(f:F) 
    where F:Fn()
{
    f();
}

fn f1 () {}