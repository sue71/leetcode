/*
 * @lc app=leetcode id=204 lang=typescript
 *
 * [204] Count Primes
 */

// @lc code=start
function countPrimes(n: number): number {
  const arr = [...Array(n).keys()].map(() => true);

  arr[0] = arr[1] = false;

  for (let i = 2; i * i < n; i++) {
    if (arr[i]) {
      for (let j = i * i; j < n; j += i) {
        arr[j] = false;
      }
    }
  }

  return arr.filter(Boolean).length;
}
// @lc code=end
