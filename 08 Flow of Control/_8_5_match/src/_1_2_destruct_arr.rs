/* 
- 和解构元组基本相同
- 解构中使用 _ , 通配单个元素; 元组同样适用
- @.. 将剩余未解构元素，组合成一个数组; 元组不行
*/
pub fn show () {
    let arr = [9, 2, 3, 9];

    match arr {
        [1, second, third, fourth] => {},
        [2, ..] => {},
        [3, .., 4] => {},
        [4, _, _, 5] => {},
        [5, rest_arr@..] => println!("rest arr: {:?}", rest_arr),
        [first, rest_arr@.., last] => println!("first: {} rest arr: {:?} last: {}", first, rest_arr, last)
    }
}