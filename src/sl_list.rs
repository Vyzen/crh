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
        List {
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

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node_box| { &node_box.element })
    }

    pub fn reverse(&mut self)  {
        let mut rev = List::<T>::new();
        while !self.is_empty() {
            rev.push(self.pop().unwrap());
        }
        self.head = rev.head;
        self.length = rev.length;
    }

}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> { IntoIter(self) }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { self.0.pop() }
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
    assert_eq!(l.length(), 1);
    l.push(10);
    assert_eq!(l.length(), 2);
}

#[test]
fn sll_pop() {
    let mut l = List::<i32>::new();
    l.push(5);
    assert_eq!(l.length(), 1);
    l.push(10);
    assert_eq!(l.length(), 2);

    let x = l.pop();
    assert!(x.is_some());
    assert_eq!(x.unwrap(), 10);
    assert_eq!(l.length(), 1);

    let y = l.pop();
    assert!(y.is_some());
    assert_eq!(y.unwrap(), 5);
    assert_eq!(l.length(), 0);
    assert!(l.is_empty());

    let z = l.pop();
    assert!(z.is_none());
}

#[test]
fn sll_peek() {
    let mut l = List::<i32>::new();

    {
        let x = l.peek();
        assert!(x.is_none());
    } // This is because x now has a reference to something in l,
      //which will prevent push for mutating l.  The extra {} causes
      // x to fall out of scope.

    l.push(5);
    l.push(10);

    let y = l.peek();
    assert!(y.is_some());
    assert_eq!(*y.unwrap(), 10);
    assert_eq!(l.length(), 2);
}

#[test]
fn sll_into_iter() {
    let mut l = List::<i32>::new();
    l.push(1);
    l.push(2);
    l.push(3);
    assert_eq!(l.length(), 3);
    let mut iter = l.into_iter();

    let mut item = iter.next();
    assert!(item.is_some());
    assert_eq!(item.unwrap(), 3);

    item = iter.next();
    assert!(item.is_some());
    assert_eq!(item.unwrap(), 2);

    item = iter.next();
    assert!(item.is_some());
    assert_eq!(item.unwrap(), 1);

    item = iter.next();
    assert!(item.is_none());
}

#[test]
fn sll_reverse() {
    let mut l = List::<i32>::new();
    l.push(1);
    l.push(2);
    l.push(3);
    assert_eq!(l.length(), 3);

    l.reverse();
    assert_eq!(l.length(), 3);

    let x = l.pop();
    assert!(x.is_some());
    assert_eq!(x.unwrap(), 1);

    let y = l.pop();
    assert!(y.is_some());
    assert_eq!(y.unwrap(), 2);

    let z = l.pop();
    assert!(z.is_some());
    assert_eq!(z.unwrap(), 3);

    assert!(l.is_empty());
}
