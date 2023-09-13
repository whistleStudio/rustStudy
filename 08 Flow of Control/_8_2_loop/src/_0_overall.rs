pub fn show () {
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("its over");
            break;
        }
    }
}