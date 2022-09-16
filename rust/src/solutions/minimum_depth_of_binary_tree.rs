/*
 * @lc app=leetcode id=111 lang=rust
 *
 * [111] Minimum Depth of Binary Tree
 */
pub struct Solution {}

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
// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        pub fn _min_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root_node) = root {
                let node_borrow = root_node.borrow();
                if node_borrow.left.is_some() && node_borrow.right.is_some() {
                    let left_depth = _min_depth(&node_borrow.left);
                    let right_depth = _min_depth(&node_borrow.right);
                    std::cmp::min(left_depth, right_depth) + 1
                } else if node_borrow.left.is_none() {
                    _min_depth(&node_borrow.right) + 1
                } else if node_borrow.right.is_none() {
                    _min_depth(&node_borrow.left) + 1
                } else {
                    0
                }
            } else {
                0
            }
        }
        _min_depth(&root)
    }
}
// @lc code=end

