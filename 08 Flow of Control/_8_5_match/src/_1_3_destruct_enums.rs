/* 
- 和前面类似
*/
enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    CMYK(u32, u32, u32, u32)
}

pub fn show () {
    let color = Color::RGB(1, 1, 1);
    match color {
        Color::Red => {},
        Color::RGB(r, g, b) => println!("r:{}, g:{}, b:{}", r, g, b),
        _ => {}
    }
}