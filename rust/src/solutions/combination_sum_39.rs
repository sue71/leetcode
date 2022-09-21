/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 */

// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        pub fn dfs(
            selected: Vec<i32>,
            sum: i32,
            min: i32,
            res: &mut Vec<Vec<i32>>,
            target: i32,
            candidates: &Vec<i32>,
        ) {
            for num in candidates {
                if sum + num > target {
                    continue;
                }
                if *num < min {
                    continue;
                }
                let mut c = selected.clone();
                c.push(*num);
                if (sum + num) == target {
                    res.push(c);
                    return;
                }
                dfs(c, sum + num, *num, res, target, candidates);
            }
        }

        dfs(vec![], 0, 0, &mut res, target, &candidates);

        res
    }
}
// @lc code=end
