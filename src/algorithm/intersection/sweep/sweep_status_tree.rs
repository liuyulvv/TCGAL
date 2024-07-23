use std::{cell::RefCell, rc::Rc};

use crate::kernel::number_type::NumberType;

use super::{sweep_status_node::SweepStatusNode, sweep_status_tree_node::SweepStatusTreeNode};

pub struct SweepStatusTree<T: NumberType> {
    root: Option<Rc<RefCell<SweepStatusTreeNode<T>>>>,
}

impl<T: NumberType> SweepStatusTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: Rc<RefCell<SweepStatusNode<T>>>) {
        self.root = SweepStatusTreeNode::insert(self.root.clone(), value);
    }

    pub fn delete(&mut self, value: Rc<RefCell<SweepStatusNode<T>>>) {
        self.root = SweepStatusTreeNode::delete(self.root.clone(), value);
    }

    pub fn clear(&mut self) {
        self.root = None;
    }

    pub fn mid_order_traversal(&self) -> Vec<Rc<RefCell<SweepStatusNode<T>>>> {
        let mut result = Vec::new();
        SweepStatusTreeNode::mid_order_traversal(self.root.clone(), &mut result);
        result
    }
}
