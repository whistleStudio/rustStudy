pub fn test4 () {
    let a = "".to_string();
    let f1 = move || {println!("{}", a); std::mem::drop(a)};
    // println!("{}",a);

    // let a = 0;
    // let f1 = move || println!("{}", a);

    let mut b = "".to_string();
    let mut f2 = move || {b+="x"; println!("{}", b);};
    f2();
    f2();
    // println!("{}", b);

    let mut c = "".to_string();
    let mut f3 = || {c += "x";};
    f3();
    f3();
    println!("c:{}", c);

    let mut e = "".to_string();
    let f5 = || e+="";
    f5();
}