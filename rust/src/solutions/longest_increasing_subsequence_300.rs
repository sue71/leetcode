/*
 * @lc app=leetcode id=300 lang=rust
 *
 * [300] Longest Increasing Subsequence
 */

// dp[i][j] = 最初のi個の要素について、最後の要素がa_jとなるように選んだときの最長の長さ
pub struct Solution {}

// @lc code=start

use std::cmp::*;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut res = 0;

        for i in 0..nums.len() {
            dp[i] = 1;
            for j in 0..i+1  {
                if nums[j] < nums[i] {
                    dp[i] = max(dp[i], dp[j] + 1);
                }
                res = max(dp[i], res);
            }
        }

        println!("{:?}", dp);

        res
    }
}

// @lc code=end

