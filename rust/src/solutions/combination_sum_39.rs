/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 */

use std::{collections::HashMap, fs::canonicalize};

struct Solution {}

// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::backtrack(&candidates, target, 0, 0, &mut current, &mut result);
        result
    }

    pub fn backtrack(
        candidates: &Vec<i32>,
        target: i32,
        num: i32,
        cursor: usize,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        let mut num = num;

        for (i, v) in candidates.iter().enumerate() {
            if i < cursor {
                continue;
            }
            if num > target {
                return;
            }
            if num == target {
                result.push(current.clone());
                return;
            }
            current.push(*v);
            num += v;
            Self::backtrack(candidates, target, num, i, current, result);
            num -= v;
            current.pop();
        }
    }
}
// @lc code=end
