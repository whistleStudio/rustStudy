/* 
不同于像target_os这种由rustc隐式提供的条件，
自定义条件需要在编译时手动添加上 --cfg customConditionals

不希望出现可能的异常话，需考虑到条件不成立时状况
*/

fn main () {
    work_in_cond();
}

//rustc --cfg my_condition .\cfg_custom.rs; ./cfg_custom
#[cfg(my_condition)]
fn work_in_cond () {
    println!("work in condition")
}

#[cfg(not(my_condition))]
fn work_in_cond () {
    println!("not work in condition")
}