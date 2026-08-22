use rand::prelude::*;

struct Solution {
    p: Vec<f64>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        let sum = w.iter().sum::<i32>() as f64;
        Self {
            p: w.into_iter().map(|v| (v as f64 / sum)).collect(),
        }
    }
    
    fn pick_index(&self) -> i32 {
        let mut x: f64 = rand::random();
        for i in 0..self.p.len() {
            if x < self.p[i] {
                return i as _;
            }
            x -= self.p[i];
        }
        0
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
