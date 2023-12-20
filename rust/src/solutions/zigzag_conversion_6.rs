/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] Zigzag Conversion
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut s = String::new();
        let mut rows: Vec<Vec<String>> = vec![];
        let mut row: i32 = 0;
        let mut dir: i32 = 1;

        for i in 0..num_rows {
            rows.push(vec![]);
        }

        for (i, c) in chars.iter().enumerate() {
            rows[row as usize].push(c.to_string());

            if (num_rows == 1) {
                continue;
            }

            if (row as i32) == num_rows - 1 {
                dir = -1;
            } else if row == 0 {
                dir = 1;
            }

            row += dir;
        }

        for row in rows {
            row.iter().for_each(|c| s.push_str(c));
        }

        s
    }
}
// @lc code=end

