use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

pub trait BasePoint2<T: BaseNumberTypeTrait> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}
