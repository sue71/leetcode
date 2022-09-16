/*
 * @lc app=leetcode id=200 lang=rust
 *
 * [200] Number of Islands
 */
pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1'  {
                    count += 1;
                    dfs(i as i32, j as i32, &mut grid);
                }
            }
        }
        count
    }
}

fn dfs(i: i32, j: i32, c: &mut Vec<Vec<char>>) {
    if i < 0 || j < 0 || i + 1 > c.len() as i32 || j + 1 > c[0].len() as i32 {
        return;
    }

    if c[i as usize][j as usize] == '0' {
        return;
    }

    // check seen
    c[i as usize][j as usize] = '0';

    dfs(i - 1, j, c);
    dfs(i + 1, j, c);
    dfs(i, j - 1, c);
    dfs(i, j + 1, c);
}
// @lc code=end

