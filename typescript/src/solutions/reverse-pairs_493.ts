/*
 * @lc app=leetcode id=493 lang=typescript
 *
 * [493] Reverse Pairs
 */

// @lc code=start
function reversePairs(nums: number[]): number {
  return internal(nums, 0, nums.length - 1);
}

function internal(nums: number[], start: number, end: number): number {
  if (start >= end) {
    return 0;
  }

  const mid = Math.floor((start + end) / 2);

  let count = internal(nums, start, mid);
  count += internal(nums, mid + 1, end);

  let j = mid + 1;
  for (let i = start; i <= mid; i++) {
    while (j <= end && nums[i] > 2 * nums[j]) {
      j++;
    }
    count += j - (mid + 1);
  }

  const temp: number[] = [];
  let p1 = start,
    p2 = mid + 1;
  while (p1 <= mid || p2 <= end) {
    if (p2 > end || (p1 <= mid && nums[p1] <= nums[p2])) {
      temp.push(nums[p1++]);
    } else {
      temp.push(nums[p2++]);
    }
  }

  for (let i = 0; i < temp.length; i++) {
    nums[start + i] = temp[i];
  }

  return count;
}
// @lc code=end
