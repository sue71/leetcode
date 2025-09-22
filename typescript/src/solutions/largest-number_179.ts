/*
 * @lc app=leetcode id=179 lang=typescript
 *
 * [179] Largest Number
 */

// @lc code=start
function largestNumber(nums: number[]): string {
  nums.sort((lhs, rhs) => {
    if (lhs === rhs) {
      return 0;
    }
    const num1 = parseInt(lhs + "" + rhs);
    const num2 = parseInt(rhs + "" + lhs);
    return num2 - num1;
    // const len = Math.max(String(lhs).length, String(rhs).length);
    // const s1 = Number(String(lhs)[0]);
    // const s2 = Number(String(rhs)[0]);

    // if (s1 !== s2) {
    //   return s2 - s1;
    // }

    // const base = s1;

    // for (let i of [...Array(len).keys()]) {
    //   if (i === 0) {
    //     continue;
    //   }
    //   const s1 = String(lhs)[i];
    //   const s2 = String(rhs)[i];

    //   const s1n = s1 ? parseInt(s1) : undefined;
    //   const s2n = s2 ? parseInt(s2) : undefined;

    //   if (s1n !== undefined && s2n !== undefined) {
    //     if (s1n !== s2n) {
    //       return s2n - s1n;
    //     }
    //     if (s1n === s2n) {
    //       continue;
    //     }
    //   }
    //   if (s1n === undefined && s2n === undefined) {
    //     return 0;
    //   }
    //   if (s1n === undefined && s2n !== undefined) {
    //     if (s2n > base) {
    //       return 1;
    //     } else {
    //       return -1;
    //     }
    //   }
    //   if (s2n === undefined && s1n !== undefined) {
    //     if (s1n > base) {
    //       return -1;
    //     } else {
    //       return 1;
    //     }
    //   }
    // }
    // return 0;
  });

  const res = nums.join("");
  if (parseInt(res) === 0) {
    return "0";
  }
  return res;
}

// @lc code=end
