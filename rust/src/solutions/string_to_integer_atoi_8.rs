/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        for (i, c) in s.chars().enumerate() {
            if c == ' ' {
                continue;
            }
            if c == '-' || c == '+' || c.is_digit(10) {
                let mut j = i + 1;
                while j < s.len() && s.chars().nth(j).unwrap().is_digit(10) {
                    j += 1;
                }

                let num = s[i..j].parse::<i64>().unwrap_or(0);

                if num < std::i32::MIN as i64 {
                    return std::i32::MIN;
                } else if num > std::i32::MAX as i64 {
                    return std::i32::MAX;
                } else {
                    return num as i32;
                }
            } else {
                return 0;
            }
        }
        0
    }
}
// @lc code=end
