use std::mem;

// A simple singly-linked list type.
// Doesn't need to be too sophisticated.
struct ListNode<T> {
    element : T,
    next : Option<Box<ListNode<T>>>
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
        let new_head = Some(Box::new(ListNode::<T>{
            element : x,
            next : self.head.take() //mem::replace(&mut self.head, None)
        }));
        self.head = new_head;
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node_box| {
            let node = *node_box;
            self.head = node.next;
            self.length -= 1;
            node.element})
    }

}

#[test]
fn sll_create() {
    let l = List::<i32>::new();
    assert!(l.is_empty())
}

#[test]
fn sll_push() {
    let mut l = List::<i32>::new();
    l.push(5);
    assert!(l.length() == 1);
    l.push(10);
    assert!(l.length() == 2);
}

#[test]
fn sll_pop() {
    let mut l = List::<i32>::new();
    l.push(5);
    assert!(l.length() == 1);
    l.push(10);
    assert!(l.length() == 2);

    let x = l.pop();
    assert!(x.is_some());
    assert!(x.unwrap() == 10);
    assert!(l.length() == 1);

    let y = l.pop();
    assert!(y.is_some());
    assert!(y.unwrap() == 5);
    assert!(l.length() == 0);
    assert!(l.is_empty());

    let z = l.pop();
    assert!(z.is_none());
}
