use std::{cell::RefCell, rc::Rc};

use super::{
    arc_segment_2::ArcSegment2, face_2::Face2, line_segment_2::LineSegment2,
    number_type::NumberType, util_enum::Edge2Type, vertex_2::Vertex2,
};

#[derive(Debug, Clone)]
pub struct Edge2<T: NumberType> {
    source: Rc<RefCell<Vertex2<T>>>,
    target: Rc<RefCell<Vertex2<T>>>,
    twin: Option<Rc<RefCell<Self>>>,
    next: Option<Rc<RefCell<Self>>>,
    prev: Option<Rc<RefCell<Self>>>,
    face: Option<Rc<RefCell<Face2<T>>>>,
    edge_type: Edge2Type,
}

impl<T: NumberType> Edge2<T> {
    pub fn new_segment(source: Rc<RefCell<Vertex2<T>>>, target: Rc<RefCell<Vertex2<T>>>) -> Self {
        Self {
            source,
            target,
            twin: None,
            next: None,
            prev: None,
            face: None,
            edge_type: Edge2Type::Segment,
        }
    }

    pub fn new_arc(source: Rc<RefCell<Vertex2<T>>>, target: Rc<RefCell<Vertex2<T>>>) -> Self {
        Self {
            source,
            target,
            twin: None,
            next: None,
            prev: None,
            face: None,
            edge_type: Edge2Type::Arc,
        }
    }

    pub fn source(&self) -> Rc<RefCell<Vertex2<T>>> {
        self.source.clone()
    }

    pub fn set_source(&mut self, source: Rc<RefCell<Vertex2<T>>>) {
        self.source = source
    }

    pub fn target(&self) -> Rc<RefCell<Vertex2<T>>> {
        self.target.clone()
    }

    pub fn set_target(&mut self, target: Rc<RefCell<Vertex2<T>>>) {
        self.target = target
    }

    pub fn twin(&self) -> Option<Rc<RefCell<Self>>> {
        self.twin.clone()
    }

    pub fn set_twin(&mut self, twin: Rc<RefCell<Self>>) {
        self.twin = Some(twin)
    }

    pub fn next(&self) -> Option<Rc<RefCell<Self>>> {
        self.next.clone()
    }

    pub fn set_next(&mut self, next: Rc<RefCell<Self>>) {
        self.next = Some(next)
    }

    pub fn prev(&self) -> Option<Rc<RefCell<Self>>> {
        self.prev.clone()
    }

    pub fn set_prev(&mut self, prev: Rc<RefCell<Self>>) {
        self.prev = Some(prev)
    }

    pub fn face(&self) -> Option<Rc<RefCell<Face2<T>>>> {
        self.face.clone()
    }

    pub fn set_face(&mut self, face: Rc<RefCell<Face2<T>>>) {
        self.face = Some(face.clone())
    }

    pub fn edge_type(&self) -> Edge2Type {
        self.edge_type
    }

    pub fn to_segment(&self) -> LineSegment2<T> {
        let source = self.source.borrow();
        let target = self.target.borrow();
        LineSegment2::new(source.to_point(), target.to_point())
    }

    pub fn to_arc(&self) -> ArcSegment2<T> {
        todo!()
    }
}

impl<T: NumberType> PartialEq for Edge2<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.edge_type != other.edge_type {
            return false;
        }
        match self.edge_type {
            Edge2Type::Segment => {
                let self_segment = self.to_segment();
                let other_segment = other.to_segment();
                self_segment == other_segment
            }
            Edge2Type::Arc => {
                let self_arc = self.to_arc();
                let other_arc = other.to_arc();
                self_arc == other_arc
            }
        }
    }
}
