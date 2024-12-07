use std::collections::VecDeque;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut mp = vec![vec![]; n];
        let mut degree = vec![0; n];
        for rel in relations {
            mp[rel[0] as usize - 1].push(rel[1] as usize - 1);
            degree[rel[1] as usize - 1] += 1;
        }

        let mut q = VecDeque::new();
        let mut min_times = vec![0; n];
        for i in 0..n {
            if degree[i] == 0 {
                q.push_back(i);
                min_times[i] = time[i];
            }
        }

        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let i = q.pop_front().unwrap();
                for &j in &mp[i] {
                    min_times[j] = min_times[j].max(min_times[i] + time[j]);
                    degree[j] -= 1;
                    if degree[j] == 0 {
                        q.push_back(j);
                    }
                }
            }
        }

        min_times.into_iter().max().unwrap()
    }
}
