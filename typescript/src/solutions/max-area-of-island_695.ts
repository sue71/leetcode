/*
 * @lc app=leetcode id=695 lang=typescript
 *
 * [695] Max Area of Island
 */

// @lc code=start
function maxAreaOfIsland(grid: number[][]): number {
  let res = 0;
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] !== 0) {
        res = Math.max(_maxAreaOfIsland(grid, i, j, 0), res);
      }
    }
  }

  return res;
}

function _maxAreaOfIsland(
  grid: number[][],
  i: number,
  j: number,
  count: number
): number {
  if (i < 0 || i > grid.length - 1) {
    return 0;
  }
  if (j < 0 || j > grid[0].length - 1) {
    return 0;
  }
  if (grid[i][j] === 0) {
    return 0;
  }
  if (grid[i][j] === 1) {
    count += 1;
    grid[i][j] = 0;
  }

  count += _maxAreaOfIsland(grid, i + 1, j, 0);
  count += _maxAreaOfIsland(grid, i, j + 1, 0);
  count += _maxAreaOfIsland(grid, i - 1, j, 0);
  count += _maxAreaOfIsland(grid, i, j - 1, 0);

  return count;
}
// @lc code=end
