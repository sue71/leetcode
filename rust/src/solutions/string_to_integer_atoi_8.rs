/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut ss = String::from("");
        let trim_s = s.trim();
        let mut sign = 1;
        let mut cursor = 0;
        match trim_s.chars().nth(0) {
            Some('+') => {
                sign = 1;
                cursor += 1;
            }
            Some('-') => {
                sign = -1;
                cursor += 1;
            },
            None => {
                return 0
            },
            _ => { }
        }
        for i in cursor..trim_s.len() {
            let c = trim_s.chars().nth(i).unwrap();
            if c.is_ascii_digit() {
                ss.push(c);
            } else {
                break;
            }
        }
        if ss.len() == 0 {
            return 0;
        }
        return ss
            .parse::<i32>()
            .map(|n| n * sign)
            .unwrap_or(if sign == 1 { i32::MAX  } else { i32::MIN });
    }
}
// @lc code=end

