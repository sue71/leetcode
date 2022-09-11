/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
 */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        pub fn _dfs(node: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
            if let Some(n) = node {
                let c = n.as_ref().borrow();
                if c.left.is_none() && c.right.is_none() && sum - c.val == 0 {
                  return true
                }
                if _dfs(&c.left, sum - c.val) {
                  return true
                } 
                if _dfs(&c.right, sum - c.val) {
                  return true
                }

                false
            } else {
              false
            }
        }

        _dfs(&root, target_sum)
    } 
}
// @lc code=end

