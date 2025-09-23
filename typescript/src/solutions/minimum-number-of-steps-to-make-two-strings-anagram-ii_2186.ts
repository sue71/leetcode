/*
 * @lc app=leetcode id=2186 lang=typescript
 *
 * [2186] Minimum Number of Steps to Make Two Strings Anagram II
 */

// @lc code=start
function minSteps(s: string, t: string): number {
  const memo1: Record<string, number> = {};
  const memo2: Record<string, number> = {};

  for (let c of s) {
    if (memo1[c]) {
      memo1[c] += 1;
    } else {
      memo1[c] = 1;
    }
  }
  for (let c of t) {
    if (memo2[c]) {
      memo2[c] += 1;
    } else {
      memo2[c] = 1;
    }
  }

  let res = 0;

  for (const c of s) {
    if (memo2[c]) {
      memo2[c] -= 1;
    }
  }

  for (const c of t) {
    if (memo1[c]) {
      memo1[c] -= 1;
    }
  }

  const a = Object.entries(memo1)
    .map(([c, num]) => {
      return num;
    })
    .reduce((v, acc) => acc + v);
  const b = Object.entries(memo2)
    .map(([c, num]) => {
      return num;
    })
    .reduce((v, acc) => acc + v);

  return a + b;
}
// @lc code=end
