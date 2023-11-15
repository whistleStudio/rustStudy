/* 
- struct内部元素同样存在可见性，仅于定义struct的模块外生效，模块内accessable
- 可以利用公共的方法，为私有属性设值
*/
#![allow(unused_variables, dead_code)]

mod my {
    pub struct OpenBox<T> {
        pub val: T
    }

    pub struct ClosedBox<T> {
        val: T
    }

    impl<T> ClosedBox<T> {
        pub fn new (val: T) -> ClosedBox<T> {
            ClosedBox{val}
        }
    }
}

fn main() {
    let open_box = my::OpenBox {val: 1};
    println!("open_box.val: {}", open_box.val);
    let closed_box = my::ClosedBox::new(0);

    // println!("open_box.val: {}", closed_box.val); // wrong
    
}
