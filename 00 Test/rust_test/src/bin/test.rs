#[derive(Debug)]
struct S<T>(T);

fn main () {
    let s;
    {
        s = &S(1);
    }
    println!("{:?}", s)
}