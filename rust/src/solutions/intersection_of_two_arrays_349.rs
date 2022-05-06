/*
 * @lc app=leetcode id=349 lang=rust
 *
 * [349] Intersection of Two Arrays
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut set: HashSet<i32> = HashSet::new();

        for i in &nums1 {
            for  j in &nums2 {
                if i == j {
                    set.insert(*i);
                }
            }
        }

        set.into_iter().collect()
    }
}
// @lc code=end

