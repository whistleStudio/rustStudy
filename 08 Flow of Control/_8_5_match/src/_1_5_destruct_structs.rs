/* 
和前面类似 
*/
pub fn show () {
    struct Circle {
        pos: (i32, i32),
        radius: i32
    }
    let c1 = Circle {pos: (1, 2), radius: 5};

    match c1 {
        Circle {pos: (2, y), radius} => {},
        Circle {radius: 3, pos} => println!("{:?}", pos),
        Circle {pos: (_, 1), radius: 4} => {},
        Circle {pos: (..), radius: 4} => {},
        Circle {radius, ..} => println!("default match")
    }

    let Circle {pos: (x, _), ..} = c1;
    println!("destruct c1, get x: {}", x)
}