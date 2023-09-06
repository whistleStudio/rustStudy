/* 
数组：同样类型已知长度的一组数据，栈上连续分布；长度编译时已知
切片：两个字段的对象，指向数据的指针和数据的长度；长度编译时未知
*/
use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first el of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len())
}

fn main() {
    let xs = [1, 2, 3, 4, 5];
    let ys = [0; 100];

    println!("xs: {:?}, xs[0]: {}", xs, xs[0]);
    println!("{}", ys.len());

    println!("mem allocated: {} bytes", mem::size_of_val(&xs));

    analyze_slice(&xs);
    analyze_slice(&xs[1..3]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);

    // 数组索引超出范围，报编译错误
    // println!("{}", xs[5]);
    // 切片索引超出范围，报运行错误
    // println!("{}", &xs[1..3][3]);

    // .get 返回Option 安全通过
    match xs.get(5) {
        Some(v) => {println!("{}", v)},
        None => {println!("idx out of range")}
    }
    println!("end...");
    // expect 解包；正确返回v，错误panick
    println!("{}", xs.get(4).expect("err"));
    println!("{}", xs.get(5).expect("err"));
}
