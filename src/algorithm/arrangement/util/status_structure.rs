use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{
    algorithm::location::point_2_segment_2::is_point_2_on_segment_2,
    kernel::{
        edge_2::{Edge2, Edge2Type},
        number_type::NumberType,
        vertex_2::Vertex2,
    },
};

pub struct Neighbors<NT: NumberType> {
    pub left: Option<Rc<RefCell<Edge2<NT>>>>,
    pub right: Option<Rc<RefCell<Edge2<NT>>>>,
    phantom: PhantomData<NT>,
}

pub struct PEdges<NT: NumberType> {
    pub u: Vec<Rc<RefCell<Edge2<NT>>>>,
    pub l: Vec<Rc<RefCell<Edge2<NT>>>>,
    pub c: Vec<Rc<RefCell<Edge2<NT>>>>,
}

#[derive(Debug, Clone)]
pub struct StatusStructure<NT: NumberType> {
    vertex_vec: Vec<NT>,
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
        self.update_x(edge.clone());
        let insert_index = self.calculate_insert_index(edge.clone());
        let edge_binding = edge.borrow();
        match insert_index {
            Some(index) => {
                self.vertex_vec
                    .insert(index, edge_binding.source().borrow().x());
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
                self.vertex_vec.push(edge_binding.source().borrow().x());
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
                    _ => None,
                }
            }
            None => None,
        }
    }

    pub fn swap(
        &mut self,
        left: Rc<RefCell<Edge2<NT>>>,
        right: Rc<RefCell<Edge2<NT>>>,
    ) -> Vec<Neighbors<NT>> {
        let left_index = self.edge_vec.iter().position(|e| {
            let e_binding = e.borrow();
            let left_binding = left.borrow();
            *e_binding == *left_binding
        });
        let right_index = self.edge_vec.iter().position(|e| {
            let e_binding = e.borrow();
            let right_binding = right.borrow();
            *e_binding == *right_binding
        });
        match (left_index, right_index) {
            (Some(left_index), Some(right_index)) => {
                self.edge_vec.swap(left_index, right_index);
                self.vertex_vec.swap(left_index, right_index);
                let new_left = self.edge_vec.get(left_index.wrapping_sub(1));
                let new_right = self.edge_vec.get(right_index.wrapping_add(1));
                match (new_left, new_right) {
                    (Some(new_left), Some(new_right)) => {
                        let left_neighbor = Neighbors {
                            left: Some(new_left.clone()),
                            right: Some(right.clone()),
                            phantom: PhantomData,
                        };
                        let right_neighbor = Neighbors {
                            left: Some(left.clone()),
                            right: Some(new_right.clone()),
                            phantom: PhantomData,
                        };
                        vec![left_neighbor, right_neighbor]
                    }
                    (Some(new_left), None) => {
                        let left_neighbor = Neighbors {
                            left: Some(new_left.clone()),
                            right: Some(right.clone()),
                            phantom: PhantomData,
                        };
                        vec![left_neighbor]
                    }
                    (None, Some(new_right)) => {
                        let right_neighbor = Neighbors {
                            left: Some(left.clone()),
                            right: Some(new_right.clone()),
                            phantom: PhantomData,
                        };
                        vec![right_neighbor]
                    }
                    (None, None) => {
                        vec![]
                    }
                }
            }
            _ => {
                vec![]
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vertex_vec.is_empty() && self.edge_vec.is_empty()
    }

    pub fn get_p_edges(&self, p: Rc<RefCell<Vertex2<NT>>>) -> PEdges<NT> {
        let p_binding = p.borrow();
        let mut u = Vec::new();
        let mut l = Vec::new();
        let mut c = Vec::new();
        for edge in self.edge_vec.iter() {
            let edge_binding = edge.borrow();
            let source = edge_binding.source();
            let target = edge_binding.target();
            if source.borrow().equals(&p_binding) {
                u.push(edge.clone());
            } else if target.borrow().equals(&p_binding) {
                l.push(edge.clone());
            } else {
                match edge_binding.edge_type() {
                    Edge2Type::Segment => {
                        let point_2 = p_binding.to_point();
                        let segment_2 = edge_binding.to_segment();
                        if is_point_2_on_segment_2(&point_2, &segment_2) {
                            c.push(edge.clone());
                        }
                    }
                    Edge2Type::Arc => {
                        todo!()
                    }
                }
            }
        }

        let len = c.len();
        let mut i = 0;
        let mut horizontal_index = Vec::new();

        while i < len {
            let edge = c.get_mut(i).unwrap();
            let edge_binding = edge.borrow();
            if edge_binding.is_horizontal() {
                horizontal_index.push(i);
            }
            i += 1;
        }

        horizontal_index.iter().for_each(|index| {
            c.push(c.get(*index).unwrap().clone());
        });

        horizontal_index.iter().for_each(|index| {
            c.remove(*index);
        });

        PEdges { u, l, c }
    }

    pub fn calculate_insert_index(&mut self, edge: Rc<RefCell<Edge2<NT>>>) -> Option<usize> {
        let edge_binding = edge.borrow();
        let x = edge_binding.source().borrow().x();
        self.vertex_vec.iter().position(|e| e >= &x)
    }

    pub fn update_x(&mut self, edge: Rc<RefCell<Edge2<NT>>>) {
        let edge_binding = edge.borrow();
        let source = edge_binding.source();
        let y = source.borrow().y();
        for (index, edge) in self.edge_vec.iter_mut().enumerate() {
            let x = Self::calculate_x(edge.clone(), y);
            self.vertex_vec[index] = x;
        }
    }

    fn calculate_x(edge: Rc<RefCell<Edge2<NT>>>, y: NT) -> NT {
        let edge_binding = edge.borrow();
        let source = edge_binding.source();
        let target = edge_binding.target();
        let source_binding = source.borrow();
        let target_binding = target.borrow();
        let x = source_binding.x()
            + (y - source_binding.y()) * (target_binding.x() - source_binding.x())
                / (target_binding.y() - source_binding.y());
        x
    }
}
