/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest substring without repeating characters.
 *  
 * Example 1:
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 * Example 2:
 *
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 * Example 3:
 *
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 * Example 4:
 *
 * Input: s = ""
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of English letters, digits, symbols and spaces.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set: HashSet<char> = HashSet::with_capacity(s.len());
        let (mut ans, mut i, mut j): (i32, usize, usize) = (0, 0, 0);

        loop {
            if i == s.len() || j == s.len() {
                break;
            }

            let ci = s.chars().nth(i).unwrap();
            let cj = s.chars().nth(j).unwrap();
            if set.contains(&cj) {
                i = i + 1;
                set.remove(&ci);
            } else {
                j = j + 1;
                set.insert(cj);
                ans = max(ans, set.len() as i32);
            }
        }

        ans
    }

    pub fn length_of_longest_substring_map(s: String) -> i32 {
        let mut map = HashMap::new();
        let (mut ans, mut i, mut j): (i32, usize, usize) = (0, 0, 0);
        let c: Vec<char> = s.chars().collect();
        let n = s.len();

        loop {
            if i == n || j == n {
                break;
            }

            let ci = &c[i];
            let cj = &c[j];

            // Update window left if needed
            if map.contains_key(&cj) {
                i = max(map[&cj], i);
            }
            // Update window right
            j = j + 1;

            // Update index map
            map.insert(cj, j);

            ans = max(ans, (j - i) as i32);
        }

        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbbbbbb")),
            1
        );
        assert_eq!(Solution::length_of_longest_substring(String::from("")), 0);
        assert_eq!(
            Solution::length_of_longest_substring_map(String::from("abcabcbbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring_map(String::from("bbbbbbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring_map(String::from("")),
            0
        );
    }
}
