/* 
if let 
- match的精简版，无需考虑穷尽性；
- if let 左值 = 右值；左值相当于match的pattern, 右值相当于需要match匹配的值
*/
fn main() {
    let n = 1;
    
    if let 1 = n {
        
    }
}
