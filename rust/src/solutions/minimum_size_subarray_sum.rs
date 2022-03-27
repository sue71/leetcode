/*
 * @lc app=leetcode id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 */

// @lc code=start
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut lc: i32 = 0; 
        let mut rc: i32 = -1; 
        let mut res: i32 = f32::INFINITY as i32;
        let mut sum: i32 = 0;

        loop {
            if lc as usize >= nums.len() {
                break;
            }

            if sum < target && (rc as usize + 1) < nums.len() {
                rc = rc + 1;
                sum = sum + nums[rc as usize];
            } else {
                sum = sum - nums[lc as usize];
                lc = lc + 1;
            }

            if sum >= target {
                res = std::cmp::min(res, rc - lc + 1);
            }
        }

        if res == f32::INFINITY as i32 {
            0
        } else {
            res
        }
    }
    // pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    //     let mut sum_list: Vec<i32> = vec![];
    //     let mut sum: i32 = 0;
    //     let mut res: i32 = f32::INFINITY as i32;

    //     for (i, num) in nums.iter().enumerate() {
    //         sum = 0;
    //         sum_list = vec![];
    //         for k in i..nums.len() {
    //             sum_list.push(nums[k]);
    //             sum = sum + nums[k];
    //             if sum >= target {
    //                 res = std::cmp::min(sum_list.len() as i32, res);
    //                 break;
    //             }
    //         }
    //     }

    //     if res == f32::INFINITY as i32 {
    //         0
    //     } else {
    //         res
    //     }
    // }
}
// @lc code=end

