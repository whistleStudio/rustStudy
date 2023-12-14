/* 
- 【只讨论引用】引用存在生命周期，引用保持有效的作用域；
    引用：引用的值有没有效; 非引用：存不存在   
-  避免悬垂指针，引用指向空对象
- 'a 在'a的作用域中，引用保持有效
- 本质也是个参数，传递实参时自动传递作用域
- 借用检查器 比较借用者（引用）作用域 和 【出借者（引用指向对象）的作用域】 = 【引用的生命周期】（错误画图）
- 结构体定义时，要显示标注（rust后期可能会改进）
  函数存在省略规则
- fn f<'a> (p1: &'a u8, p2: &'a u8) -> &'a u8
  传递实参时，会比较p1,p2，选择重合的最小生命周期，进行传递

    fn f<'a> (a: &'a u8, b: &'a u8) -> &'a u8 {
        a
    }

    let a = &1;
    let c;
    {
        let b = 1;
        c = f(a, &b);
    }
    println!("{}", c);
*/
#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct S<'a> {
    a: &'a u8,
    b: u8
}


#[derive(Debug)]
struct S2<'a, T>(&'a T);
// impl S {
//     fn test (&self, a: &u8) -> &u8 {
//         a
//     }
// }

#[derive(Debug)]
struct  S3(u8);
fn main () {
    let s;
    {
        let b = 1;
        // s = S2(&S{a: &1, b: 1});
        s = S2(&S3(1))
    }
    println!("{:?}", s.0);

    let s2 = S3(1);
    let s3 = s2;
    // s2;
}