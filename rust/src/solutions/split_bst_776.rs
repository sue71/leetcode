/*
 * @lc app=leetcode id=776 lang=rust
 *
 * [776] Split BST
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

struct Solution {}

// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn split_bst(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if let Some(node) = root.clone() {
            if node.borrow().val <= target {
                let right_split = Self::split_bst(node.borrow().right.clone(), target);
                node.borrow_mut().right = right_split[0].clone();
                vec![Some(node), right_split[1].clone()]
            } else {
                let left_split = Self::split_bst(node.borrow().left.clone(), target);
                node.borrow_mut().left = left_split[1].clone();
                vec![left_split[0].clone(), Some(node)]
            }
        } else {
            vec![None, None]
        }
    }
}
// @lc code=end

