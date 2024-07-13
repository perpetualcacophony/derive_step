#![feature(step_trait)]

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, derive_step::Step)]
pub enum MyEnum {
    A,
    B,
    C,
}

#[allow(dead_code)]
const MY_ENUM_RANGE: std::ops::RangeInclusive<MyEnum> = MyEnum::A..=MyEnum::C;

macro_rules! test_steps_between {
    {$enum:ident=> $($first:ident, $second:ident => $expected:expr),+} => {
        paste::paste!{
        mod steps_between {
            use std::iter::Step;
            use super::MyEnum;

        $(
            #[test]
            #[allow(non_snake_case)]
            fn [< $first _ $second >]() {
                assert_eq!(
                    Step::steps_between(&$enum::$first, &$enum::$second),
                    $expected
                )
            }
        )+
        }
        }
    };
}

test_steps_between! { MyEnum=>
    A, A => Some(0),
    A, B => Some(1),
    A, C => Some(2),

    B, A => None,
    B, B => Some(0),
    B, C => Some(1),

    C, A => None,
    C, B => None,
    C, C => Some(0)
}

#[test]
fn forward() {
    use std::iter::Step;
    assert_eq!(Step::forward_checked(MyEnum::A, 1), Some(MyEnum::B));
    assert_eq!(Step::forward_checked(MyEnum::A, 2), Some(MyEnum::C));
    assert_eq!(Step::forward_checked(MyEnum::A, 3), None);
}
