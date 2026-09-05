impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut ma = 0;
        let mut ma_ct = 0;
        let mut mp = vec![0; 26];
        let total_task = tasks.len() as i32;

        for t in tasks {
            let i = (t as u8 - b'A') as usize;
            mp[i] += 1;

            if ma < mp[i] {
                ma = mp[i];
                ma_ct = 1;
            } else if ma == mp[i] {
                ma_ct += 1;
            }
        }

        let gaps = ma - 1;
        let run_tasks = ma * ma_ct;
        let slots = (n - ma_ct + 1) * gaps;
        let idles = 0.max(slots - (total_task - run_tasks));
        total_task + idles
    }
}
