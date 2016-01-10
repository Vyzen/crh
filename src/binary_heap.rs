use std::cmp::Eq;
use std::cmp::Ord;
use heap::Heap;

/// A basic Binary heap implementation based on a vector.
pub struct BinaryHeap<T : Eq + Ord> {
	data: Vec<T>
}

impl<T : Eq + Ord> BinaryHeap<T> {
	pub fn new() -> BinaryHeap<T> {
		BinaryHeap::<T> {
			data : Vec::new()
		}
	}

	fn left(&self, idx: usize) -> Option<usize> {
		let proposed = 2*idx + 1;
		if proposed < self.data.len() { Some(proposed) }
		else { None }
	}

	fn right(&self, idx: usize) -> Option<usize> {
		let proposed = 2*idx + 2;
		if proposed < self.data.len() { Some(proposed) }
		else { None }
	}

	fn parent(&self, idx: usize) -> Option<usize> {
		if idx == 0 { None }
		else { Some((idx - 1) / 2) }
	}

	fn trickle_up(&mut self, idx: usize) {
		let parent = self.parent(idx);
		match parent {
			Some(parent_idx) => {
				if self.data[parent_idx] > self.data[idx] {
					self.data.swap(idx, parent_idx);
					self.trickle_up(parent_idx);
				}
			}
			None => {}
		}
	}

	fn heapify(&mut self, idx: usize) {
		let left = self.left(idx);
		let right = self.right(idx);
		match left {
			Some(left_idx) => {
				match right {
					Some(right_idx) => {
						// Both children exist.
						if self.data[idx] > self.data[left_idx] || self.data[idx] > self.data[right_idx] {
							// promote the smaller one.
							let smaller_idx = if self.data[left_idx] < self.data[right_idx] { left_idx } else { right_idx };
							self.data.swap(idx, smaller_idx);
							self.heapify(smaller_idx);
						}
					}
					None => {
						// Only left exists.
						if self.data[idx] > self.data[left_idx] {
							self.data.swap(idx, left_idx);
							self.heapify(left_idx);
						}
					}
				}
			}

			None => {
				// If left does not exist, right can't exist either.
			}
		}
	}
}

impl<T : Eq + Ord> Heap<T> for BinaryHeap<T> {
	fn add(&mut self, x:T) {
		let idx = self.data.len();
		self.data.push(x);
		self.trickle_up(idx);
	}

	fn size(&self) -> usize { self.data.len() }

	fn peek_min(&self) -> Option<&T> {
		self.data.first()
	}

	fn remove_min(&mut self) -> Option<T> {
		if self.data.is_empty() { None }
		else {
			let ret = Some(self.data.swap_remove(0));
			self.heapify(0);
			ret
		}
	}

}

#[test]
fn binary_heap_creation() {
	let mut h = BinaryHeap::<i32>::new();
	h.add(3);
	h.add(4);
	h.add(2);
	assert!(h.size() == 3);
}

#[test]
fn binary_heap_remove_min() {
	let mut h = BinaryHeap::<i32>::new();
	h.add(3);
	h.add(4);
	h.add(2);
	assert!(h.size() == 3);

	let x = h.remove_min().unwrap();
	assert!(x == 2);
	assert!(h.size() == 2);

	let y = h.remove_min().unwrap();
	assert!(y == 3);
	assert!(h.size() == 1);

	let z = h.remove_min().unwrap();
	assert!(z == 4);
	assert!(h.size() == 0);

	assert!(h.remove_min().is_none());
}

#[test]
fn binary_heap_peek_min() {
	let mut h = BinaryHeap::<i32>::new();
	h.add(3);
	h.add(4);
	h.add(2);
	assert!(h.size() == 3);

	let x = h.peek_min().unwrap();
	assert!(*x == 2);
	assert!(h.size() == 3);

}
