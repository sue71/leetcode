/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize]; m as usize];
        for i in 0..m as usize {
            for j in 0..n as usize {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
                }
            }
        }
        dp[m as usize - 1][n as usize - 1]
    }
}
// @lc code=end
