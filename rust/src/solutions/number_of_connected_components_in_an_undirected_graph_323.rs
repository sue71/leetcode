/*
 * @lc app=leetcode id=323 lang=rust
 *
 * [323] Number of Connected Components in an Undirected Graph
 */

// @lc code=start
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut seen = vec![false; n as usize];
        let mut res = 0;
        for i in 0..n {
            if Self::dfs(&edges, i as usize, n as usize, &mut seen) {
                res += 1;
            }
        }
        res
    }

    fn dfs(edges: &Vec<Vec<i32>>, i: usize, n: usize, seen: &mut Vec<bool>) -> bool {
        if seen[i] {
            return false;
        }
        seen[i] = true;

        for edge in edges.iter() {
            if 
                i == edge[0] as usize
            {
                Self::dfs(edges, edge[1] as usize, n, seen);
            }

            if 
                i == edge[1] as usize
            {
                Self::dfs(edges, edge[0] as usize, n, seen);
            }
        }

        true
    }
}
// @lc code=end

