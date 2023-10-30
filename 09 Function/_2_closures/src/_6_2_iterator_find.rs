/* 
Iterator::find
传递一元谓词，遍历过程中，又满足条件的就返回Some(T); 无一满足条件，就返回None

signature
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool;
}
*/

pub fn show () {
    let v1 = vec![1, 2, 3, 3];
    let mut iter1 = v1.iter(); 
    // v1iter本身Self::Item是&i32类型，find<P>传递的向谓词里传递的是&Self::Item，这里也就是&&i32
    println!("find 2 in v1: {:?}", iter1.find(|&&v| v == 4)); //&&解构两次获得i32

    let mut iter2 = v1.into_iter();
    println!("find 2 in v1: {:?}", iter2.find(|&v| v == 1)); // 这里会把第一个元素1作为返回值移除

    // 找索引位置要用Iterator::positon
    let idx3 = iter2.position(|v| v == 3);
    assert_eq!(idx3, Some(1), "-----");
}