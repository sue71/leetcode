/*
 * @lc app=leetcode id=139 lang=rust
 *
 * [139] Word Break
 */


pub struct Solution;

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_set: HashSet<&str> = word_dict.iter().map(|s| s.as_str()).collect();
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 1..=n {
            for word in &word_set {
                let len = word.len();
                if i >= len && dp[i - len] && &s[i - len..i] == *word {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[n]
    }
}
// @lc code=end

