/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */
pub struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = Vec::new();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            let sorted: String = chars.iter().collect();

            if map.contains_key(&sorted) {
                map.get_mut(&sorted).unwrap().push(str)
            } else {
                map.insert(chars.iter().collect(), vec![str]);
            }

        }

        map.values().cloned().collect()
    }
}
// @lc code=end

