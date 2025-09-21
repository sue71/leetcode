/*
 * @lc app=leetcode id=232 lang=typescript
 *
 * [232] Implement Queue using Stacks
 */

// @lc code=start
class MyQueue {
    #stack: number[] = [];
    constructor() {
    }

    push(x: number): void {
        this.#stack.push(x)
    }

    pop(): number {
        const [pop, ...rest] = this.#stack
        this.#stack = rest;
        return pop
    }

    peek(): number {
        return this.#stack[0];
    }

    empty(): boolean {
        return this.#stack.length === 0
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * var obj = new MyQueue()
 * obj.push(x)
 * var param_2 = obj.pop()
 * var param_3 = obj.peek()
 * var param_4 = obj.empty()
 */
// @lc code=end

