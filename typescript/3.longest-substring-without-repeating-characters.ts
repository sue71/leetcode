/*
 * @lc app=leetcode id=3 lang=typescript
 *
 * [3] Longest Substring Without Repeating Characters
 */


// @lc code=start
function lengthOfLongestSubstring(s: string): number {
    const set = new Set();
    let lc = 0;
    let rc = 0;
    let res = 0;

    [...s].forEach((v) => {
        rc++;
        if (set.has(v)) {
            set.delete(v);
            lc++;
        } else {
            res = Math.max(res, rc - lc);
        }

        set.add(v);
        console.log(v, lc, rc, Array.from(s).slice(lc, rc), set);
    });

    return res;
};
// @lc code=end

