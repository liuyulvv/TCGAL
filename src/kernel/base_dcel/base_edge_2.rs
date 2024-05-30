use std::fmt::Debug;

use crate::{
    kernel::base_kernel::{base_arc_2::BaseArc2, base_segment_2::BaseSegment2},
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::{base_face_2::BaseFace2, base_vertex_2::BaseVertex2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseEdge2Type {
    Segment,
    Arc,
}

pub trait BaseEdge2<'a, NT: BaseNumberTypeTrait>: Clone + Sized + Debug + PartialEq {
    type Segment: BaseSegment2<NT>;
    type Arc: BaseArc2<NT>;
    type Vertex: BaseVertex2<'a, NT>;
    type Face: BaseFace2<'a, NT>;

    fn new_segment(source: &'a Self::Vertex, target: &'a Self::Vertex) -> Self;
    fn new_arc(source: &'a Self::Vertex, target: &'a Self::Vertex) -> Self;
    fn source(&self) -> &Self::Vertex;
    fn set_source(&mut self, source: &'a Self::Vertex);
    fn target(&self) -> &Self::Vertex;
    fn set_target(&mut self, target: &'a Self::Vertex);
    fn twin(&self) -> Option<&Self>;
    fn set_twin(&mut self, twin: &'a Self);
    fn next(&self) -> Option<&Self>;
    fn set_next(&mut self, next: &'a Self);
    fn prev(&self) -> Option<&Self>;
    fn set_prev(&mut self, prev: &'a Self);
    fn face(&self) -> Option<&Self::Face>;
    fn set_face(&mut self, face: &'a Self::Face);
    fn edge_type(&self) -> BaseEdge2Type;
    fn to_segment(&self) -> Self::Segment;
    fn to_arc(&self) -> Self::Arc;
}
