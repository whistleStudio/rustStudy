/* 
- match 的pattern可以同时进行解构(pattern可以近似理解成是赋值操作时的等号左值)
- .. 用作忽略元组中剩余未匹配元素
- _ 通配符，不产生值的绑定
*/
pub fn show () {
   let tup = (1, 2, 3);

    match tup {
        (1, b, c) => println!("1st el must 1, then b:{}, c:{}", b, c),
        (2, ..) => println!("1st el must 2, the rest el not matter"),
        (.., 3) => println!("last el must 3, the rest el not matter"),
        (3, .., 4) => println!("1st must 3, last el must 4, rest whatever"),
        _ => {}
    }
}