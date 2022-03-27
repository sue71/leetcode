/*
 * @lc app=leetcode id=153 lang=rust
 *
 * [153] Find Minimum in Rotated Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut lc = 0;
        let mut rc = nums.len() / 2;

        loop {
            if rc - lc == 0 {
                break
            }
        }

        0
    }
}
// @lc code=end

