#![feature(step_trait)]

use derive_step::Step;
use std::iter::Step;

#[derive(Copy, Clone, PartialEq, PartialOrd, Step)]
enum MyEnum {
    A,
    B,
    C,
}
