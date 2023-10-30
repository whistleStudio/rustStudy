fn main() {
    let mut sum = 0;
    for n in 0.. {
        if is_odd(n) {
            if n*n > 1000 {break}
            sum += n;
        }
    }
    println!("sum of all the squared odd numbers under 1000:");
    println!("{}", sum);


    // use HOF(higher order functions)高阶函数
    // Iterator::map/take_while/filter等都是惰性迭代器，又称迭代器适配器，生成新的迭代器
    // Iterator::sum产生消费行为
    let sum2 = (0..).filter(|&n| n % 2 == 1)
                    .take_while(|&n| n * n < 1000);
                    // .sum();

    println!("{}", sum2.sum::<u32>());
    // println!("{:?}", sum2);
}

fn is_odd (n: u32) -> bool {
    n % 2 == 1
}

