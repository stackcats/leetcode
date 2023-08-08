impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let mut sum = 0;
        let arr = [3, 5, 7];
        for i in (1..=n) {
            if arr.iter().any(|&j| i % j == 0) {
                sum += i;
            }
        }
        sum
    }
}
