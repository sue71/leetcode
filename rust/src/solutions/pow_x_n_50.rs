/*
 * @lc app=leetcode id=50 lang=rust
 *
 * [50] Pow(x, n)
 */

// @lc code=start
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        Self::pow(x, n as i64)
    }

    pub fn pow(x: f64, n: i64) -> f64 {
        if n == 0 {
            return 1.0;
        }

        if n < 0 {
            return 1.0 / Self::pow(x, -n);
        }

        if n % 2 == 0 {
            return Self::pow(x * x, n / 2);
        } else {
            return x * Self::pow(x * x, n / 2);
        }
    }
}
// @lc code=end
