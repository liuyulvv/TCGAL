use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::kernel::{edge_2::Edge2, number_type::NumberType, vertex_2::Vertex2};

pub struct Neighbors<NT: NumberType> {
    pub left: Option<Rc<RefCell<Edge2<NT>>>>,
    pub right: Option<Rc<RefCell<Edge2<NT>>>>,
    phantom: PhantomData<NT>,
}

#[derive(Debug, Clone)]
pub struct StatusStructure<NT: NumberType> {
    vertex_vec: Vec<Rc<RefCell<Vertex2<NT>>>>,
    edge_vec: Vec<Rc<RefCell<Edge2<NT>>>>,
}

impl<NT: NumberType> StatusStructure<NT> {
    pub fn new() -> Self {
        Self {
            vertex_vec: Vec::new(),
            edge_vec: Vec::new(),
        }
    }

    pub fn insert(&mut self, edge: Rc<RefCell<Edge2<NT>>>) -> Option<Neighbors<NT>> {
        let insert_index = self.insert_index(edge.clone());
        let edge_binding = edge.borrow();
        match insert_index {
            Some(index) => {
                self.vertex_vec.insert(index, edge_binding.source());
                self.edge_vec.insert(index, edge.clone());
                let left = self.edge_vec.get(index.wrapping_sub(1));
                let right = self.edge_vec.get(index.wrapping_add(1));
                match (left, right) {
                    (Some(left), Some(right)) => Some(Neighbors {
                        left: Some(left.clone()),
                        right: Some(right.clone()),
                        phantom: PhantomData,
                    }),
                    (Some(left), None) => Some(Neighbors {
                        left: Some(left.clone()),
                        right: None,
                        phantom: PhantomData,
                    }),
                    (None, Some(right)) => Some(Neighbors::<NT> {
                        left: None,
                        right: Some(right.clone()),
                        phantom: PhantomData,
                    }),
                    (None, None) => None,
                }
            }
            None => {
                let is_empty = self.vertex_vec.is_empty();
                self.vertex_vec.push(edge_binding.source());
                self.edge_vec.push(edge.clone());
                if is_empty {
                    None
                } else {
                    let left = self.edge_vec.get(self.edge_vec.len() - 2);
                    Some(Neighbors {
                        left: Some(left.unwrap().clone()),
                        right: None,
                        phantom: PhantomData,
                    })
                }
                // self.vertex_vec.insert(0, edge_binding.source());
                // self.edge_vec.insert(0, edge.clone());
                // if is_empty {
                //     None
                // } else {
                //     let right = self.edge_vec.get(1);
                //     Some(Neighbors {
                //         left: None,
                //         right: Some(right.unwrap().clone()),
                //         phantom: PhantomData,
                //     })
                // }
            }
        }
    }

    pub fn remove(&mut self, edge: Rc<RefCell<Edge2<NT>>>) -> Option<Neighbors<NT>> {
        let remove_index = self.edge_vec.iter().position(|e| {
            let e_binding = e.borrow();
            let edge_binding: std::cell::Ref<Edge2<NT>> = edge.borrow();
            *e_binding == *edge_binding
        });
        match remove_index {
            Some(index) => {
                self.vertex_vec.remove(index);
                self.edge_vec.remove(index);
                let left = self.edge_vec.get(index.wrapping_sub(1));
                let right = self.edge_vec.get(index);
                match (left, right) {
                    (Some(left), Some(right)) => Some(Neighbors {
                        left: Some(left.clone()),
                        right: Some(right.clone()),
                        phantom: PhantomData,
                    }),
                    (Some(left), None) => Some(Neighbors {
                        left: Some(left.clone()),
                        right: None,
                        phantom: PhantomData,
                    }),
                    (None, Some(right)) => Some(Neighbors::<NT> {
                        left: None,
                        right: Some(right.clone()),
                        phantom: PhantomData,
                    }),
                    (None, None) => None,
                }
            }
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vertex_vec.is_empty() && self.edge_vec.is_empty()
    }

    pub fn insert_index(&self, edge: Rc<RefCell<Edge2<NT>>>) -> Option<usize> {
        let edge_binding = edge.borrow();
        let source = edge_binding.source();
        self.vertex_vec.iter().position(|e| {
            let e_binding = e.borrow();
            let source_binding = source.borrow();
            e_binding.x() >= source_binding.x()
        })
    }
}
