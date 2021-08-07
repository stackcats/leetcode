use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    maxHeap: BinaryHeap<i32>,
    minHeap: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            maxHeap: BinaryHeap::new(),
            minHeap: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        if self.maxHeap.is_empty() {
            self.maxHeap.push(num);
            return;
        }

        let n = self.maxHeap.peek().unwrap();
        if *n > num {
            self.maxHeap.push(num);
            if self.maxHeap.len() - self.minHeap.len() > 1 {
                let num = self.maxHeap.pop().unwrap();
                self.minHeap.push(Reverse(num));
            }
        } else {
            self.minHeap.push(Reverse(num));
            if self.maxHeap.len() < self.minHeap.len() {
                let Reverse(num) = self.minHeap.pop().unwrap();
                self.maxHeap.push(num);
            }
        }

    }
    
    fn find_median(&self) -> f64 {
        if self.maxHeap.is_empty() {
            return 0.0;
        }
        
        let n = self.maxHeap.peek().unwrap();
        let n = *n as f64;
        if self.maxHeap.len() > self.minHeap.len() {
            return n;
        }
        
        let Reverse(m) = self.minHeap.peek().unwrap();
        let m = *m as f64;
        (n + m) / 2.0
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
