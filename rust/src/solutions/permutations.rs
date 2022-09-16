/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 */

pub struct Solution {
}

// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();

        fn _permute(selected: Vec<i32>, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
            for v in nums {
                if selected.contains(&v) {
                    continue;
                }
                let mut c = selected.clone();
                c.push(*v);

                if c.len() == nums.len() {
                    res.push(c);
                    return;
                }
                _permute(c, nums, res);
            }
        }

        _permute(vec![], &nums, &mut res);

        res
    }
}
// @lc code=end

