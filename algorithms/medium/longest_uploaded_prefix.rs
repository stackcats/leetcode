struct LUPrefix {
    ma: usize,
    arr: Vec<bool>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LUPrefix {

    fn new(n: i32) -> Self {
        Self {
            ma: 0,
            arr: vec![false; n as usize],
        }
    }
    

    fn upload(&mut self, video: i32) {
        self.arr[video as usize - 1] = true;
    }


    fn longest(&mut self) -> i32 {
        while self.ma < self.arr.len() && self.arr[self.ma] {
            self.ma += 1;
        }
        self.ma as _
    }
}

/**
 * Your LUPrefix object will be instantiated and called as such:
 * let obj = LUPrefix::new(n);
 * obj.upload(video);
 * let ret_2: i32 = obj.longest();
 */
