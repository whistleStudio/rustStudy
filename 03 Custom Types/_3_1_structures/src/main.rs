/* 
- struct Unit 单元结构体; 定义时加分号
- struct Pair(T1, T2...)元组结构体; 定义时加分号
- struct Structure {k1: T1, k2: T2...}经典C风格一般结构体; 定义时不加分号
*/

struct Unit;

struct Pair(u8, u8);

#[derive(Debug, Clone)]
struct Point {x: f32, y: f32}

#[derive(Debug)]
struct Rectangle {top_left: Point, bottom_right: Point}

fn rect_area (rec: Rectangle) {
    let Rectangle{top_left: Point{x: x1, y: y1}, bottom_right: Point{x: x2, y: y2}} = rec;
    println!("rec_area: {}", (x2-x1)*(y2-y1));
}

fn square (top_left: Point, side: f32) -> Rectangle {
    // 变量名与结构体定义的键名同名，也可以直接创建
    // derive Clone trait; 否则这里top_left会产生所有权move, 后面的bottom_right就没法再用top_left了
    Rectangle { top_left: top_left.clone(), bottom_right: Point{x: top_left.x+side, y: top_left.y+side}}
}

#[allow(unused_variables)]
fn main() {
    let stru_unit = Unit;
    // 单元结构体解构
    let stru_pair = Pair(1, 2);
    let Pair(p1, p2) = stru_pair;
    // 一般结构体解构
    let point1 = Point{x: 1.0, y: 2.0};
    let Point{x:x1, y:y1} = point1;
    let Point{x, y} = point1; // 键名和变量名相同时，可以省去键名直接解构
    println!("x:{}, y:{}", x, y);

    // 利用结构体升级语法 创建
    let point2 = Point{x:5.0, ..point1};
    println!("point2: {:?}", point2);

    /* Activity */
    println!("-------------");
    let rec = Rectangle{top_left: Point{x:1.0, y:2.0}, bottom_right: Point { x: 3.0, y: 4.0 }};
    rect_area(rec);

    println!("{:#?}", square(Point{x:1.0, y:2.0}, 4.0))
}
