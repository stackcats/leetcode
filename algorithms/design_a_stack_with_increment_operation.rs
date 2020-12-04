struct CustomStack {
    arr: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(maxSize: i32) -> Self {
        Self { arr: Vec::with_capacity(maxSize as usize) }
    }
    
    fn push(&mut self, x: i32) {
        if self.arr.len() < self.arr.capacity() {
            self.arr.push(x);
        }
    }
    
    fn pop(&mut self) -> i32 {
        self.arr.pop().unwrap_or(-1)
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        let mut i = 0;
        while i < self.arr.len() && i < (k as usize) {
            self.arr[i] += val;
            i += 1;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */
