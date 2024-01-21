/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

// @lc code=start
use std::{cmp, vec};
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut max = i32::MIN;

        dp[0] = nums.first().unwrap().clone();

        for i in 0..nums.len() - 1 {
            let n = nums.get(i + 1).unwrap();
            dp[i + 1] = cmp::max(*n, dp[i] + *n)
        }

        for i in 0..nums.len() {
            max = cmp::max(max, dp[i]);
        }

        max
    }
}
// @lc code=end
