use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::base_edge_2::BaseEdge2;

pub trait BaseVertex2<'a, T: BaseNumberTypeTrait> {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn edges(&self) -> Vec<Box<dyn BaseEdge2<'a, T> + 'a>>;
}
