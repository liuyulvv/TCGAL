use std::{
    fmt::Debug,
    ops::{Add, Sub},
};

use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::base_vector_2::BaseVector2;

pub trait BasePoint2<NT: BaseNumberTypeTrait>:
    Add<Output = Self::Vector2>
    + Sub<Output = Self::Vector2>
    + Copy
    + Clone
    + Sized
    + Debug
    + PartialOrd
{
    type Vector2: BaseVector2<NT>;

    fn new(x: NT, y: NT) -> Self;
    fn x(&self) -> NT;
    fn y(&self) -> NT;
    fn equals(&self, other: &Self) -> bool;
    fn get_vector(&self) -> Self::Vector2;
}
