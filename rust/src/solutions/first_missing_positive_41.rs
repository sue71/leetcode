/*
 * @lc app=leetcode id=41 lang=rust
 *
 * [41] First Missing Positive
 */


// @lc code=start
use std::{cmp, collections::HashMap};
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;
        let mut max = -1;
        let mut map = HashMap::<usize, bool>::new();
        for (i, n) in nums.iter().enumerate() {
            if *n > 0 {
                map.insert(*n as usize, true);
            }
            max = cmp::max(max, *n)
        }

        for n in 1..nums.len()+1 {
            println!("{:?}", map.contains_key(&n));
            if !map.contains_key(&n) {
                return n as i32
            }
        }

        return max + 1;
    }
}
// @lc code=end

