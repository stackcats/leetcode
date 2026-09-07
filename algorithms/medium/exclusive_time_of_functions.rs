impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        let mut st = Vec::new();
        let mut prev_t = -1;

        for log in logs {
            let v: Vec<_> = log.split(":").collect();
            let id: usize = v[0].parse().unwrap();
            let curr_t: i32 = v[2].parse().unwrap();

            if v[1] == "start" {
                if let Some(&prev_id) = st.last() {
                    ans[prev_id] += curr_t - prev_t - 1;
                }
                st.push(id);
            } else {
                st.pop();
                ans[id] += curr_t - prev_t + 1;
            }
            prev_t = curr_t;
        }

        ans
    }
}
