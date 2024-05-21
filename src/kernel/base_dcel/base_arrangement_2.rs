use std::fmt::Debug;

use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::{base_edge_2::BaseEdge2, base_face_2::BaseFace2, base_vertex_2::BaseVertex2};

pub trait BaseArrangement2<'a, NT: BaseNumberTypeTrait>: Copy + Clone + Sized + Debug {
    type Vertex: BaseVertex2<'a, NT>;
    type Edge: BaseEdge2<'a, NT>;
    type Face: BaseFace2<'a, NT>;

    fn vertices(&self) -> Vec<Self::Vertex>;
    fn edges(&self) -> Vec<Self::Edge>;
    fn faces(&self) -> Vec<Self::Face>;
}
