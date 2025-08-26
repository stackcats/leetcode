impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        for q in queries {
            let mut l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as usize;
            let v = q[3] as i64;
            while l <= r {
                nums[l] = ((nums[l] as i64 * v) % 1000000007) as i32;
                l += k;
            }
        }

        nums.iter().fold(0, |acc, n| acc ^ n)
    }
}
