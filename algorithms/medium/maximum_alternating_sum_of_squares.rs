impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut arr: Vec<_> = nums.into_iter().map(|n| (n * n) as i64).collect();
        arr.sort_unstable();
        let (a, b) = arr.split_at(arr.len() / 2);
        b.iter().sum::<i64>() - a.iter().sum::<i64>()
    }
}
