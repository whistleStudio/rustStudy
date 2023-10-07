/* 
守卫：辅助匹配分支
- 编译器计算patterns穷尽性时，不会将有守卫的分支计入；需要注意match是否穷尽
*/
pub fn show () {
    enum Foo {
        A(i32),
        B(i32)
    }

    let foo = Foo::A(1);

    match foo {
        Foo::A(n) if n < 1 => {},
        Foo::A(n) if n == 1 => println!("match == 1 !"),
        Foo::A(n) if n > 1 => {},
        // 尽管↑guard考虑到了A所有可能，但编译器不计入；所以还需重新考虑匹配A时情况↓
        Foo::A(_) => println!("never print me"), // 实际不运行，但必须有，否则未穷尽报错
        Foo::B(_) => {}
    }
}
