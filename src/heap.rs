use std::cmp::Ord;

/// Specification for a data type that implements a
/// min-heap.
pub trait Heap<T : Ord> {

	/// Add an element to this heap.
	fn add(&mut self, x:T);

	/// Get the minimum element of this heap.
	fn peek_min(&self) -> Option<&T>;

	/// Removes the minimum element of this heap.
	fn remove_min(&mut self) -> Option<T>;

	/// Gets the current size of this heap.
	fn size(&self) -> usize;

	/// Tells whether the heap is empty.
	fn is_empty(&self) -> bool;
}


/// An "into" iterator type that works generically on all heaps.
pub struct HeapIntoIter<T : Ord>(Heap<T>);

impl<T : Ord> Iterator for HeapIntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> { self.0.remove_min() }
}
