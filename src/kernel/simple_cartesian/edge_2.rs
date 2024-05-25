use crate::{
    kernel::base_dcel::base_edge_2::{BaseEdge2, BaseEdge2Type},
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::{face_2::Face2, vertex_2::Vertex2};

#[derive(Debug, Clone)]
pub struct Edge2<'a, NT: BaseNumberTypeTrait> {
    source: &'a Vertex2<'a, NT>,
    target: &'a Vertex2<'a, NT>,
    twin: Option<&'a Self>,
    next: Option<&'a Self>,
    prev: Option<&'a Self>,
    face: Option<Face2<'a, NT>>,
    edge_type: BaseEdge2Type,
}

impl<'a, NT: BaseNumberTypeTrait> BaseEdge2<'a, NT> for Edge2<'a, NT> {
    type Vertex = Vertex2<'a, NT>;
    type Face = Face2<'a, NT>;

    fn new_segment(source: &'a Self::Vertex, target: &'a Self::Vertex) -> Self {
        Self {
            source,
            target,
            twin: None,
            next: None,
            prev: None,
            face: None,
            edge_type: BaseEdge2Type::Segment,
        }
    }

    fn new_arc(source: &'a Self::Vertex, target: &'a Self::Vertex) -> Self {
        Self {
            source,
            target,
            twin: None,
            next: None,
            prev: None,
            face: None,
            edge_type: BaseEdge2Type::Arc,
        }
    }

    fn source(&self) -> &Self::Vertex {
        self.source
    }

    fn set_source(&mut self, source: &'a Self::Vertex) {
        self.source = source
    }

    fn target(&self) -> &Self::Vertex {
        self.target
    }

    fn set_target(&mut self, target: &'a Self::Vertex) {
        self.target = target
    }

    fn twin(&self) -> Option<&Self> {
        self.twin
    }

    fn set_twin(&mut self, twin: &'a Self) {
        self.twin = Some(twin)
    }

    fn next(&self) -> Option<&Self> {
        self.next
    }

    fn set_next(&mut self, next: &'a Self) {
        self.next = Some(next)
    }

    fn prev(&self) -> Option<&Self> {
        self.prev
    }

    fn set_prev(&mut self, prev: &'a Self) {
        self.prev = Some(prev)
    }

    fn face(&self) -> Option<&Self::Face> {
        self.face.as_ref()
    }

    fn set_face(&mut self, face: &'a Self::Face) {
        self.face = Some(face.clone())
    }

    fn edge_type(&self) -> BaseEdge2Type {
        self.edge_type
    }
}

impl<'a, NT: BaseNumberTypeTrait> PartialEq for Edge2<'a, NT> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
