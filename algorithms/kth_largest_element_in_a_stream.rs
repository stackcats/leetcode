// https://leetcode.com/problems/kth-largest-element-in-a-stream/

struct MinHeap {
    items: Box<Vec<i32>>,
    size: usize,
    capacity: usize,
}

impl MinHeap {
    fn new(cap: usize) -> Self {
        MinHeap {
            items: Box::new(vec![0; cap]),
            size: 0,
            capacity: cap,
        }
    }

    fn left_child_index(&self, parnet_index: usize) -> usize {
        parnet_index * 2 + 1
    }

    fn right_child_index(&self, parnet_index: usize) -> usize {
        parnet_index * 2 + 2
    }

    fn parent_index(&self, child_index: usize) -> usize {
        (child_index - 1) / 2
    }

    fn has_left_child(&self, index: usize) -> bool {
        self.left_child_index(index) < self.size
    }

    fn has_right_child(&self, index: usize) -> bool {
        self.right_child_index(index) < self.size
    }

    fn has_parnet(&self, index: usize) -> bool {
        index > 0
    }

    fn left_child(&self, index: usize) -> i32 {
        self.items[self.left_child_index(index)]
    }

    fn right_child(&self, index: usize) -> i32 {
        self.items[self.right_child_index(index)]
    }

    fn parnet(&self, index: usize) -> i32 {
        self.items[self.parent_index(index)]
    }

    fn swap(&mut self, i1: usize, i2: usize) {
        let t = self.items[i1];
        self.items[i1] = self.items[i2];
        self.items[i2] = t;
    }

    fn peek(&self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }

        Some(self.items[0])
    }

    fn add(&mut self, val: i32) {
        if self.size < self.capacity {
            self.items[self.size] = val;
            self.size += 1;
            self.heapify_up();
        } else if val > self.peek().unwrap() {
            self.items[0] = val;
            self.heapify_down();
        }
    }

    fn heapify_up(&mut self) {
        let mut index = self.size - 1;
        while self.has_parnet(index) && self.parnet(index) > self.items[index] {
            self.swap(index, self.parent_index(index));
            index = self.parent_index(index);
        }
    }

    fn heapify_down(&mut self) {
        let mut index = 0;
        while self.has_left_child(index) {
            let mut smaller_child_index = self.left_child_index(index);
            if self.has_right_child(index) {
                let right_child_index = self.right_child_index(index);
                if self.items[smaller_child_index] > self.items[right_child_index] {
                    smaller_child_index = right_child_index;
                }
            }

            if self.items[index] < self.items[smaller_child_index] {
                break;
            }

            self.swap(index, smaller_child_index);
            index = smaller_child_index;
        }
    }
}

struct KthLargest {
    heap: MinHeap,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = MinHeap::new(k as usize);
        for n in nums.iter() {
            heap.add(*n);
        }

        KthLargest { heap: heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.add(val);
        self.heap.peek().unwrap()
    }
}
