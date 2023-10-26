/* 
迭代器调用 Iterator::any 方法，若任一元素符合谓词(predicate: 返回bool的函数), 返回true
函数签名：
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `Self::Item` states it takes
        // arguments to the closure by value.
        F: FnMut(Self::Item) -> bool;
}

- iter() 原序列仍然可用
    core::slice
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

- into_iter() 原序列被消耗
  alloc::vec::Vec
  fn into_iter(self) -> Self::IntoIter

- Iterator::any 方法调用的是&mut self, 所以iterator不会被消耗
*/

pub fn show () {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("1 in vec1: {}", vec1.iter().any(|&x| x == 1));
    println!("vec1 still can be used: {:?}", vec1);
    let mut v2iter = vec2.into_iter();
    println!("1 in vec2: {}", v2iter.any(|x| x == 1));
    println!("1 in vec2: {}", v2iter.any(|x| x == 4)); // iterator不会被消耗
    // println!("{:?}", vec2); // 错误，vec2被into_iter()消耗
}