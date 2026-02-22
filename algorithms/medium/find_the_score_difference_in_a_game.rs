impl Solution {
    pub fn score_difference(nums: Vec<i32>) -> i32 {
        let mut p1 = true;
        let mut ans = 0;

        for (i, n) in nums.into_iter().enumerate() {
            if n % 2 == 1 {
                p1 = !p1;
            }

            if i % 6 == 5 {
                p1 = !p1;
            }

            if p1 {
                ans += n;
            } else {
                ans -= n;
            }
        }

        ans
    }
}
