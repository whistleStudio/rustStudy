/* 
- 泛型参数用大驼峰形式表示
*/
#![allow(unused_variables)]

struct A; // 类型A

struct UnitA(A); // 类型UnitA; 元组结构体中第一个元素类型A

// 第一次使用T是在<T>中出现，所以这里T是泛型参数
struct GenA<T>(T); // 泛型类型GenA
fn main() {
    let unit_a = UnitA(A);
    
    // 显示指定
    let gen1: GenA<A> = GenA(A);
    let gen11: GenA<A> = GenA::<A>(A);

    // 隐式推导
    let gen2 = GenA(A);
    let gen3 = GenA(unit_a);
}
