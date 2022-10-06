/*
 * @lc app=leetcode id=63 lang=rust
 *
 * [63] Unique Paths II
 */

// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid.len();
        let m = obstacle_grid[0].len();
        let mut dp = vec![vec![0; m + 1 as usize]; n + 1 as usize];

        dp[1][0] = 1;
        for i in 1..(n + 1) as usize {
            for j in 1..(m + 1) as usize {
                if obstacle_grid[i - 1][j - 1] == 1 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
                }
            }
        }

        dp[n][m]
    }
}
// @lc code=end
