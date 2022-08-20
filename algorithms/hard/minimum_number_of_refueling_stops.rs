use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {
        let mut pq = BinaryHeap::new();
        let mut tank = start_fuel;
        let mut ans = 0;
        stations.push(vec![target, 0]);
        for station in &stations {
            while tank < station[0] {
                if pq.is_empty() {
                    return -1;
                }
                tank += pq.pop().unwrap();
                ans += 1;
            }
            pq.push(station[1]);
        }
        ans
    }
}
