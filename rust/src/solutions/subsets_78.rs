/*
 * @lc app=leetcode id=78 lang=rust
 *
 * [78] Subsets
 */

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();

        Self::dfs(&nums, 0, &mut Vec::new(), &mut res);

        res
    }

    pub fn dfs(nums: &Vec<i32>, d: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if nums.len() == d {
            result.push(current.clone());
            return;
        }
        current.push(nums[d]);
        Self::dfs(&nums, d + 1, current, result);
        current.pop();
        Self::dfs(&nums, d + 1, current, result);
    }
}
// @lc code=end
