use std::collections::HashMap;

fn h(mut n: i32) -> i32 {
    let mut m = 0;
    while n > 0 {
        m += n % 10;
        n /= 10;
    }
    m
}

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0;
        for i in low_limit..=high_limit {
            let n = h(i);
            let ct = map.entry(n).or_insert(0);
            *ct += 1;
            ans = ans.max(*ct);
        }
        ans
    }
}
