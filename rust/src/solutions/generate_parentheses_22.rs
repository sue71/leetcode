/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 */

pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();

        fn generate(n: i32, left: i32, right: i32, p: String, result: &mut Vec<String>) {
            if right > left || left > n || right > n {
                return;
            }
            if left == n && right == n {
                result.push(p);
                return;
            }

            generate(n, left + 1, right, p.clone() + "(", result);
            generate(n, left, right + 1, p + ")", result);
        }

        generate(n, 0, 0, String::from(""), &mut result);

        result
    }
}
// @lc code=end
