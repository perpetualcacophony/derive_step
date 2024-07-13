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
            use super::$enum;

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

macro_rules! test_forward {
    {$enum:ident=> $($first:ident, $steps:literal => $expected:expr),+} => {
        paste::paste!{
        mod forward {
            use std::iter::Step;
            use super::$enum;

            $(

            #[test]
            #[allow(non_snake_case)]
            fn [< $first _ $steps >]() {
                assert_eq!(
                    Step::forward_checked($enum::$first, $steps),
                    $expected
                )
            }

            )+
        }
        }
    };
}

test_forward! { MyEnum=>
    A, 0 => Some(MyEnum::A),
    A, 1 => Some(MyEnum::B),
    A, 2 => Some(MyEnum::C),
    A, 3 => None,

    B, 0 => Some(MyEnum::B),
    B, 1 => Some(MyEnum::C),
    B, 2 => None,

    C, 0 => Some(MyEnum::C),
    C, 1 => None
}

macro_rules! test_backward {
    {$enum:ident=> $($first:ident, $steps:literal => $expected:expr),+} => {
        paste::paste!{
        mod backward {
            use std::iter::Step;
            use super::$enum;

            $(

            #[test]
            #[allow(non_snake_case)]
            fn [< $first _ $steps >]() {
                assert_eq!(
                    Step::backward_checked($enum::$first, $steps),
                    $expected
                )
            }

            )+
        }
        }
    };
}

test_backward! { MyEnum=>
    A, 0 => Some(MyEnum::A),
    A, 1 => None,

    B, 0 => Some(MyEnum::B),
    B, 1 => Some(MyEnum::A),
    B, 2 => None,

    C, 0 => Some(MyEnum::C),
    C, 1 => Some(MyEnum::B),
    C, 2 => Some(MyEnum::A),
    C, 3 => None
}
