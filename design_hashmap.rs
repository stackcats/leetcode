// https://leetcode.com/problems/design-hashmap/

struct MyHashMap {
    arr: Box<Vec<i32>>,
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            arr: Box::new(vec![0; 1000000]),
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.arr[key as usize] = value + 1;
    }

    fn get(&self, key: i32) -> i32 {
        self.arr[key as usize] - 1
    }

    fn remove(&mut self, key: i32) {
        self.arr[key as usize] = 0;
    }
}
