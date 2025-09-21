/*
 * @lc app=leetcode id=94 lang=typescript
 *
 * [94] Binary Tree Inorder Traversal
 */
class TreeNode {
    val: number
    left: TreeNode | null
    right: TreeNode | null
    constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.left = (left===undefined ? null : left)
        this.right = (right===undefined ? null : right)
    }
}

// @lc code=start

// Definition for a binary tree node.

function inorderTraversal(root: TreeNode | null): number[] {
    const memo: number[] = [];
    internal(root, memo)
    return memo;
};

function internal(root: TreeNode | null, memo: number[]) {
    if (root === null) {
        return memo;
    }

    if (root?.left) {
        internal(root.left, memo)
    }

    memo.push(root.val);

    if (root?.right) {
        internal(root.right, memo)
    }
}
// @lc code=end

