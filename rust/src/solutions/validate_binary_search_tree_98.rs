/*
 * @lc app=leetcode id=98 lang=rust
 *
 * [98] Validate Binary Search Tree
 */
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]

pub struct Solution {}

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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        pub fn _is_valid_bst(
            node: Option<Rc<RefCell<TreeNode>>>,
            min: Option<Rc<RefCell<TreeNode>>>,
            max: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match node {
                Some(n) => {
                    if let Some(max) = max.clone() {
                        if max.borrow().val <= n.borrow().val {
                            return false;
                        }
                    }
                    if let Some(min) = min.clone() {
                        if min.borrow().val >= n.borrow().val {
                            return false;
                        }
                    }
                    _is_valid_bst(n.borrow().left.clone(), min, Some(n.clone()))
                        && _is_valid_bst(n.borrow().right.clone(), Some(n.clone()), max)
                }
                None => true,
            }
        }
        _is_valid_bst(root.clone(), None, None)
    }
}
// @lc code=end
