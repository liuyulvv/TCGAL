use std::{cell::RefCell, rc::Rc};

type OptionNodeRc<T> = Option<Rc<RefCell<ListNode<T>>>>;

#[derive(Debug, Clone)]
pub struct ListNode<T: Clone + Copy> {
    pub data: T,
    pub next: OptionNodeRc<T>,
    pub prev: OptionNodeRc<T>,
}

pub struct CircularDoubleLinkedList<T: Clone + Copy> {
    head: OptionNodeRc<T>,
    tail: OptionNodeRc<T>,
    length: usize,
}

impl<T: Clone + Copy> CircularDoubleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn head(&self) -> OptionNodeRc<T> {
        self.head.clone()
    }

    pub fn tail(&self) -> OptionNodeRc<T> {
        self.tail.clone()
    }

    pub fn insert(&mut self, node: OptionNodeRc<T>, data: T) -> Rc<RefCell<ListNode<T>>> {
        match node {
            Some(node) => {
                let new_node = Rc::new(RefCell::new(ListNode {
                    data,
                    next: None,
                    prev: None,
                }));
                let next = node.borrow().next.clone();
                node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(node.clone());
                new_node.borrow_mut().next = next.clone();
                next.unwrap().borrow_mut().prev = Some(new_node.clone());
                self.length += 1;
                new_node
            }
            None => {
                let new_node = Rc::new(RefCell::new(ListNode {
                    data,
                    next: None,
                    prev: None,
                }));
                new_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(new_node.clone());
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
                self.length += 1;
                new_node
            }
        }
    }

    pub fn delete(&mut self, node: OptionNodeRc<T>) {
        match node {
            Some(node) => {
                let prev = node.borrow().prev.clone();
                let next = node.borrow().next.clone();
                match (prev, next) {
                    (Some(prev), Some(next)) => {
                        prev.borrow_mut().next = Some(next.clone());
                        next.borrow_mut().prev = Some(prev.clone());
                    }
                    (Some(prev), None) => {
                        prev.borrow_mut().next = None;
                        self.tail = Some(prev.clone());
                    }
                    (None, Some(next)) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next.clone());
                    }
                    (None, None) => {
                        self.head = None;
                        self.tail = None;
                    }
                }
                self.length -= 1;
            }
            None => {}
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}
