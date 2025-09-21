/*
 * @lc app=leetcode id=295 lang=typescript
 *
 * [295] Find Median from Data Stream
 */
// @lc code=start


abstract class MyHeap<T> {
  protected heap: T[] = [];
  protected abstract compare(a: T, b: T): boolean;

  private getParentIndex(i: number): number { return Math.floor((i - 1) / 2); }
  private getLeftChildIndex(i: number): number { return 2 * i + 1; }
  private getRightChildIndex(i: number): number { return 2 * i + 2; }
  
  private hasParent(i: number): boolean { return this.getParentIndex(i) >= 0; }
  private hasLeftChild(i: number): boolean { return this.getLeftChildIndex(i) < this.heap.length; }
  
  public get size(): number { return this.heap.length; }
  public isEmpty(): boolean { return this.size === 0; }
  
  public front(): T | null {
    return this.isEmpty() ? null : this.heap[0];
  }

  private swap(i: number, j: number): void {
    [this.heap[i], this.heap[j]] = [this.heap[j], this.heap[i]];
  }

  public enqueue(value: T): void {
    this.heap.push(value);
    this.siftUp();
  }

  public dequeue(): T | null {
    if (this.isEmpty()) return null;
    if (this.size === 1) return this.heap.pop()!;
    
    const root = this.heap[0];
    this.heap[0] = this.heap.pop()!; // 末尾の要素を先頭に移動
    this.siftDown();
    return root;
  }
  
  private siftUp(): void {
    let index = this.size - 1;
    while (this.hasParent(index) && this.compare(this.heap[index], this.heap[this.getParentIndex(index)])) {
      const parentIndex = this.getParentIndex(index);
      this.swap(index, parentIndex);
      index = parentIndex;
    }
  }

  private siftDown(): void {
    let index = 0;
    while (this.hasLeftChild(index)) {
      let smallerChildIndex = this.getLeftChildIndex(index);
      const rightChildIndex = this.getRightChildIndex(index);

      if (this.heap[rightChildIndex] !== undefined && this.compare(this.heap[rightChildIndex], this.heap[smallerChildIndex])) {
        smallerChildIndex = rightChildIndex;
      }

      if (this.compare(this.heap[index], this.heap[smallerChildIndex])) {
        break;
      }
      
      this.swap(index, smallerChildIndex);
      index = smallerChildIndex;
    }
  }
}

class MyMinHeap extends MyHeap<number> {
  protected compare(a: number, b: number): boolean {
    return a < b;
  }
}

class MyMaxHeap extends MyHeap<number> {
  protected compare(a: number, b: number): boolean {
    return a > b;
  }
}

class MedianFinder{
    smallHalf = new MyMaxHeap();
    largeHalf = new MyMinHeap();
    constructor() {
        
    }

    addNum(num: number): void {
    this.smallHalf.enqueue(num);

    if (!this.smallHalf.isEmpty() && !this.largeHalf.isEmpty() && this.smallHalf.front()! > this.largeHalf.front()!) {
      this.largeHalf.enqueue(this.smallHalf.dequeue()!);
    }
    
    if (this.smallHalf.size > this.largeHalf.size + 1) {
        this.largeHalf.enqueue(this.smallHalf.dequeue()!);
    } else if (this.largeHalf.size > this.smallHalf.size + 1) {
        this.smallHalf.enqueue(this.largeHalf.dequeue()!);
    }
    }

    findMedian(): number {
      if (this.smallHalf.size > this.largeHalf.size) {
        return this.smallHalf.front()!;
      }
      else if (this.largeHalf.size > this.smallHalf.size) {
        return this.largeHalf.front()!;
      }
      else {
        return (this.smallHalf.front()! + this.largeHalf.front()!) / 2;
      }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * var obj = new MedianFinder()
 * obj.addNum(num)
 * var param_2 = obj.findMedian()
 */
// @lc code=end

