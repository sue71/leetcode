/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r: i32 = nums.len() as i32 - 1;

        while l <= r {
            let mid = (l + r) / 2;
            if nums.get(mid as usize).unwrap() == &target {
                return mid as i32;
            }
            if nums.get(mid as usize).unwrap() > &target {
                r = mid - 1;
            } else {
                l = mid + 1;
            }

            println!("l: {}, r: {}, mid: {}", l, r, mid)
        }
        r + 1
    }
}
// @lc code=end
