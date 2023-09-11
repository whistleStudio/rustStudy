fn main() {
    let mut _v = 1;
    {
        let _v = _v;
        // _v = 2; // 数据被同名不可变绑定后，冻结变为不可变
    }
    _v = 3;
}
