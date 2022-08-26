use std::collections::HashMap;

fn count(mut n: i32) -> HashMap<i32, i32> {
    let mut ct = HashMap::new();
    while n > 0 {
        *ct.entry(n % 10).or_insert(0) += 1;
        n /= 10;
    }
    ct
}

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let m = count(n);
        (0..=32).any(|i| count(2_i32.pow(i)) == m)
    }
}
