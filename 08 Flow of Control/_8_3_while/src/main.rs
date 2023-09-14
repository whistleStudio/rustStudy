/* 
fizzbuzz：便利若干个数，被3整除 输出fizz; 被5整除，输出buzz; 被15整除，输出fizzbuzz
*/

fn main() {
    let mut n = 1;
    while n <= 50 {
        if n % 15 == 0 {println!("fizzbuz")}
        else if n % 3 == 0 {println!("fizz")}
        else if n % 5 == 0 {println!("buzz")}
        else {println!("{}", n)}
        n += 1;
    }
}
