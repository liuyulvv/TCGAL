use std::{cell::RefCell, rc::Rc};

use super::{edge_2::Edge2, number_type::NumberType};

#[derive(Debug, Clone)]
pub struct Face2<T: NumberType> {
    edges: Vec<Rc<RefCell<Edge2<T>>>>,
}

impl<T: NumberType> Face2<T> {
    pub fn edges(&self) -> Vec<Rc<RefCell<Edge2<T>>>> {
        self.edges.clone()
    }
}

impl<T: NumberType> PartialEq for Face2<T> {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
