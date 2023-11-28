#[derive(Debug)]
struct Foo<'a> {
  v: &'a i32
}

fn main() {
  let foo;                    // -------------+-- foo start
  {                           //              |
    let v = 123;              // -------------+-- v start
    foo = Foo {               //              |
      v: &v                   //              |
    }                         //              |
  } 
//   println!("foo: {:?}", foo); //              |
}                             // -------------+-- foo over