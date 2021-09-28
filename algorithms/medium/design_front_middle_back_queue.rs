use std::collections::VecDeque;

#[derive(Debug)]
struct FrontMiddleBackQueue {
    left: VecDeque<i32>,
    right: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {

    fn new() -> Self {
        Self {
            left: VecDeque::new(),
            right: VecDeque::new(),
        }
    }
    
    fn push_front(&mut self, val: i32) {
        self.left.push_front(val);
        self.rebalance();
    }
    
    fn push_middle(&mut self, val: i32) {
        if self.left.len() == self.right.len() {
            self.left.push_back(val);
        } else if self.left.len() < self.right.len() {
            self.left.push_back(val);
        } else {
            let v = self.left.pop_back().unwrap();
            self.right.push_front(v);
            self.left.push_back(val);
        }

    }
    
    fn push_back(&mut self, val: i32) {
        self.right.push_back(val);
        self.rebalance();
    }
    
    fn pop_front(&mut self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        
        if self.left.is_empty() {
            return self.right.pop_front().unwrap();
        }
        let v = self.left.pop_front().unwrap();
        self.rebalance();    
        v
    }
    
    fn pop_middle(&mut self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        if self.left.len() >= self.right.len() {
            return self.left.pop_back().unwrap();
        }

        self.right.pop_front().unwrap()
    }
    
    fn pop_back(&mut self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        if self.right.len() > 0 {
           let v = self.right.pop_back().unwrap();
            self.rebalance();
            return v; 
        }
        self.left.pop_back().unwrap()      
    }
    
    #[inline]
    fn is_empty(&self) -> bool {
        self.left.is_empty() && self.right.is_empty()
    }
    
    fn rebalance(&mut self) {
        if self.left.len() == self.right.len() {
            return;
        }
        if self.left.len() < self.right.len() {
            let v = self.right.pop_front().unwrap();
            self.left.push_back(v);
        } else if self.left.len() - 1 > self.right.len() {
            let v = self.left.pop_back().unwrap();
            self.right.push_front(v);
        }
    }
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */
