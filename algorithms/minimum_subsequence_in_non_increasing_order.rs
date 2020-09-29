impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by(|a, b| b.cmp(a));
        let half = nums.iter().fold(0, |acc, x| acc + x) / 2;
        let mut arr = Vec::new();
        let mut sum = 0;
        for n in &nums {
            sum += *n;
            arr.push(*n);
            if sum > half {
                break;
            }
        }
        arr
    }
}
