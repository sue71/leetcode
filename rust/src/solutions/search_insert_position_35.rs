/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */
pub struct Solution {
}

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut index: i32 = 0;
        let mut lc: i32 = 0;
        let mut rc: i32 = nums.len() as i32 - 1;
        while rc >= lc {
            let mid = lc + (rc - lc) / 2;

            println!("mid: {}, val: {} lc: {} rc: {}", mid, nums[mid as usize], lc, rc);

            let v = nums[mid as usize]; 

            if target == v {
                lc = mid as i32;
                break;
            } else if target > v {
                lc = mid + 1;
                continue;
            } else {
                rc = mid - 1;
                continue;
            }
        }
        lc
    }
}
// @lc code=end

