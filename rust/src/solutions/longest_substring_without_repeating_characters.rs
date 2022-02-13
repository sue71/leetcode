/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let cs: Vec<char> = s.chars().collect();
        let mut cursor = 0;
        let mut len = 0;
        let mut res = 0;
        let mut map = HashMap::new();

        for (i, c) in cs.iter().enumerate() {
            if map.contains_key(c) && map[c] >= cursor {
                println!("contains {} at {}", c, map[c]);
                cursor = map[c] + 1;
                len = i - map[c];
                map.insert(c, i);
                continue;
            } else {
                map.insert(c, i);
                len += 1;
            }
            res = std::cmp::max(res, len);
        }
        res as i32
    }
}
// @lc code=end

