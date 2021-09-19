/*
 * @lc app=leetcode id=20 lang=typescript
 *
 * [20] Valid Parentheses
 */

// @lc code=start
function isValid(s: string): boolean {
  const chars = [];
  const exp = {
    "{": "}",
    "[": "]",
    "(": ")",
  };
  for (let c of s) {
    if (c === "}" || c === "]" || c === ")") {
      const last = chars.pop();
      if (c === exp[last]) {
        continue;
      } else {
        return false;
      }
    } else {
      chars.push(c);
    }
  }
  if (chars.length) {
    return false;
  }
  return true;
}
// @lc code=end
