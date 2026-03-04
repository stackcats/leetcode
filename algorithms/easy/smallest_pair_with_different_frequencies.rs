impl Solution {
    pub fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
        let mut mp = vec![0; 102];
        for &n in &nums {
            mp[n as usize] += 1;
        }

        for i in 0..mp.len() {
            if mp[i] == 0 {
                continue;
            }
            for j in (i + 1)..mp.len() {
                if mp[j] != 0 && mp[j] != mp[i] {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![-1, -1]
    }
}
