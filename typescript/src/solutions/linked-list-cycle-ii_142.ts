/*
 * @lc app=leetcode id=142 lang=typescript
 *
 * [142] Linked List Cycle II
 */
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}

// @lc code=start

function detectCycle(head: ListNode | null): ListNode | null {
    const set = new Set();
    while (head) {
        if (head.next == null) {
            break
        }
        if (set.has(head)) {
            return head;
        }
        set.add(head);
        head = head.next;
    }
    return null
};
// @lc code=end

