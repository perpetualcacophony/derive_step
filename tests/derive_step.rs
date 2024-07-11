#![feature(step_trait)]

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, derive_step::Step)]
pub enum MyEnum {
    A,
    B,
    C,
}

#[allow(dead_code)]
const MY_ENUM_RANGE: std::ops::RangeInclusive<MyEnum> = MyEnum::A..=MyEnum::C;

#[test]
fn steps_between() {
    use std::iter::Step;
    assert_eq!(Step::steps_between(&MyEnum::A, &MyEnum::C), Some(2))
}

#[test]
fn forward() {
    use std::iter::Step;
    assert_eq!(Step::forward_checked(MyEnum::A, 1), Some(MyEnum::B));
    assert_eq!(Step::forward_checked(MyEnum::A, 2), Some(MyEnum::C));
    assert_eq!(Step::forward_checked(MyEnum::A, 3), None);
}
