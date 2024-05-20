use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

pub trait BaseVector2<T: BaseNumberTypeTrait> {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn length(&self) -> T;
    fn normalize(&self) -> Self;
    fn dot(&self, other: &Self) -> T;
    fn cross(&self, other: &Self) -> T;
}
