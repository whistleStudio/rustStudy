fn main() {
    // 变量名_打头，就不会warn unused variables
    let _imt;
    _imt = 1;
    let mut _m = 1;
    _m = 2;
}
