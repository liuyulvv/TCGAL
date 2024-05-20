use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::base_point_2::BasePoint2;

pub trait BaseCircle2<'a, T: BaseNumberTypeTrait> {
    fn center(&self) -> Box<dyn BasePoint2<T> + 'a>;
    fn radius(&self) -> T;
}
