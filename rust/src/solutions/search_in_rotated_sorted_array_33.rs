/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // for (i, v) in nums.iter().enumerate() {
        //     if *v == target {
        //         return i as i32;
        //     }
        // }
        // return -1;

        let mut lc: i32 = 0;
        let mut rc: i32 = nums.len() as i32 - 1;
        let mut res = 0;

        loop {
            if rc < lc {
                break;
            }

            let mut mid = (rc + lc) / 2;

            if target == nums[mid as usize] {
                return mid;
            }

            println!("{} {} {}", lc, rc, mid);

            if nums[lc as usize] <= nums[mid as usize] {
                // left sorted
                if target > nums[mid as usize] || target < nums[lc as usize] {
                    lc = mid + 1;
                } else {
                    rc = mid - 1;
                }
            } else {
                // right sorted
                if target < nums[mid as usize] || target > nums[rc as usize] {
                    rc = mid - 1;
                } else {
                    lc = mid + 1;
                }
            }
        }

        -1
    }
}
// @lc code=end
