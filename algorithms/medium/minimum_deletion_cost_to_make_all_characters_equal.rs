impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i64 {
        let mut ct = [0; 26];
        let mut total = 0;
        for (i, c) in s.chars().enumerate() {
            ct[((c as u8) - b'a') as usize] += cost[i] as i64;
            total += cost[i] as i64;
        }

        let mut ma = 0;
        for i in 0..26 {
            ma = ma.max(ct[i]);
        }

        total - ma
    }
}
