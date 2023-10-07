/* 
绑定
- 和守卫差不多
*/
pub fn show () {
    let age = 19;

    match age {
        n@1 ..= 19 => {},
        n => {} 
    }

    let num = Some(1);

    match num {
        Some(n@1) => {},
        Some(n) => {},
        None => {}
    }
}