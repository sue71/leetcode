/*
 * @lc app=leetcode id=929 lang=rust
 *
 * [929] Unique Email Addresses
 */


// @lc code=start
use std::collections::{HashMap};
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut memo = HashMap::new();
        for email in emails.iter() {
            let normalized = Self::normalize(email);
            memo.insert(normalized, true);
        }

        memo.len() as i32
    }

    pub fn normalize(email: &str) -> String {
        let mut res = String::new();
        let mut skip = false;
        let mut domain = false;
        for c in email.chars() {
            if c == '@' {
                domain = true;
            }
            if domain {
                res.push(c);
            } else {
                if c == '.' {
                    continue;
                }
                if c == '+' {
                    skip = true;
                    continue;
                }
                if !skip {
                    res.push(c);
                }
            }
        }
        res
    }
}
// @lc code=end

