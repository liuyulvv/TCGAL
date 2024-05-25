use crate::{
    kernel::base_dcel::base_face_2::BaseFace2,
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::edge_2::Edge2;

#[derive(Debug, Clone)]
pub struct Face2<'a, NT: BaseNumberTypeTrait> {
    edges: Vec<&'a Edge2<'a, NT>>,
}

impl<'a, NT: BaseNumberTypeTrait> BaseFace2<'a, NT> for Face2<'a, NT> {
    type Edge = Edge2<'a, NT>;

    fn edges(&self) -> &Vec<&Self::Edge> {
        &self.edges
    }
}

impl<'a, NT: BaseNumberTypeTrait> PartialEq for Face2<'a, NT> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
