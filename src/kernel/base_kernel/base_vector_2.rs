use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

pub trait BaseVector2<NT: BaseNumberTypeTrait>:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<NT, Output = Self>
    + Div<NT, Output = Self>
    + Copy
    + Clone
    + Sized
    + Debug
{
    fn new(x: NT, y: NT) -> Self;
    fn x(&self) -> NT;
    fn y(&self) -> NT;
    fn length(&self) -> NT;
    fn normalize(&self) -> Self;
    fn dot(&self, other: &Self) -> NT;
    fn cross(&self, other: &Self) -> NT;
}
