/* 
闭包里的&mut变量，破坏了它的Copy，导致is_fnmut(cf2);传参变成了move
*/

pub fn test2 () {
    let mut b = 1;
    let mut cf2 = || {
       b += 1;
    };
    is_fnmut(cf2);
    is_fnmut(cf2);
}
fn is_fnmut<F>(_: F) where F: FnMut() -> () {}