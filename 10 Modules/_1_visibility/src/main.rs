/* 
- 模块中的元素默认私有
- 只有加了pub修饰符的元素，模块外可访问
- 内嵌模块元素可访问外层模块元素，反之需要根据修饰符
- out_mod不用加pub，main函数仍能访问；是因为out_mod和main在同一作用域，可以想象成外面还有一层根mod，在同一mod内
*/
#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    // out_mod::private_fn(); // wrong
    out_mod::pub_fn();
    out_mod::in_mod::crate_fn();
}

mod out_mod {
    // use self::in_mod::super_fn;

    fn private_fn () {println!("out_mod::private_fn")}
    pub fn pub_fn () {println!("out_mod::pub_fn")}
    // 模块内部，相互可访问
    fn indirect_access_fn () {
        private_fn();
        in_mod::super_fn() // 这里in_mod有没有pub都能访问，因为in_mod和indirect_access_fn同属out_mod元素
    }
    pub mod in_mod {
        // 还是在out_mod里，非模块外，忽略private
        use super::private_fn;
        fn indirect_access_fn2 () {
            private_fn()
        }

        fn indirect_access_fn3 () {
            crate::out_mod::private_fn();
        }

        // pub(in path)仅在给定路径可见，路径必须是父级或祖先模块
        pub(in crate::out_mod) fn pub_fn_in_out_mod () {println!("pub_fn_in_out_mod")}
        // pub(self) == 默认private
        pub(self) fn self_fn () {println!("self_fn")}
        // super 仅父级模块内
        pub(super) fn super_fn () {println!("super_fn")}
        // crate 仅当前crate 
        pub(crate) fn crate_fn () {println!("crate_fn")}

    }
}