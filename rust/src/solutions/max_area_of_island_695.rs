/*
 * @lc app=leetcode id=695 lang=rust
 *
 * [695] Max Area of Island
 */

// @lc code=start
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    count = std::cmp::max(dfs(i as i32, j as i32, &mut grid), count);
                }
            }
        }
        count
    }
}

    fn dfs(i: i32, j: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
        if j < 0 || i < 0 || j + 1 > grid[0].len() as i32 || i + 1 > grid.len() as i32 {
            return 0;
        }

        if grid[i as usize][j as usize] == 0 {
            return 0;
        }

        // check seen
        grid[i as usize][j as usize] = 0;

        1 +  
        dfs(i - 1, j, grid) + 
        dfs(i + 1, j, grid) + 
        dfs(i, j - 1, grid) +
        dfs(i, j + 1, grid)
    }
// @lc code=end

