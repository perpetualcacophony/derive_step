#![feature(step_trait)]

use derive_step::Step;

#[derive(Copy, Clone, PartialEq, PartialOrd, Step)]
pub enum MyEnum {
    A,
    B,
    C,
}

#[allow(dead_code)]
const MY_ENUM_RANGE: std::ops::RangeInclusive<MyEnum> = MyEnum::A..=MyEnum::C;
