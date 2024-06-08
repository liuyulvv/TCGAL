use std::{cell::RefCell, rc::Rc};

type OptionNodeRc<T> = Option<Rc<RefCell<AVLTreeNode<T>>>>;

#[derive(Debug, Clone)]
pub struct AVLTreeNode<T: Ord + Clone + Copy> {
    value: T,
    height: i32,
    left: OptionNodeRc<T>,
    right: OptionNodeRc<T>,
}

impl<T> AVLTreeNode<T>
where
    T: Ord + Clone + Copy,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            height: 0,
            left: None,
            right: None,
        }
    }

    pub fn height(node: OptionNodeRc<T>) -> i32 {
        match node {
            Some(node) => node.borrow().height,
            None => -1,
        }
    }

    pub fn insert(node: OptionNodeRc<T>, value: T, option: &AVLTreeOption) -> OptionNodeRc<T> {
        match node {
            Some(mut node) => {
                if value < node.borrow().value {
                    let left = node.borrow().left.clone();
                    node.borrow_mut().left = Self::insert(left, value, option);
                } else if value > node.borrow().value {
                    let right = node.borrow().right.clone();
                    node.borrow_mut().right = Self::insert(right, value, option);
                } else {
                    match option {
                        AVLTreeOption::DisableSameNode => {}
                        AVLTreeOption::SameNodeInsertRight => {
                            let right = node.borrow().right.clone();
                            node.borrow_mut().right = Self::insert(right, value, option);
                        }
                        AVLTreeOption::SameNodeInsertLeft => {
                            let left = node.borrow().left.clone();
                            node.borrow_mut().left = Self::insert(left, value, option);
                        }
                    }
                }
                Self::update_height(Some(node.clone()));
                node = Self::rotate(Some(node)).unwrap();
                Some(node)
            }
            None => Some(Rc::new(RefCell::new(AVLTreeNode::new(value)))),
        }
    }

    pub fn delete(node: OptionNodeRc<T>, value: T) -> OptionNodeRc<T> {
        match node {
            Some(mut node) => {
                if value < node.borrow().value {
                    let left = node.borrow().left.clone();
                    node.borrow_mut().left = Self::delete(left, value);
                } else if value > node.borrow().value {
                    let right = node.borrow().right.clone();
                    node.borrow_mut().right = Self::delete(right, value);
                } else if node.borrow().left.is_none() || node.borrow().right.is_none() {
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
                    node.borrow_mut().right = Self::delete(right, temp.borrow().value);
                    node.borrow_mut().value = temp.borrow().value;
                }
                Self::update_height(Some(node.clone()));
                node = Self::rotate(Some(node)).unwrap();
                Some(node)
            }
            None => None,
        }
    }

    fn update_height(node: OptionNodeRc<T>) {
        if let Some(node) = node {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().height = std::cmp::max(Self::height(left), Self::height(right)) + 1;
        }
    }

    fn balance_factor(node: OptionNodeRc<T>) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                Self::height(node.borrow().left.clone()) - Self::height(node.borrow().right.clone())
            }
        }
    }

    fn right_rotate(node: OptionNodeRc<T>) -> OptionNodeRc<T> {
        match node {
            Some(node) => {
                let child = node.borrow().left.clone().unwrap();
                let grand_child = child.borrow().right.clone();
                child.borrow_mut().right = Some(node.clone());
                node.borrow_mut().left = grand_child;
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));
                Some(child)
            }
            None => None,
        }
    }

    fn left_rotate(node: OptionNodeRc<T>) -> OptionNodeRc<T> {
        match node {
            Some(node) => {
                let child = node.borrow().right.clone().unwrap();
                let grand_child = child.borrow().left.clone();
                child.borrow_mut().left = Some(node.clone());
                node.borrow_mut().right = grand_child;
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));
                Some(child)
            }
            None => None,
        }
    }

    fn rotate(node: OptionNodeRc<T>) -> OptionNodeRc<T> {
        let balance_factor = Self::balance_factor(node.clone());
        if balance_factor > 1 {
            let node = node.unwrap();
            if Self::balance_factor(node.borrow().left.clone()) >= 0 {
                Self::right_rotate(Some(node))
            } else {
                let left = node.borrow().left.clone();
                node.borrow_mut().left = Self::left_rotate(left);
                Self::right_rotate(Some(node))
            }
        } else if balance_factor < -1 {
            let node = node.unwrap();
            if Self::balance_factor(node.borrow().right.clone()) <= 0 {
                Self::left_rotate(Some(node))
            } else {
                let right = node.borrow().right.clone();
                node.borrow_mut().right = Self::right_rotate(right);
                Self::left_rotate(Some(node))
            }
        } else {
            node
        }
    }

    fn mid_order_traversal(node: OptionNodeRc<T>, result: &mut Vec<T>) {
        if let Some(node) = node {
            Self::mid_order_traversal(node.borrow().left.clone(), result);
            result.push(node.borrow().value);
            Self::mid_order_traversal(node.borrow().right.clone(), result);
        }
    }
}

#[derive(Debug, Clone)]
pub enum AVLTreeOption {
    DisableSameNode,
    SameNodeInsertRight,
    SameNodeInsertLeft,
}

#[derive(Debug, Clone)]
pub struct AVLTree<T: Ord + Clone + Copy> {
    root: OptionNodeRc<T>,
    option: AVLTreeOption,
}

impl<T> AVLTree<T>
where
    T: Ord + Clone + Copy,
{
    pub fn new(option: AVLTreeOption) -> Self {
        Self { root: None, option }
    }

    pub fn insert(&mut self, value: T) {
        self.root = AVLTreeNode::insert(self.root.clone(), value, &self.option);
    }

    pub fn delete(&mut self, value: T) {
        self.root = AVLTreeNode::delete(self.root.clone(), value);
    }

    pub fn clear(&mut self) {
        self.root = None;
    }

    pub fn mid_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        AVLTreeNode::mid_order_traversal(self.root.clone(), &mut result);
        result
    }
}
