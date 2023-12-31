/*
 * @lc app=leetcode id=31 lang=rust
 *
 * [31] Next Permutation
 */

struct Solution {}
// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut pivot = nums.len() - 1;

        while pivot > 0 {
            if nums[pivot - 1] < nums[pivot] {
                break;
            }
            pivot -= 1;
        }

        if pivot == 0 {
            nums.reverse();
            return;
        }

        for i in (pivot..nums.len()).rev() {
            if nums[i] > nums[pivot - 1] {
                nums.swap(i, pivot - 1);
                break;
            }
        }
        nums[pivot..].sort_by(|a, b| a.cmp(b));
    }
}
// @lc code=end
