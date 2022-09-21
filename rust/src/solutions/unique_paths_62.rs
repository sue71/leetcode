/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // let mut c = 0;
        // pub fn dfs(i: i32, j: i32, c: &mut i32, m: i32, n: i32) {
        //     if i > m || j > n {
        //         return;
        //     }
        //     if i == m && j == n {
        //         *c += 1;
        //         return;
        //     }
        //     dfs(i + 1, j, c, m, n);
        //     dfs(i, j + 1, c, m, n);
        // }
        // dfs(0, 0, &mut c, m - 1, n - 1);

        // c

        // dp[i][j] = dp[i-1][j] + dp[i][j-1]
        let mut dp = vec![vec![1; n as usize]; m as usize];

        for i in 1..m as usize {
            for j in 1..n as usize {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                println!("{}", dp[i][j]);
            }
        }

        dp[m as usize - 1][n as usize - 1]
    }
}
// @lc code=end
