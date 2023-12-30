/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */
struct Solution {}

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: i32 = 0;
        let mut n: i32 = nums.len() as i32;
        let mut r: i32 = n - 1;

        while l <= r {
            let m = (l + r) / 2;
            let mi = nums.get(m as usize).unwrap();
            let ri = nums.get(n as usize - 1).unwrap();

            if mi > ri {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        let mut res = Self::binary_search(&nums, 0, l - 1, target);
        if res != -1 {
            return res;
        }

        Self::binary_search(&nums, l, n - 1, target)
    }

    pub fn binary_search(nums: &Vec<i32>, l: i32, r: i32, target: i32) -> i32 {
        let mut l = l;
        let mut r = r;

        while l <= r {
            let m = (l + r) / 2;
            let mi = nums.get(m as usize).unwrap();

            if mi == &target {
                return m;
            } else if mi < &target {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        -1
    }
}
// @lc code=end
