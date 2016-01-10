use std::mem;

// A simple singly-linked list type.
// Doesn't need to be too sophisticated.
struct ListNode<T> {
    pub element : T,
    pub next : Option<Box<ListNode<T>>>
}

pub struct List<T> {
    length : usize,
    head : Option<Box<ListNode<T>>>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::<T> {
            length : 0,
            head : None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, x : T) {
        let mut newHead = Some(Box::new(ListNode::<T>{
            element : x,
            next : mem::replace(&mut self.head, None)
        }));
        self.head = newHead
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head {
            None => { None }
            Some(nodeBox) => {
                let ret = Some(nodeBox.element);
                self.head = nodeBox.next;
                ret
            }
        }
    }

    pub fn peek(&self) -> Option<T> {
        match self.head {
            None => { None }
            Some(nodeBox) => {
                Some(nodeBox.element)
            }
        }
    }
}
