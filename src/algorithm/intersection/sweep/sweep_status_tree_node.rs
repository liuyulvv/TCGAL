use super::sweep_status_node::SweepStatusNode;
use crate::kernel::number_type::NumberType;
use std::{cell::RefCell, rc::Rc};

type ValueRc<T> = Rc<RefCell<SweepStatusNode<T>>>;
type OptionTreeNodeRc<T> = Option<Rc<RefCell<SweepStatusTreeNode<T>>>>;

#[derive(Debug, Clone)]
pub struct SweepStatusTreeNode<T: NumberType> {
    value: ValueRc<T>,
    height: i32,
    left: OptionTreeNodeRc<T>,
    right: OptionTreeNodeRc<T>,
}

impl<T: NumberType> SweepStatusTreeNode<T> {
    pub fn new(value: ValueRc<T>) -> Self {
        Self {
            value,
            height: 0,
            left: None,
            right: None,
        }
    }

    pub fn height(node: OptionTreeNodeRc<T>) -> i32 {
        match node {
            Some(node) => node.borrow().height,
            None => -1,
        }
    }

    pub fn insert(node: OptionTreeNodeRc<T>, value: ValueRc<T>) -> OptionTreeNodeRc<T> {
        match node {
            Some(mut node) => {
                {
                    let node_borrow = node.borrow();
                    let node_value_borrow = node_borrow.value.borrow();
                    let value_borrow = value.borrow();
                    if *value_borrow < *node_value_borrow {
                        drop(value_borrow);
                        drop(node_value_borrow);
                        drop(node_borrow);
                        // let old_left = node_borrow.left.clone();
                        let new_left =
                            Self::insert(Some(node.borrow_mut()), value.clone()).unwrap();
                        {
                            let new_left_borrow = new_left.borrow_mut();
                            let mut new_left_value_borrow = new_left_borrow.value.borrow_mut();

                            node_value_borrow.left = Some(value.clone());
                            drop(node_value_borrow);
                            new_left_value_borrow.right = Some(node_borrow.value.clone());
                            match old_left {
                                Some(old_left) => {
                                    if !Rc::ptr_eq(&old_left, &new_left) {
                                        new_left_value_borrow.left =
                                            Some(old_left.borrow_mut().value.clone());
                                        old_left.borrow_mut().value.borrow_mut().right =
                                            Some(value.clone());
                                    }
                                }
                                None => {}
                            }
                        }
                        node_borrow.left = Some(new_left);
                    } else if *value_borrow >= *node_value_borrow {
                        drop(value_borrow);
                        let old_right = node_borrow.right.clone();
                        let new_right = Self::insert(old_right.clone(), value.clone()).unwrap();
                        {
                            let new_right_borrow = new_right.borrow_mut();
                            let mut new_right_value_borrow = new_right_borrow.value.borrow_mut();

                            node_value_borrow.right = Some(value.clone());
                            drop(node_value_borrow);
                            new_right_value_borrow.left = Some(node_borrow.value.clone());
                            match old_right {
                                Some(old_right) => {
                                    if !Rc::ptr_eq(&old_right, &new_right) {
                                        new_right_value_borrow.right =
                                            Some(old_right.borrow_mut().value.clone());
                                        old_right.borrow_mut().value.borrow_mut().left =
                                            Some(value.clone());
                                    }
                                }
                                None => {}
                            }
                        }
                        node_borrow.right = Some(new_right);
                    }
                }
                Self::update_height(Some(node.clone()));
                node = Self::rotate(Some(node)).unwrap();
                Some(node)
            }
            None => Some(Rc::new(RefCell::new(SweepStatusTreeNode::new(value)))),
        }
    }

