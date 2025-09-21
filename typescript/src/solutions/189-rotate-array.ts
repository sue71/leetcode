/*
 * @lc app=leetcode id=189 lang=typescript
 *
 * [189] Rotate Array
 */

// @lc code=start
/**
 Do not return anything, modify nums in-place instead.
 */
function rotate(nums: number[], k: number): void {
    k = k % nums.length;
    const newNums = [...nums.slice(nums.length - k, nums.length), ...nums.slice(0, nums.length - k)];
    for (const [i, v] of newNums.entries()) {
        nums[i] = v;
    }
};

// @lc code=end

