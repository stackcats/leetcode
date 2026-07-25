impl Solution {
    pub fn min_adjacent_swaps(nums: Vec<i32>, a: i32, b: i32) -> i32 {
        let m = 1000000000 + 7;
        let mut ct1 = 0;
        let mut ct2 = 0;
        let mut ans = 0;
        for n in nums {
            if n < a {
                ans = (ans + ct1 + ct2) % m
            } else if n <= b {
                ans = (ans + ct2) % m;
                ct1 += 1;
            } else {
                ct2 += 1;
            }
        }

        ans
    }
}
