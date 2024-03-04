impl Solution {
    pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
        let mut prefix = 0i64;
        let mut ma = 0i64;
        let mut arr = vec![0i64; nums.len()];
        for (i, n) in nums.into_iter().enumerate() {
            ma = ma.max(n as i64);
            let c = ma + n as i64;
            arr[i] = prefix + c;
            prefix = arr[i]
        }
        arr
    }
}
