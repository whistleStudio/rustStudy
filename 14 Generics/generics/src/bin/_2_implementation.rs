/* 

*/
#![allow(dead_code, unused_variables, non_snake_case)]

struct T {val: i32} // 具体类型T
struct Gen<T>(T); // 泛型T

impl Gen<T> {
    fn sta_show_T () {println!("static specify Gen<T>")}
    fn show_T (&self) {println!("specify Gen<T>")}
    fn get_val_T (&self) -> i32 {self.0.val} // i32实现了copy trait
} // 为具体Gen<T>类型实现

impl<T> Gen<T> {
    fn sta_show () {println!("static generic Gen")}
    fn show (&self) {println!("generic Gen")}
    fn get_val (&self) -> &T {&self.0} // struct T未实现copy trait, 形参用了&self, 返回也要引用才行，否则会导致T内部值发生move
} // 为泛型类型Gen实现
fn main () {
    let t = T{val: 1};
    let gen_t = Gen(t);
    let gen = Gen(2);

    Gen::<T>::sta_show_T();
    gen_t.show_T();
    println!("{}", gen_t.get_val_T());

    Gen::<i32>::sta_show();
    gen.show();
    println!("{}", gen.get_val());
}