use std::collections::{HashSet, BinaryHeap};
use std::cmp::Reverse;

#[derive(Default)]
struct SmallestInfiniteSet {
    smallest: i32,
    set: HashSet<i32>,
    pq: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        Self {
            smallest: 1,
            ..Default::default()
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {        
        if !self.pq.is_empty() {
            let n = self.pq.pop().unwrap().0;
            self.set.remove(&n);
            return n;
        }

        self.smallest += 1;
        
        self.smallest - 1
    }
    
    fn add_back(&mut self, num: i32) {
        if num < self.smallest && !self.set.contains(&num) {
            self.pq.push(Reverse(num));
            self.set.insert(num);
        }        
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
