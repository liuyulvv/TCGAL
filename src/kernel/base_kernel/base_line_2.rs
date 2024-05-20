use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

pub trait BaseLine2<T: BaseNumberTypeTrait> {
    fn a(&self) -> T;
    fn b(&self) -> T;
    fn c(&self) -> T;
}
