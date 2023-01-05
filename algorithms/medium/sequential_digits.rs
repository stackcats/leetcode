impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 1..=9 {
            let mut n = i;
            while n < low && n % 10 != 0 {
                n = n * 10 + n % 10 + 1;
            }
            while n <= high && n % 10 != 0 {
                ans.push(n);
                n = n * 10 + n % 10 + 1;
            }
        }
        ans.sort();
        ans
    }
}
