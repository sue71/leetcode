/*
 * @lc app=leetcode id=703 lang=typescript
 *
 * [703] Kth Largest Element in a Stream
 */

class MinHeap {
  constructor(private readonly items: number[]) {}
  add(val: number) {}

  pop(k: number) {
    return this.items[this.items.length - k];
  }
}

// @lc code=start
class KthLargest {
  k: number;
  items: number[] = [];
  constructor(k: number, nums: number[]) {
    this.k = k;
    for (let i of nums) {
      this.add(i);
    }
  }

  add(val: number): number {
    this.items.push(val);
    this.heapifyUp(this.items.length - 1);

    if (this.items.length < this.k) {
      return null;
    }

    while (this.items.length <= this.k) {
      this.extract();
    }

    return this.items[0];
  }

  extract() {
    let item = this.items[0];
    this.items[0] = this.items.pop();
    this.heapifyDown(0);
    return item;
  }

  heapifyUp(index: number) {
    const p = Math.floor((index - 1) / 2);
    if (this.items[p] >= this.items[index]) {
      this.swap(index, p);
      this.heapifyUp(p);
    }
  }

  heapifyDown(index: number) {
    let smallest = index;
    let leftChild = index * 2 + 1;
    let rightChild = index * 2 + 2;

    if (
      leftChild < this.items.length &&
      this.items[leftChild] < this.items[index]
    ) {
      smallest = leftChild;
    }
    if (
      rightChild < this.items.length &&
      this.items[rightChild] < this.items[smallest]
    ) {
      smallest = rightChild;
    }
    if (smallest != index) {
      this.swap(index, smallest);
      this.heapifyDown(smallest);
    }
  }

  swap(a: number, b: number) {
    [this.items[a], this.items[b]] = [this.items[b], this.items[a]];
  }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * var obj = new KthLargest(k, nums)
 * var param_1 = obj.add(val)
 */
// @lc code=end
