use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut pq = BinaryHeap::new();
        let mut usedBricks = 0;
        for i in 1..heights.len() {
            let diff = heights[i] - heights[i-1];
            if diff <= 0 {
                continue;
            }
            pq.push(Reverse(diff));           
            if pq.len() > ladders as usize {
                usedBricks += pq.pop().unwrap().0;
                if usedBricks > bricks {
                    return (i - 1) as i32;
                }
            }
        }

        (heights.len() - 1) as i32
    }
}
