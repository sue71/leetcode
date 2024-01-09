/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut chars = s.chars().collect::<Vec<char>>();
            chars.sort_by(|a, b| a.cmp(b));

            let key = chars.iter().collect::<String>();
            if map.contains_key(&key) {
                map.get_mut(&key).unwrap().push(s);
            } else {
                map.insert(key, vec![s]);
            }
        }

        map.iter().map(|(_, v)| v.clone()).collect()
    }
}
// @lc code=end
