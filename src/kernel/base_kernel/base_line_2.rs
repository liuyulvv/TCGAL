use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

pub trait BaseLine2<NT: BaseNumberTypeTrait>: Copy + Clone + Sized {
    fn a(&self) -> NT;
    fn b(&self) -> NT;
    fn c(&self) -> NT;
}
