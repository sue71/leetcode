/*
 * @lc app=leetcode id=276 lang=rust
 *
 * [276] Paint Fence
 */

// @lc code=start
impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];

        dp[0] = 0;

        for i in 1..n+1 {
            if i == 1 {
                dp[i as usize] = k;
            } else if i == 2 {
                dp[i as usize] = k * k;
            } else {
                dp[i as usize] = dp[i as usize - 1] * (k - 1) + dp[i as usize - 2] * (k - 1);
            }
        }

        dp[n as usize]
    }
}
// @lc code=end

