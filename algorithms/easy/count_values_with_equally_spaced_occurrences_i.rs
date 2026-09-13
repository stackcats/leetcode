impl Solution {
    pub fn count_special_integers(nums: Vec<i32>) -> i32 {
        let mut mp = vec![vec![]; 102];
        for (i, n) in nums.into_iter().enumerate() {
            mp[n as usize].push(i);
        }

        let mut ans = 0;
        for i in 0..mp.len() {
            let [i, j, k] = mp[i].as_slice() else {
                continue;
            };
            if j - i == k - j {
                ans += 1;
            }
        }
        ans
    }
}
