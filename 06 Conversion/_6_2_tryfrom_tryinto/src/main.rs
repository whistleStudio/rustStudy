/* 
- TryFrom TryInto 和 From Into 类似; 机制都相同
- 只是会返回Result
- 自定义类型，想要实现比较==，可以通过添加PartialEq属性，派生比较方法
- 涉及到自定类型的断言assert_eq!时，不仅要实现PartialEq,还要实现Debug
*/

#[derive(PartialEq, Debug)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[derive(PartialEq, Debug)]
struct OddNumber(i32);

impl TryInto<OddNumber> for i32 {
    type Error = String;

    fn try_into(self) -> Result<OddNumber, Self::Error> {
        if self % 2 == 0 {
            Err("not Odd".to_string())
        } else {
            Ok(OddNumber(self))
        }
    }
}

fn main() {
    println!("1st line print: {}", EvenNumber(1) == EvenNumber(2));
    assert_eq!(EvenNumber::try_from(1), Err(()));
    let n1: Result<EvenNumber, _> = 2.try_into();
    assert_eq!(n1, Ok(EvenNumber(2)));
    // assert_eq!(EvenNumber::try_from(1), Ok(EvenNumber(1))); // 不相等，报错

    let n2: Result<OddNumber, _> = 1.try_into();
    assert_eq!(n2, Ok(OddNumber(1)));
    // assert_eq!(OddNumber::try_from(1), Ok(OddNumber(1))); // 编译错误，不会自动实现try_from
}
