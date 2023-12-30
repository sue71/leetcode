/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums = nums.clone();
        let mut path = vec![];
        let mut used = vec![false; nums.len()];

        Self::dfs(&mut nums, &mut path, &mut used, &mut res);

        res
    }

    pub fn dfs(
        nums: &mut Vec<i32>,
        path: &mut Vec<i32>,
        used: &mut Vec<bool>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            path.push(nums[i]);
            used[i] = true;
            Self::dfs(nums, path, used, res);
            path.pop();
            used[i] = false;
        }
    }
}
// @lc code=end

mod tests {
    use crate::solutions::permutations_46::Solution;

    fn vec_compare<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }

    #[test]
    fn it_works() {
        let exp = vec![vec![1, 2], vec![2, 1]];
        let out = Solution::permute(vec![1, 2]);

        println!("{:?}", out);
        assert_eq!(vec_compare(&out, &exp), true);
    }
}
