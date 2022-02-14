/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        let mut map = HashMap::new();
        map.insert('(', ')');
        map.insert('[', ']');
        map.insert('{', '}');

        for c in s.chars() {
            if c == '(' || c == '{' || c == '[' {
                stack.push(c);
            } else {
                if stack.len() == 0 || map[&stack.pop().unwrap()] != c {
                    return false
                }
            }
        }

        stack.len() == 0
    }
}
// @lc code=end