    pub fn delete(node: OptionTreeNodeRc<T>, value: ValueRc<T>) -> OptionTreeNodeRc<T> {
        match node {
            Some(mut node) => {
                if Rc::ptr_eq(&node.borrow().value.clone(), &value) {
                    if node.borrow().left.is_none() || node.borrow().right.is_none() {
                        let child = if node.borrow().left.is_some() {
                            node.borrow().left.clone()
                        } else {
                            node.borrow().right.clone()
                        };
                        match child {
                            Some(child) => {
                                node = child;
                            }
                            None => {
                                return None;
                            }
                        }
                    } else {
                        let mut temp = node.borrow().right.clone().unwrap();
                        loop {
                            let temp_left = temp.borrow().left.clone();
                            if temp_left.is_none() {
                                break;
                            }
                            temp = temp_left.unwrap();
                        }
                        let right = node.borrow().right.clone();
                        node.borrow_mut().right = Self::delete(right, temp.borrow().value.clone());
                        node.borrow_mut().value = temp.borrow().value.clone();
                    }
                    Self::update_height(Some(node.clone()));
                    node = Self::rotate(Some(node)).unwrap();
                    Some(node)
                } else {
                    if value < node.borrow().value {
                        let left = node.borrow().left.clone();
                        node.borrow_mut().left = Self::delete(left, value);
                    } else if value > node.borrow().value {
                        let right = node.borrow().right.clone();
                        node.borrow_mut().right = Self::delete(right, value);
                    }
                    Self::update_height(Some(node.clone()));
                    node = Self::rotate(Some(node)).unwrap();
                    Some(node)
                }
            }
            None => None,
        }
    }

    pub fn mid_order_traversal(node: OptionTreeNodeRc<T>, result: &mut Vec<ValueRc<T>>) {
        if let Some(node) = node {
            Self::mid_order_traversal(node.borrow().left.clone(), result);
            result.push(node.borrow().value.clone());
            Self::mid_order_traversal(node.borrow().right.clone(), result);
        }
    }

    fn update_height(node: OptionTreeNodeRc<T>) {
        if let Some(node) = node {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().height = std::cmp::max(Self::height(left), Self::height(right)) + 1;
        }
    }

    fn balance_factor(node: OptionTreeNodeRc<T>) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                Self::height(node.borrow().left.clone()) - Self::height(node.borrow().right.clone())
            }
        }
    }

    fn right_rotate(node: OptionTreeNodeRc<T>) -> OptionTreeNodeRc<T> {
        match node {
            Some(node) => {
                let child = node.borrow().left.clone().unwrap();
                let grand_child = child.borrow().right.clone();
                child.borrow_mut().right = Some(node.clone());
                child.borrow_mut().value.borrow_mut().right = Some(node.borrow().value.clone());
                node.borrow_mut().left = grand_child.clone();
                match grand_child {
                    Some(grand_child) => {
                        node.borrow_mut().value.borrow_mut().left =
                            Some(grand_child.borrow().value.clone());
                    }
                    None => {}
                }
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));
                Some(child)
            }
            None => None,
        }
    }

    fn left_rotate(node: OptionTreeNodeRc<T>) -> OptionTreeNodeRc<T> {
        match node {
            Some(node) => {
                let child = node.borrow().right.clone().unwrap();
                let grand_child = child.borrow().left.clone();
                child.borrow_mut().left = Some(node.clone());
                child.borrow_mut().value.borrow_mut().left = Some(node.borrow().value.clone());
                node.borrow_mut().right = grand_child.clone();
                match grand_child {
                    Some(grand_child) => {
                        node.borrow_mut().value.borrow_mut().right =
                            Some(grand_child.borrow().value.clone());
                    }
                    None => {}
                }
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));
                Some(child)
            }
            None => None,
        }
    }

    fn rotate(node: OptionTreeNodeRc<T>) -> OptionTreeNodeRc<T> {
        let balance_factor = Self::balance_factor(node.clone());
        if balance_factor > 1 {
            let node = node.unwrap();
            if Self::balance_factor(node.borrow().left.clone()) >= 0 {
                Self::right_rotate(Some(node))
            } else {
                let old_left = node.borrow().left.clone();
                let new_left = Self::left_rotate(old_left);
                node.borrow_mut().left = new_left.clone();
                match new_left {
                    Some(new_left) => {
                        node.borrow_mut().value.borrow_mut().left =
                            Some(new_left.borrow().value.clone());
                    }
                    None => {}
                }
                Self::right_rotate(Some(node))
            }
        } else if balance_factor < -1 {
            let node = node.unwrap();
            if Self::balance_factor(node.borrow().right.clone()) <= 0 {
                Self::left_rotate(Some(node))
            } else {
                let old_right = node.borrow().right.clone();
                let new_right = Self::right_rotate(old_right);
                node.borrow_mut().right = new_right.clone();
                match new_right {
                    Some(new_right) => {
                        node.borrow_mut().value.borrow_mut().right =
                            Some(new_right.borrow().value.clone());
                    }
                    None => {}
                }
                Self::left_rotate(Some(node))
            }
        } else {
            node
        }
    }
}
