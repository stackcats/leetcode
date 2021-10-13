struct RLEIterator {
    encoding: Vec<i32>,
    curr: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RLEIterator {

    fn new(encoding: Vec<i32>) -> Self {
        Self {
            encoding,
            curr: 0,
        }
    }
    
    fn next(&mut self, n: i32) -> i32 {
        if self.curr >= self.encoding.len() {
            return -1;
        }
        self.encoding[self.curr] -= n;
        if self.encoding[self.curr] < 0 {
            let m = -self.encoding[self.curr];
            self.curr += 2;
            return self.next(m);
        }
        self.encoding[self.curr + 1]
    }
}

/**
 * Your RLEIterator object will be instantiated and called as such:
 * let obj = RLEIterator::new(encoding);
 * let ret_1: i32 = obj.next(n);
 */
