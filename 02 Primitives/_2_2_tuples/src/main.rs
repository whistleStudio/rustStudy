/* 
(T1, T2...)元组可以由不同类型的数据组成
*/
use std::fmt;

fn reverse_tup (pair: (i32, bool)) -> (bool, i32) {
    let (int_p, bool_p) = pair;
    (bool_p, int_p)
}

struct Matrix(f32, f32, f32, f32);

fn transpose (m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "({}, {})", self.0, self.1)?;
        write!(f, "({}, {})", self.2, self.3)
    }
}

fn main() {
    // tuple做参数和返回值
    let tup = (1, true);
    println!("{:?}", reverse_tup(tup));

    // 单个元组超过12个元素无法被打印
    // let long_tup = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("{:?}", long_tup)

    // 元组解构
    let des_tup = (1, "abc", false);
    let (p1, p2, p3) = des_tup;
    println!("p1:{}, p2:{}, p3:{}", p1, p2, p3);

    /* Activity */
    println!("-------------");
    let mat = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", mat);
    println!("transpose:\n{}", transpose(mat));
}

