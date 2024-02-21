/*
 * @lc app=leetcode id=63 lang=rust
 *
 * [63] Unique Paths II
 */

// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; obstacle_grid[0].len()]; obstacle_grid.len()];
        dp[0][0] = 1;

        for i in 0..obstacle_grid.len() {
            for j in 0..obstacle_grid[0].len() {
                if obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                } else {
                    if i > 0 {
                        dp[i][j] += dp[i - 1][j];
                    }
                    if j > 0 {
                        dp[i][j] += dp[i][j - 1];
                    }
                }
            }
        }

        dp[obstacle_grid.len() - 1][obstacle_grid[0].len() - 1]
    }
}
// @lc code=end
