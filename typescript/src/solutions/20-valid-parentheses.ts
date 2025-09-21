/*
 * @lc app=leetcode id=20 lang=typescript
 *
 * [20] Valid Parentheses
 */

// @lc code=start
function isValid(s: string): boolean {
    const queue: string[] = [];
    const memo: Record<string, string> = {
        ')': '(',
        '}': '{',
        ']': '['
    } as const
    const open = ['(', '{', '[']
    let validCount = 0;
    for (let c of s) {
        // check valid
        if (memo[c]) {
            const p = queue.pop();
            if (memo[c] === p) {
                validCount += 1
                continue
            } else {
                return false;
            }
        }
        if (open.includes(c)) {
            queue.push(c)
        }
    }

    return s.length / 2 === validCount
};
// @lc code=end

