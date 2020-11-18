struct OrderedStream {
    cur: usize,
    len: usize,
    arr: Vec<Option<String>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {

    fn new(n: i32) -> Self {
        OrderedStream {
            cur: 0,
            len: n as usize,
            arr: vec![None; n as usize]
        }
    }
    
    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        self.arr[id as usize - 1] = Some(value);
        let mut ans = Vec::new();
        while self.cur < self.len && self.arr[self.cur].is_some() {
            let s = self.arr[self.cur].take();
            ans.push(s.unwrap());
            self.cur += 1;
        }
        ans
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(id, value);
 */
