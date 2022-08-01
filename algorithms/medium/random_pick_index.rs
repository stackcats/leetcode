use std::collections::{HashMap, VecDeque};

struct Solution {
    nums: HashMap<i32, VecDeque<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            map.entry(n).or_insert(VecDeque::new()).push_back(i as i32);
        }
        Self { nums: map }
    }
    
    fn pick(&mut self, target: i32) -> i32 {
        self.nums.entry(target).and_modify(|q| q.rotate_left(1));
        let n = self.nums[&target].front().unwrap();
        *n
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
