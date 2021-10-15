struct MyCircularQueue {
    arr: Vec<i32>,
    front: usize,
    rear: usize,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            arr: vec![-1; k as usize],
            front: 0,
            rear: 0,
            size: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if !self.is_empty() {
            self.rear = (self.rear + 1) % self.arr.len();
        }

        self.arr[self.rear] = value;
        self.size += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.arr[self.front] = -1;
        self.front = (self.front + 1) % self.arr.len();
        self.size -= 1;
        if self.is_empty() {
            self.front = 0;
            self.rear = 0;
        }
        true
    }

    fn front(&self) -> i32 {
        self.arr[self.front]
    }

    fn rear(&self) -> i32 {
        self.arr[self.rear]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.arr.len()
    }
}
