impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut ans = 0;
        for d in details {
            let age = &d[11..=12].parse::<i32>().unwrap();
            if *age > 60 {
                ans += 1;
            }
        }
        ans
    }
}
