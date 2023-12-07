impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut ans = divisors[0];
        let mut max = 0;
        for d in divisors {
            let mut ct = 0;
            for n in &nums {
                if n % d == 0 {
                    ct += 1;
                }
            }
            if max < ct {
                max = ct;
                ans = d;
            } else if max == ct {
                ans = ans.min(d);
            }
        }
        ans
    }
}
