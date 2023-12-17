/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// @lc code=start
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut res = ListNode::new(0);

      fn node_next(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
        match (l1.as_ref(), l2.as_ref()) {
          (Some(l1), Some(l2)) => {
            let mut sum = l1.val + l2.val + carry;
            let next_carry = sum / 10;
            sum %= 10;
            let mut res = ListNode::new(sum);
            res.next = node_next(&l1.next, &l2.next, next_carry);
            return Some(Box::new(res))
          },
          (Some(l1), None) => {
            let mut sum = l1.val + carry;
            let next_carry = sum / 10;
            sum %= 10;
            let mut res = ListNode::new(sum);
            res.next = node_next(&l1.next, &None, next_carry);
            return Some(Box::new(res))
          },
          (None, Some(l2)) => {
            let mut sum = l2.val + carry;
            let next_carry = sum / 10;
            sum %= 10;
            let mut res = ListNode::new(sum);
            res.next = node_next(&None, &l2.next, next_carry);
            return Some(Box::new(res))
          },
          (None, None) => {
            if carry == 0 {
              return None;
            }
            let mut res = ListNode::new(carry);
            res.next = None;
            return Some(Box::new(res))
          },
        }
      }

      node_next(&l1, &l2, 0)
    }

}
// @lc code=end

