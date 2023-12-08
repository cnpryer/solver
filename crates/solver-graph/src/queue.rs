use std::{
    cmp::Ord,
    ops::{Deref, DerefMut},
};

use crate::small_array::SmallArray;

#[derive(Debug)]
pub(crate) struct PriorityQueue<V: Ord> {
    heap: SmallArray<V>,
}

impl<V: Ord> PriorityQueue<V> {
    pub(crate) fn new() -> Self {
        PriorityQueue {
            heap: SmallArray::Dynamic(Vec::new()),
        }
    }

    pub(crate) fn with_capacity(size: usize) -> Self {
        PriorityQueue {
            heap: SmallArray::Dynamic(Vec::with_capacity(size)),
        }
    }

    pub(crate) fn push(&mut self, value: V) {
        self.heap.push(value);
        self.heapify_up(self.len() - 1);
    }

    pub(crate) fn pop(&mut self) -> Option<V> {
        if self.is_empty() {
            return None;
        }

        let last_index = self.len() - 1;
        self.heap.swap(0, last_index);
        let popped = self.heap.pop();
        self.heapify_down(0);
        popped
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;
            if self.heap[index] >= self.heap[parent_index] {
                break;
            }
            self.heap.swap(index, parent_index);
            index = parent_index;
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.heap.len();
        loop {
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;
            let mut smallest = index;

            if left_child < len && self.heap[left_child] < self.heap[smallest] {
                smallest = left_child;
            }
            if right_child < len && self.heap[right_child] < self.heap[smallest] {
                smallest = right_child;
            }

            if smallest != index {
                self.heap.swap(index, smallest);
                index = smallest;
            } else {
                break;
            }
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.heap.len()
    }
}

impl<V: Ord> Deref for PriorityQueue<V> {
    type Target = SmallArray<V>;

    fn deref(&self) -> &Self::Target {
        &self.heap
    }
}

impl<V: Ord> DerefMut for PriorityQueue<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.heap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue() {
        let mut q = PriorityQueue::new();

        q.push(0);
        q.push(2);
        q.push(3);
        q.push(15);

        while !q.is_empty() {
            _ = q.pop();
        }

        assert!(q.is_empty())
    }
}
