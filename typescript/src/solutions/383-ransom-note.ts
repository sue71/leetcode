/*
 * @lc app=leetcode id=383 lang=typescript
 *
 * [383] Ransom Note
 */

// @lc code=start
function canConstruct(ransomNote: string, magazine: string): boolean {
    const memo:Record<string, number> = {};

    for (let c of magazine) {
        if (memo[c]) {
          memo[c] +=1;
        } else {
            memo[c] = 1;
        }
    }

    for (let c of ransomNote) {
        if (memo[c]) {
            memo[c] -=1
        } else {
            return false;
        }
    }

    return true
};
// @lc code=end

