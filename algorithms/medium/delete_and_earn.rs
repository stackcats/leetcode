impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut mp = vec![0; 10001];
        for n in nums {
            mp[n as usize] += n;
        }

        for i in 2..mp.len() {
            mp[i] = (mp[i] + mp[i - 2]).max(mp[i - 1]);
        }

        mp.last().copied().unwrap()
    }
}
