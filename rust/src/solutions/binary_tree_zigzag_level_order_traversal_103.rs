/*
 * @lc app=leetcode id=103 lang=rust
 *
 * [103] Binary Tree Zigzag Level Order Traversal
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
use std::collections::VecDeque;
use std::cell::RefCell;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::<Vec<i32>>::new();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut level = 0;

        if root.is_none() { 
          return vec![];
        } else {
          queue.push_front(root.unwrap());
        }

        while !queue.is_empty() {
          let is_left_to_right = level % 2 == 0;
          let mut v = Vec::<i32>::new();
          let size = queue.len();

          for _ in 0..size {
            if let Some(node) = queue.pop_front() {
              v.push(node.borrow().val);

              if let Some(l) = node.borrow().left.as_ref() {
                queue.push_back(l.clone());
              }
              if let Some(r) = node.borrow().right.as_ref() {
                queue.push_back(r.clone());
              }
            }
          }

          if is_left_to_right {
            res.push(v.clone());
          } else {
            v.reverse();
            res.push(v.clone());
          }

          level += 1;
        }

        res
    }
}
// @lc code=end

