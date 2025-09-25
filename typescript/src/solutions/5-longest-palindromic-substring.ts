/*
 * @lc app=leetcode id=5 lang=typescript
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
function longestPalindrome(s: string): string {
    let max = -1;
    let res = '';
    for (const i of [...Array(s.length).keys()]) {
        const odd = expand(i, i, s);
        const even = expand(i, i+1, s);
        const a = even[1] - even[0];
        const b = odd[1] - odd[0];

        if (a > max) {
            max = a;
            res = s.slice(even[0], even[1] + 1);
        }

        if (b > max) {
            max = b;
            res = s.slice(odd[0], odd[1] + 1);
        }
    }

    return res;
};

function expand(left: number, right: number, s: string): [number, number] {
    while (left >= 0 && right < s.length && s.charAt(left) === s.charAt(right)) {
        left -= 1;
        right += 1
    }

    return [left + 1, right -1];
}
// @lc code=end

