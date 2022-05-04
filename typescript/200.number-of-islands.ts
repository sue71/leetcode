/*
 * @lc app=leetcode id=200 lang=typescript
 *
 * [200] Number of Islands
 */

// @lc code=start
function numIslands(grid: string[][]): number {
    let res = 0;

    const sweep = (i: number, j: number) => {
        if (i < 0) {
            return;
        }
        if (j < 0) {
            return;
        }
        if (i > grid.length - 1) {
            return;
        }
        if (j > grid[0].length - 1) {
            return;
        }

        if (grid[i][j] === "1")  {
            grid[i][j] = "0";
            sweep(i - 1, j);
            sweep(i + 1, j);
            sweep(i, j - 1);
            sweep(i, j + 1);
        }
    }

    grid.forEach((v, i) => {
        v.forEach((_, j) => {
            if (grid[i][j] === "1") {
                sweep(i, j);
                res++;
            }
        });
    });

    return res;


};
// @lc code=end

