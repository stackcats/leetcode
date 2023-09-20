fn count(mut i: i32) -> i32 {
    let mut ct = 0;
    while i > 0 {
        ct += i % 2;
        i /= 2;
    }
    ct
}

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        for (i, n) in nums.into_iter().enumerate() {
            if count(i as i32) == k {
                sum += n;
            }
        }
        sum
    }
}
