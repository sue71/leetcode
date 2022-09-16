/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] Zigzag Conversion
 */
pub struct Solution {
}

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut strs: Vec<String> = Vec::<String>::with_capacity(num_rows as usize);
        let chars: Vec<_> = s.chars().collect();

        for i in 0..num_rows {
            strs.push(String::from(""));
        }
        let mut step = 0;
        let mut direction: i32 = 1;

        if num_rows == 1 {
            return s;
        }
        for j in 0..s.len() {
            strs[step as usize] = strs[step as usize].to_string() + &chars[j].to_string();

            step = step + 1 * direction;
            if step == num_rows - 1 || step == 0 {
                direction = direction * -1;
            }

        }
        strs.join("")
    }
}
// @lc code=end

