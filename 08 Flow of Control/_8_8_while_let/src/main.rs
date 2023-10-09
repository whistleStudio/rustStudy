/* 
while let
- while let [pattern] = [val] {}
val 循环匹配 pattern
比普通的while 语句，比较条件更宽泛，还可以顺便解构
*/
fn main() {
    let mut n = Some(0);

    while let Some(i) = n {
        if i < 5 {
            println!("n is {:?}", n);
            n = Some(i+1)
        } else {n = None}
    }
}
