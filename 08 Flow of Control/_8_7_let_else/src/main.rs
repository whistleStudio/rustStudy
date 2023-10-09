/* 
let - else
- 直接执行非匹配情况, 返回类型 ！（diverge分歧），可以用panic! 或 return
*/

fn main() {
    let a = 1;
    let 2 = a else {
        println!("2 not equal {}", a);
        return
    };
}
