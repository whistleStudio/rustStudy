/*
crate是rust的编译单元，执行rustc xxx.rs时，xxx.rs会被当做一个crate文件;
如果xxx.rs有mod声明的话，会在编译前，先把mod里的内容引入；
只有crate文件会被编译

- 默认rustc 会将crate编译成binary文件
-   
1 rustc --crate-type=lib xxx.rs将会编译成library文件
默认library名为libxxx.rlib(可通过--crate-name配置项修改)

2 rustc .\executable.rs --extern xxx=libxxx.rlib; ./executable 
将library链接到待编译为binary的文件，编译完成后执行

    executable.rs 待编译为binary的crate
    xxx=libxxx.rlib 左值xxx名称任意，这里相当于将xxx.rs以mod xxx形式引入executable.rs, 
    ; 命令行连续指令
    ./executable 编译成功就会生成同名的executable.exe可执行文件，执行它

*/

fn main () {
    a::pf();
    // a::f();
}