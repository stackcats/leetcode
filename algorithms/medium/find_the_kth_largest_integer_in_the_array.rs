use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Number(String);

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.cmp(&other.0))
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let mut hp = BinaryHeap::with_capacity(k as usize);
        nums.into_iter().for_each(|n| {
            let n = Number(n);
            if hp.len() < (k as usize) {
                hp.push(Reverse(n));
            } else if let Some(Reverse(top)) = hp.peek() {
                if &n > top {
                    hp.pop();
                    hp.push(Reverse(n));
                }
            }
        });
        let Reverse(Number(s)) = hp.pop().unwrap();
        s
    }
}
