use rand::prelude::*;

struct Solution {
    x_center: f64,
    y_center: f64,
    radius: f64
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            x_center, y_center, radius,
        }
    }
    
    fn rand_point(&self) -> Vec<f64> {
        loop {
            let x = rand::random::<f64>() * 2.0 - 1.0;
            let y = rand::random::<f64>() * 2.0 - 1.0;
            if x * x + y * y > 1.0 {
                continue;
            }

            return vec![x * self.radius + self.x_center, y * self.radius + self.y_center];
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */
