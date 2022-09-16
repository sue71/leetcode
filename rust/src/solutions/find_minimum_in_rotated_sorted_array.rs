/*
 * @lc app=leetcode id=153 lang=rust
 *
 * [153] Find Minimum in Rotated Sorted Array
 */

pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut lc = 0;
        let mut rc = nums.len() - 1;
        let mut mid = 0;

        loop {
            if lc >= rc {
                break;
            }
            mid = (rc + lc) / 2;
            if nums[mid] > nums[rc] {
                lc = mid + 1;
            } else {
                rc = mid;
            }
        }

        nums[lc]
    }
}
// @lc code=end

