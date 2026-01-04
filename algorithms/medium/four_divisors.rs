use std::collections::HashSet;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for n in nums {
            let mut m = (n as f64).sqrt() as i32 + 1;
            let mut divs = HashSet::new();
            for i in 1..m {
                if n % i == 0 {
                    divs.insert(i);
                    divs.insert(n / i);
                }
            }

            if divs.len() == 4 {
                ans += divs.iter().sum::<i32>();
            }
        }

        ans
    }
}
