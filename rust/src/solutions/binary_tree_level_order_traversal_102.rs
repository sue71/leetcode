/*
 * @lc app=leetcode id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
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
            right: None,
        }
    }
}

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut cursors = vec![root];
        let mut res: Vec<Vec<i32>> = vec![];

        while cursors.len() > 0 {
            let mut items = vec![];
            let mut c = vec![];
            for cursor in cursors {
                if let Some(node) = cursor {
                    items.push(node.borrow().val);

                    if let Some(left) = &node.borrow().left {
                        c.push(Some(left.clone()));
                    }
                    if let Some(right) = &node.borrow().right {
                        c.push(Some(right.clone()));
                    }
                }
            }
            res.push(items);
            cursors = c;
        }

        res
    }
}
// @lc code=end
