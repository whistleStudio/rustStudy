//! - `format!`: 输出为字符串
//! - `print!`: 输出至控制台(io::stdout)
//! - `println!`: 输出至控制台，换行
//! - `eprint!`: 输出至控制台(io::stderr)
//! - `eprintln!`: 输出至控制台(io::stderr)

#[allow(dead_code)]
pub mod overall{
  pub fn show() {
    let format_str = format!("{} like this", "format!");
    println!("{}", format_str);
    eprintln!("err print");
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Tom", "Jerry");
    println!("{subject} {verb} {object}, {object} {verb} {subject} too?", object="u", verb="love", subject="i");

    println!("Base 10:\t{}", 111);
    println!("Base 2(binary):\t{:b}", 111);
    println!("Base 8(octal):\t{:o}", 111);
    println!("Base 16(hex):\t{:x}", 111);
    println!("Base 16(HEX):\t{:X}", 111);

    // 右对齐调整至指定宽度，超出时就正常显示，宽度限制就无效了
    println!("{:>5}", "abcd");
    println!{"{word:0>5}", word="abc"};
    // 左对齐
    println!("{word:0<5}", word="abc");
    // 格式标识符用 命名参数$ 表示
    println!("{word:0<width$}", word="abc", width=5); 
    // rust1.58以上，直接捕获环境变量
    let num = 1.0;
    let wid = 5;
    println!("{num:0>wid$}");

    /* activity */
    #[derive(Debug)]
    struct Structure(i32);
    
    let s = Structure(1);
    println!("{:?}",s);
  }
}