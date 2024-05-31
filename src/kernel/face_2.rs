use std::{cell::RefCell, rc::Rc};

use super::{edge_2::Edge2, number_type::NumberType};

#[derive(Debug, Clone)]
pub struct Face2<NT: NumberType> {
    edges: Vec<Rc<RefCell<Edge2<NT>>>>,
}

impl<NT: NumberType> Face2<NT> {
    pub fn edges(&self) -> Vec<Rc<RefCell<Edge2<NT>>>> {
        self.edges.clone()
    }
}

impl<NT: NumberType> PartialEq for Face2<NT> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
