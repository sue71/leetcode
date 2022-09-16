/*
 * @lc app=leetcode id=560 lang=rust
 *
 * [560] Subarray Sum Equals K
 */

pub struct Solution {
}

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut res = 0;
        let mut sum = 0;

        map.insert(0, 1);
        for n in nums {
            sum = sum + n;

            let key = sum - k;

            if map.contains_key(&key) {
                res = res + map.get(&key).unwrap();
            }

            if map.contains_key(&sum) {
                let count = map.get(&sum).unwrap() + 1;
                map.insert(sum, count);
            } else {
                map.insert(sum, 1);
            }
        }

        res
    }
}
// @lc code=end

