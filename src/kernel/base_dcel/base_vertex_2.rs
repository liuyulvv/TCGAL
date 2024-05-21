use std::fmt::Debug;

use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::base_edge_2::BaseEdge2;

pub trait BaseVertex2<'a, NT: BaseNumberTypeTrait>: Copy + Clone + Sized + Debug {
    type Edge: BaseEdge2<'a, NT>;

    fn x(&self) -> NT;
    fn y(&self) -> NT;
    fn edges(&self) -> Vec<Self::Edge>;
}
