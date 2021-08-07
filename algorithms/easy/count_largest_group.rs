use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map = HashMap::new();
        let mut largest = 0;
        let mut ans = 0;
        for i in 1..=n {
            let mut t = i;
            let mut sum = 0;
            while t > 0 {
                sum += t % 10;
                t /= 10;
            }
            let counter = map.entry(sum.clone()).or_insert(0);
            *counter += 1;
            if *counter > largest {
                largest = *counter;
                ans = 1;
            } else if *counter == largest {
                ans += 1;
            }
        }
        ans
    }
}
