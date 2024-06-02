/*
 * @lc app=leetcode id=387 lang=rust
 *
 * [387] First Unique Character in a String
 */


// @lc code=start
use std::{cmp::max, collections::HashMap};
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::<String, usize>::new();
        for (i, c) in s.chars().into_iter().enumerate() {
            let key = c.to_string();
            let count = map.get(&key).unwrap_or(&0) + 1;
            map.insert(key, count);
        }

        for (i, c) in s.chars().into_iter().enumerate() {
            let key = c.to_string();
            let count = map.get(&key).unwrap();
            if *count == 1 {
                return i as i32
            }
        }

        -1

    }
}
// @lc code=end

