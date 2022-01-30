/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
use std::rc::Rc;
use std::cell::RefCell;
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // using reference to avoid clone
        pub fn _max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root_node) = root {
                let node_borrow = root_node.borrow();
                let left_depth = _max_depth(&node_borrow.left);
                let right_depth = _max_depth(&node_borrow.right);
                std::cmp::max(left_depth, right_depth) + 1
            } else {
                0
            }
        }
        _max_depth(&root)
    }
}

// @lc code=end

