/* 
方法是依附于对象的函数, impl实现
- 静态方法，关联函数associated function(不定义形参self), 类型::调用，常见构造器
- 实例方法： (定义形参self, 实参self隐式自动传递) 实例.调用 或 类型::传递self及剩余参数
- 注意参数self的使用，&self / &mut self / self / mut self,
非引用时，实例会在方法中发生所有权的转移，方法调用结束，实例将无法继续使用 
*/
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn origin() -> Point {
        Point {x:0f32, y:0f32}
    }
    fn new(x: f32, y: f32) -> Point {
        Point {x, y}
    }
}

#[derive(Debug)]
struct Rec {
    p1: Point,
    p2: Point
}

impl Rec {
    // &self 是 self: &Self 语法糖, type Self = Rec
    fn area (&self) -> f32 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter (&self) -> f32 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;
        2.0 * ((x1 - x2) * (y1 - y2)).abs()
    }
    // &mut self 是 self: &mut Self 语法糖
    fn translate (&mut self, x: f32, y: f32) -> &mut Self {
        self.p1.x += x;
        self.p1.y += y;
        self.p2.x += x;
        self.p2.y += y;
        self
    }

    fn destroy (self) {
        println!("{:?} cant used anymore", self)
    }
}

fn main() {
    let mut r1 = Rec {
        p1: Point::origin(),
        p2: Point::new(1.0, 2.0)
    };
    println!("r1 area: {}", r1.area());
    println!("r1 perimeter: {}", Rec::perimeter(&r1)); // === r1.perimeter()
    println!("r1 translate 10: {:?}", r1.translate(10.0, 10.0));
    r1.destroy();
    // r1.area(); // 报错，borrow moved value
}
