/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut i = 0;
        let mut j = 0;

        while i < len {
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
            i += 1;
        }
    }
}
// @lc code=end

