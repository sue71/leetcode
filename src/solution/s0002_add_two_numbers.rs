/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg" style="width: 483px; height: 342px;" />
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 *
 * Example 2:
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 *
 * Example 3:
 *
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in each linked list is in the range [1, 100].
 * 	0 <= Node.val <= 9
 * 	It is guaranteed that the list represents a number that does not have leading zeros.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/add-two-numbers/
// discuss: https://leetcode.com/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        let (mut a, mut b) = (l1, l2);
        let (mut step_in, mut a_end, mut b_end) = (false, false, false);
        loop {
            let a_val = match a {
                Some(node) => {
                    a = node.next;
                    node.val
                }
                None => {
                    a_end = true;
                    0
                }
            };
            let b_val = match b {
                Some(node) => {
                    b = node.next;
                    node.val
                }
                None => {
                    b_end = true;
                    0
                }
            };
            if a_end && b_end && !step_in {
                return head.unwrap().next;
            }
            let mut num = (a_val + b_val + if step_in { 1 } else { 0 });
            step_in = num >= 10;
            num = num % 10;
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(num)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        head
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![1, 2, 3]), to_list(vec![1, 2, 3])),
            to_list(vec![2, 4, 6])
        );
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9])),
            to_list(vec![8, 9, 9, 0, 1])
        );
    }
}
