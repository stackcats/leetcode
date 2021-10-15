struct MyCircularDeque {
    arr: Vec<i32>,
    front: usize,
    rear: usize,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            arr: vec![-1; k as usize],
            front: 0,
            rear: 0,
            size: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if !self.is_empty() {
            if self.front == 0 {
                self.front = self.arr.len() - 1;
            } else {
                self.front -= 1;
            }
        }

        self.arr[self.front] = value;

        self.size += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
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

    fn delete_front(&mut self) -> bool {
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

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.arr[self.rear] = -1;
        if self.rear == 0 {
            self.rear = self.arr.len() - 1;
        } else {
            self.rear -= 1;
        }
        self.size -= 1;
        if self.is_empty() {
            self.front = 0;
            self.rear = 0;
        }
        true
    }

    fn get_front(&self) -> i32 {
        self.arr[self.front]
    }

    fn get_rear(&self) -> i32 {
        self.arr[self.rear]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.arr.len()
    }
}
