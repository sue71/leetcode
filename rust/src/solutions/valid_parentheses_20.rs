/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let chars = s.chars().collect::<Vec<char>>();

        let mut map = HashMap::new();
        map.insert(')', '(');
        map.insert('}', '{');
        map.insert(']', '[');

        if chars.len() % 2 != 0 {
            return false;
        }

        for c in chars {
            if c == '(' || c == '{' || c == '[' {
                stack.push(c);
            } else {
                if let Some(&v) = map.get(&c) {
                    if let Some(last) = stack.pop() {
                        if last != v {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        if stack.len() > 0 {
            return false;
        }

        true
    }
}
// @lc code=end
