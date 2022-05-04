/*
 * @lc app=leetcode id=695 lang=typescript
 *
 * [695] Max Area of Island
 */

// @lc code=start
function maxAreaOfIsland(grid: number[][]): number {
    let res = 0;

    const _dfs = (i: number, j: number, count: number) => {
        if (i < 0) {
            return count;
        }
        if (j < 0) {
            return count;
        }
        if (i > grid.length - 1) {
            return count;
        }
        if (j > grid[0].length - 1) {
            return count;
        }

        if (grid[i][j] === 0)  {
            return count;
        } else {
            grid[i][j] = 0;

            count ++;

            count += _dfs(i - 1, j, 0);
            count += _dfs(i + 1, j, 0);
            count += _dfs(i, j - 1, 0);
            count += _dfs(i, j + 1, 0);

            return count;
        }
    }

    grid.forEach((v, i) => {
        v.forEach((_, j) => {
            if (grid[i][j] === 1) {
                res = Math.max(_dfs(i, j, 0), res);
            }
        });
    });

    return res;

}
// @lc code=end
