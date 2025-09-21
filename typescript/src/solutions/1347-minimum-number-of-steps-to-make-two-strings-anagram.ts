/*
 * @lc app=leetcode id=1347 lang=typescript
 *
 * [1347] Minimum Number of Steps to Make Two Strings Anagram
 */

// @lc code=start
function minSteps(s: string, t: string): number {
    const memo: Record<string, number> = {};
    const memot: Record<string, number> = {};
    for (let c of s) {
        if (memo[c]) {
            memo[c] += 1;
        } else {
            memo[c] = 1;
        }
    }

    for (let c of t) {
        if (memo[c]) {
            memo[c] -=1;
        }
    }

    let res = 0;
    for (const [_, value] of Object.entries(memo)) {
        res += value;
    }

    return res;
};
// @lc code=end

