/*
 * @lc app=leetcode id=617 lang=rust
 *
 * [617] Merge Two Binary Trees
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
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      match (root1, root2) {
        (None, None) => {
          return None
        },
        (Some(n1), None) => {
          Some(n1)
        },
        (None, Some(n2)) => {
          Some(n2)
        },
        (Some(n1), Some(n2)) => {
          let mut node = TreeNode::new(n1.borrow().val + n2.borrow().val);
          node.left = Self::merge_trees(n1.borrow().left.clone(), n2.borrow().left.clone());
          node.right = Self::merge_trees(n1.borrow().right.clone(), n2.borrow().right.clone());
          Some(Rc::new(RefCell::new(node)))
        }
      }
    }

}
// @lc code=endot

