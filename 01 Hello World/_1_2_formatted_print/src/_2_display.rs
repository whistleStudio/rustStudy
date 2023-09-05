/* 
- {} 要实现 fmt::Display trait的fmt方法
*/
use std::fmt;
struct StruTup(i32);

impl fmt::Display for StruTup {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
  }
}

struct MinMax(i32, i32);

impl fmt::Display for MinMax {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} - {}", self.0, self.1)
  }
}

#[derive(Debug)]
struct Point2D {
  x: f32,
  y: f32
}

impl fmt::Display for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "x:{:.2}, y:{:.2}", self.x, self.y)
  }
}

// Activity
#[derive(Debug)]
struct Complex {
  real: f64,
  imag: f64
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{} + {}i", self.real, self.imag)
  }
}

#[allow(dead_code)]
pub fn show () {
  // 打印实现了Display的元组结构体
  println!("StruTup(1): {}", StruTup(1));
  // 打印MinMax Point2D
  let top_range = MinMax(100, 200);
  let bot_range = MinMax(-200, -100);
  println!("top_range: {}, \nbot_range: {}", top_range, bot_range);

  let p = Point2D{x:10_f32, y:20.0};
  println!("Debug print:\n{:?}", p);
  println!("Display print:\n{}", p);

  println!("--- activity ---");
  let c_num = Complex{real: 3.3, imag: 7.2};
  println!("Display: {}", c_num);
  println!("Debug: {:?}", c_num)
}