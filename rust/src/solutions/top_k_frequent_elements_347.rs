/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */


// @lc code=start
use std::collections::{BTreeMap, BinaryHeap, HashMap};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();
        for (key, value) in map {
            heap.push((value, key));
        }

        let mut result = Vec::new();
        for _ in 0..k {
            if let Some((_, key)) = heap.pop() {
                result.push(key);
            }
        }

        result
    }
}
// @lc code=end

