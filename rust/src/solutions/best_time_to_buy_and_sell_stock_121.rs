/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min = i32::MAX;

        for (i, v)  in prices.iter().enumerate() {
            min = std::cmp::min(min, *v);
            max_profit = std::cmp::max(max_profit, v - min);
        }

        max_profit
    }
}
// @lc code=end


