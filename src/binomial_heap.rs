use std::cmp::Ord;
use std::mem;
use heap::Heap;

// A simple singly-linked list type.
// Doesn't need to be too sophisticated.
struct ListNode<T> {
    pub element : T,
    pub next : Option<Box<ListNode<T>>>
}

struct List<T> {
    length : usize,
    head : Option<Box<ListNode<T>>>
}

impl<T> List<T> {
    fn new() -> Self {
        List::<T> {
            length : 0,
            head : None
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn length(&self) -> usize {
        self.length
    }

    fn cons(&mut self, x : T) {
        let mut newHead = Some(Box::new(ListNode::<T>{
            element : x,
            next : mem::replace(&mut self.head, None)
        }));
        self.head = newHead
    }
}

// A struct that represents a binomial tree.
struct BinomialTree<T> {
    root : T,
    children : List<BinomialTree<T>>
}

impl<T : Ord> BinomialTree<T> {
    fn new(x: T) -> Self {
        BinomialTree::<T> {
            root : x,
            children : List::new()
        }
    }

    fn order(&self) -> usize {
        self.children.length()
    }

    /// Performs a binomial link operation.
    /// other is linked as a subtree of self without regard to the
    /// min-heap property.
    fn link(&mut self, other : BinomialTree<T>) {
        if self.order() != other.order() { panic!(); }
        else { self.children.cons(other); }
    }
}

/// An implementation of Heap as a binomial heap.
pub struct BinomialHeap<T : Ord> {
    trees: List<BinomialTree<T>>
}

impl<T : Ord> BinomialHeap<T> {
    pub fn new() -> Self {
        BinomialHeap {
            trees : List::new()
        }
    }
}

impl<T : Ord> Heap<T> for BinomialHeap<T> {
	fn add(&mut self, x:T) {
        BinomialTree::new(x);
    }

	fn peek_min(&self) -> Option<&T> {
        None
    }

	fn remove_min(&mut self) -> Option<T> {
        None
    }

	fn size(&self) -> usize {
        0
    }

	fn is_empty(&self) -> bool {
        self.trees.is_empty()
    }
}
