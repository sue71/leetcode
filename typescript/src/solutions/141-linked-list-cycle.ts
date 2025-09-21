/*
 * @lc app=leetcode id=141 lang=typescript
 *
 * [141] Linked List Cycle
 */

//Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}


// @lc code=start

function hasCycle(head: ListNode | null): boolean {
    if (head === null) {
        return false;
    }
    let slow: ListNode | null = head
    let fast: ListNode | null = head
    let loop = false;
    while (slow && fast) {
        slow = slow.next ?? null
        fast= fast.next?.next ?? null
        if (slow === null || fast === null) {
            return false;
        }
        if (slow === fast) {
            return true;
        }
    }
    return loop;
};
// @lc code=end

