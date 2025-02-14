/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 */

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        for i in 0..nums.len() {
            if i == 0 {
                dp[i] = nums[i];
            } else if i == 1 {
                dp[i] = std::cmp::max(nums[i - 1], nums[i]);
            } else {
                dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
            }
        }

        dp[nums.len() - 1]
    }

    // TLE
    // pub fn dfs(nums: &Vec<i32>, res: i32, i: usize) -> i32 {
    //     if i >= nums.len() {
    //         return res;
    //     }
    //     std::cmp::max(Self::dfs(nums, res, i + 1), Self::dfs(nums, res + nums[i], i + 2))
    // }
}
// @lc code=end

