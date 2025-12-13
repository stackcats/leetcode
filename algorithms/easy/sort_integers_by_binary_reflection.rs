impl Solution {
    pub fn sort_by_reflection(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|n| {
            let mut r = 0;
            let mut m = *n;
            while m > 0 {
                r = r * 2 + m % 2;
                m /= 2;
            }

            (r, *n)
        });
        nums
    }
}
