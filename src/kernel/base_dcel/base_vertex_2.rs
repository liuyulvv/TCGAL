use std::fmt::Debug;

use crate::{
    kernel::base_kernel::base_point_2::BasePoint2,
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::base_edge_2::BaseEdge2;

pub trait BaseVertex2<'a, NT: BaseNumberTypeTrait>:
    Clone + Sized + Debug + PartialEq + PartialOrd
{
    type Point: BasePoint2<NT>;
    type Edge: BaseEdge2<'a, NT>;

    fn new(x: NT, y: NT) -> Self;
    fn x(&self) -> NT;
    fn y(&self) -> NT;
    fn edges(&self) -> &Vec<&Self::Edge>;
    fn add_edge(&mut self, edge: &'a Self::Edge);
    fn remove_edge(&mut self, edge: &'a Self::Edge);
    fn equals(&self, other: &Self) -> bool;
    fn to_point(&self) -> Self::Point;
}
