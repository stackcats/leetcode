impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut times = 0.0;
        let mut start_time = 0;
        for c in &customers {
            start_time = start_time.max(c[0]);
            let finish_time = start_time + c[1];
            times += (finish_time - c[0]) as f64;
            start_time = finish_time;
        }
        times / (customers.len() as f64)
    }
}
