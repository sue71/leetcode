/*
 * @lc app=leetcode id=779 lang=rust
 *
 * [779] K-th Symbol in Grammar
 */

// @lc code=start

impl Solution {

    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        
        let mid = 2_i32.pow(n as u32 - 1) / 2;
    
        if k <= mid {
            return Self::kth_grammar(n - 1, k);
        } else {
            let original = Self::kth_grammar(n - 1, k - mid);
            return if original == 0 { 1 } else { 0 };
        }

        // TLE
        // let mut s = String::from("0");
        // for i in 0..n {
        //     if i == 0 {
        //         continue
        //     }
        //     s = s.to_string() + &s.chars().map(|c| if c == '0' { '1' } else { '0' }).collect::<String>();

        //     println!("row{}= {}", i, s);
        // }
        // s.chars().nth(k as usize - 1).unwrap() as i32 - '0' as i32 
        
        // Cannot store the whole bit
        // let mut num: u128 = 0b0;
        // for i in 0..n {
        //     if i == 0 {
        //         continue
        //     }
        //     let width = 2_i32.pow(i as u32) / 2;
        //     let mask = (1 << width) - 1; 
        //     let rhs = !num & &mask;

        //     num = num << width | rhs;
        // }

        // (num >> (2_i32.pow(n as u32 - 1) - k) & 1) as i32
    }
}
// @lc code=end

