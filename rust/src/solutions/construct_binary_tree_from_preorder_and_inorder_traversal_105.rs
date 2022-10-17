/*
 * @lc app=leetcode id=105 lang=rust
 *
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = preorder[0];
        let mut node = TreeNode::new(root);

        pub fn dfs(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(root) = preorder.first() {
                let cursor = inorder.iter().position(|x| x == root).unwrap();
                return Some(Rc::new(RefCell::new(TreeNode {
                    val: *root,
                    left: dfs(&preorder[1..(cursor + 1)], &inorder[0..cursor]),
                    right: dfs(&preorder[(1 + cursor)..], &inorder[(cursor + 1)..]),
                })));
            } else {
                None
            }
        }

        dfs(&preorder, &inorder)
    }
}
// @lc code=end
