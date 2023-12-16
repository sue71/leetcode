/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */
pub struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let x = target - num;
            if (map.contains_key(&x)) {
                if let Some(&i) = map.get(&x) {
                    res.push(i as i32);
                }
                res.push(i as i32);
                break;
            }
            map.insert(num, i);
        }

        res
    }
}
// @lc code=end
