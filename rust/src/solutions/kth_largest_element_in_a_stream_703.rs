/*
 * @lc app=leetcode id=703 lang=rust
 *
 * [703] Kth Largest Element in a Stream
 */

// @lc code=start
struct KthLargest {
    internal: KthLargestInternal
}
struct KthLargestInternal {
    k: i32,
    items: Vec<i32>
}

impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut internal = KthLargestInternal::new(k);
        for v in nums {
            internal.add(v);
        }
        KthLargest { internal: internal }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.internal.add(val)
    }
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargestInternal {

    fn new(k: i32) -> Self {
        KthLargestInternal { k: k, items: vec![] }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.items.push(val);

        self.heapifyUp(self.items.len() - 1);

        if self.items.len() > self.k as usize {
            self.extract();
        }

        *self.items.first().unwrap()
    }

    fn extract(&mut self) {
        let item = self.items.pop().unwrap();
        self.items[0]= item;
        self.heapifyDown(0);
    }

    fn heapifyUp(&mut self, index: usize) {
        if index < 1 {
            return;
        }

        let i = index;
        let p: usize = (i - 1) / 2;

        if self.items[p] > self.items[i] {
            self.items.swap(p, i);
            self.heapifyUp(p);
        }
    }

    fn heapifyDown(&mut self, index: usize) {
        let mut smallest: usize = index;
        let lc = index * 2 + 1;
        let rc = index * 2 + 2;

        if lc < self.items.len() && self.items[lc] < self.items[smallest] {
            smallest = lc;
        }

        if rc < self.items.len() && self.items[rc] < self.items[smallest] {
            smallest = rc;
        }

        if index == smallest {
            return
        }
        
        self.items.swap(index, smallest);
        self.heapifyDown(smallest)
    }
}

// @lc code=end

