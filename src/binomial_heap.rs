use std::cmp::Ord;
use heap::Heap;
use sl_list::List;

// A struct that represents a binomial tree.
struct BinomialTree<T> {
    root : T,
    children : List<BinomialTree<T>>
}

impl<T : Ord> BinomialTree<T> {
    fn new(x: T) -> Self {
        BinomialTree {
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
        if self.order() != other.order() { panic!() }
        else { self.children.push(other) }
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

    fn meld_tree_lists(x : List<BinomialTree<T>>, y : List<BinomialTree<T>>) -> List<BinomialTree<T>> {
        // Easy cases.
        if x.is_empty() { y }
        else if y.is_empty() { x }
        else
        {
            let mut ret = List::<BinomialTree<T>>::new();

            // The carry flag.
            let mut carry:Option<BinomialTree<T>>  = None;

            // Create iterators out of the two lists.
            let mut x = x.into_iter();
            let mut y = y.into_iter();

            let mut curx = x.next();
            let mut cury = y.next();

            loop {
                if carry.is_some() {

                } else {
                    if curx.is_some() && cury.is_some() {
                        let orderx = curx.as_ref().unwrap().order();
                        let ordery = cury.as_ref().unwrap().order();

                        if orderx < ordery {
                            ret.push(curx.take().unwrap());
                            curx = x.next();
                        }
                        else if ordery < orderx {
                            ret.push(cury.take().unwrap());
                            cury = y.next();
                        }
                        else {
                            let mut xx:BinomialTree<T> = curx.take().unwrap();
                            let mut yy:BinomialTree<T> = cury.take().unwrap();

                            if xx.root < yy.root {
                                xx.link(yy);
                                carry = Some(xx);
                            }
                            else {
                                yy.link(xx);
                                carry = Some(yy);
                            }
                            curx = x.next();
                            cury = y.next();
                        }
                    } else if curx.is_some() {
                        ret.push(curx.unwrap());
                        curx = x.next();
                    } else if cury.is_some() {
                        ret.push(cury.unwrap());
                        cury = y.next();
                    } else {
                        break; // Both lists and carry are exhausted.  We are done.
                    }
                }
            }

            ret.reverse();
            ret
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
