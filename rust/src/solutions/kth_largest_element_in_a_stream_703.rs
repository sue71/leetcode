/*
 * @lc app=leetcode id=703 lang=rust
 *
 * [703] Kth Largest Element in a Stream
 */

// @lc code=start
use std::{cmp::Reverse, collections::BinaryHeap};
struct KthLargest {
    size: usize,
    items: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let size = k as usize;
        let mut heap = BinaryHeap::with_capacity(size + 1);
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > size {
                heap.pop();
            }
        }
        return KthLargest { size, items: heap };
    }

    fn add(&mut self, val: i32) -> i32 {
        self.items.push(Reverse(val));
        if self.items.len() > self.size {
            self.items.pop();
        }

        self.items.peek().unwrap().0
    }
}

// @lc code=end
