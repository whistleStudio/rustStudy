/* 
关联类型 type XXX
trait内部（Self::XXX）
作用： 增加代码整体可读性，将外部的泛型参数移入trait内部

为什么使用关联类型
存在的问题：
因为Rust要求 —— 如果的容器(参数)类型约束(trait)是泛型的, 则必须明确地标出所有泛型类型
*/


// 一般写法difference, container类型因为存在泛型约束，所以要把Contains的泛型参数也带上
trait Contains<A> {
    fn contains (&self, _: &A) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

fn difference<A, C> (container: &C) -> i32 where
    C: Contains<A> {
    container.last() - container.first()
}


struct Container(i32, i32);

impl Contains<i32> for Container {
    fn contains (&self, a: &i32) -> bool {
        self.0 == *a || self.1 == *a
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

// 使用关联类型type, 约束trait内部定义，避免将泛型参数暴露在外面
trait Contains2 {
    type A;

    fn contains2 (&self, _: &Self::A) -> bool;
    fn first (&self) -> i32;
    fn last (&self) -> i32; 
}

fn difference2<C> (container: &C) -> i32 where
    C: Contains2{
    container.last() - container.first()
}

impl Contains2 for Container {
    type A = i32;

    fn contains2 (&self, a: &Self::A) -> bool {
        self.0 == *a || self.1 == *a
    }
    fn first (&self) -> i32 {self.0}
    fn last (&self) -> i32 {self.1}
}
fn main () {
    let con = Container(1, 2);
    println!("con contains 1: {}", con.contains(&1));
    println!("con difference is {}", difference(&con));

    println!("con contains 3: {}", con.contains2(&3));
    println!("con difference is {}", difference2(&con))
}