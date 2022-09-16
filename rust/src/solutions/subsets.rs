/*
 * @lc app=leetcode id=78 lang=rust
 *
 * [78] Subsets
 */

pub struct Solution {
}
// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]];

        fn dfs(nums: &Vec<i32>, selected: Vec<i32>, d: usize, res: &mut Vec<Vec<i32>>) {
            if (d >= nums.len()) {
                return;
            }

            let mut n = selected.clone();
            n.push(nums[d]);

            res.push(n.clone());
            dfs(nums, n, d + 1, res);
            dfs(nums, selected, d + 1, res);
        }

        dfs(&nums, vec![], 0, &mut res);

        res
    }
}
// @lc code=end

