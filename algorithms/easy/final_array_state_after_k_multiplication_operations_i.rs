use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut hp = BinaryHeap::new();
        for (i, n) in nums.iter().enumerate() {
            hp.push(Reverse((*n, i)));
        }

        for _ in 0..k {
            let Reverse((n, i)) = hp.pop().unwrap();
            hp.push(Reverse((n * multiplier, i)));
        }

        let mut arr = vec![0; nums.len()];
        for Reverse((n, i)) in hp {
            arr[i] = n;
        }

        arr
    }
}
