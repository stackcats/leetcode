use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut q = VecDeque::new();
        for i in 0..is_water.len() {
            for j in 0..is_water[0].len() {
                if is_water[i][j] == 1 {
                    q.push_back((i, j));
                    is_water[i][j] = 0;
                } else {
                    is_water[i][j] = -1;
                }
            }
        }

        let mut h = 1;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let (i, j) = q.pop_front().unwrap();
                for (i, j) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                    if i < is_water.len() && j < is_water[0].len() && is_water[i][j] < 0 {
                        q.push_back((i, j));
                        is_water[i][j] = h;
                    }
                }
            }
            h += 1;
        }

        is_water
    }
}

