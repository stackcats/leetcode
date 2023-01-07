impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut diff = vec![0; gas.len()];
        let mut total = 0;
        for i in 0..gas.len() {
            diff[i] = gas[i] - cost[i];
            total += diff[i];
        }
        if total < 0 {
            return -1;
        }
        let mut ans = 0;
        let mut g = 0;
        for (i, d) in diff.into_iter().enumerate() {
            if g < 0 {
                ans = i;
                g = d;
            } else {
                g += d;
            }
        }
        ans as _
    }
}
