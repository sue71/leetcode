/*
 * @lc app=leetcode id=31 lang=rust
 *
 * [31] Next Permutation
 */

// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut k: i32 = -1;
        let mut l: i32 = -1;
        let mut prev = i32::MAX;
        for (i, v) in nums.iter().enumerate() {
            if prev < *v {
                k = i as i32;
            }
            if k > 0 && nums[k as usize - 1] < *v {
                l = i as i32;
            }
            prev = *v;
        }

        if k == -1 {
            nums.reverse();
            return;
        }

        if l == -1 {
            nums.swap(k as usize - 1, k as usize);
            return;
        }

        nums.swap(k as usize - 1, l as usize);

        let end = nums.len() - 1;

        quick_sort(nums, k as isize, end as isize);
    }
}

fn quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        quick_sort(arr, low, p - 1);
        quick_sort(arr, p + 1, high);
    }
}

fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as isize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot as usize] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot as usize] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}
// @lc code=end
