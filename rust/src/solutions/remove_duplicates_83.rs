/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cursor = head.as_mut();
        loop {
            // head is none
            if cursor.is_none() {
                break;
            }

            // next is none
            if cursor.as_ref().unwrap().next.is_none() {
                break;
            }

            // next has same value 
            if cursor.as_ref().unwrap().val == cursor.as_ref().unwrap().next.as_ref().unwrap().val {
                let next = cursor.as_mut().unwrap().next.as_mut().unwrap().next.take();
                cursor.as_mut().unwrap().next = next;
                continue;
            }

            // move cursor
            cursor = cursor.unwrap().next.as_mut()
        }
        head
    }
}
// @lc code=end

