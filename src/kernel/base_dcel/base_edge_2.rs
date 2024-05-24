use std::fmt::Debug;

use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::{base_face_2::BaseFace2, base_vertex_2::BaseVertex2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseEdge2Type {
    Segment,
    Arc,
}

pub trait BaseEdge2<'a, NT: BaseNumberTypeTrait>: Clone + Sized + Debug + PartialEq {
    type Vertex: BaseVertex2<'a, NT>;
    type Face: BaseFace2<'a, NT>;

    fn new_segment(source: &Self::Vertex, target: &Self::Vertex) -> Self;
    fn new_arc(source: &Self::Vertex, target: &Self::Vertex) -> Self;
    fn source(&self) -> &Self::Vertex;
    fn set_source(&mut self, source: &Self::Vertex);
    fn target(&self) -> &Self::Vertex;
    fn set_target(&mut self, target: &Self::Vertex);
    fn twin(&self) -> Option<&Self>;
    fn set_twin(&mut self, twin: &Self);
    fn next(&self) -> Option<&Self>;
    fn set_next(&mut self, next: &Self);
    fn prev(&self) -> Option<&Self>;
    fn set_prev(&mut self, prev: &Self);
    fn face(&self) -> Option<&Self::Face>;
    fn set_face(&mut self, face: &Self::Face);
    fn edge_type(&self) -> BaseEdge2Type;
}
