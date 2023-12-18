/*
 * @lc app=leetcode id=200 lang=rust
 *
 * [200] Number of Islands
 */

// @lc code=start
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        let mut grid_mut = grid.clone();

        for i in 0..grid_mut.len() {
            for j in 0..grid_mut[0].len() {
                if (grid_mut[i][j] == '1') {
                    res += 1;
                    dfs(i, j, &mut grid_mut);
                }
            }
        }

        fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
            if grid[i][j] == '0' {
                return
            }
            if grid[i][j] == '1' {
                grid[i][j] = '0';
            }
            if i > 0 {
                dfs(i - 1, j, grid);
            }
            if j > 0 {
                dfs(i, j - 1, grid);
            }
            if i < grid.len() - 1 {
                dfs(i + 1, j, grid);
            }
            if j < grid[0].len() - 1 {
                dfs(i, j + 1, grid);
            }
        }

        res
    }

}
// @lc code=end

