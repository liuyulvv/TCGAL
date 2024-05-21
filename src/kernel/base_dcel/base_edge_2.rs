use std::fmt::Debug;

use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::{base_face_2::BaseFace2, base_vertex_2::BaseVertex2};

pub trait BaseEdge2<'a, NT: BaseNumberTypeTrait>: Copy + Clone + Sized + Debug {
    type Vertex: BaseVertex2<'a, NT>;
    type Face: BaseFace2<'a, NT>;

    fn source(&self) -> Self::Vertex;
    fn target(&self) -> Self::Vertex;
    fn twin(&self) -> Option<Self>;
    fn next(&self) -> Option<Self>;
    fn prev(&self) -> Option<Self>;
    fn face(&self) -> Option<Self::Face>;
}
