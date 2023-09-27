/* 
- 
*/
#![allow(dead_code, unused_variables, unused_must_use)]

fn main() {
    for n in 1..=50 {
        if n % 15 == 0 {print!("fizzbuz ")}
        else if n % 3 == 0 {print!("fizz ")}
        else if n % 5 == 0 {print!("buz ")}
        else {print!("{} ", n)}
    }

    // iter_use();
    // iter_mut_use();
    // let s = &String::from("rust");
    // println!("{}", s);
    let a = b'a';
}

fn into_iter_use () {
    let names = vec!["a", "b", "c"];
    // names.into_iter(); // 创建消耗型迭代器consuming iterator; 集合里的值丢失所有权

    // for循环默认 集合调用into_iter
    for name in names {
        
    }
    // print!("{:?}", names); // 错误 names move
}

fn iter_use () {
    let names = vec!["a", "b", "c"];
    names.iter(); // 借用 
    for name in names.iter() {
        match name {
            &"a" => println!("this is a"),
            _ => println!("nothing happens")
        }
    }
    println!("after iter(): {:?}", names)
}

fn iter_mut_use () {
    let mut names = vec!["a", "b", "c"];
    // let mut change_name;
    names.iter_mut(); // 可变借用
    for name in names.iter_mut() {
        *name = Box::leak(Box::new(format!("{}1", *name)));
    }
    println!("after iter(): {:?}", names)
}