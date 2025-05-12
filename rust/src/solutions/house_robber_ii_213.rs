/*
 * @lc app=leetcode id=213 lang=rust
 *
 * [213] House Robber II
 */

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp_1 = vec![0; nums.len()];
        let mut dp_2 = vec![0; nums.len()];

        if nums.len() == 1 {
            return nums[0];
        }

        for i in 0..nums.len() {
            if i == 0 {
                dp_1[i] = nums[i];
                dp_2[i] = 0;
            } else if i == 1 {
                dp_1[i] = std::cmp::max(nums[i - 1], nums[i]);
                dp_2[i] = nums[i];
            } else {
                dp_1[i] = std::cmp::max(dp_1[i - 1], dp_1[i - 2] + nums[i]);
                dp_2[i] = std::cmp::max(dp_2[i - 1], dp_2[i - 2] + nums[i]);
            }
        }

        std::cmp::max(dp_1[nums.len() - 2], dp_2[nums.len() - 1])
    }
}
// @lc code=end

