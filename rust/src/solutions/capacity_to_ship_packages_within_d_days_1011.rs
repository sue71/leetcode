/*
 * @lc app=leetcode id=1011 lang=rust
 *
 * [1011] Capacity To Ship Packages Within D Days
 */

// @lc code=start
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut right = weights.iter().sum();
        let mut left = weights.iter().max().unwrap().clone();
        let mut mid = 0;

        while left < right {
            mid = (left + right) / 2;
            if Self::check(&weights, days, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        right
    }

    pub fn check(weights: &Vec<i32>, days: i32, mid: i32) -> bool {
        let mut sum = 0;
        let mut d = 1;
        for w in weights.iter() {
            sum += w;
            if sum > mid {
                d += 1;
                sum = *w;
            }
        }
        d <= days
    }
}
// @lc code=end

