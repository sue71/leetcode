/*
 * @lc app=leetcode id=98 lang=rust
 *
 * [98] Validate Binary Search Tree
 */
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
            right: None,
        }
    }
}

// @lc code=start
// Definition for a binary tree node.
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::_is_valid_bst(root, i64::MIN, i64::MAX)
    }
    pub fn _is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match root {
            Some(r) => {
                if let Some(left) = r.borrow().left.clone() {
                    if left.borrow().val >= r.borrow().val
                        || left.borrow().val as i64 >= max
                        || left.borrow().val as i64 <= min
                    {
                        return false;
                    }
                    if Self::_is_valid_bst(Some(left), min, cmp::min(r.borrow().val as i64, max))
                        == false
                    {
                        return false;
                    }
                }
                if let Some(right) = r.borrow().right.clone() {
                    if right.borrow().val <= r.borrow().val
                        || right.borrow().val as i64 >= max
                        || right.borrow().val as i64 <= min
                    {
                        return false;
                    }
                    if Self::_is_valid_bst(Some(right), cmp::max(r.borrow().val as i64, min), max)
                        == false
                    {
                        return false;
                    }
                }
                return true;
            }
            None => return true,
        }
    }
}
// @lc code=end
