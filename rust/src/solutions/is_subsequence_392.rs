/*
 * @lc app=leetcode id=392 lang=rust
 *
 * [392] Is Subsequence
 */


// @lc code=start
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        return Self::rec_is_subsequence(&s.chars().collect(), &t.chars().collect(), 0, 0);
    }

    pub fn rec_is_subsequence(s: &Vec<char>, t: &Vec<char>, left_index: usize, right_index: usize) -> bool {
        if left_index == s.len() {
            return true;
        }
        if right_index == t.len() {
            return false;
        }
        if s[left_index] == t[right_index] {
            return Self::rec_is_subsequence(s, t, left_index + 1, right_index + 1);
        }
        return Self::rec_is_subsequence(s, t, left_index, right_index + 1);
    }
}
// @lc code=end

