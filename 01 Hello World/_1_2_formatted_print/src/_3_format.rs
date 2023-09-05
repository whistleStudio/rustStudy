/* 
- format! 需要实现相应的fmt trait才行，要么Debug, 要么Display
*/

#[derive(Debug)]
#[allow(dead_code)]
struct Day {
    day: String,
    temp: u8
}

#[allow(dead_code)]
pub fn show () {
    let s = format!("{:?}", (1,));
    println!("{}", s);
    let d = Day {day: "Tuesday".to_string(), temp: 28};
    let s2 = format!("{:?}", d);
    println!("{}", s2);
}