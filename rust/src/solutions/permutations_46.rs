/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 */


struct Solution {}

// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 1 {
            return vec![nums];
        }
        nums.iter()
            .flat_map(|&num| {
                let mut sub = nums.clone().into_iter().filter(|&x| x != num).collect();
                Solution::permute(sub)
                    .into_iter()
                    .map(|vec| {
                        let mut vec = vec;
                        vec.push(num);
                        vec
                    })
                    .collect::<Vec<Vec<i32>>>()
            })
            .collect()
    }
}
// @lc code=end