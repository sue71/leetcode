/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */


// dp[i] = 任意の要素からiまでの総和の最大値
// dp[i + 1] = dp[i] + nums[i + 1] or nums[i + 1]
// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        let mut res = nums[0];

        for i in 0..nums.len() {
            dp[i + 1] = std::cmp::max(dp[i] + nums[i], nums[i]);
            res = std::cmp::max(res, dp[i + 1]);
        }
        res
    }
}
// @lc code=end

