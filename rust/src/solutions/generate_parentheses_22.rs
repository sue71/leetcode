/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];

        Self::backtrack(&mut res, "".to_string(), 0, 0, n);

        res
    }

    pub fn backtrack(answer: &mut Vec<String>, answer_str: String, lc: i32, rc: i32, n: i32) {
        if lc < rc {
            return;
        }

        if answer_str.len() == (n as usize) * 2 {
            answer.push(answer_str.clone());
        }

        if lc < n {
            Self::backtrack(answer, answer_str.clone() + "(", lc + 1, rc, n);
        }

        if rc < n {
            Self::backtrack(answer, answer_str.clone() + ")", lc, rc + 1, n);
        }
    }
}
// @lc code=end
